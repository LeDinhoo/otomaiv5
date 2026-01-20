use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::{BOOL, HWND, POINT, LPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT,
};
use windows::Win32::Graphics::Gdi::ClientToScreen;

use windows::Win32::UI::WindowsAndMessaging::{
    SetCursorPos, SetForegroundWindow, ShowWindow, SW_RESTORE,
    EnumWindows, GetWindowTextW, IsWindowVisible
};

pub struct MouseManager;

impl MouseManager {
    pub fn new() -> Self {
        Self
    }

    pub fn click_at_position(&self, window_title: &str, x: i32, y: i32) -> Result<String, String> {
        unsafe {
            let hwnd = self.find_window_by_title(window_title)
                .ok_or_else(|| format!("Fenêtre '{}' introuvable", window_title))?;

            let _ = ShowWindow(hwnd, SW_RESTORE);
            SetForegroundWindow(hwnd);
            thread::sleep(Duration::from_millis(50));

            let mut point = POINT { x, y };
            if !ClientToScreen(hwnd, &mut point).as_bool() {
                return Err("Impossible de calculer les coordonnées d'écran.".to_string());
            }

            // Double Tap pour fixer le multi-écran
            SetCursorPos(point.x, point.y).map_err(|e| e.to_string())?;
            thread::sleep(Duration::from_millis(50));
            SetCursorPos(point.x, point.y).map_err(|e| e.to_string())?;

            self.send_left_click();

            Ok(format!("Clic effectué à [{}, {}] (Global: {}, {})", x, y, point.x, point.y))
        }
    }

    unsafe fn send_left_click(&self) {
        let input_down = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0, dy: 0, mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTDOWN,
                    time: 0, dwExtraInfo: 0,
                },
            },
        };

        let input_up = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0, dy: 0, mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTUP,
                    time: 0, dwExtraInfo: 0,
                },
            },
        };

        SendInput(&[input_down], std::mem::size_of::<INPUT>() as i32);
        thread::sleep(Duration::from_millis(50));
        SendInput(&[input_up], std::mem::size_of::<INPUT>() as i32);
    }

    unsafe fn find_window_by_title(&self, partial_title: &str) -> Option<HWND> {
        struct SearchContext { title: String, hwnd: Option<HWND> }

        unsafe extern "system" fn enum_window_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let context = &mut *(lparam.0 as *mut SearchContext);
            if IsWindowVisible(hwnd).as_bool() {
                let mut buffer = [0u16; 512];
                let len = GetWindowTextW(hwnd, &mut buffer);
                if len > 0 {
                    let window_title = String::from_utf16_lossy(&buffer[..len as usize]);
                    if window_title.contains(&context.title) {
                        context.hwnd = Some(hwnd);
                        return BOOL::from(false);
                    }
                }
            }
            BOOL::from(true)
        }

        let mut context = SearchContext { title: partial_title.to_string(), hwnd: None };
        let _ = EnumWindows(Some(enum_window_callback), LPARAM(&mut context as *mut SearchContext as isize));
        context.hwnd
    }
}