use enigo::{Direction, Enigo, Key as EnigoKey, Keyboard, Settings};
use std::thread;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    MapVirtualKeyW, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, MAP_VIRTUAL_KEY_TYPE, VIRTUAL_KEY,
};

pub fn press_key_multiple_times(key_str: &str, count: u32) -> Result<String, String> {
    // On initialise Enigo pour les caractères spéciaux
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

    for _ in 0..count {
        match key_str.to_lowercase().as_str() {
            // 1. Touches critiques : On utilise les Scancodes natifs (très robuste pour les jeux)
            "enter" | "esc" | "escape" | "space" | "tab" | "backspace" | "left" | "up"
            | "right" | "down" => {
                let vk = match key_str.to_lowercase().as_str() {
                    "enter" => 0x0D,
                    "esc" | "escape" => 0x1B,
                    "space" => 0x20,
                    "tab" => 0x09,
                    "backspace" => 0x08,
                    "left" => 0x25,
                    "up" => 0x26,
                    "right" => 0x27,
                    "down" => 0x28,
                    _ => 0,
                };
                let extended = matches!(
                    key_str.to_lowercase().as_str(),
                    "left" | "up" | "right" | "down"
                );
                unsafe {
                    send_native_scancode(vk, true, extended);
                    thread::sleep(Duration::from_millis(50));
                    send_native_scancode(vk, false, extended);
                }
            }

            // 2. Lettres et symboles (; ' [ ] ! @ etc.) : On utilise Enigo
            // C'est beaucoup plus simple car Enigo gère le shift/alt-gr tout seul
            s => {
                // Si c'est une seule lettre ou un symbole
                if s.len() == 1 {
                    let _ = enigo.key(
                        EnigoKey::Unicode(s.chars().next().unwrap()),
                        Direction::Click,
                    );
                } else {
                    // Si c'est un mot complet
                    let _ = enigo.text(s);
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    Ok(format!("Action '{}' effectuée {} fois.", key_str, count))
}

// Fonction pour l'envoi natif par Scancode (inchangée)
unsafe fn send_native_scancode(vk: u16, is_down: bool, extended: bool) {
    let scan_code = MapVirtualKeyW(vk as u32, MAP_VIRTUAL_KEY_TYPE(0)) as u16;
    let mut flags = KEYEVENTF_SCANCODE;
    if !is_down {
        flags |= KEYEVENTF_KEYUP;
    }
    if extended {
        flags |= KEYEVENTF_EXTENDEDKEY;
    }

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: scan_code,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
}
