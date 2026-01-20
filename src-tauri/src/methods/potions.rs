use crate::methods::press_key;
use crate::window_manager::WindowManager;
use std::{thread, time::Duration};

pub fn potion_bonta(window_title: &str) -> Result<String, String> {
    let win_manager = WindowManager::new();
    win_manager.focus_by_title(window_title)?;
    thread::sleep(Duration::from_millis(200));

    // Slot 1 + Argument None
    press_key::press_key_multiple_times("1", 1, None)?; 
    thread::sleep(Duration::from_secs(4));
    Ok("Potion Bonta utilisée".to_string())
}

pub fn potion_brakmar(window_title: &str) -> Result<String, String> {
    let win_manager = WindowManager::new();
    win_manager.focus_by_title(window_title)?;
    thread::sleep(Duration::from_millis(200));

    // Slot 2 + Argument None
    press_key::press_key_multiple_times("2", 1, None)?;
    thread::sleep(Duration::from_secs(4));
    Ok("Potion Brakmar utilisée".to_string())
}