// use super::config::*;
// use super::engine::{click, execute, key, press, wait, write, wait_image};
// use super::guide_parser::GuideResult;
// use super::potions;
// use crate::settings::AutomationSettings;
// use crate::window_manager::WindowManager;
// use std::thread;
// use std::time::Duration;
// use tauri::AppHandle;

// pub fn execute_step_automation(
//     app_handle: &AppHandle, // 👈 Ajouté ici
//     step: &GuideResult,
//     window_title: &str,
//     settings: &AutomationSettings,
// ) -> Result<String, String> {
//     println!("🚀 [Automation] Démarrage de l'étape : {:?}", step.macro_type);
    
//     let win_manager = WindowManager::new();
//     win_manager.focus_by_title(window_title)?;
//     println!("✅ [Automation] Fenêtre '{}' focus.", window_title);

//     // Délai de démarrage
//     thread::sleep(Duration::from_millis(100)); // J'ai mis 100ms par sécurité au lieu de 0

//     match step.macro_type.as_str() {
//         "classic" => {
//             if let Some(cmd) = &step.travel_cmd {
//                 println!("🚶 [Classic] Déplacement vers : {}", cmd);
//                 send_chat_command(cmd, window_title, settings)?;
//                 Ok("Travel classique effectué.".to_string())
//             } else {
//                 println!("🛑 [Classic] Aucun déplacement requis.");
//                 Ok("Aucun déplacement requis.".to_string())
//             }
//         }
//         "potion_direct" => {
//             if let Some(cmd) = &step.travel_cmd {
//                 println!("🧪 [Potion] Utilisation de : {}", cmd);
//                 match cmd.as_str() {
//                     "potion_bonta" => potions::potion_bonta(window_title)?,
//                     "potion_brakmar" => potions::potion_brakmar(window_title)?,
//                     _ => return Err(format!("Potion inconnue : {}", cmd)),
//                 };
//             }
//             Ok("Potion directe utilisée.".to_string())
//         },
//         "skis_souples" => {
//             if let Some(dest) = &step.travel_cmd {
//                 println!("🎿 [Skis Souples] Vers : {}", dest);
//                 use_skis_souples(dest, step.travel_cmd.as_ref(), window_title, settings)?;
//             }
//             Ok("Skis souples utilisés.".to_string())
//         },
//         "skis_rustiques" => {
//             if let Some(dest) = &step.travel_cmd {
//                 println!("🎿 [Skis Rustiques] Vers : {}", dest);
//                 use_skis_rustiques(dest, step.travel_cmd.as_ref(), window_title, settings)?;
//             }
//             Ok("Skis rustiques utilisés.".to_string())
//         },
//         "skis_sombres" => {
//             if let Some(dest) = &step.travel_cmd {
//                 println!("🎿 [Skis Sombres] Vers : {}", dest);
//                 use_skis_sombres(dest, step.travel_cmd.as_ref(), window_title, settings)?;
//             }
//             Ok("Skis sombres utilisés.".to_string())
//         },
//         "skis_glissants" => {
//             if let Some(dest) = &step.travel_cmd {
//                 println!("🎿 [Skis Glissants] Vers : {}", dest);
//                 use_skis_glissants(dest, step.travel_cmd.as_ref(), window_title, settings)?;
//             }
//             Ok("Skis glissants utilisés.".to_string())
//         },
//         "zaap" => {
//             if let Some(dest) = &step.macro_arg {
//                 println!("🌀 [Zaap] Destination : {}", dest);
//                 travel_with_zaap(dest, step.travel_cmd.as_ref(), window_title, settings)
//             } else {
//                 Err("Nom du Zaap manquant".to_string())
//             }
//         }
//         "zaapi" => {
//             if let Some(dest) = &step.macro_arg {
//                 println!("🚕 [Zaapi] Destination : {}", dest);
//                 run_zaapi_sequence(dest, step.travel_cmd.as_ref(), window_title, settings)
//             } else {
//                 Err("Nom du Zaapi manquant".to_string())
//             }
//         }
//         "zaap_zaapi" => {
//             if let (Some(zaap_name), Some(zaapi_name)) = (&step.macro_arg, &step.macro_arg2) {
//                 println!("🌀🚕 [Zaap -> Zaapi] {} -> {}", zaap_name, zaapi_name);
//                 run_zaap_zaapi_sequence(
//                     zaap_name,
//                     zaapi_name,
//                     step.travel_cmd.as_ref(),
//                     window_title,
//                     settings,
//                 )
//             } else {
//                 Err("Il manque le nom du Zaap ou du Zaapi.".to_string())
//             }
//         }
//         "potion_zaapi" => {
//             println!("🧪🚕 [Potion -> Zaapi]");
//             if let Some(potion_cmd) = &step.macro_arg2 {
//                 println!("   -> Potion : {}", potion_cmd);
//                 match potion_cmd.as_str() {
//                     "potion_bonta" => potions::potion_bonta(window_title)?,
//                     "potion_brakmar" => potions::potion_brakmar(window_title)?,
//                     _ => return Err("Potion d'optimisation inconnue".to_string()),
//                 };
//                 // Utilisation du délai d'animation potion configuré
//                 println!("   -> Attente animation potion : {}ms", settings.potion_anim_delay);
//                 thread::sleep(Duration::from_millis(settings.potion_anim_delay));
//             }
//             if let Some(dest) = &step.macro_arg {
//                 println!("   -> Zaapi vers : {}", dest);
//                 run_zaapi_sequence(dest, step.travel_cmd.as_ref(), window_title, settings)
//             } else {
//                 Err("Destination Zaapi manquante".to_string())
//             }
//         }
//         _ => {
//             println!("❌ Type de macro inconnu : {}", step.macro_type);
//             Err(format!("Type de macro inconnu : {}", step.macro_type))
//         },
//     }
// }

