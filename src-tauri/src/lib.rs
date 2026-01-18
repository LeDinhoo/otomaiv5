// On déclare les modules ici
mod window_manager;
use window_manager::WindowManager;
mod mouse_manager;
use mouse_manager::MouseManager;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            focus_window, // <-- Très important : on l'enregistre ici
            click_window_center
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}