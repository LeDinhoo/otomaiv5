use windows::core::{PCWSTR, HSTRING};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, SetForegroundWindow, ShowWindow, IsIconic, SW_RESTORE
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

        // 1. On tente de trouver la fenêtre
        // On retire le '?' ici pour gérer le résultat manuellement
        let result = FindWindowW(None, title_pcwstr);

        // 2. Vérification si le résultat est une erreur réelle ou un handle vide
        let hwnd = match result {
            Ok(h) if h.0.is_null() => {
                return Err(format!("Fenêtre introuvable : '{}'. Vérifiez le titre exact.", title));
            }
            Ok(h) => h,
            Err(e) => return Err(format!("Erreur système Win32 : {}", e)),
        };

        // 3. Suite de la logique (Focus)
        if IsIconic(hwnd).as_bool() {
            let _ = ShowWindow(hwnd, SW_RESTORE);
        }

        let success = SetForegroundWindow(hwnd);

        if success.as_bool() {
            Ok(format!("Succès : '{}' est maintenant au premier plan.", title))
        } else {
            Err("Le focus a été refusé par Windows (l'application doit être active pour changer le focus).".to_string())
        }
    }
}
}