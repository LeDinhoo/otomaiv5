use crate::mouse_manager::MouseManager; // Correction de l'import ici
use super::press_key::press_key_multiple_times;
use super::type_text::type_text_fast;
use std::thread;
use std::time::Duration;

// 1. Définition des briques Lego (Mise à jour)
#[derive(Debug, Clone)]
pub enum Action {
    Click((i32, i32)),           // Position x, y
    Wait(u64),                   // Temps en ms
    Type(String),                // Texte à écrire
    // On stocke : Touche, Nombre de fois, Intervalle en ms
    Press(String, u32, u64),     
}

// 2. L'Exécuteur
pub fn execute(actions: Vec<Action>, window_title: &str) -> Result<(), String> {
    let mouse = MouseManager::new();
    
    // Focus initial de la fenêtre (recommandé)
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
            // Mise à jour de la logique Press
            Action::Press(k, count, interval) => {
                press_key_multiple_times(&k, count, Some(interval))?;
            }
        }
        // Petit délai de sécurité entre les briques (très important pour Dofus)
        thread::sleep(Duration::from_millis(100));
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

// Helper simple : Appuie 1 fois, intervalle par défaut (100ms)
// Utilisation : key("enter")
pub fn key(k: &str) -> Action {
    Action::Press(k.to_string(), 1, 100)
}

// Helper avancé : Appuie X fois avec Y intervalle
// Utilisation : press("space", 5, 300) -> Spamme Espace 5 fois toutes les 300ms
pub fn press(k: &str, count: u32, interval: u64) -> Action {
    Action::Press(k.to_string(), count, interval)
}