use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowRect};

pub struct MouseManager;

impl MouseManager {
    pub fn new() -> Self {
        Self
    }

    // On change le nom et on ajoute x, y (relatifs au coin haut-gauche de la fenêtre)
    pub fn click_at_position(&self, title: &str, x: i32, y: i32) -> Result<String, String> {
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

            // 2. Récupération de la position de la fenêtre
            let mut rect = RECT::default();
            GetWindowRect(hwnd, &mut rect).map_err(|e| e.to_string())?;

            // Calcul de la position absolue sur l'écran
            // On part du bord haut-gauche (rect.left, rect.top) et on ajoute l'offset x, y
            let target_x = rect.left + x;
            let target_y = rect.top + y;

            // 3. Initialisation d'Enigo v0.2
            let mut enigo = Enigo::new(&Settings::default())
                .map_err(|e| format!("Erreur d'initialisation Enigo: {:?}", e))?;

            // 4. Mouvement de la souris vers la position calculée
            enigo
                .move_mouse(target_x, target_y, Coordinate::Abs)
                .map_err(|e| format!("Erreur de mouvement: {:?}", e))?;

            // 5. Clic
            enigo
                .button(Button::Left, Direction::Click)
                .map_err(|e| format!("Erreur de clic: {:?}", e))?;

            Ok(format!(
                "Clic effectué à ({}, {}) [Relatif: {}, {}]",
                target_x, target_y, x, y
            ))
        }
    }
}
