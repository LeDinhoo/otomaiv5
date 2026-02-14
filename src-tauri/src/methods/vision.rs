// use anyhow::{Context, Result};
// use opencv::{
//     core::{self, Mat, Point},
//     imgcodecs, imgproc,
//     prelude::*,
// };
// use std::{thread, time::Duration};
// use xcap::Monitor;

// /// Fonction bloquante : Cherche une image indéfiniment jusqu'à la trouver.
// /// - Charge l'image une seule fois (perf).
// /// - Renvoie `true` une fois trouvée.
// pub fn wait_for_image(filename: &str) -> Result<bool> {

//     // let current_dir = std::env::current_dir()?;
//     // println!("📂 Dossier de travail actuel : {:?}", current_dir);

//     // println!("👀 Recherche en cours pour : '{}'...", filename);

//     // 1. On charge l'image CIBLE une seule fois au début (Optimisation majeure)
//     let target_mat = imgcodecs::imread(filename, imgcodecs::IMREAD_COLOR)
//         .context(format!("ERREUR: Impossible de lire le fichier '{}'", filename))?;

//     // 2. Boucle infinie jusqu'à ce qu'on trouve
//     loop {
//         // On tente une détection
//         // On passe directement la Matrice cible pour ne pas la recharger
//         if let Some((x, y)) = scan_screen(&target_mat, 0.8)? {
//             println!("✅ TROUVÉ ! '{}' détecté en X={}, Y={}", filename, x, y);
//             return Ok(true); // On arrête la fonction et on renvoie true
//         }

//         // Petite pause pour ne pas surcharger le CPU (50ms)
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// /// Fonction interne (privée) qui fait juste une capture et compare avec la matrice cible
// fn scan_screen(target_mat: &Mat, threshold: f64) -> Result<Option<(i32, i32)>> {
//     let monitors = Monitor::all().context("Erreur moniteur")?;
//     let monitor = monitors.first().context("Aucun moniteur")?;
    
//     // Capture
//     let screenshot = monitor.capture_image().context("Erreur capture")?;
    
//     // Conversion Xcap -> OpenCV
//     let width = screenshot.width() as i32;
//     let height = screenshot.height() as i32;
//     let raw_pixels = screenshot.as_raw();

//     let screenshot_mat_linear = Mat::from_slice(raw_pixels)?;
//     let screenshot_mat_rgba = screenshot_mat_linear.reshape(4, height)?;
    
//     let mut screenshot_mat_bgr = Mat::default();
//     imgproc::cvt_color(&screenshot_mat_rgba, &mut screenshot_mat_bgr, imgproc::COLOR_RGBA2BGR, 0)?;

//     // Template Matching
//     let mut result = Mat::default();
//     imgproc::match_template(
//         &screenshot_mat_bgr, 
//         target_mat, // On utilise l'image déjà chargée
//         &mut result, 
//         imgproc::TM_CCOEFF_NORMED, 
//         &core::no_array()
//     )?;

//     // Analyse du résultat
//     let mut max_val = 0.0;
//     let mut max_loc = Point::default();

//     core::min_max_loc(
//         &result, 
//         None, // On se fiche du minimum
//         Some(&mut max_val), 
//         None, // On se fiche de la position du min
//         Some(&mut max_loc), 
//         &core::no_array()
//     )?;

//     if max_val >= threshold {
//         Ok(Some((max_loc.x, max_loc.y)))
//     } else {
//         Ok(None)
//     }
// }

use anyhow::{Context, Result};
use opencv::{
    core::{self, Mat, Point, Rect},
    imgcodecs,
    imgproc,
    prelude::*,
};
use std::{thread, time::Duration};
use xcap::Monitor;

/// Scan unique sur le top 1/10 de l'écran. Renvoie true si l'image est trouvée.
pub fn scan_top_region(abs_path: &str) -> Result<bool> {
    let target_mat = imgcodecs::imread(abs_path, imgcodecs::IMREAD_GRAYSCALE)
        .context(format!("ERREUR VISION: Impossible de lire '{}'", abs_path))?;

    let monitors = Monitor::all().context("Erreur: Impossible de lister les moniteurs")?;
    let monitor = monitors.first().context("Erreur: Aucun moniteur détecté")?;

    let screenshot = monitor.capture_image().context("Erreur capture d'écran")?;
    let width = screenshot.width() as i32;
    let height = screenshot.height() as i32;
    let raw_pixels = screenshot.as_raw();

    let mat_linear = Mat::from_slice(raw_pixels)?;
    let mat_rgba = mat_linear.reshape(4, height)?;

    let mut screen_gray = Mat::default();
    imgproc::cvt_color(&mat_rgba, &mut screen_gray, imgproc::COLOR_RGBA2GRAY, 0)?;

    let mut result = Mat::default();
    imgproc::match_template(
        &screen_gray,
        &target_mat,
        &mut result,
        imgproc::TM_CCOEFF_NORMED,
        &core::no_array(),
    )?;

    let mut max_val = 0.0;
    core::min_max_loc(
        &result,
        None,
        Some(&mut max_val),
        None,
        None,
        &core::no_array(),
    )?;

    Ok(max_val >= 0.9)
}

