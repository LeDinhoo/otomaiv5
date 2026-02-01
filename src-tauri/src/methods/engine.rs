use super::press_key::press_key_multiple_times;
use super::type_text::type_text_fast;
use crate::mouse_manager::MouseManager;
use std::thread;
use std::time::Duration;
use super::vision;

// 👇 Nouveaux imports nécessaires pour Tauri v2
use tauri::{AppHandle, Manager}; 
use tauri::path::BaseDirectory; 

// 1. Définition des briques Lego (Mise à jour pour Tauri Resources)
#[derive(Debug, Clone)]
pub enum Action {
    Click((i32, i32)), 
    Wait(u64),         
    Type(String),      
    Press(String, u32, u64),
    
    // 👇 MODIFICATION : On stocke le chemin relatif (String)
    // plus de 'static [u8] car on lit le fichier au runtime
    WaitForImage { path: String },
}

// 2. L'Exécuteur
// 👇 MODIFICATION : On ajoute app_handle en 1er argument
pub fn execute(app_handle: &AppHandle, actions: Vec<Action>, window_title: &str) -> Result<(), String> {
    let mouse = MouseManager::new();

    // Focus initial (optionnel)
    // crate::window_manager::WindowManager::new().focus_by_title(window_title)?;

    for action in actions {
        match action {
            Action::Click((x, y)) => {
                mouse.click_at_position(window_title, x, y)?;
            }
            Action::Wait(ms) => {
                thread::sleep(Duration::from_millis(ms));
            }
            Action::Type(text) => {
                type_text_fast(&text)?;
            }
            Action::Press(k, count, interval) => {
                press_key_multiple_times(&k, count, Some(interval))?;
            }
            
            Action::WaitForImage { path } => {
                println!("👁️ Moteur: Recherche du fichier ressource '{}'", path);
                
                // 1. Résolution via Tauri
                let resource_path = app_handle.path()
                    .resolve(&path, BaseDirectory::Resource)
                    .map_err(|e| format!("Impossible de résoudre le chemin ressource '{}': {}", path, e))?;

                // 2. Conversion en String
                let path_str = resource_path.to_str()
                    .ok_or_else(|| "Erreur de conversion du chemin en String".to_string())?;

                // 👇👇👇 FIX CRITIQUE POUR OPENCV / WINDOWS 👇👇👇
                // On retire le préfixe "\\?\" si présent, sinon OpenCV ne trouve pas le fichier.
                let fixed_path = if cfg!(windows) {
                    path_str.trim_start_matches("\\\\?\\")
                } else {
                    path_str
                };

                println!("📂 Chemin absolu corrigé : {}", fixed_path);

                // 3. Appel Vision
                vision::wait_for_image(fixed_path)
                    .map_err(|e| format!("Echec vision : {}", e))?;
            },
        }
        
        // Petit délai de sécurité entre les briques
        thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

// --- Helpers pour écrire tes macros "Lego" ultra vite ---

pub fn click(pos: (i32, i32)) -> Action {
    Action::Click(pos)
}

pub fn wait(ms: u64) -> Action {
    Action::Wait(ms)
}

pub fn write(text: &str) -> Action {
    Action::Type(text.to_string())
}

pub fn key(k: &str) -> Action {
    Action::Press(k.to_string(), 1, 100)
}

pub fn press(k: &str, count: u32, interval: u64) -> Action {
    Action::Press(k.to_string(), count, interval)
}

// 👇 MODIFICATION DU HELPER
// On prend juste le chemin relatif (ex: "ui/zaap.png")
pub fn wait_image(path: &str) -> Action {
    Action::WaitForImage {
        path: path.to_string(),
    }
}