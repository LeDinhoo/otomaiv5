use windows::Win32::Foundation::{RECT};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowRect, FindWindowW};
use windows::core::{PCWSTR, HSTRING};
// Importations spécifiques à Enigo v0.2
use enigo::{Enigo, Mouse, Settings, Coordinate, Button, Direction}; 

pub struct MouseManager;

impl MouseManager {
    pub fn new() -> Self {
        Self
    }

    pub fn click_center_of_window(&self, title: &str) -> Result<String, String> {
        unsafe {
            // 1. Recherche de la fenêtre
            let window_title = HSTRING::from(title);
            let title_pcwstr = PCWSTR::from_raw(window_title.as_ptr());
            
            let result = FindWindowW(None, title_pcwstr);
            let hwnd = match result {
                Ok(h) if h.0.is_null() => return Err(format!("Fenêtre '{}' non trouvée", title)),
                Ok(h) => h,
                Err(e) => return Err(format!("Erreur Windows: {}", e)),
            };

            // 2. Récupération des dimensions de la fenêtre
            let mut rect = RECT::default();
            GetWindowRect(hwnd, &mut rect).map_err(|e| e.to_string())?;

            // Calcul du centre exact dans l'espace de l'écran (Screen Space)
            let center_x = rect.left + ((rect.right - rect.left) / 2);
            let center_y = rect.top + ((rect.bottom - rect.top) / 2);

            // 3. Initialisation d'Enigo v0.2
            let mut enigo = Enigo::new(&Settings::default())
                .map_err(|e| format!("Erreur d'initialisation Enigo: {:?}", e))?;

            // 4. Mouvement de la souris
            // En v0.2, on utilise move_mouse avec l'énumération Coordinate
            enigo.move_mouse(center_x, center_y, Coordinate::Abs)
                .map_err(|e| format!("Erreur de mouvement: {:?}", e))?;
            
            // 5. Clic de la souris
            // En v0.2, on utilise la méthode button avec Direction::Click
            enigo.button(Button::Left, Direction::Click)
                .map_err(|e| format!("Erreur de clic: {:?}", e))?;

            Ok(format!("Clic effectué au centre ({}, {})", center_x, center_y))
        }
    }
}