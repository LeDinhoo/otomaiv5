use crate::methods::{press_key, type_text};
use std::thread;
use std::time::Duration;

pub fn travel_with_zaap(destination: &str, window_title: &str) -> Result<String, String> {
    press_key::press_key_multiple_times("h", 1, None)?;
    thread::sleep(Duration::from_millis(1500));

    type_text::type_text_fast(destination)?;
    thread::sleep(Duration::from_millis(500));

    press_key::press_key_multiple_times("enter", 1, None)?;

    Ok(format!("Voyage vers '{}' initié.", destination))
}