use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter, Manager};
use rdev::{listen, Event, EventType};

pub struct KeyListenerState {
    pub active: bool,
    pub target_keys: Vec<String>, // On stocke une liste maintenant
}

pub type SharedKeyListenerState = Arc<Mutex<KeyListenerState>>;

#[tauri::command]
pub async fn set_key_listener(
    state: tauri::State<'_, SharedKeyListenerState>,
    active: bool,
    keys: Vec<String>, // On accepte un tableau de chaînes
) -> Result<String, String> {
    let mut data = state.lock().map_err(|_| "Failed to lock state")?;
    data.active = active;
    // On convertit toutes les touches en minuscules pour la comparaison
    data.target_keys = keys.iter().map(|k| k.to_lowercase()).collect();
    
    Ok(if active {
        format!("Écoute activée pour les touches : {:?}", data.target_keys)
    } else {
        "Écoute désactivée".to_string()
    })
}

pub fn init_background_listener(app: &AppHandle) {
    let state = app.state::<SharedKeyListenerState>();
    let state_clone = state.inner().clone();
    let app_handle = app.clone();

    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            process_event(event, &state_clone, &app_handle);
        }) {
            eprintln!("Erreur du listener clavier : {:?}", error);
        }
    });
}

fn process_event(event: Event, state: &SharedKeyListenerState, app: &AppHandle) {
    if let EventType::KeyPress(key) = event.event_type {
        let data = match state.lock() {
            Ok(d) => d,
            Err(_) => return,
        };

        if !data.active {
            return;
        }

        // Vérification simplifiée
        let detected_key = if let Some(key_name) = event.name {
            key_name.to_lowercase()
        } else {
            format!("{:?}", key).to_lowercase()
        };

        // Si la touche détectée est dans notre liste cible
        if data.target_keys.contains(&detected_key) {
            let _ = app.emit("key-detected", detected_key.clone());
            println!("Touche détectée : {}", detected_key);
        }
    }
}