/// Scan unique sur le 1/3 bas-gauche de l'écran. Cherche plusieurs images, renvoie true si l'une est trouvée.
pub fn scan_bottom_left_region(abs_paths: &[&str]) -> Result<bool> {
    let targets: Vec<Mat> = abs_paths
        .iter()
        .map(|p| {
            imgcodecs::imread(p, imgcodecs::IMREAD_GRAYSCALE)
                .context(format!("ERREUR VISION: Impossible de lire '{}'", p))
        })
        .collect::<Result<Vec<_>>>()?;

    let monitors = Monitor::all().context("Erreur: Impossible de lister les moniteurs")?;
    let monitor = monitors.first().context("Erreur: Aucun moniteur détecté")?;

    let screenshot = monitor.capture_image().context("Erreur capture d'écran")?;
    let width = screenshot.width() as i32;
    let height = screenshot.height() as i32;
    let raw_pixels = screenshot.as_raw();

    let mat_linear = Mat::from_slice(raw_pixels)?;
    let mat_rgba = mat_linear.reshape(4, height)?;

    let mut screen_gray = Mat::default();
    imgproc::cvt_color(&mat_rgba, &mut screen_gray, imgproc::COLOR_RGBA2GRAY, 0)?;

    let mut result = Mat::default();
    for target in &targets {
        imgproc::match_template(
            &screen_gray,
            target,
            &mut result,
            imgproc::TM_CCOEFF_NORMED,
            &core::no_array(),
        )?;

        let mut max_val = 0.0;
        core::min_max_loc(
            &result,
            None,
            Some(&mut max_val),
            None,
            None,
            &core::no_array(),
        )?;

        if max_val >= 0.9 {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Fonction bloquante optimisée.
/// Elle attend un chemin ABSOLU (résolu via Tauri) vers l'image.
pub fn wait_for_image(abs_path: &str) -> Result<bool> {
    
    // --- 1. CHARGEMENT CIBLE (Optimisé) ---
    // On charge l'image depuis le disque en NIVEAUX DE GRIS directement.
    // Cela évite de traiter la couleur inutilement (gain perf x3).
    let target_mat = imgcodecs::imread(abs_path, imgcodecs::IMREAD_GRAYSCALE)
        .context(format!("ERREUR VISION: Impossible de lire le fichier '{}'. Vérifiez le path resolved.", abs_path))?;

    // --- 2. PRÉ-ALLOCATION MÉMOIRE ---
    // On crée les matrices une seule fois pour ne pas saturer la RAM dans la boucle.
    let mut screen_gray = Mat::default();
    let mut result = Mat::default();

    // On récupère le moniteur (écran principal)
    let monitors = Monitor::all().context("Erreur: Impossible de lister les moniteurs")?;
    let monitor = monitors.first().context("Erreur: Aucun moniteur détecté")?;

    // println!("👁️ [Vision] Scan démarré pour : {}", abs_path);

    // --- 3. BOUCLE DE RECHERCHE ---
    loop {
        // A. Capture d'écran (xcap)
        let screenshot = monitor.capture_image().context("Erreur lors de la capture d'écran")?;
        let height = screenshot.height() as i32;
        let raw_pixels = screenshot.as_raw();

        // B. Conversion des données brutes en Matrice OpenCV
        // from_slice est très rapide (pas de copie profonde)
        let mat_linear = Mat::from_slice(raw_pixels)?;
        let mat_rgba = mat_linear.reshape(4, height)?; // 4 channels (RGBA)

        // C. Conversion en Grayscale (Noir & Blanc)
        // C'est ici qu'on gagne le plus de temps de calcul
        imgproc::cvt_color(&mat_rgba, &mut screen_gray, imgproc::COLOR_RGBA2GRAY, 0)?;

        // D. Template Matching (Comparaison)
        imgproc::match_template(
            &screen_gray,
            &target_mat,
            &mut result,
            imgproc::TM_CCOEFF_NORMED,
            &core::no_array(),
        )?;

        // E. Trouver le meilleur score
        let mut max_val = 0.0;
        let mut max_loc = Point::default();

        core::min_max_loc(
            &result,
            None,               // On ignore le min
            Some(&mut max_val), // On veut le max (score de ressemblance)
            None,               // On ignore la pos du min
            Some(&mut max_loc), // On veut la pos du max
            &core::no_array(),
        )?;

        // F. Vérification du seuil (0.9 = 90% de ressemblance)
        if max_val >= 0.9 {
            println!("✅ [Vision] TROUVÉ ! (Score: {:.2}) aux coordonnées X={}, Y={}", max_val, max_loc.x, max_loc.y);
            return Ok(true);
        }

        // G. Petite pause pour soulager le CPU
        // 10ms est un bon compromis (très réactif mais laisse le PC respirer)
        thread::sleep(Duration::from_millis(10));
    }
}