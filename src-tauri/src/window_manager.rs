use windows::core::{HSTRING, PCWSTR};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_MENU,
};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, IsIconic, SetForegroundWindow, ShowWindow, SW_RESTORE,
};

pub struct WindowManager;

impl WindowManager {
    pub fn new() -> Self {
        Self
    }

    pub fn focus_by_title(&self, title: &str) -> Result<String, String> {
        unsafe {
            let window_title = HSTRING::from(title);
            let title_pcwstr = PCWSTR::from_raw(window_title.as_ptr());

            let result = FindWindowW(None, title_pcwstr);

            let hwnd = match result {
                Ok(h) if h.0.is_null() => {
                    return Err(format!(
                        "Fenêtre introuvable : '{}'. Vérifiez le titre exact.",
                        title
                    ));
                }
                Ok(h) => h,
                Err(e) => return Err(format!("Erreur système Win32 : {}", e)),
            };

            if IsIconic(hwnd).as_bool() {
                let _ = ShowWindow(hwnd, SW_RESTORE);
            }

            // Trick ALT pour débloquer SetForegroundWindow
            let alt_down = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_MENU,
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            let alt_up = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_MENU,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            SendInput(&[alt_down, alt_up], std::mem::size_of::<INPUT>() as i32);

            let success = SetForegroundWindow(hwnd);

            if success.as_bool() {
                Ok(format!(
                    "Succès : '{}' est maintenant au premier plan.",
                    title
                ))
            } else {
                Err("Le focus a été refusé par Windows.".to_string())
            }
        }
    }
}
