use device_query::{DeviceQuery, DeviceState, MouseState};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, RECT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetForegroundWindow, GetWindowRect, GetWindowTextW, PostMessageW,
    WM_LBUTTONDOWN, WM_LBUTTONUP,
};

pub struct ClickMirrorState {
    pub active: bool,
    pub leader_title: String,
    pub follower_titles: Vec<String>,
}

pub type SharedClickMirrorState = Arc<Mutex<ClickMirrorState>>;

#[tauri::command]
pub fn set_click_mirror(
    state: tauri::State<'_, SharedClickMirrorState>,
    active: bool,
    leader_title: String,
    follower_titles: Vec<String>,
) -> Result<String, String> {
    let mut data = state.lock().map_err(|_| "Failed to lock state")?;
    data.active = active;
    data.leader_title = leader_title;
    data.follower_titles = follower_titles;

    Ok(if active {
        format!("Click mirror activé ({} suiveurs)", data.follower_titles.len())
    } else {
        "Click mirror désactivé".to_string()
    })
}

pub fn init_click_mirror(state: SharedClickMirrorState) {
    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut prev_left = false;

        loop {
            let (active, leader, followers) = {
                let lock = state.lock().unwrap();
                (
                    lock.active,
                    lock.leader_title.clone(),
                    lock.follower_titles.clone(),
                )
            };

            if active && !leader.is_empty() && !followers.is_empty() {
                let mouse: MouseState = device_state.get_mouse();
                let left_pressed = mouse.button_pressed.get(1).copied().unwrap_or(false);

                // Détection front montant (clic)
                if left_pressed && !prev_left {
                    if is_leader_focused(&leader) {
                        let (mx, my) = mouse.coords;
                        if let Some((rx, ry)) = get_relative_pos(&leader, mx, my) {
                            for title in &followers {
                                send_click_to_window(title, rx, ry);
                            }
                        }
                    }
                }

                prev_left = left_pressed;
            } else {
                prev_left = false;
            }

            thread::sleep(Duration::from_millis(15));
        }
    });
}

fn is_leader_focused(leader_title: &str) -> bool {
    unsafe {
        let fg = GetForegroundWindow();
        if fg == HWND::default() {
            return false;
        }
        let mut buf = [0u16; 512];
        let len = GetWindowTextW(fg, &mut buf);
        if len == 0 {
            return false;
        }
        let title = String::from_utf16_lossy(&buf[..len as usize]);
        title == leader_title
    }
}

fn get_relative_pos(title: &str, screen_x: i32, screen_y: i32) -> Option<(i32, i32)> {
    unsafe {
        let htitle = HSTRING::from(title);
        let hwnd = FindWindowW(None, PCWSTR::from_raw(htitle.as_ptr())).ok()?;
        if hwnd.0.is_null() {
            return None;
        }
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect).ok()?;
        Some((screen_x - rect.left, screen_y - rect.top))
    }
}

fn send_click_to_window(title: &str, rel_x: i32, rel_y: i32) {
    unsafe {
        let htitle = HSTRING::from(title);
        let hwnd = match FindWindowW(None, PCWSTR::from_raw(htitle.as_ptr())) {
            Ok(h) if !h.0.is_null() => h,
            _ => return,
        };

        let lparam = LPARAM(((rel_y as u32 & 0xFFFF) << 16 | (rel_x as u32 & 0xFFFF)) as isize);

        let _ = PostMessageW(hwnd, WM_LBUTTONDOWN, WPARAM(1), lparam);
        thread::sleep(Duration::from_millis(30));
        let _ = PostMessageW(hwnd, WM_LBUTTONUP, WPARAM(0), lparam);
    }
}
