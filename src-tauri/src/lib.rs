mod window_manager;
use window_manager::WindowManager;

mod mouse_manager;
use mouse_manager::MouseManager;

mod methods;
mod window_finder;

use std::sync::{Arc, Mutex};
use methods::key_listener::{self, SharedKeyListenerState, KeyListenerState};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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

#[tauri::command]
fn press_key(key: String, count: u32) -> Result<String, String> {
    methods::press_key::press_key_multiple_times(&key, count)
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // État initial : désactivé, pas de touche cible
    let listener_state: SharedKeyListenerState = Arc::new(Mutex::new(KeyListenerState {
        active: false,
        target_keys: vec![], 
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        // 1. On enregistre l'état
        .manage(listener_state) 
        // 2. On lance le listener au setup
        .setup(|app| {
            key_listener::init_background_listener(app.handle());
            Ok(())
        })
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
            // 3. Ajouter la nouvelle commande ici
            methods::key_listener::set_key_listener 
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
