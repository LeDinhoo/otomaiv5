use regex::Regex;
use std::{thread, time::Duration}; // Nécessaire pour le sleep
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
};

struct SearchContext {
    regex: Regex,
    result: Option<String>,
}

pub fn find_specific_game_window(character_name: &str) -> Option<String> {
    let clean_name = character_name.split(" - ").next().unwrap_or(character_name);

    // Pattern optimisé
    let pattern = format!(
        r"^{} - .* - \d+(\.\d+)* - Release$",
        regex::escape(clean_name)
    );

    let Ok(re) = Regex::new(&pattern) else {
        println!("ERREUR: Le Regex est invalide !");
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

/// NOUVELLE FONCTION : Tente de retrouver la fenêtre (30 essais, 2s délai)
pub fn try_autorecovery(character_name: &str) -> Option<String> {
    let max_retries = 30;
    let delay = Duration::from_secs(2);

    println!("🔄 AUTO-RECOVERY: Démarrage pour '{}'", character_name);

    for i in 1..=max_retries {
        // On tente une recherche standard
        if let Some(title) = find_specific_game_window(character_name) {
            println!(
                "✅ AUTO-RECOVERY: Succès à la tentative {}/{}",
                i, max_retries
            );
            return Some(title);
        }

        println!(
            "⚠️ Tentative {}/{} échouée. Nouvelle essai dans 2s...",
            i, max_retries
        );
        thread::sleep(delay);
    }

    println!(
        "❌ AUTO-RECOVERY: Échec total après {} tentatives.",
        max_retries
    );
    None
}

unsafe extern "system" fn enumerate_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let context = &mut *(lparam.0 as *mut SearchContext);

    if !IsWindowVisible(hwnd).as_bool() {
        return BOOL(1);
    }

    let length = GetWindowTextLengthW(hwnd);
    if length == 0 {
        return BOOL(1);
    }

    let mut buffer = vec![0u16; (length + 1) as usize];
    GetWindowTextW(hwnd, &mut buffer);
    let window_title = String::from_utf16_lossy(&buffer[..length as usize]);

    // Optimisation : On ne log pas tout en mode automatique pour éviter de spammer
    // On vérifie juste si ça match
    if context.regex.is_match(&window_title) {
        // println!("MATCH TROUVÉ : {}", window_title); // Décommenter si besoin de debug
        context.result = Some(window_title);
        return BOOL(0);
    }

    BOOL(1)
}
