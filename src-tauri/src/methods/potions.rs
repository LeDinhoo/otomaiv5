use crate::methods::press_key;
use crate::window_manager::WindowManager;
use std::thread;
use std::time::Duration;

pub fn potion_bonta(window_title: &str) -> Result<String, String> {
    // 1. Focus de la fenêtre
    let win_manager = WindowManager::new();
    win_manager.focus_by_title(window_title)?;

    // Petite pause après le focus
    thread::sleep(Duration::from_millis(100));

    // 2. Appui sur la touche '-' deux fois
    // On utilise votre méthode native qui est la plus fiable
    press_key::press_key_multiple_times("-", 2)?;

    Ok("Potion de cité Bonta utilisée (touche - x2)".to_string())
}

pub fn potion_brakmar(window_title: &str) -> Result<String, String> {
    // 1. Focus de la fenêtre
    let win_manager = WindowManager::new();
    win_manager.focus_by_title(window_title)?;

    // Petite pause après le focus
    thread::sleep(Duration::from_millis(100));

    // 2. Appui sur la touche '=' deux fois
    press_key::press_key_multiple_times("=", 2)?;

    Ok("Potion de cité Brakmar utilisée (touche = x2)".to_string())
}