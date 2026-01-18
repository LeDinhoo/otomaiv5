// src-tauri/src/methods/travel_with_zaap.rs
use std::thread;
use std::time::Duration;
use crate::methods::{press_key, type_text, config};
use crate::mouse_manager::MouseManager;
use crate::window_manager::WindowManager; //

pub fn travel_with_zaap(destination: &str, window_title: &str) -> Result<String, String> {
    // 1. Remettre le focus sur la fenêtre avant toute chose
    let win_manager = WindowManager::new(); //
    win_manager.focus_by_title(window_title)?; //

    // Petite pause pour laisser Windows traiter le changement de focus
    thread::sleep(Duration::from_millis(300));

    let mouse = MouseManager::new(); //

    // 2. Appui sur la touche 'H' (via votre méthode SendInput native)
    press_key::press_key_multiple_times("h", 1)?;

    // 3. Attente de l'ouverture de l'interface (2 secondes)
    thread::sleep(Duration::from_secs(2));

    // 4. Click à la position du champ Zaap définie dans config.rs (725, 497)
    mouse.click_at_position(
        window_title, 
        config::POS_ZAAP_INPUT.0, 
        config::POS_ZAAP_INPUT.1
    )?; //

    // Pause pour s'assurer que le clic a activé le champ de texte
    thread::sleep(Duration::from_millis(200));

    // 5. Écrit la destination via Unicode (votre méthode quick_type)
    type_text::type_text_fast(destination)?;

    // 6. Appui sur Enter via SendInput natif
    press_key::press_key_multiple_times("enter", 1)?;

    Ok(format!("Voyage vers '{}' effectué avec focus.", destination))
}