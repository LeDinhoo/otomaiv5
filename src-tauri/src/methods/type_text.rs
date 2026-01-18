use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, 
    KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VIRTUAL_KEY
};

pub fn type_text_fast(text: &str) -> Result<String, String> {
    unsafe {
        for c in text.chars() {
            // On envoie le caractère (Down)
            send_unicode_char(c, true);
            // On relâche le caractère immédiatement (Up)
            send_unicode_char(c, false);
        }
    }
    Ok(format!("Texte envoyé avec succès : {}", text))
}

unsafe fn send_unicode_char(c: char, is_down: bool) {
    let mut flags = KEYEVENTF_UNICODE;
    if !is_down {
        flags |= KEYEVENTF_KEYUP;
    }

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: c as u16,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
}