// pub fn is_zaap_destination(cmd: &str, zaap_positions: &[(i32, i32)]) -> bool {
//     let clean_cmd = cmd.trim().trim_start_matches("/travel ").trim();
//     let parts: Vec<&str> = clean_cmd.split(',').collect();

//     if parts.len() == 2 {
//         if let (Ok(x), Ok(y)) = (
//             parts[0].trim().parse::<i32>(),
//             parts[1].trim().parse::<i32>(),
//         ) {
//             let is_zaap = zaap_positions.contains(&(x, y));
//             // if is_zaap { println!("   ℹ️ La destination {},{} est un Zaap connu.", x, y); }
//             return is_zaap;
//         }
//     }
//     false
// }

// fn use_skis_souples(
//     dest: &str,
//     travel_cmd: Option<&String>,
//     window_title: &str,
//     settings: &AutomationSettings,
// ) -> Result<String, String> {
//     println!("   🎿 [Action] Utilisation Skis Souples (Touche 7)");
//     let mut sequence = vec![
//         press("7", 2, 100),
//         wait(settings.map_load_delay),
//     ];

//     if let Some(cmd) = travel_cmd {
//         if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
//             println!("   📍 [Action] Ajout travel post-tp : {}", cmd);
//             sequence.extend(vec![
//                 press("space", 1, 0),
//                 wait(settings.chat_type_delay),
//                 write(cmd),
//                 wait(settings.chat_type_delay),
//                 key("enter"),
//                 wait(settings.chat_validate_delay),
//                 key("enter"),
//             ]);
//         }
//     }

//     execute(sequence, window_title)?;

//     Ok(format!(
//         "Skis souples utilisés pour la destination '{}'.",
//         dest
//     ))
// }

// pub fn use_skis_rustiques(
//     dest: &str,
//     travel_cmd: Option<&String>,
//     window_title: &str,
//     settings: &AutomationSettings,
// ) -> Result<String, String> {
//     println!("   🎿 [Action] Utilisation Skis Rustiques (Touche 8)");
//     let mut sequence = vec![
//         press("8", 2, 100),
//         wait(settings.map_load_delay),
//     ];

