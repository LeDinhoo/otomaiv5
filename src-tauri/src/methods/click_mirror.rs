use device_query::{DeviceQuery, DeviceState, Keycode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, FindWindowW, GetCursorPos, GetForegroundWindow, GetMessageW,
    GetWindowRect, GetWindowTextW, PostMessageW, SetWindowsHookExW,
    KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_LBUTTONDOWN, WM_LBUTTONUP,
};
use crate::window_manager::WindowManager;
use tauri::{AppHandle, Emitter};

const WM_KEYDOWN: u32 = 0x0100;
const VK_F11: u32 = 0x7A;

pub struct ClickMirrorState {
    pub paused: bool,
    pub titles: Vec<String>,
    pub focus_keybinds: HashMap<String, String>, // key name -> window title
}

pub type SharedClickMirrorState = Arc<Mutex<ClickMirrorState>>;

#[tauri::command]
pub fn set_click_mirror(
    state: tauri::State<'_, SharedClickMirrorState>,
    titles: Vec<String>,
) -> Result<String, String> {
    let mut data = state.lock().map_err(|_| "Failed to lock state")?;
    data.titles = titles;
    Ok(format!("Click mirror: {} fenêtres", data.titles.len()))
}

#[tauri::command]
pub fn pause_click_mirror(
    state: tauri::State<'_, SharedClickMirrorState>,
    paused: bool,
) -> Result<String, String> {
    let mut data = state.lock().map_err(|_| "Failed to lock state")?;
    data.paused = paused;
    Ok(if paused {
        "Click mirror en pause".to_string()
    } else {
        "Click mirror repris".to_string()
    })
}

#[tauri::command]
pub fn set_focus_keybinds(
    state: tauri::State<'_, SharedClickMirrorState>,
    keybinds: HashMap<String, String>,
) -> Result<String, String> {
    let mut data = state.lock().map_err(|_| "Failed to lock state")?;
    data.focus_keybinds = keybinds;
    Ok(format!("Focus keybinds: {} raccourcis", data.focus_keybinds.len()))
}

fn find_hwnd_by_title(title: &str) -> Option<HWND> {
    unsafe {
        let window_title = HSTRING::from(title);
        let title_pcwstr = PCWSTR::from_raw(window_title.as_ptr());
        match FindWindowW(None, title_pcwstr) {
            Ok(h) if !h.0.is_null() => Some(h),
            _ => None,
        }
    }
}

fn get_window_rect(hwnd: HWND) -> Option<RECT> {
    unsafe {
        let mut rect = RECT::default();
        if GetWindowRect(hwnd, &mut rect).is_ok() {
            Some(rect)
        } else {
            None
        }
    }
}

fn post_click_to_window(hwnd: HWND, rel_x: i32, rel_y: i32) {
    unsafe {
        let lparam = LPARAM((rel_y << 16 | (rel_x & 0xFFFF)) as isize);
        let wparam = WPARAM(0x0001); // MK_LBUTTON
        let _ = PostMessageW(hwnd, WM_LBUTTONDOWN, wparam, lparam);
        thread::sleep(Duration::from_millis(10));
        let _ = PostMessageW(hwnd, WM_LBUTTONUP, WPARAM(0), lparam);
    }
}

/// Convertit un nom de touche (depuis le frontend) en Keycode device_query
fn key_name_to_keycode(name: &str) -> Option<Keycode> {
    match name.to_uppercase().as_str() {
        "F1" => Some(Keycode::F1),
        "F2" => Some(Keycode::F2),
        "F3" => Some(Keycode::F3),
        "F4" => Some(Keycode::F4),
        "F5" => Some(Keycode::F5),
        "F6" => Some(Keycode::F6),
        "F7" => Some(Keycode::F7),
        "F8" => Some(Keycode::F8),
        "F9" => Some(Keycode::F9),
        "F10" => Some(Keycode::F10),
        "F11" => Some(Keycode::F11),
        "F12" => Some(Keycode::F12),
        "1" | "DIGIT1" => Some(Keycode::Key1),
        "2" | "DIGIT2" => Some(Keycode::Key2),
        "3" | "DIGIT3" => Some(Keycode::Key3),
        "4" | "DIGIT4" => Some(Keycode::Key4),
        "5" | "DIGIT5" => Some(Keycode::Key5),
        "6" | "DIGIT6" => Some(Keycode::Key6),
        "7" | "DIGIT7" => Some(Keycode::Key7),
        "8" | "DIGIT8" => Some(Keycode::Key8),
        "9" | "DIGIT9" => Some(Keycode::Key9),
        "0" | "DIGIT0" => Some(Keycode::Key0),
        "NUMPAD1" => Some(Keycode::Numpad1),
        "NUMPAD2" => Some(Keycode::Numpad2),
        "NUMPAD3" => Some(Keycode::Numpad3),
        "NUMPAD4" => Some(Keycode::Numpad4),
        "NUMPAD5" => Some(Keycode::Numpad5),
        "NUMPAD6" => Some(Keycode::Numpad6),
        "NUMPAD7" => Some(Keycode::Numpad7),
        "NUMPAD8" => Some(Keycode::Numpad8),
        "NUMPAD9" => Some(Keycode::Numpad9),
        "NUMPAD0" => Some(Keycode::Numpad0),
        _ => None,
    }
}

