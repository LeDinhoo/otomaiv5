use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use device_query::{DeviceQuery, DeviceState, Keycode}; // Nouvelle librairie

// Structure d'état
pub struct KeyListenerState {
    pub active: bool,
    pub target_keys: Vec<String>,
}

pub type SharedKeyListenerState = Arc<Mutex<KeyListenerState>>;

#[tauri::command]
pub async fn set_key_listener(
    state: tauri::State<'_, SharedKeyListenerState>,
    active: bool,
    keys: Vec<String>,
) -> Result<String, String> {
    let mut data = state.lock().map_err(|_| "Failed to lock state")?;
    data.active = active;
    // On stocke les touches en minuscule pour comparer facilement
    data.target_keys = keys.iter().map(|k| k.to_lowercase()).collect();
    
    Ok(if active {
        format!("Écoute (Polling) activée pour : {:?}", data.target_keys)
    } else {
        "Écoute désactivée".to_string()
    })
}

pub fn init_background_listener(app: &AppHandle) {
    let state = app.state::<SharedKeyListenerState>();
    let state_clone = state.inner().clone();
    let app_handle = app.clone();

    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut previous_keys: Vec<Keycode> = vec![];

        loop {
            // 1. Récupérer l'état actuel du listener (Active ou Pas ?)
            let (is_active, targets) = {
                let lock = state_clone.lock().unwrap();
                (lock.active, lock.target_keys.clone())
            };

            if is_active {
                // 2. Demander au matériel quelles touches sont enfoncées
                let current_keys = device_state.get_keys();

                // 3. Détecter uniquement les NOUVELLES touches (Front montant)
                // Si une touche est dans 'current' mais pas dans 'previous', c'est une frappe.
                for key in &current_keys {
                    if !previous_keys.contains(key) {
                        let key_string = key.to_string().to_lowercase();
                        
                        // Petite astuce : device_query retourne "Key1" pour "1", "A" pour "a".
                        // On nettoie un peu si nécessaire, ou on compare brut.
                        
                        if targets.contains(&key_string) {
                            println!("✅ Touche détectée (Polling) : {}", key_string);
                            let _ = app_handle.emit("key-detected", key_string);
                        }
                    }
                }

                // Mémoriser l'état pour le prochain tour
                previous_keys = current_keys;
            }

            // 4. Pause de 50ms (C'est ta sécurité anti-rebond ET ça économise le CPU)
            // C'est assez rapide pour être réactif, assez lent pour éviter les doubles clics
            thread::sleep(Duration::from_millis(50));
        }
    });
}