//     if let Some(cmd) = travel_cmd {
//         if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
//             println!("   📍 [Action] Ajout travel post-tp : {}", cmd);
//             sequence.extend(vec![
//                 press("space", 1, 0),
//                 wait(settings.chat_type_delay),
//                 write(cmd),
//                 wait(settings.chat_type_delay),
//                 key("enter"),
//                 wait(settings.chat_validate_delay),
//                 key("enter"),
//             ]);
//         }
//     }

//     execute(sequence, window_title)?;

//     // CORRECTION ICI : C'était "Skis souples" dans ton message d'erreur
//     Ok(format!(
//         "Skis rustiques utilisés pour la destination '{}'.",
//         dest
//     ))
// }

// pub fn use_skis_sombres(
//     dest: &str,
//     travel_cmd: Option<&String>,
//     window_title: &str,
//     settings: &AutomationSettings,
// ) -> Result<String, String> {
//     println!("   🎿 [Action] Utilisation Skis Sombres (Touche 9)");
//     let mut sequence = vec![
//         press("9", 2, 100),
//         wait(settings.map_load_delay),
//     ];

//     if let Some(cmd) = travel_cmd {
//         if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
//             println!("   📍 [Action] Ajout travel post-tp : {}", cmd);
//             sequence.extend(vec![
//                 press("space", 1, 0),
//                 wait(settings.chat_type_delay),
//                 write(cmd),
//                 wait(settings.chat_type_delay),
//                 key("enter"),
//                 wait(settings.chat_validate_delay),
//                 key("enter"),
//             ]);
//         }
//     }

//     execute(sequence, window_title)?;

//     Ok(format!(
//         "Skis sombres utilisés pour la destination '{}'.",
//         dest
//     ))
// }

// pub fn use_skis_glissants(
//     dest: &str,
//     travel_cmd: Option<&String>,
//     window_title: &str,
//     settings: &AutomationSettings,
// ) -> Result<String, String> {
//     println!("   🎿 [Action] Utilisation Skis Glissants (Touche 0)");
//     let mut sequence = vec![
//         press("0", 2, 100),
//         wait(settings.map_load_delay),
//     ];

//     if let Some(cmd) = travel_cmd {
//         if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
//             println!("   📍 [Action] Ajout travel post-tp : {}", cmd);
//             sequence.extend(vec![
//                 press("space", 1, 0),
//                 wait(settings.chat_type_delay),
//                 write(cmd),
//                 wait(settings.chat_type_delay),
//                 key("enter"),
//                 wait(settings.chat_validate_delay),
//                 key("enter"),
//             ]);
//         }
//     }

//     execute(sequence, window_title)?;

//     Ok(format!(
//         "Skis glissants utilisés pour la destination '{}'.",
//         dest
//     ))
// }

// fn extract_coordinates(cmd: &str) -> Option<(i32, i32)> {
//     let clean = cmd.trim().trim_start_matches("/travel ").trim();
//     let parts: Vec<&str> = clean.split(',').collect();

//     if parts.len() == 2 {
//         let x = parts[0].trim().parse::<i32>().ok()?;
//         let y = parts[1].trim().parse::<i32>().ok()?;
//         return Some((x, y));
//     }
//     None
// }

// pub fn get_name_of_closest_city_zaap(dest: (i32, i32)) -> String {
//     let cities = [
//         ("Frigost", POS_FRIGOST),
//         ("Sufokia", POS_SUFOKIA),
//         ("Brakmar", POS_BRAKMAR),
//         ("Bonta", POS_BONTA),
//     ];

//     let (dx, dy) = dest;
//     let mut min_dist = i32::MAX;
//     let mut best_city = "Bonta";

//     for (name, (cx, cy)) in cities.iter() {
//         let dist = (dx - cx).pow(2) + (dy - cy).pow(2);
//         if dist < min_dist {
//             min_dist = dist;
//             best_city = name;
//         }
//     }
//     println!("   📐 Ville la plus proche de [{},{}] : {}", dx, dy, best_city);
//     best_city.to_string()
// }

// // Fonction générique pour chat avec settings
// pub fn send_chat_command(
//     command: &str,
//     window_title: &str,
//     settings: &AutomationSettings,
// ) -> Result<(), String> {
//     let sequence = vec![
//         press("space", 1, 0),
//         wait(settings.chat_type_delay),
//         write(command),
//         wait(settings.chat_type_delay),
//         key("enter"),
//         wait(settings.chat_validate_delay),
//         key("enter"),
//     ];

