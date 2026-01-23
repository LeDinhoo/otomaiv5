mod window_manager;
use window_manager::WindowManager;

mod mouse_manager;
use mouse_manager::MouseManager;

mod methods;
mod window_finder;

mod settings;
use settings::{AutomationSettings, SettingsState};

use methods::guide_parser::{GuideParser, GuideResult};
use methods::key_listener::{self, KeyListenerState, SharedKeyListenerState};
use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter, Manager}; // <--- AJOUT CRITIQUE ICI : "Manager"

pub struct ActiveSession {
    pub window_title: Mutex<Option<String>>,
    pub target_char_name: Mutex<Option<String>>,
    pub is_recovering: Mutex<bool>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn send_chat_command(command: String, window_title: String) -> Result<String, String> {
    let manager = window_manager::WindowManager::new();
    manager.focus_by_title(&window_title)?;
    methods::type_text::type_text_fast(&command)?;
    methods::press_key::press_key_multiple_times("enter", 1, Some(100))?;
    Ok("Commande envoyée".to_string())
}

#[tauri::command]
async fn sync_window_title(
    character_name: String,
    state: tauri::State<'_, ActiveSession>,
    app_handle: AppHandle,
) -> Result<String, String> {
    println!("DEBUG INPUT: Sync demandée pour {:?}", character_name);

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

    let state_clone = state.inner().clone();

    let found_title = window_finder::try_autorecovery(&target_name);

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
    step: GuideResult,
    state: tauri::State<'_, ActiveSession>,
    settings_state: tauri::State<'_, SettingsState>, // <--- AJOUTE ÇA
) -> Result<String, String> {
    let window_title = {
        let title_lock = state.window_title.lock().unwrap();
        title_lock.clone().ok_or("Aucune fenêtre synchronisée.")?
    };

    // CRUCIAL : On récupère une COPIE des settings MAINTENANT pour les donner au thread
    let settings = settings_state.0.lock().unwrap().clone();

    println!("DEBUG - Titre : {}", window_title);

    // On passe les settings à la fonction
    let result = std::thread::spawn(move || {
        methods::automations::execute_step_automation(&step, &window_title, &settings)
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

// #[tauri::command]
// async fn execute_step_automation(
//     step: GuideResult,
//     state: tauri::State<'_, ActiveSession>
// ) -> Result<String, String> {
//     let window_title = {
//         let title_lock = state.window_title.lock().unwrap();
//         title_lock.clone().ok_or("Aucune fenêtre synchronisée.")?
//     };

//     println!("DEBUG - Titre de la fenêtre récupéré : {}", window_title);

//     let result = std::thread::spawn(move || {
//         methods::automations::execute_step_automation(&step, &window_title)
//     }).join();

//     match result {
//         Ok(res) => res,
//         Err(_) => Err("Crash critique du thread d'automatisation.".to_string()),
//     }
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let listener_state: SharedKeyListenerState = Arc::new(Mutex::new(KeyListenerState {
        active: false,
        target_keys: vec![],
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .manage(listener_state)
        .plugin(tauri_plugin_notification::init())
        // 1. GESTION DE SESSION
        .manage(ActiveSession {
            window_title: Mutex::new(None),
            target_char_name: Mutex::new(None),
            is_recovering: Mutex::new(false),
        })
        // 2. GESTION DES SETTINGS (CORRECTION ICI)
        // On l'enregistre ici avec ::default() pour être sûr qu'il existe dès le lancement.
        // Cela évite l'erreur "state not managed".
        .manage(SettingsState(Mutex::new(AutomationSettings::default())))
        .setup(|app| {
            // 3. CHARGEMENT RÉEL (Dans le setup)
            // Maintenant que l'app est lancée, on peut accéder aux fichiers
            let loaded_settings = settings::load_settings(app.handle());

            // On récupère l'état qu'on vient de créer juste au-dessus
            let state = app.state::<SettingsState>();

            // Et on remplace les valeurs par celles du fichier
            *state.0.lock().unwrap() = loaded_settings;

            // Lancement du listener clavier
            key_listener::init_background_listener(app.handle());
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
            methods::key_listener::set_key_listener,
            send_chat_command,
            get_settings,
            save_settings_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
