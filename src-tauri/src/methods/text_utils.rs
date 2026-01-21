use strsim::levenshtein;

/// Corrige une string en cherchant la plus proche dans une liste de référence.
/// Si aucune correspondance assez proche n'est trouvée, renvoie l'originale.
pub fn correct_text(input: &str, candidates: &[String]) -> String {
    let mut best_candidate: Option<&String> = None;
    let mut min_distance = usize::MAX;

    // On normalise l'input (minuscule) pour la comparaison
    let input_lower = input.to_lowercase();

    for candidate in candidates {
        let candidate_lower = candidate.to_lowercase();

        // 1. Calculer la distance de Levenshtein
        // "Dune des ossements" vs "DuneS des ossements" = distance de 1
        let distance = levenshtein(&input_lower, &candidate_lower);

        if distance < min_distance {
            min_distance = distance;
            best_candidate = Some(candidate);
        }
    }

    // 2. Le Seuil de tolérance (Threshold)
    // On accepte la correction SEULEMENT si la différence est petite.
    // Ici : on tolère max 3 fautes de frappe/différences.
    // Tu peux ajuster ce chiffre.
    const MAX_TOLERANCE: usize = 3;

    if min_distance <= MAX_TOLERANCE {
        if let Some(valid_string) = best_candidate {
            println!("✨ Correction : '{}' -> '{}' (Dist: {})", input, valid_string, min_distance);
            return valid_string.clone();
        }
    }

    // Si on est trop loin, on renvoie l'original
    println!("⚠️ Pas de correspondance proche pour '{}', on garde l'original.", input);
    input.to_string()
}