//     execute(sequence, window_title)
// }

// pub fn travel_with_zaap(
//     zaap_name: &str,
//     travel_cmd: Option<&String>,
//     window_title: &str,
//     settings: &AutomationSettings,
// ) -> Result<String, String> {
//     let candidates: Vec<String> = ZAAP_NAMES.iter().map(|s| s.to_string()).collect();
//     let clean_name = crate::methods::text_utils::correct_text(zaap_name, &candidates);

//     if clean_name != zaap_name {
//         println!(
//             "   ✨ Auto-Correction Zaap : '{}' -> '{}'",
//             zaap_name, clean_name
//         );
//     } else {
//         println!("   ✨ Nom du Zaap valide : '{}'", clean_name);
//     }

//     // Séquence Zaap utilisant les settings
//     let mut sequence = vec![
//         key("h"),
//         wait_image(include_bytes!("../../resources/ui/zaap.png"), "Zaap Input"),
//         click(POS_ZAAP_INPUT),
//         wait_image(include_bytes!("../../resources/ui/zaap_text.png"), "Zaap Text"),
//         write(&clean_name),
//         key("enter"),
//         wait(settings.map_load_delay),
//     ];

//     // Ajout du Travel si nécessaire
//     if let Some(cmd) = travel_cmd {
//         if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
//             println!("   📍 [Zaap] Ajout travel final : {}", cmd);
//             sequence.extend(vec![
//                 press("space", 1, 0),
//                 wait(settings.chat_type_delay),
//                 write(cmd),
//                 wait(settings.chat_type_delay),
//                 key("enter"),
//                 wait(settings.chat_validate_delay),
//                 key("enter"),
//             ]);
//         }
//     }

//     execute(sequence, window_title)?;

//     Ok(format!(
//         "Zaap '{}' effectué (Travel: {})",
//         clean_name,
//         travel_cmd.is_some()
//     ))
// }

// fn get_zaapi_category_pos(zaapi_name: &str) -> (i32, i32) {
//     let name_lower = zaapi_name.to_lowercase();
//     let pos = if name_lower.contains("atelier") {
//         POS_TAB_ZAAPI_ATELIER
//     } else if name_lower.contains("hotel") || name_lower.contains("hôtel") {
//         POS_TAB_ZAAPI_HOTEL
//     } else {
//         POS_TAB_ZAAPI_DIVERS
//     };
//     // println!("   🔍 Catégorie Zaapi pour '{}' -> {:?}", zaapi_name, pos);
//     pos
// }

// fn run_zaapi_sequence(
//     zaapi_name: &str,
//     travel_cmd: Option<&String>,
//     window_title: &str,
//     settings: &AutomationSettings,
// ) -> Result<String, String> {
//     let closest_city_name = {
//         if let Some(cmd) = travel_cmd {
//             if let Some(coords) = extract_coordinates(cmd) {
//                 get_name_of_closest_city_zaap(coords)
//             } else {
//                 "Bonta".to_string()
//             }
//         } else {
//             "Bonta".to_string()
//         }
//     };

//     println!("   🌆 Ville de référence pour Zaapi : {}", closest_city_name);

//     let pos_zaapi = match closest_city_name.as_str() {
//         "Sufokia" => POS_ZAAPI_SUFOKIA,
//         "Brakmar" => POS_ZAAPI_BRAKMAR,
//         "Frigost" => POS_ZAAPI_FRIGOST,
//         _ => POS_ZAAPI_BONTA,
//     };

//     let zaapi_wait_time = match closest_city_name.as_str() {
//         "Sufokia" => settings.walk_sufokia,
//         "Brakmar" => settings.walk_brakmar,
//         "Frigost" => settings.walk_frigost,
//         _ => settings.walk_bonta,
//     };
    
//     println!("   ⏱️ Temps de marche vers Zaapi : {}ms", zaapi_wait_time);

