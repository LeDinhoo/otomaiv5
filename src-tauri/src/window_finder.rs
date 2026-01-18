use regex::Regex;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible};

struct SearchContext {
    regex: Regex,
    result: Option<String>,
}

pub fn find_specific_game_window(character_name: &str) -> Option<String> {
    // Le regex devient : ^NomDuPerso - .* - \d+(\.\d+)* - Release$
    // On utilise regex::escape pour éviter les erreurs si le nom a des caractères spéciaux
    let pattern = format!(
        r"^{} - .* - \d+(\.\d+)* - Release$",
        regex::escape(character_name)
    );

    let Ok(re) = Regex::new(&pattern) else {
        return None;
    };

    let mut context = SearchContext {
        regex: re,
        result: None,
    };

    unsafe {
        let _ = EnumWindows(
            Some(enumerate_callback),
            LPARAM(&mut context as *mut SearchContext as isize),
        );
    }

    context.result
}

unsafe extern "system" fn enumerate_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let context = &mut *(lparam.0 as *mut SearchContext);

    if IsWindowVisible(hwnd).as_bool() {
        let mut text = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut text);
        if len > 0 {
            let title = String::from_utf16_lossy(&text[..len as usize]);
            if context.regex.is_match(&title) {
                context.result = Some(title);
                return BOOL::from(false); // Trouvé !
            }
        }
    }
    BOOL::from(true)
}
