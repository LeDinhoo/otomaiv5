mod window_manager;
use window_manager::WindowManager;

mod mouse_manager;
use mouse_manager::MouseManager;

mod window_finder;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Déplace la commande focus_window ici
#[tauri::command]
fn focus_window(window_title: String) -> Result<String, String> {
    let manager = WindowManager::new();
    manager.focus_by_title(&window_title)
}

#[tauri::command]
fn click_window_center(title: String) -> Result<String, String> {
    let manager = MouseManager::new();
    manager.click_center_of_window(&title)
}

#[tauri::command]
fn get_game_title_by_name(character_name: String) -> Result<String, String> {
    window_finder::find_specific_game_window(&character_name)
        .ok_or_else(|| format!("Fenêtre pour le personnage '{}' introuvable.", character_name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            focus_window,
            click_window_center,
            get_game_title_by_name
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}