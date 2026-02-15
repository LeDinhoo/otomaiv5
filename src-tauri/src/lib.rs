// mod window_manager;
// use window_manager::WindowManager;

// mod mouse_manager;
// use mouse_manager::MouseManager;

// mod methods;
// mod window_finder;

// mod settings;
// use settings::{AutomationSettings, SettingsState};

// use methods::guide_parser::{GuideParser, GuideResult};
// use methods::key_listener::{self, KeyListenerState, SharedKeyListenerState};
// use std::sync::{Arc, Mutex};

// use tauri::{AppHandle, Emitter, Manager}; // <--- AJOUT CRITIQUE ICI : "Manager"

// pub struct ActiveSession {
//     pub window_title: Mutex<Option<String>>,
//     pub target_char_name: Mutex<Option<String>>,
//     pub is_recovering: Mutex<bool>,
// }

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// // #[tauri::command]
// // fn send_chat_command(command: String, window_title: String) -> Result<String, String> {
// //     let manager = window_manager::WindowManager::new();
// //     manager.focus_by_title(&window_title)?;
// //     methods::type_text::type_text_fast(&command)?;
// //     methods::press_key::press_key_multiple_times("enter", 1, Some(100))?;
// //     Ok("Commande envoyée".to_string())
// // }

// #[tauri::command]
// fn send_chat_command(
//     command: String, 
//     window_title: String, 
//     state: tauri::State<'_, SettingsState>
// ) -> Result<String, String> {
//     // 1. Log à l'entrée de la fonction
//     println!("🚀 [Rust] send_chat_command appelée !");
//     println!("   - Commande : {}", command);
//     println!("   - Fenêtre : {}", window_title);

//     let win_manager = WindowManager::new();
//     win_manager.focus_by_title(&window_title)?;

//     // 2. Récupération des settings
//     let settings = match state.0.lock() {
//         Ok(s) => s.clone(),
//         Err(_) => {
//             println!("❌ [Rust] CRASH : Impossible de verrouiller les settings (Mutex Poisoned)");
//             return Err("Erreur interne settings".to_string());
//         }
//     };
//     println!("⚙️ [Rust] Settings chargés. Délai frappe : {}ms", settings.chat_type_delay);

//     // 3. Appel de l'automation avec gestion d'erreur explicite
//     match methods::automations::send_chat_command(&command, &window_title, &settings) {
//         Ok(_) => {
//             println!("✅ [Rust] Automation terminée avec SUCCÈS.");
//             Ok("Commande envoyée".to_string())
//         },
//         Err(e) => {
//             println!("❌ [Rust] Automation ÉCHOUÉE : {}", e);
//             Err(e.to_string()) // Renvoie l'erreur au JS (promesse rejetée)
//         }
//     }
// }

// #[tauri::command]
// async fn sync_window_title(
//     character_name: String,
//     state: tauri::State<'_, ActiveSession>,
//     app_handle: AppHandle,
// ) -> Result<String, String> {
//     println!("DEBUG INPUT: Sync demandée pour {:?}", character_name);

//     {
//         let mut target_storage = state.target_char_name.lock().unwrap();
//         *target_storage = Some(character_name.clone());
//     }

//     if let Some(full_title) = window_finder::find_specific_game_window(&character_name) {
//         let mut title_storage = state.window_title.lock().unwrap();
//         *title_storage = Some(full_title.clone());

//         println!("✅ Fenêtre trouvée et synchronisée : {}", full_title);

//         let _ = app_handle.emit("sync-status", "synced");

//         Ok(full_title)
//     } else {
//         println!("❌ ECHEC Sync initiale");
//         let _ = app_handle.emit("sync-status", "lost");
//         Err(format!(
//             "Impossible de trouver une fenêtre pour '{}'.",
//             character_name
//         ))
//     }
// }

// #[tauri::command]
// async fn trigger_auto_recovery(
//     state: tauri::State<'_, ActiveSession>,
//     app_handle: AppHandle,
// ) -> Result<String, String> {
//     {
//         let mut recovering = state.is_recovering.lock().unwrap();
//         if *recovering {
//             return Ok("Récupération déjà en cours...".to_string());
//         }
//         *recovering = true;
//     }

