use super::config::*;
use super::engine::{click, execute, key, press, wait, write};
use super::guide_parser::GuideResult;
use super::potions;
use crate::settings::AutomationSettings;
use crate::window_manager::WindowManager;
use std::thread;
use std::time::Duration; // <--- Import indispensable

pub fn execute_step_automation(
    step: &GuideResult,
    window_title: &str,
    settings: &AutomationSettings, // <--- Ajout du paramètre
) -> Result<String, String> {
    let win_manager = WindowManager::new();
    win_manager.focus_by_title(window_title)?;
    println!("Bonjour tout le monde !");

    // Délai de démarrage (Peut rester fixe ou être configuré)
    thread::sleep(Duration::from_millis(200));

    match step.macro_type.as_str() {
        "classic" => {
            if let Some(cmd) = &step.travel_cmd {
                send_chat_command(cmd, window_title, settings)?;
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
                travel_with_zaap(dest, step.travel_cmd.as_ref(), window_title, settings)
            } else {
                Err("Nom du Zaap manquant".to_string())
            }
        }
        "zaapi" => {
            if let Some(dest) = &step.macro_arg {
                run_zaapi_sequence(dest, step.travel_cmd.as_ref(), window_title, settings)
            } else {
                Err("Nom du Zaapi manquant".to_string())
            }
        }
        "zaap_zaapi" => {
            if let (Some(zaap_name), Some(zaapi_name)) = (&step.macro_arg, &step.macro_arg2) {
                run_zaap_zaapi_sequence(
                    zaap_name,
                    zaapi_name,
                    step.travel_cmd.as_ref(),
                    window_title,
                    settings,
                )
            } else {
                Err("Il manque le nom du Zaap ou du Zaapi.".to_string())
            }
        }
        "potion_zaapi" => {
            if let Some(potion_cmd) = &step.macro_arg2 {
                match potion_cmd.as_str() {
                    "potion_bonta" => potions::potion_bonta(window_title)?,
                    "potion_brakmar" => potions::potion_brakmar(window_title)?,
                    _ => return Err("Potion d'optimisation inconnue".to_string()),
                };
                // Utilisation du délai d'animation potion configuré
                thread::sleep(Duration::from_millis(settings.potion_anim_delay));
            }
            if let Some(dest) = &step.macro_arg {
                run_zaapi_sequence(dest, step.travel_cmd.as_ref(), window_title, settings)
            } else {
                Err("Destination Zaapi manquante".to_string())
            }
        }
        _ => Err(format!("Type de macro inconnu : {}", step.macro_type)),
    }
}

pub fn is_zaap_destination(cmd: &str, zaap_positions: &[(i32, i32)]) -> bool {
    let clean_cmd = cmd.trim().trim_start_matches("/travel ").trim();
    let parts: Vec<&str> = clean_cmd.split(',').collect();

    if parts.len() == 2 {
        if let (Ok(x), Ok(y)) = (
            parts[0].trim().parse::<i32>(),
            parts[1].trim().parse::<i32>(),
        ) {
            return zaap_positions.contains(&(x, y));
        }
    }
    false
}

fn extract_coordinates(cmd: &str) -> Option<(i32, i32)> {
    let clean = cmd.trim().trim_start_matches("/travel ").trim();
    let parts: Vec<&str> = clean.split(',').collect();

    if parts.len() == 2 {
        let x = parts[0].trim().parse::<i32>().ok()?;
        let y = parts[1].trim().parse::<i32>().ok()?;
        return Some((x, y));
    }
    None
}

pub fn get_name_of_closest_city_zaap(dest: (i32, i32)) -> String {
    let cities = [
        ("Frigost", POS_FRIGOST),
        ("Sufokia", POS_SUFOKIA),
        ("Brakmar", POS_BRAKMAR),
        ("Bonta", POS_BONTA),
    ];

    let (dx, dy) = dest;
    let mut min_dist = i32::MAX;
    let mut best_city = "Bonta";

    for (name, (cx, cy)) in cities.iter() {
        let dist = (dx - cx).pow(2) + (dy - cy).pow(2);
        if dist < min_dist {
            min_dist = dist;
            best_city = name;
        }
    }
    best_city.to_string()
}

// Fonction générique pour chat avec settings
pub fn send_chat_command(
    command: &str,
    window_title: &str,
    settings: &AutomationSettings,
) -> Result<(), String> {
    let sequence = vec![
        press("space", 1, 0),
        wait(settings.chat_type_delay), // Configurable
        write(command),
        wait(settings.chat_type_delay), // Configurable
        key("enter"),
        wait(settings.chat_validate_delay), // Configurable
        key("enter"),
    ];

    execute(sequence, window_title)
}

pub fn travel_with_zaap(
    zaap_name: &str,
    travel_cmd: Option<&String>,
    window_title: &str,
    settings: &AutomationSettings, // <--- Settings
) -> Result<String, String> {
    let candidates: Vec<String> = ZAAP_NAMES.iter().map(|s| s.to_string()).collect();
    let clean_name = crate::methods::text_utils::correct_text(zaap_name, &candidates);

    if clean_name != zaap_name {
        println!(
            "✨ Auto-Correction Zaap : '{}' -> '{}'",
            zaap_name, clean_name
        );
    }

    // Séquence Zaap utilisant les settings
    let mut sequence = vec![
        key("h"),
        wait(settings.ui_open_delay), // "H" delay
        click(POS_ZAAP_INPUT),
        wait(settings.input_react_delay), // Input delay
        write(&clean_name),
        key("enter"),
        wait(settings.map_load_delay), // Map load delay
    ];

    // Ajout du Travel si nécessaire
    if let Some(cmd) = travel_cmd {
        if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
            sequence.extend(vec![
                press("space", 1, 0),
                wait(settings.chat_type_delay),
                write(cmd),
                wait(settings.chat_type_delay),
                key("enter"),
                wait(settings.chat_validate_delay),
                key("enter"),
            ]);
        }
    }

    execute(sequence, window_title)?;

    Ok(format!(
        "Zaap '{}' effectué (Travel: {})",
        clean_name,
        travel_cmd.is_some()
    ))
}

