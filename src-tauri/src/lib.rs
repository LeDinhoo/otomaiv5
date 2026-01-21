mod window_manager;
use window_manager::WindowManager;

mod mouse_manager;
use mouse_manager::MouseManager;

mod methods;
mod window_finder;

use std::sync::{Arc, Mutex};
use methods::key_listener::{self, SharedKeyListenerState, KeyListenerState};
use methods::guide_parser::{GuideParser, GuideResult};

// --- GESTION DE L'ÉTAT (TITRE FENÊTRE) ---
pub struct ActiveSession {
    pub window_title: Mutex<Option<String>>,
}

// --- COMMANDES TAURI ---

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn sync_window_title(character_name: String, state: tauri::State<'_, ActiveSession>) -> Result<String, String> {
    // DEBUG CRITIQUE : {:?} permet de voir les espaces cachés ou les \n
    // Si vous voyez "Nom " au lieu de "Nom", c'est là le problème.
    println!("DEBUG INPUT: Reçu character_name = {:?}", character_name);

    if let Some(full_title) = window_finder::find_specific_game_window(&character_name) {
        let mut title_storage = state.window_title.lock().unwrap();
        *title_storage = Some(full_title.clone());
        println!("✅ Fenêtre trouvée et synchronisée : {}", full_title);
        Ok(full_title)
    } else {
        // On renvoie une erreur plus précise
        println!("❌ ECHEC: Aucune correspondance trouvée pour {:?}", character_name);
        Err(format!("Impossible de trouver une fenêtre pour '{}'. Vérifiez les logs serveurs.", character_name))
    }
}

#[tauri::command]
fn focus_window(window_title: String) -> Result<String, String> {
    let manager = WindowManager::new();
    manager.focus_by_title(&window_title)
}

#[tauri::command]
fn click_window_at(title: String, x: i32, y: i32) -> Result<String, String> {
    let manager = MouseManager::new();
    manager.click_at_position(&title, x, y)
}

#[tauri::command]
fn get_game_title_by_name(character_name: String) -> Result<String, String> {
    window_finder::find_specific_game_window(&character_name).ok_or_else(|| {
        format!(
            "Fenêtre pour le personnage '{}' introuvable.",
            character_name
        )
    })
}

// NOTE : Accepte maintenant le délai optionnel
#[tauri::command]
fn press_key(key: String, count: u32, delay: Option<u64>) -> Result<String, String> {
    methods::press_key::press_key_multiple_times(&key, count, delay)
}

#[tauri::command]
fn quick_type(text: String, window_title: String) -> Result<String, String> {
    let manager = window_manager::WindowManager::new();
    manager.focus_by_title(&window_title)?;
    methods::type_text::type_text_fast(&text)
}

#[tauri::command]
fn travel_with_zaap(destination: String, window_title: String) -> Result<String, String> {
    methods::travel_with_zaap::travel_with_zaap(&destination, &window_title)
}

#[tauri::command]
fn use_potion_bonta(window_title: String) -> Result<String, String> {
    methods::potions::potion_bonta(&window_title)
}

#[tauri::command]
fn use_potion_brakmar(window_title: String) -> Result<String, String> {
    methods::potions::potion_brakmar(&window_title)
}

#[tauri::command]
fn parse_guide_step(html_content: String) -> GuideResult {
    GuideParser::parse_step(&html_content)
}

#[tauri::command]
async fn execute_step_automation(
    step: GuideResult, 
    state: tauri::State<'_, ActiveSession>
) -> Result<String, String> {
    // Récupération sécurisée du titre depuis la mémoire
    let window_title = {
        let title_lock = state.window_title.lock().unwrap();
        title_lock.clone().ok_or("Aucune fenêtre synchronisée. Lancez la recherche d'abord.")?
    };

    // --- AJOUT DU PRINT ICI ---
    // On affiche le titre dans la console du terminal (là où vous avez lancé `tauri dev`)
    println!("DEBUG - Titre de la fenêtre récupéré : {}", window_title);
    // -------------------------

    // Exécution dans un thread séparé
    // Note : Le mot clé `move` va déplacer `window_title` dans le thread juste après
    let result = std::thread::spawn(move || {
        methods::automations::execute_step_automation(&step, &window_title)
    }).join();

    match result {
        Ok(res) => res,
        Err(_) => Err("Crash critique du thread d'automatisation.".to_string()),
    }
}

// --- MAIN RUN ---

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // État initial Listener
    let listener_state: SharedKeyListenerState = Arc::new(Mutex::new(KeyListenerState {
        active: false,
        target_keys: vec![], 
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        
        // 1. Enregistrement des États
        .manage(listener_state) 
        .manage(ActiveSession { window_title: Mutex::new(None) }) 

        // 2. Setup Listener
        .setup(|app| {
            key_listener::init_background_listener(app.handle());
            Ok(())
        })
        
        // 3. Enregistrement des Commandes
        .invoke_handler(tauri::generate_handler![
            greet,
            focus_window,
            click_window_at,
            get_game_title_by_name,
            press_key,
            quick_type,
            travel_with_zaap,
            use_potion_bonta,
            use_potion_brakmar,
            sync_window_title,
            parse_guide_step,
            execute_step_automation,
            methods::key_listener::set_key_listener 
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}