//     let target_name = {
//         let lock = state.target_char_name.lock().unwrap();
//         match lock.clone() {
//             Some(name) => name,
//             None => {
//                 let mut recovering = state.is_recovering.lock().unwrap();
//                 *recovering = false;
//                 return Err("Aucun personnage cible défini.".to_string());
//             }
//         }
//     };

//     println!("🚨 DÉCLENCHEMENT AUTO-RECOVERY pour '{}'", target_name);

//     let _ = app_handle.emit("sync-status", "recovering");

//     let state_clone = state.inner().clone();

//     let found_title = window_finder::try_autorecovery(&target_name);

//     let mut recovering = state.is_recovering.lock().unwrap();
//     *recovering = false;

//     if let Some(title) = found_title {
//         let mut title_storage = state.window_title.lock().unwrap();
//         *title_storage = Some(title.clone());

//         println!("✅ RECOVERY SUCCÈS ! Nouvelle fenêtre : {}", title);
//         let _ = app_handle.emit("sync-status", "synced");
//         Ok("Récupération réussie".to_string())
//     } else {
//         println!("❌ RECOVERY ÉCHEC FINAL.");

//         let mut title_storage = state.window_title.lock().unwrap();
//         *title_storage = None;

//         let _ = app_handle.emit("sync-status", "lost");
//         Err("Impossible de retrouver la fenêtre après 30 essais.".to_string())
//     }
// }

// #[tauri::command]
// fn focus_window(window_title: String) -> Result<String, String> {
//     let manager = WindowManager::new();
//     manager.focus_by_title(&window_title)
// }

// #[tauri::command]
// fn click_window_at(title: String, x: i32, y: i32) -> Result<String, String> {
//     let manager = MouseManager::new();
//     manager.click_at_position(&title, x, y)
// }

// #[tauri::command]
// fn get_settings(state: tauri::State<'_, SettingsState>) -> AutomationSettings {
//     state.0.lock().unwrap().clone()
// }

// #[tauri::command]
// async fn execute_step_automation(
//     step: GuideResult,
//     state: tauri::State<'_, ActiveSession>,
//     settings_state: tauri::State<'_, SettingsState>, // <--- AJOUTE ÇA
// ) -> Result<String, String> {
//     let window_title = {
//         let title_lock = state.window_title.lock().unwrap();
//         title_lock.clone().ok_or("Aucune fenêtre synchronisée.")?
//     };

//     // CRUCIAL : On récupère une COPIE des settings MAINTENANT pour les donner au thread
//     let settings = settings_state.0.lock().unwrap().clone();

//     println!("DEBUG - Titre : {}", window_title);

//     // On passe les settings à la fonction
//     let result = std::thread::spawn(move || {
//         methods::automations::execute_step_automation(&step, &window_title, &settings)
//     })
//     .join();

//     match result {
//         Ok(res) => res,
//         Err(_) => Err("Crash critique thread.".to_string()),
//     }
// }

// #[tauri::command]
// fn save_settings_cmd(
//     app: tauri::AppHandle,
//     state: tauri::State<'_, SettingsState>,
//     new_settings: AutomationSettings,
// ) -> Result<(), String> {
//     // 1. Mise à jour mémoire
//     let mut lock = state.0.lock().unwrap();
//     *lock = new_settings.clone();

//     // 2. Sauvegarde disque
//     settings::save_settings(&app, &new_settings)
// }

// #[tauri::command]
// fn get_game_title_by_name(character_name: String) -> Result<String, String> {
//     window_finder::find_specific_game_window(&character_name).ok_or_else(|| {
//         format!(
//             "Fenêtre pour le personnage '{}' introuvable.",
//             character_name
//         )
//     })
// }

// #[tauri::command]
// fn press_key(key: String, count: u32, delay: Option<u64>) -> Result<String, String> {
//     methods::press_key::press_key_multiple_times(&key, count, delay)
// }

// #[tauri::command]
// fn quick_type(text: String, window_title: String) -> Result<String, String> {
//     let manager = window_manager::WindowManager::new();
//     manager.focus_by_title(&window_title)?;
//     methods::type_text::type_text_fast(&text)
// }

// #[tauri::command]
// fn use_potion_bonta(window_title: String) -> Result<String, String> {
//     methods::potions::potion_bonta(&window_title)
// }

// #[tauri::command]
// fn use_potion_brakmar(window_title: String) -> Result<String, String> {
//     methods::potions::potion_brakmar(&window_title)
// }