fn get_zaapi_category_pos(zaapi_name: &str) -> (i32, i32) {
    let name_lower = zaapi_name.to_lowercase();
    if name_lower.contains("atelier") {
        POS_TAB_ZAAPI_ATELIER
    } else if name_lower.contains("hotel") || name_lower.contains("hôtel") {
        POS_TAB_ZAAPI_HOTEL
    } else {
        POS_TAB_ZAAPI_DIVERS
    }
}

fn run_zaapi_sequence(
    zaapi_name: &str,
    travel_cmd: Option<&String>,
    window_title: &str,
    settings: &AutomationSettings,
) -> Result<String, String> {
    let closest_city_name = {
        if let Some(cmd) = travel_cmd {
            if let Some(coords) = extract_coordinates(cmd) {
                get_name_of_closest_city_zaap(coords)
            } else {
                "Bonta".to_string()
            }
        } else {
            "Bonta".to_string()
        }
    };

    let pos_zaapi = match closest_city_name.as_str() {
        "Sufokia" => POS_ZAAPI_SUFOKIA,
        "Brakmar" => POS_ZAAPI_BRAKMAR,
        "Frigost" => POS_ZAAPI_FRIGOST,
        _ => POS_ZAAPI_BONTA,
    };

    // --- UTILISATION DES SETTINGS POUR LA MARCHE ---
    let zaapi_wait_time = match closest_city_name.as_str() {
        "Sufokia" => settings.walk_sufokia,
        "Brakmar" => settings.walk_brakmar,
        "Frigost" => settings.walk_frigost,
        _ => settings.walk_bonta,
    };

    let mut sequence = vec![
        key("h"),
        wait(settings.ui_open_delay),
        click(POS_ZAAP_INPUT),
        wait(settings.input_react_delay),
        write(closest_city_name.as_str()),
        key("enter"),
        wait(settings.map_load_delay),
        click(pos_zaapi),
        wait(zaapi_wait_time), // Temps de marche configurable
        click(get_zaapi_category_pos(zaapi_name)),
        wait(settings.input_react_delay),
        click(POS_INPUT_TEXT_ZAAPI),
        write(zaapi_name),
        key("enter"),
        wait(settings.map_load_delay),
        wait(settings.chat_validate_delay), // Petit délai fin
    ];

    if let Some(cmd) = travel_cmd {
        if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
            sequence.extend(vec![
                press("space", 1, 0),
                wait(settings.chat_type_delay),
                write(cmd),
                wait(settings.chat_type_delay),
                key("enter"),
                wait(settings.chat_validate_delay),
                key("enter"),
            ]);
        }
    }

    execute(sequence, window_title)?;

    Ok(format!(
        "Séquence Zaap '{}' + Zaapi '{}' terminée.",
        closest_city_name, zaapi_name
    ))
}

fn run_zaap_zaapi_sequence(
    zaap_name: &str,
    zaapi_name: &str,
    travel_cmd: Option<&String>,
    window_title: &str,
    settings: &AutomationSettings,
) -> Result<String, String> {
    // --- LOGIQUE IDENTIQUE, ADAPTÉE AUX SETTINGS ---

    let closest_city_name = {
        if let Some(cmd) = travel_cmd {
            if let Some(coords) = extract_coordinates(cmd) {
                get_name_of_closest_city_zaap(coords)
            } else {
                "Bonta".to_string()
            }
        } else {
            "Bonta".to_string()
        }
    };

    let pos_zaapi = match closest_city_name.as_str() {
        "Sufokia" => POS_ZAAPI_SUFOKIA,
        "Brakmar" => POS_ZAAPI_BRAKMAR,
        "Frigost" => POS_ZAAPI_FRIGOST,
        _ => POS_ZAAPI_BONTA,
    };

    let zaapi_wait_time = match closest_city_name.as_str() {
        "Sufokia" => settings.walk_sufokia,
        "Brakmar" => settings.walk_brakmar,
        "Frigost" => settings.walk_frigost,
        _ => settings.walk_bonta,
    };

    let mut sequence = vec![
        key("h"),
        wait(settings.ui_open_delay),
        click(POS_ZAAP_INPUT),
        wait(settings.input_react_delay),
        write(zaap_name),
        key("enter"),
        wait(settings.map_load_delay),
        click(pos_zaapi),
        wait(zaapi_wait_time),
        click(get_zaapi_category_pos(zaapi_name)),
        wait(settings.input_react_delay),
        click(POS_INPUT_TEXT_ZAAPI),
        write(zaapi_name),
        key("enter"),
        wait(settings.map_load_delay),
        wait(settings.chat_validate_delay),
    ];

    if let Some(cmd) = travel_cmd {
        if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
            sequence.extend(vec![
                press("space", 1, 0),
                wait(settings.chat_type_delay),
                write(cmd),
                wait(settings.chat_type_delay),
                key("enter"),
                wait(settings.chat_validate_delay),
                key("enter"),
            ]);
        }
    }

    execute(sequence, window_title)?;

    Ok(format!(
        "Séquence Zaap '{}' + Zaapi '{}' terminée.",
        closest_city_name, zaapi_name
    ))
}
