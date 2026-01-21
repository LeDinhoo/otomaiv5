use super::guide_parser::GuideResult;
use super::potions;
use super::travel_with_zaapi;
use crate::window_manager::WindowManager;
use std::thread;
use std::time::Duration;
use super::config::*;
use super::engine::{execute, click, wait, write, key, press};

pub fn execute_step_automation(step: &GuideResult, window_title: &str) -> Result<String, String> {
    let win_manager = WindowManager::new();
    win_manager.focus_by_title(window_title)?;
    println!("Bonjour tout le monde !");

    thread::sleep(Duration::from_millis(200));

    match step.macro_type.as_str() {
        "classic" => {
            if let Some(cmd) = &step.travel_cmd {
                send_chat_command(cmd, window_title)?;
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
                travel_with_zaap(dest, step.travel_cmd.as_ref(), window_title)
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
        // --- NOUVEAU CAS ---
        "zaap_zaapi" => {
            // arg1 = Zaap, arg2 = Zaapi (défini dans le parser)
            if let (Some(zaap_name), Some(zaapi_name)) = (&step.macro_arg, &step.macro_arg2) {
                run_zaap_zaapi_sequence(zaap_name, zaapi_name, step.travel_cmd.as_ref(), window_title)
            } else {
                Err("Il manque le nom du Zaap ou du Zaapi.".to_string())
            }
        }
        // -------------------
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

/// Transforme "/travel 12,-45" en Some((12, -45))
fn extract_coordinates(cmd: &str) -> Option<(i32, i32)> {
    // On nettoie la commande pour ne garder que "x,y"
    let clean = cmd.trim().trim_start_matches("/travel ").trim();
    
    let parts: Vec<&str> = clean.split(',').collect();
    
    if parts.len() == 2 {
        // On tente de parser x et y
        let x = parts[0].trim().parse::<i32>().ok()?;
        let y = parts[1].trim().parse::<i32>().ok()?;
        return Some((x, y));
    }
    None
}

pub fn get_name_of_closest_city_zaap(dest: (i32, i32)) -> String {
    // Liste des candidats avec leurs positions importées de config.rs
    let cities = [
        ("Frigost", POS_FRIGOST),
        ("Sufokia", POS_SUFOKIA),
        ("Brakmar", POS_BRAKMAR),
        ("Bonta",   POS_BONTA),
    ];

    let (dx, dy) = dest;
    let mut min_dist = i32::MAX;
    let mut best_city = "Bonta"; // Valeur par défaut de sécurité

    for (name, (cx, cy)) in cities.iter() {
        // Formule de distance Euclidienne (au carré) : (x2 - x1)² + (y2 - y1)²
        // Pas besoin de sqrt() pour comparer des distances
        let dist = (dx - cx).pow(2) + (dy - cy).pow(2);

        if dist < min_dist {
            min_dist = dist;
            best_city = name;
        }
    }

    best_city.to_string()
}

pub fn send_chat_command(command: &str, window_title: &str) -> Result<(), String> {
    let sequence = vec![
        press("space", 1, 0),
        wait(500),
        write(command),
        key("enter"),
        wait(500),
        key("enter")
    ];

    execute(sequence, window_title)
}

pub fn travel_with_zaap(zaap_name: &str, travel_cmd: Option<&String>, window_title: &str) -> Result<String, String> {
    
    let mut sequence = vec![
        key("h"),                
        wait(1500),
        click(POS_ZAAP_INPUT),   
        write(zaap_name),        
        key("enter"),            
        wait(2000),          
    ];

    
    if let Some(cmd) = travel_cmd {
        
        sequence.extend(vec![
            press("space", 1, 100), 
            wait(500),
            write(cmd),             
            key("enter"),           
            wait(500),
            key("enter")            
        ]);
    }

    
    execute(sequence, window_title)?;

    Ok(format!("Zaap '{}' effectué (Travel: {})", zaap_name, travel_cmd.is_some()))
}

fn get_zaapi_category_pos(zaapi_name: &str) -> (i32, i32) {
    let name_lower = zaapi_name.to_lowercase();

    if name_lower.contains("atelier") {
        POS_TAB_ZAAPI_ATELIER
    } else if name_lower.contains("hotel") || name_lower.contains("hôtel") {
        // Gère "Hotel" et "Hôtel"
        POS_TAB_ZAAPI_HOTEL
    } else {
        POS_TAB_ZAAPI_DIVERS
    }
}

fn run_zaapi_sequence(zaapi_name: &str, travel_cmd: Option<&String>, window_title: &str) -> Result<String, String> {
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
        _ => POS_ZAAPI_BONTA, // Bonta par défaut (le plus sûr)
    };

        let mut sequence = vec![
        key("h"),                
        wait(1500),
        click(POS_ZAAP_INPUT),   
        write(closest_city_name.as_str()),        
        key("enter"),            
        wait(2000),
        click(pos_zaapi),
        wait(4000),
        click(get_zaapi_category_pos(zaapi_name)),
        wait(100),
        click(POS_INPUT_TEXT_ZAAPI),
        write(zaapi_name),
        key("enter")
    ];

    if let Some(cmd) = travel_cmd {
        
        sequence.extend(vec![
            press("space", 1, 100), 
            wait(500),
            write(cmd),             
            key("enter"),           
            wait(500),
            key("enter")            
        ]);
    }
    
    execute(sequence, window_title)?;

    Ok(format!("Séquence Zaap '{}' + Zaapi '{}' terminée.", closest_city_name, zaapi_name))
}

fn run_zaap_zaapi_sequence(
    zaap_name: &str, 
    zaapi_name: &str, 
    travel_cmd: Option<&String>, 
    window_title: &str
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
        _ => POS_ZAAPI_BONTA, // Bonta par défaut (le plus sûr)
    };
    
    let mut sequence = vec![
        key("h"),                
        wait(1500),
        click(POS_ZAAP_INPUT),   
        write(zaap_name),        
        key("enter"),            
        wait(2000),
        click(pos_zaapi),
        wait(4000),
        click(get_zaapi_category_pos(zaapi_name)),
        wait(100),
        click(POS_INPUT_TEXT_ZAAPI),
        write(zaapi_name),
        key("enter"),
        wait(100)
    ];

    if let Some(cmd) = travel_cmd {
        
        sequence.extend(vec![
            press("space", 1, 100), 
            wait(500),
            write(cmd),             
            key("enter"),           
            wait(500),
            key("enter")            
        ]);
    }

    execute(sequence, window_title)?;

    Ok(format!("Séquence Zaap '{}' + Zaapi '{}' terminée.", closest_city_name, zaapi_name))
}