//     let mut sequence = vec![
//         key("h"),
//         wait(settings.ui_open_delay),
//         click(POS_ZAAP_INPUT),
//         wait(settings.input_react_delay),
//         write(closest_city_name.as_str()),
//         key("enter"),
//         wait(settings.map_load_delay),
//         click(pos_zaapi),
//         wait(zaapi_wait_time),
//         click(get_zaapi_category_pos(zaapi_name)),
//         wait(settings.input_react_delay),
//         click(POS_INPUT_TEXT_ZAAPI),
//         write(zaapi_name),
//         key("enter"),
//         wait(settings.map_load_delay),
//         wait(settings.chat_validate_delay),
//     ];

//     if let Some(cmd) = travel_cmd {
//         if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
//             sequence.extend(vec![
//                 press("space", 1, 0),
//                 wait(settings.chat_type_delay),
//                 write(cmd),
//                 wait(settings.chat_type_delay),
//                 key("enter"),
//                 wait(settings.chat_validate_delay),
//                 key("enter"),
//             ]);
//         }
//     }

//     execute(sequence, window_title)?;

//     Ok(format!(
//         "Séquence Zaap '{}' + Zaapi '{}' terminée.",
//         closest_city_name, zaapi_name
//     ))
// }

// fn run_zaap_zaapi_sequence(
//     zaap_name: &str,
//     zaapi_name: &str,
//     travel_cmd: Option<&String>,
//     window_title: &str,
//     settings: &AutomationSettings,
// ) -> Result<String, String> {
//     // Note : Ici on force la ville de départ avec le zaap_name fourni par le guide
//     // Cependant, le code original recalculait la ville la plus proche via `travel_cmd`.
//     // J'ai laissé ta logique originale, mais ajouté des logs pour vérifier.
    
//     let closest_city_name = {
//         if let Some(cmd) = travel_cmd {
//             if let Some(coords) = extract_coordinates(cmd) {
//                 get_name_of_closest_city_zaap(coords)
//             } else {
//                 "Bonta".to_string()
//             }
//         } else {
//             "Bonta".to_string()
//         }
//     };
    
//     println!("   🌆 [Zaap->Zaapi] Ville calculée via coords : {}", closest_city_name);
//     if closest_city_name.to_lowercase() != zaap_name.to_lowercase() {
//          println!("   ⚠️ ATTENTION: Le guide demande le Zaap '{}' mais la destination finale suggère '{}'", zaap_name, closest_city_name);
//     }

//     let pos_zaapi = match closest_city_name.as_str() {
//         "Sufokia" => POS_ZAAPI_SUFOKIA,
//         "Brakmar" => POS_ZAAPI_BRAKMAR,
//         "Frigost" => POS_ZAAPI_FRIGOST,
//         _ => POS_ZAAPI_BONTA,
//     };

//     let zaapi_wait_time = match closest_city_name.as_str() {
//         "Sufokia" => settings.walk_sufokia,
//         "Brakmar" => settings.walk_brakmar,
//         "Frigost" => settings.walk_frigost,
//         _ => settings.walk_bonta,
//     };

//     let mut sequence = vec![
//         key("h"),
//         wait(settings.ui_open_delay),
//         click(POS_ZAAP_INPUT),
//         wait(settings.input_react_delay),
//         write(zaap_name), // On utilise explicitement le nom du Zaap du guide ici
//         key("enter"),
//         wait(settings.map_load_delay),
//         click(pos_zaapi),
//         wait(zaapi_wait_time),
//         click(get_zaapi_category_pos(zaapi_name)),
//         wait(settings.input_react_delay),
//         click(POS_INPUT_TEXT_ZAAPI),
//         write(zaapi_name),
//         key("enter"),
//         wait(settings.map_load_delay),
//         wait(settings.chat_validate_delay),
//     ];

//     if let Some(cmd) = travel_cmd {
//         if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
//             sequence.extend(vec![
//                 press("space", 1, 0),
//                 wait(settings.chat_type_delay),
//                 write(cmd),
//                 wait(settings.chat_type_delay),
//                 key("enter"),
//                 wait(settings.chat_validate_delay),
//                 key("enter"),
//             ]);
//         }
//     }