// #[tauri::command]
// fn parse_guide_step(html_content: String) -> GuideResult {
//     GuideParser::parse_step(&html_content)
// }

// // #[tauri::command]
// // async fn execute_step_automation(
// //     step: GuideResult,
// //     state: tauri::State<'_, ActiveSession>
// // ) -> Result<String, String> {
// //     let window_title = {
// //         let title_lock = state.window_title.lock().unwrap();
// //         title_lock.clone().ok_or("Aucune fenêtre synchronisée.")?
// //     };

// //     println!("DEBUG - Titre de la fenêtre récupéré : {}", window_title);

// //     let result = std::thread::spawn(move || {
// //         methods::automations::execute_step_automation(&step, &window_title)
// //     }).join();

// //     match result {
// //         Ok(res) => res,
// //         Err(_) => Err("Crash critique du thread d'automatisation.".to_string()),
// //     }
// // }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     let listener_state: SharedKeyListenerState = Arc::new(Mutex::new(KeyListenerState {
//         active: false,
//         target_keys: vec![],
//     }));

//     tauri::Builder::default()
//         .plugin(tauri_plugin_notification::init())
//         .plugin(tauri_plugin_fs::init())
//         .plugin(tauri_plugin_http::init())
//         .plugin(tauri_plugin_opener::init())
//         .manage(listener_state)
//         .plugin(tauri_plugin_notification::init())
//         // 1. GESTION DE SESSION
//         .manage(ActiveSession {
//             window_title: Mutex::new(None),
//             target_char_name: Mutex::new(None),
//             is_recovering: Mutex::new(false),
//         })
//         // 2. GESTION DES SETTINGS (CORRECTION ICI)
//         // On l'enregistre ici avec ::default() pour être sûr qu'il existe dès le lancement.
//         // Cela évite l'erreur "state not managed".
//         .manage(SettingsState(Mutex::new(AutomationSettings::default())))
//         .setup(|app| {
//             // 3. CHARGEMENT RÉEL (Dans le setup)
//             // Maintenant que l'app est lancée, on peut accéder aux fichiers
//             let loaded_settings = settings::load_settings(app.handle());

//             // On récupère l'état qu'on vient de créer juste au-dessus
//             let state = app.state::<SettingsState>();

//             // Et on remplace les valeurs par celles du fichier
//             *state.0.lock().unwrap() = loaded_settings;

//             // Lancement du listener clavier
//             key_listener::init_background_listener(app.handle());
//             Ok(())
//         })
//         .invoke_handler(tauri::generate_handler![
//             greet,
//             focus_window,
//             click_window_at,
//             get_game_title_by_name,
//             press_key,
//             quick_type,
//             use_potion_bonta,
//             use_potion_brakmar,
//             sync_window_title,
//             trigger_auto_recovery,
//             parse_guide_step,
//             execute_step_automation,
//             methods::key_listener::set_key_listener,
//             send_chat_command,
//             get_settings,
//             save_settings_cmd
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

mod window_manager;
use window_manager::WindowManager;

mod mouse_manager;
use mouse_manager::MouseManager;

mod methods;
mod window_finder;

mod settings;
use settings::{AutomationSettings, SettingsState};

use methods::click_mirror::{self, ClickMirrorState, SharedClickMirrorState};
use methods::guide_parser::{GuideParser, GuideResult};
use methods::key_listener::{self, KeyListenerState, SharedKeyListenerState};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter, Manager};

pub struct ActiveSession {
    pub window_title: Mutex<Option<String>>,
    pub target_char_name: Mutex<Option<String>>,
    pub is_recovering: Mutex<bool>,
    pub cancel_recovery: Arc<AtomicBool>,
}

