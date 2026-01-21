use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;

// La structure qui contient TOUS tes délais
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationSettings {
    // UI & Interface
    pub ui_open_delay: u64,        // "H" Havre-sac (1200ms)
    pub input_react_delay: u64,    // Clic input -> Écriture (100ms)
    pub map_load_delay: u64,       // Chargement map après Zaap (2000ms)
    
    // Chat
    pub chat_type_delay: u64,      // Vitesse écriture chat (50ms)
    pub chat_validate_delay: u64,  // Touche Entrée après cmd (200ms)

    // Gameplay
    pub potion_anim_delay: u64,    // Animation Potion Bonta/Brak (4000ms)

    // Marche vers Zaapi (Tes constantes actuelles)
    pub walk_bonta: u64,           // 1500ms
    pub walk_brakmar: u64,         // 2500ms
    pub walk_sufokia: u64,         // 1500ms
    pub walk_frigost: u64,         // 1500ms
}

// Valeurs par défaut (si le fichier config n'existe pas encore)
impl Default for AutomationSettings {
    fn default() -> Self {
        Self {
            ui_open_delay: 1200,
            input_react_delay: 100,
            map_load_delay: 2000,
            chat_type_delay: 50,
            chat_validate_delay: 200,
            potion_anim_delay: 4000,
            walk_bonta: 1500,
            walk_brakmar: 2500,
            walk_sufokia: 1500,
            walk_frigost: 1500,
        }
    }
}

pub struct SettingsState(pub Mutex<AutomationSettings>);

// --- FONCTIONS UTILES ---

fn get_config_path(app: &AppHandle) -> PathBuf {
    app.path().app_config_dir().unwrap().join("settings.json")
}

pub fn load_settings(app: &AppHandle) -> AutomationSettings {
    let path = get_config_path(app);
    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AutomationSettings::default()
    }
}

pub fn save_settings(app: &AppHandle, settings: &AutomationSettings) -> Result<(), String> {
    let path = get_config_path(app);
    // Créer le dossier si besoin
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}