//     execute(sequence, window_title)?;

//     Ok(format!(
//         "Séquence Zaap '{}' + Zaapi '{}' terminée.",
//         closest_city_name, zaapi_name
//     ))
// }

use super::config::*;
use super::engine::{click, execute, key, press, wait, write, wait_image};
use super::guide_parser::GuideResult;
use super::potions;
use crate::settings::AutomationSettings;
use crate::window_manager::WindowManager;
use std::thread;
use std::time::Duration;
use tauri::AppHandle; // 👈 Import nécessaire pour le Path Resolver

// On ajoute app_handle dans la signature
pub fn execute_step_automation(
    app_handle: &AppHandle, // 👈 Ajouté ici
    step: &GuideResult,
    window_title: &str,
    settings: &AutomationSettings,
) -> Result<String, String> {
    println!("🚀 [Automation] Démarrage de l'étape : {:?}", step.macro_type);
    
    let win_manager = WindowManager::new();
    win_manager.focus_by_title(window_title)?;
    println!("✅ [Automation] Fenêtre '{}' focus.", window_title);

    thread::sleep(Duration::from_millis(100));

    match step.macro_type.as_str() {
        "classic" => {
            if let Some(cmd) = &step.travel_cmd {
                println!("🚶 [Classic] Déplacement vers : {}", cmd);
                // On passe app_handle
                send_chat_command(app_handle, cmd, window_title, settings)?;
                Ok("Travel classique effectué.".to_string())
            } else {
                println!("🛑 [Classic] Aucun déplacement requis.");
                Ok("Aucun déplacement requis.".to_string())
            }
        }
        "potion_direct" => {
            if let Some(cmd) = &step.travel_cmd {
                println!("🧪 [Potion] Utilisation de : {}", cmd);
                match cmd.as_str() {
                    "potion_bonta" => potions::potion_bonta(window_title)?,
                    "potion_brakmar" => potions::potion_brakmar(window_title)?,
                    _ => return Err(format!("Potion inconnue : {}", cmd)),
                };
            }
            Ok("Potion directe utilisée.".to_string())
        },
        "skis_souples" => {
            if let Some(dest) = &step.travel_cmd {
                println!("🎿 [Skis Souples] Vers : {}", dest);
                // On passe app_handle
                use_skis_souples(app_handle, dest, step.travel_cmd.as_ref(), window_title, settings)?;
            }
            Ok("Skis souples utilisés.".to_string())
        },
        "skis_rustiques" => {
            if let Some(dest) = &step.travel_cmd {
                println!("🎿 [Skis Rustiques] Vers : {}", dest);
                use_skis_rustiques(app_handle, dest, step.travel_cmd.as_ref(), window_title, settings)?;
            }
            Ok("Skis rustiques utilisés.".to_string())
        },
        "skis_sombres" => {
            if let Some(dest) = &step.travel_cmd {
                println!("🎿 [Skis Sombres] Vers : {}", dest);
                use_skis_sombres(app_handle, dest, step.travel_cmd.as_ref(), window_title, settings)?;
            }
            Ok("Skis sombres utilisés.".to_string())
        },
        "skis_glissants" => {
            if let Some(dest) = &step.travel_cmd {
                println!("🎿 [Skis Glissants] Vers : {}", dest);
                use_skis_glissants(app_handle, dest, step.travel_cmd.as_ref(), window_title, settings)?;
            }
            Ok("Skis glissants utilisés.".to_string())
        },
        "zaap" => {
            if let Some(dest) = &step.macro_arg {
                println!("🌀 [Zaap] Destination : {}", dest);
                // On passe app_handle
                travel_with_zaap(app_handle, dest, step.travel_cmd.as_ref(), window_title, settings)
            } else {
                Err("Nom du Zaap manquant".to_string())
            }
        }
        "zaapi" => {
            if let Some(dest) = &step.macro_arg {
                println!("🚕 [Zaapi] Destination : {}", dest);
                run_zaapi_sequence(app_handle, dest, step.travel_cmd.as_ref(), window_title, settings)
            } else {
                Err("Nom du Zaapi manquant".to_string())
            }
        }
        "zaap_zaapi" => {
            if let (Some(zaap_name), Some(zaapi_name)) = (&step.macro_arg, &step.macro_arg2) {
                println!("🌀🚕 [Zaap -> Zaapi] {} -> {}", zaap_name, zaapi_name);
                run_zaap_zaapi_sequence(
                    app_handle, // 👈
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
            println!("🧪🚕 [Potion -> Zaapi]");
            if let Some(potion_cmd) = &step.macro_arg2 {
                println!("   -> Potion : {}", potion_cmd);
                match potion_cmd.as_str() {
                    "potion_bonta" => potions::potion_bonta(window_title)?,
                    "potion_brakmar" => potions::potion_brakmar(window_title)?,
                    _ => return Err("Potion d'optimisation inconnue".to_string()),
                };
                println!("   -> Attente animation potion : {}ms", settings.potion_anim_delay);
                thread::sleep(Duration::from_millis(settings.potion_anim_delay));
            }
            if let Some(dest) = &step.macro_arg {
                println!("   -> Zaapi vers : {}", dest);
                run_zaapi_sequence(app_handle, dest, step.travel_cmd.as_ref(), window_title, settings)
            } else {
                Err("Destination Zaapi manquante".to_string())
            }
        }
        _ => {
            println!("❌ Type de macro inconnu : {}", step.macro_type);
            Err(format!("Type de macro inconnu : {}", step.macro_type))
        },
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

// --- Fonctions Skis ---
// Elles prennent maintenant app_handle pour l'envoyer à execute()

fn use_skis_souples(
    app_handle: &AppHandle,
    dest: &str,
    travel_cmd: Option<&String>,
    window_title: &str,
    settings: &AutomationSettings,
) -> Result<String, String> {
    println!("   🎿 [Action] Utilisation Skis Souples (Touche 7)");
    let mut sequence = vec![
        press("7", 2, 100),
        wait(settings.map_load_delay),
    ];

    if let Some(cmd) = travel_cmd {
        if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
            println!("   📍 [Action] Ajout travel post-tp : {}", cmd);
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

    execute(app_handle, sequence, window_title)?; // 👈 app_handle ici

    Ok(format!("Skis souples utilisés pour la destination '{}'.", dest))
}

pub fn use_skis_rustiques(
    app_handle: &AppHandle,
    dest: &str,
    travel_cmd: Option<&String>,
    window_title: &str,
    settings: &AutomationSettings,
) -> Result<String, String> {
    println!("   🎿 [Action] Utilisation Skis Rustiques (Touche 8)");
    let mut sequence = vec![
        press("8", 2, 100),
        wait(settings.map_load_delay),
    ];

    if let Some(cmd) = travel_cmd {
        if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
            println!("   📍 [Action] Ajout travel post-tp : {}", cmd);
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

    execute(app_handle, sequence, window_title)?;

    Ok(format!("Skis rustiques utilisés pour la destination '{}'.", dest))
}

pub fn use_skis_sombres(
    app_handle: &AppHandle,
    dest: &str,
    travel_cmd: Option<&String>,
    window_title: &str,
    settings: &AutomationSettings,
) -> Result<String, String> {
    println!("   🎿 [Action] Utilisation Skis Sombres (Touche 9)");
    let mut sequence = vec![
        press("9", 2, 100),
        wait(settings.map_load_delay),
    ];

    if let Some(cmd) = travel_cmd {
        if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
            println!("   📍 [Action] Ajout travel post-tp : {}", cmd);
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

    execute(app_handle, sequence, window_title)?;

    Ok(format!("Skis sombres utilisés pour la destination '{}'.", dest))
}

pub fn use_skis_glissants(
    app_handle: &AppHandle,
    dest: &str,
    travel_cmd: Option<&String>,
    window_title: &str,
    settings: &AutomationSettings,
) -> Result<String, String> {
    println!("   🎿 [Action] Utilisation Skis Glissants (Touche 0)");
    let mut sequence = vec![
        press("0", 2, 100),
        wait(settings.map_load_delay),
    ];

    if let Some(cmd) = travel_cmd {
        if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
            println!("   📍 [Action] Ajout travel post-tp : {}", cmd);
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

    execute(app_handle, sequence, window_title)?;

    Ok(format!("Skis glissants utilisés pour la destination '{}'.", dest))
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
    println!("   📐 Ville la plus proche de [{},{}] : {}", dx, dy, best_city);
    best_city.to_string()
}

pub fn send_chat_command(
    app_handle: &AppHandle,
    command: &str,
    window_title: &str,
    settings: &AutomationSettings,
) -> Result<(), String> {
    let sequence = vec![
        press("space", 1, 0),
        wait(settings.chat_type_delay),
        write(command),
        wait(settings.chat_type_delay),
        key("enter"),
        wait(settings.chat_validate_delay),
        key("enter"),
    ];

    execute(app_handle, sequence, window_title)
}

// --- LOGIQUE ZAAP AVEC VISION ---

pub fn travel_with_zaap(
    app_handle: &AppHandle,
    zaap_name: &str,
    travel_cmd: Option<&String>,
    window_title: &str,
    settings: &AutomationSettings,
) -> Result<String, String> {
    let candidates: Vec<String> = ZAAP_NAMES.iter().map(|s| s.to_string()).collect();
    let clean_name = crate::methods::text_utils::correct_text(zaap_name, &candidates);

    if clean_name != zaap_name {
        println!("   ✨ Auto-Correction Zaap : '{}' -> '{}'", zaap_name, clean_name);
    } else {
        println!("   ✨ Nom du Zaap valide : '{}'", clean_name);
    }

    // --- SÉQUENCE ZAAP ---
    // Note : On utilise "ui/zaap.png" (chemin relatif) au lieu de include_bytes!
    // Le PathResolver de Tauri trouvera le fichier réel.
    let mut sequence = vec![
        key("h"),
        // On attend que l'interface Zaap s'ouvre VISUELLEMENT
        wait_image("resources/ui/zaap.png"),
        click(POS_ZAAP_INPUT),
        // On attend que le curseur soit prêt ou que le texte change (optionnel mais robuste)
        wait_image("resources/ui/zaap_text.png"), 
        write(&clean_name),
        key("enter"),
        wait(settings.map_load_delay),
    ];

    if let Some(cmd) = travel_cmd {
        if !is_zaap_destination(cmd, ZAAP_POSITIONS) {
            println!("   📍 [Zaap] Ajout travel final : {}", cmd);
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

    execute(app_handle, sequence, window_title)?;

    Ok(format!("Zaap '{}' effectué (Travel: {})", clean_name, travel_cmd.is_some()))
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
    app_handle: &AppHandle,
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

    println!("   🌆 Ville de référence pour Zaapi : {}", closest_city_name);

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
    
    // --- SÉQUENCE ZAAPI ---
    // Pareil ici : On remplace le wait fixe par wait_image
    let mut sequence = vec![
        key("h"),
        wait_image("resources/ui/zaap.png"), // On confirme l'ouverture visuellement
        click(POS_ZAAP_INPUT),
        wait_image("resources/ui/zaap_text.png"),
        write(closest_city_name.as_str()),
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

    execute(app_handle, sequence, window_title)?;

    Ok(format!("Séquence Zaap '{}' + Zaapi '{}' terminée.", closest_city_name, zaapi_name))
}

fn run_zaap_zaapi_sequence(
    app_handle: &AppHandle,
    zaap_name: &str,
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

    let zaapi_wait_time = match closest_city_name.as_str() {
        "Sufokia" => settings.walk_sufokia,
        "Brakmar" => settings.walk_brakmar,
        "Frigost" => settings.walk_frigost,
        _ => settings.walk_bonta,
    };

    let mut sequence = vec![
        key("h"),
        wait_image("resources/ui/zaap.png"), // Confirmation visuelle
        click(POS_ZAAP_INPUT),
        wait_image("resources/ui/zaap_text.png"), // Attente visuelle du texte
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

    execute(app_handle, sequence, window_title)?;

    Ok(format!("Séquence Zaap '{}' + Zaapi '{}' terminée.", closest_city_name, zaapi_name))
}