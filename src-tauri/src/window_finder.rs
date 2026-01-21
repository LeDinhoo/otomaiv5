use regex::Regex;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, GetWindowTextLengthW, IsWindowVisible};

// Structure de contexte (inchangée, mais nécessaire pour l'exemple)
struct SearchContext {
    regex: Regex,
    result: Option<String>,
}

pub fn find_specific_game_window(character_name: &str) -> Option<String> {
    // --- CORRECTION ICI ---
    // Si l'input contient déjà " - ", on coupe pour ne garder que le pseudo.
    // Cela permet de relancer la recherche même si on a déjà le titre complet en input.
    let clean_name = character_name.split(" - ").next().unwrap_or(character_name);

    println!("--- DÉBUT RECHERCHE ---");
    println!("DEBUG: Input reçu : '{}'", character_name);
    println!("DEBUG: Nom nettoyé utilisé : '{}'", clean_name);

    let pattern = format!(
        r"^{} - .* - \d+(\.\d+)* - Release$",
        regex::escape(clean_name) // On utilise clean_name ici !
    );

    // Le reste ne change pas...
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

// C'est ICI que se fait la vérification ligne par ligne
unsafe extern "system" fn enumerate_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let context = &mut *(lparam.0 as *mut SearchContext);

    // On ignore les fenêtres invisibles pour ne pas spammer la console
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

    // 4. VERIFICATION CRITIQUE : Afficher chaque fenêtre analysée
    // On filtre un peu pour ne pas afficher les processus système inutiles
    // (optionnel : tu peux retirer le if si tu veux TOUT voir)
    if window_title.contains("Release") || window_title.contains(context.regex.as_str().split(" -").next().unwrap_or("")) {
        print!("DEBUG: Analyse de la fenêtre -> '{}' ... ", window_title);
        
        if context.regex.is_match(&window_title) {
            println!("MATCH ✅");
            context.result = Some(window_title);
            return BOOL(0); // On arrête la recherche (false)
        } else {
            println!("NON ❌");
        }
    }

    BOOL(1) // On continue la recherche (true)
}