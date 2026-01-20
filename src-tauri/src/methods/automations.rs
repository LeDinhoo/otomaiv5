use super::guide_parser::GuideResult;
use super::potions;
use super::travel_with_zaap;
use super::travel_with_zaapi;
use super::press_key::press_key_multiple_times;
use super::type_text::type_text_fast;
use crate::window_manager::WindowManager;
use std::thread;
use std::time::Duration;

fn run_zaap_sequence(zaap_name: &str, travel_cmd: Option<&String>, window_title: &str) -> Result<String, String> {
    travel_with_zaap::travel_with_zaap(zaap_name, window_title)?;
    thread::sleep(Duration::from_secs(4)); 
    if let Some(cmd) = travel_cmd {
        send_chat_command(cmd)?;
        return Ok(format!("Zaap '{}' + Travel OK.", zaap_name));
    }
    Ok(format!("Zaap '{}' seul OK.", zaap_name))
}

fn run_zaapi_sequence(zaapi_name: &str, travel_cmd: Option<&String>, window_title: &str) -> Result<String, String> {
    travel_with_zaapi::travel_with_zaapi(zaapi_name, window_title)?;
    thread::sleep(Duration::from_secs(3)); 
    if let Some(cmd) = travel_cmd {
        send_chat_command(cmd)?;
        return Ok(format!("Zaapi '{}' + Travel OK.", zaapi_name));
    }
    Ok(format!("Zaapi '{}' seul OK.", zaapi_name))
}

pub fn execute_step_automation(step: &GuideResult, window_title: &str) -> Result<String, String> {
    let win_manager = WindowManager::new();
    win_manager.focus_by_title(window_title)?;
    thread::sleep(Duration::from_millis(200));

    match step.macro_type.as_str() {
        "classic" => {
            if let Some(cmd) = &step.travel_cmd {
                send_chat_command(cmd)?;
                Ok("Travel classique effectué.".to_string())
            } else {
                Ok("Aucun déplacement requis.".to_string())
            }
        }
        "potion_direct" => {
            if let Some(cmd) = &step.travel_cmd {
                match cmd.as_str() {
                    "potion_bonta" => potions::potion_bonta(window_title)?,
                    "potion_brakmar" => potions::potion_brakmar(window_title)?,
                    _ => return Err(format!("Potion inconnue : {}", cmd)),
                };
            }
            Ok("Potion directe utilisée.".to_string())
        }
        "zaap" => {
            if let Some(dest) = &step.macro_arg {
                run_zaap_sequence(dest, step.travel_cmd.as_ref(), window_title)
            } else {
                Err("Nom du Zaap manquant".to_string())
            }
        }
        "zaapi" => {
            if let Some(dest) = &step.macro_arg {
                run_zaapi_sequence(dest, step.travel_cmd.as_ref(), window_title)
            } else {
                Err("Nom du Zaapi manquant".to_string())
            }
        }
        "potion_zaapi" => {
            if let Some(potion_cmd) = &step.macro_arg2 {
                match potion_cmd.as_str() {
                    "potion_bonta" => potions::potion_bonta(window_title)?,
                    "potion_brakmar" => potions::potion_brakmar(window_title)?,
                    _ => return Err("Potion d'optimisation inconnue".to_string()),
                };
                thread::sleep(Duration::from_secs(4));
            }
            if let Some(dest) = &step.macro_arg {
                run_zaapi_sequence(dest, step.travel_cmd.as_ref(), window_title)
            } else {
                Err("Destination Zaapi manquante".to_string())
            }
        }
        _ => Err(format!("Type de macro inconnu : {}", step.macro_type)),
    }
}

fn send_chat_command(command: &str) -> Result<(), String> {
    press_key_multiple_times("space", 1, None)?;
    thread::sleep(Duration::from_millis(200));
    type_text_fast(command)?;
    thread::sleep(Duration::from_millis(200));
    press_key_multiple_times("enter", 2, None)?;
    Ok(())
}