fn get_window_title(hwnd: HWND) -> Option<String> {
    unsafe {
        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buf);
        if len > 0 {
            Some(String::from_utf16_lossy(&buf[..len as usize]))
        } else {
            None
        }
    }
}

// --- Hook clavier F11 : détection instantanée, zéro polling ---

struct ClickEvent {
    rel_x: i32,
    rel_y: i32,
    source_hwnd: isize, // HWND stocké comme isize pour être Send
}

thread_local! {
    static HOOK_SENDER: RefCell<Option<mpsc::Sender<ClickEvent>>> = RefCell::new(None);
}

unsafe extern "system" fn keyboard_hook_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code >= 0 && wparam.0 == WM_KEYDOWN as usize {
        let kb_info = &*(lparam.0 as *const KBDLLHOOKSTRUCT);

        if kb_info.vkCode == VK_F11 {
            let fg_hwnd = GetForegroundWindow();
            let mut cursor = POINT::default();
            let _ = GetCursorPos(&mut cursor);

            if let Some(src_rect) = get_window_rect(fg_hwnd) {
                let rel_x = cursor.x - src_rect.left;
                let rel_y = cursor.y - src_rect.top;

                HOOK_SENDER.with(|s| {
                    if let Some(tx) = s.borrow().as_ref() {
                        let _ = tx.send(ClickEvent {
                            rel_x,
                            rel_y,
                            source_hwnd: fg_hwnd.0 as isize,
                        });
                    }
                });
            }
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

pub fn init_click_mirror(state: SharedClickMirrorState, app_handle: AppHandle) {
    let (tx, rx) = mpsc::channel::<ClickEvent>();
    let state_for_consumer = state.clone();

    // Thread 1 : Hook clavier bas-niveau + pompe à messages
    // Capture F11 instantanément via callback OS → push dans le channel
    thread::spawn(move || {
        HOOK_SENDER.with(|s| {
            *s.borrow_mut() = Some(tx);
        });

        unsafe {
            let _hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook_proc), None, 0)
                .expect("Impossible d'installer le hook clavier");

            let mut msg = MSG::default();
            while GetMessageW(&mut msg, HWND::default(), 0, 0).0 > 0 {
                // La pompe à messages maintient le hook en vie
            }
        }
    });

    // Thread 2 : Consommateur de clicks (bloque sur recv = traitement instantané)
    thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            let (paused, titles) = {
                let lock = state_for_consumer.lock().unwrap();
                (lock.paused, lock.titles.clone())
            };

            if !paused && titles.len() > 1 {
                let source = HWND(event.source_hwnd as *mut _);
                let is_tracked = titles.iter().any(|t| {
                    find_hwnd_by_title(t).map_or(false, |h| h == source)
                });

                if is_tracked {
                    for title in &titles {
                        if let Some(target_hwnd) = find_hwnd_by_title(title) {
                            post_click_to_window(target_hwnd, event.rel_x, event.rel_y);
                        }
                    }
                }
            }
        }
    });

    // Thread 3 : Focus tracking + keybinds (polling léger, pas time-critical)
    thread::spawn(move || {
        let device_state = DeviceState::new();
        let wm = WindowManager::new();
        let mut prev_keys: Vec<Keycode> = vec![];
        let mut prev_focus_title: Option<String> = None;

        loop {
            let (titles, focus_keybinds) = {
                let lock = state.lock().unwrap();
                (lock.titles.clone(), lock.focus_keybinds.clone())
            };

            // --- Focus tracking (émet focus-changed quand la fenêtre active change) ---
            if !titles.is_empty() {
                unsafe {
                    let fg_hwnd = GetForegroundWindow();
                    let is_tracked = titles.iter().any(|t| {
                        find_hwnd_by_title(t).map_or(false, |h| h == fg_hwnd)
                    });

                    let current_title = if is_tracked {
                        get_window_title(fg_hwnd)
                    } else {
                        None
                    };

                    if current_title != prev_focus_title {
                        prev_focus_title = current_title.clone();
                        let _ = app_handle.emit("focus-changed", current_title.unwrap_or_default());
                    }
                }
            }

            // --- Focus keybinds (toujours actif, même si paused) ---
            if !focus_keybinds.is_empty() {
                let current_keys = device_state.get_keys();

                for (key_name, window_title) in &focus_keybinds {
                    if let Some(keycode) = key_name_to_keycode(key_name) {
                        if current_keys.contains(&keycode) && !prev_keys.contains(&keycode) {
                            let _ = wm.focus_by_title(window_title);
                        }
                    }
                }

                prev_keys = current_keys;
            } else {
                prev_keys.clear();
            }

            thread::sleep(Duration::from_millis(30));
        }
    });
}