pub struct CombatWatcherState {
    pub active: Arc<AtomicBool>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn send_chat_command(
    app_handle: AppHandle, // <--- 1. AJOUT DU HANDLE ICI
    command: String, 
    window_title: String, 
    state: tauri::State<'_, SettingsState>
) -> Result<String, String> {
    
    println!("🚀 [Rust] send_chat_command appelée !");
    println!("   - Commande : {}", command);
    println!("   - Fenêtre : {}", window_title);

    let win_manager = WindowManager::new();
    win_manager.focus_by_title(&window_title)?;

    // 2. Récupération des settings
    let settings = match state.0.lock() {
        Ok(s) => s.clone(),
        Err(_) => {
            println!("❌ [Rust] CRASH : Impossible de verrouiller les settings (Mutex Poisoned)");
            return Err("Erreur interne settings".to_string());
        }
    };
    println!("⚙️ [Rust] Settings chargés. Délai frappe : {}ms", settings.chat_type_delay);

    // 3. Appel de l'automation avec gestion d'erreur explicite
    // NOTE : Tu devras aussi mettre à jour la signature de send_chat_command dans automations.rs pour qu'elle accepte &app_handle !
    match methods::automations::send_chat_command(&app_handle, &command, &window_title, &settings) {
        Ok(_) => {
            println!("✅ [Rust] Automation terminée avec SUCCÈS.");
            Ok("Commande envoyée".to_string())
        },
        Err(e) => {
            println!("❌ [Rust] Automation ÉCHOUÉE : {}", e);
            Err(e.to_string()) 
        }
    }
}

#[tauri::command]
async fn sync_window_title(
    character_name: String,
    state: tauri::State<'_, ActiveSession>,
    app_handle: AppHandle,
) -> Result<String, String> {
    println!("DEBUG INPUT: Sync demandée pour {:?}", character_name);

    // Annuler toute recovery en cours
    state.cancel_recovery.store(true, Ordering::Relaxed);

    {
        let mut target_storage = state.target_char_name.lock().unwrap();
        *target_storage = Some(character_name.clone());
    }

    if let Some(full_title) = window_finder::find_specific_game_window(&character_name) {
        let mut title_storage = state.window_title.lock().unwrap();
        *title_storage = Some(full_title.clone());

        println!("✅ Fenêtre trouvée et synchronisée : {}", full_title);

        let _ = app_handle.emit("sync-status", "synced");

        Ok(full_title)
    } else {
        println!("❌ ECHEC Sync initiale");
        let _ = app_handle.emit("sync-status", "lost");
        Err(format!(
            "Impossible de trouver une fenêtre pour '{}'.",
            character_name
        ))
    }
}

#[tauri::command]
async fn trigger_auto_recovery(
    state: tauri::State<'_, ActiveSession>,
    app_handle: AppHandle,
) -> Result<String, String> {
    {
        let mut recovering = state.is_recovering.lock().unwrap();
        if *recovering {
            return Ok("Récupération déjà en cours...".to_string());
        }
        *recovering = true;
    }

    let target_name = {
        let lock = state.target_char_name.lock().unwrap();
        match lock.clone() {
            Some(name) => name,
            None => {
                let mut recovering = state.is_recovering.lock().unwrap();
                *recovering = false;
                return Err("Aucun personnage cible défini.".to_string());
            }
        }
    };

    println!("🚨 DÉCLENCHEMENT AUTO-RECOVERY pour '{}'", target_name);

    let _ = app_handle.emit("sync-status", "recovering");

    // Reset le flag d'annulation avant de démarrer
    state.cancel_recovery.store(false, Ordering::Relaxed);
    let cancel_flag = state.cancel_recovery.clone();

    let found_title = window_finder::try_autorecovery(&target_name, &cancel_flag);

    let mut recovering = state.is_recovering.lock().unwrap();
    *recovering = false;

    if let Some(title) = found_title {
        let mut title_storage = state.window_title.lock().unwrap();
        *title_storage = Some(title.clone());

        println!("✅ RECOVERY SUCCÈS ! Nouvelle fenêtre : {}", title);
        let _ = app_handle.emit("sync-status", "synced");
        Ok("Récupération réussie".to_string())
    } else {
        println!("❌ RECOVERY ÉCHEC FINAL.");

        let mut title_storage = state.window_title.lock().unwrap();
        *title_storage = None;

        let _ = app_handle.emit("sync-status", "lost");
        Err("Impossible de retrouver la fenêtre après 30 essais.".to_string())
    }
}

#[tauri::command]
fn focus_window(window_title: String) -> Result<String, String> {
    let manager = WindowManager::new();
    manager.focus_by_title(&window_title)
}

#[tauri::command]
fn click_window_at(title: String, x: i32, y: i32) -> Result<String, String> {
    let manager = MouseManager::new();
    manager.click_at_position(&title, x, y)
}

#[tauri::command]
fn get_settings(state: tauri::State<'_, SettingsState>) -> AutomationSettings {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
async fn execute_step_automation(
    app_handle: AppHandle,
    step: GuideResult,
    window_title: Option<String>,
    state: tauri::State<'_, ActiveSession>,
    settings_state: tauri::State<'_, SettingsState>,
) -> Result<String, String> {
    let window_title = match window_title {
        Some(t) if !t.is_empty() => t,
        _ => {
            let title_lock = state.window_title.lock().unwrap();
            title_lock.clone().ok_or("Aucune fenêtre synchronisée.")?
        }
    };

    // CRUCIAL : On récupère une COPIE des settings MAINTENANT pour les donner au thread
    let settings = settings_state.0.lock().unwrap().clone();

    println!("DEBUG - Titre : {}", window_title);

    // 3. CLONAGE DU HANDLE POUR LE THREAD
    // AppHandle est thread-safe et léger, il est fait pour être cloné.
    let app_handle_clone = app_handle.clone();

    // On passe les settings ET le handle à la fonction
    let result = std::thread::spawn(move || {
        methods::automations::execute_step_automation(
            &app_handle_clone, // <--- 4. PASSAGE DU CLONE ICI
            &step, 
            &window_title, 
            &settings
        )
    })
    .join();

    match result {
        Ok(res) => res,
        Err(_) => Err("Crash critique thread.".to_string()),
    }
}

#[tauri::command]
async fn execute_step_automation_chained(
    app_handle: AppHandle,
    step: GuideResult,
    window_titles: Vec<String>,
    settings_state: tauri::State<'_, SettingsState>,
) -> Result<String, String> {
    let settings = settings_state.0.lock().unwrap().clone();
    let app_handle_clone = app_handle.clone();

    let result = std::thread::spawn(move || {
        methods::automations::execute_step_automation_chained(
            &app_handle_clone,
            &step,
            &window_titles,
            &settings,
        )
    })
    .join();

    match result {
        Ok(res) => res,
        Err(_) => Err("Crash critique thread.".to_string()),
    }
}

#[tauri::command]
fn save_settings_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, SettingsState>,
    new_settings: AutomationSettings,
) -> Result<(), String> {
    // 1. Mise à jour mémoire
    let mut lock = state.0.lock().unwrap();
    *lock = new_settings.clone();

    // 2. Sauvegarde disque
    settings::save_settings(&app, &new_settings)
}

#[tauri::command]
fn get_game_title_by_name(character_name: String) -> Result<String, String> {
    window_finder::find_specific_game_window(&character_name).ok_or_else(|| {
        format!(
            "Fenêtre pour le personnage '{}' introuvable.",
            character_name
        )
    })
}

#[tauri::command]
fn press_key(key: String, count: u32, delay: Option<u64>) -> Result<String, String> {
    methods::press_key::press_key_multiple_times(&key, count, delay)
}

#[tauri::command]
fn quick_type(text: String, window_title: String) -> Result<String, String> {
    let manager = window_manager::WindowManager::new();
    manager.focus_by_title(&window_title)?;
    methods::type_text::type_text_fast(&text)
}

#[tauri::command]
fn use_potion_bonta(window_title: String) -> Result<String, String> {
    methods::potions::potion_bonta(&window_title)
}

#[tauri::command]
fn use_potion_brakmar(window_title: String) -> Result<String, String> {
    methods::potions::potion_brakmar(&window_title)
}

#[tauri::command]
fn parse_guide_step(html_content: String) -> GuideResult {
    GuideParser::parse_step(&html_content)
}

fn resolve_resource_path(app_handle: &AppHandle, relative: &str) -> Result<String, String> {
    let resource_path = app_handle.path()
        .resolve(relative, tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("Impossible de résoudre '{}': {}", relative, e))?;

    let path_str = resource_path.to_str()
        .ok_or("Erreur conversion chemin")?;

    Ok(if cfg!(windows) {
        path_str.trim_start_matches("\\\\?\\").to_string()
    } else {
        path_str.to_string()
    })
}

#[tauri::command]
async fn start_combat_watcher(
    combat_start_image: String,
    combat_end_images: Vec<String>,
    app_handle: AppHandle,
    state: tauri::State<'_, CombatWatcherState>,
) -> Result<String, String> {
    if state.active.load(Ordering::Relaxed) {
        return Ok("Watcher déjà actif".to_string());
    }

    let start_path = resolve_resource_path(&app_handle, &combat_start_image)?;
    let end_paths: Vec<String> = combat_end_images
        .iter()
        .map(|p| resolve_resource_path(&app_handle, p))
        .collect::<Result<Vec<_>, _>>()?;

    state.active.store(true, Ordering::Relaxed);
    let active_flag = state.active.clone();

    std::thread::spawn(move || {
        println!("👁️ [Combat Watcher] Cycle démarré");

        while active_flag.load(Ordering::Relaxed) {
            // --- Phase 1 : Détection début de combat (top 1/10, toutes les 500ms) ---
            println!("👁️ [Combat Watcher] Phase: recherche début combat...");
            loop {
                if !active_flag.load(Ordering::Relaxed) { return; }

                match methods::vision::scan_top_region(&start_path) {
                    Ok(true) => {
                        println!("⚔️ [Combat Watcher] Combat détecté !");
                        let _ = app_handle.emit("combat-detected", true);
                        // Pause pour laisser la transition de combat se faire
                        std::thread::sleep(std::time::Duration::from_secs(3));
                        break;
                    }
                    Ok(false) => {}
                    Err(e) => eprintln!("❌ [Combat Watcher] Erreur scan start: {}", e),
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }

            // --- Phase 2 : Détection fin de combat (bottom-left 1/3, toutes les 1s) ---
            println!("👁️ [Combat Watcher] Phase: recherche fin combat...");
            loop {
                if !active_flag.load(Ordering::Relaxed) { return; }

                let path_refs: Vec<&str> = end_paths.iter().map(|s| s.as_str()).collect();
                match methods::vision::scan_bottom_left_region(&path_refs) {
                    Ok(true) => {
                        println!("🏁 [Combat Watcher] Fin de combat détectée !");
                        let _ = app_handle.emit("combat-ended", true);
                        // Pause avant de relancer le cycle
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        break;
                    }
                    Ok(false) => println!("👁️ [Combat Watcher] Scan fin combat... (pas trouvé)"),
                    Err(e) => eprintln!("❌ [Combat Watcher] Erreur scan end: {}", e),
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }

        println!("🛑 [Combat Watcher] Cycle arrêté");
    });

    Ok("Watcher démarré".to_string())
}

#[tauri::command]
fn stop_combat_watcher(
    state: tauri::State<'_, CombatWatcherState>,
) -> Result<String, String> {
    state.active.store(false, Ordering::Relaxed);
    Ok("Watcher arrêté".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let listener_state: SharedKeyListenerState = Arc::new(Mutex::new(KeyListenerState {
        active: false,
        target_keys: vec![],
    }));

    let click_mirror_state: SharedClickMirrorState = Arc::new(Mutex::new(ClickMirrorState {
        paused: false,
        titles: vec![],
        focus_keybinds: std::collections::HashMap::new(),
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .manage(listener_state)
        .manage(click_mirror_state)
        .plugin(tauri_plugin_notification::init())
        // COMBAT WATCHER
        .manage(CombatWatcherState {
            active: Arc::new(AtomicBool::new(false)),
        })
        // 1. GESTION DE SESSION
        .manage(ActiveSession {
            window_title: Mutex::new(None),
            target_char_name: Mutex::new(None),
            is_recovering: Mutex::new(false),
            cancel_recovery: Arc::new(AtomicBool::new(false)),
        })
        // 2. GESTION DES SETTINGS
        .manage(SettingsState(Mutex::new(AutomationSettings::default())))
        .setup(|app| {
            // 3. CHARGEMENT RÉEL
            let loaded_settings = settings::load_settings(app.handle());

            let state = app.state::<SettingsState>();
            *state.0.lock().unwrap() = loaded_settings;

            key_listener::init_background_listener(app.handle());

            let mirror_state = app.state::<SharedClickMirrorState>();
            click_mirror::init_click_mirror(mirror_state.inner().clone(), app.handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            focus_window,
            click_window_at,
            get_game_title_by_name,
            press_key,
            quick_type,
            use_potion_bonta,
            use_potion_brakmar,
            sync_window_title,
            trigger_auto_recovery,
            parse_guide_step,
            execute_step_automation,
            execute_step_automation_chained,
            methods::key_listener::set_key_listener,
            methods::click_mirror::set_click_mirror,
            methods::click_mirror::pause_click_mirror,
            methods::click_mirror::set_focus_keybinds,
            send_chat_command,
            get_settings,
            save_settings_cmd,
            start_combat_watcher,
            stop_combat_watcher
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
