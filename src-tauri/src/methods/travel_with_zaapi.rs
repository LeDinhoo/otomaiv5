use crate::mouse_manager::MouseManager;
use crate::methods::{press_key, type_text};
use std::thread;
use std::time::Duration;

// CONFIGURATION PIXELS (À ajuster)
const ZAAPI_CLICK_X: i32 = 960; 
const ZAAPI_CLICK_Y: i32 = 540; 

pub fn travel_with_zaapi(destination: &str, window_title: &str) -> Result<String, String> {
    let mouse = MouseManager::new();

    mouse.click_at_position(window_title, ZAAPI_CLICK_X, ZAAPI_CLICK_Y)?;
    thread::sleep(Duration::from_millis(1500)); 

    type_text::type_text_fast(destination)?;
    thread::sleep(Duration::from_millis(200));

    press_key::press_key_multiple_times("enter", 1, None)?;

    Ok(format!("Voyage Zaapi vers '{}' initié.", destination))
}