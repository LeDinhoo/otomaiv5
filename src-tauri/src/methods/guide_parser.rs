use regex::Regex;

// --- STRUCTURES DE SORTIE ---

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GuideResult {
    pub position: Option<(i32, i32)>,
    pub travel_cmd: Option<String>,
    pub macro_type: String, 
    pub macro_arg: Option<String>,
    pub macro_arg2: Option<String>,
}

impl GuideResult {
    fn new() -> Self {
        Self {
            position: None,
            travel_cmd: None,
            macro_type: "classic".to_string(),
            macro_arg: None,
            macro_arg2: None,
        }
    }
}

// --- LOGIQUE DU PARSER ---

pub struct GuideParser;

impl GuideParser {
    // --- CONSTANTES ---
    const TARGET_COLORS: &'static [&'static str] = &["rgb(98, 172, 255)", "#62ACFF", "#62acff"];
    const BLACKLIST_CONTEXT: &'static [&'static str] = &["départ", "depuis", "partir", "commencer"];
    
    // Positions strictes des Milices (Points d'arrivée des potions)
    pub const POS_MILICE_BONTA: (i32, i32) = (-32, -57);
    pub const POS_MILICE_BRAKMAR: (i32, i32) = (-25, 33);

    pub fn parse_step(html_content: &str) -> GuideResult {
        let mut result = GuideResult::new();
        let clean_text = Self::clean_html(html_content);

        // 1. Extraction Position & Commande Travel
        result.position = Self::extract_position(&clean_text);
        if let Some((x, y)) = result.position {
            result.travel_cmd = Some(format!("/travel {},{}", x, y));
        }

        // 2. Vérification Potion Directe (STRICTE)
        // Ne se déclenche QUE si la destination est exactement la milice
        if let Some(pos) = result.position {
            if let Some(potion_name) = Self::get_special_potion(pos) {
                result.travel_cmd = Some(potion_name.to_string());
                result.macro_type = "potion_direct".to_string();
                
                Self::print_debug_table(&result, &clean_text);
                return result;
            }
        }

        // 3. Analyse Séquentielle Zaap / Zaapi
        let blue_items = Self::get_blue_spans_raw(html_content);
        let (final_zaap, final_zaapi) = Self::extract_transport_entities(&blue_items);

        // --- ARBRE DE DÉCISION SIMPLIFIÉ ---
        // Plus d'optimisation potion_zaapi ici. On suit le guide à la lettre.

        if let (Some(zaap), Some(zaapi)) = (&final_zaap, &final_zaapi) {
            // CAS A : Zaap + Zaapi
            result.macro_type = "zaap_zaapi".to_string();
            result.macro_arg = Some(zaap.clone());   
            result.macro_arg2 = Some(zaapi.clone()); 
        } else if let Some(zaapi) = final_zaapi {
            // CAS B : Zaapi seul
            result.macro_type = "zaapi".to_string();
            result.macro_arg = Some(zaapi);
        } else if let Some(zaap) = final_zaap {
            // CAS C : Zaap Classique
            result.macro_type = "zaap".to_string();
            result.macro_arg = Some(zaap);
        }

        Self::print_debug_table(&result, &clean_text);
        result
    }

    // --- HELPERS ---

    fn clean_html(html: &str) -> String {
        let re = Regex::new(r"<[^>]+>").unwrap();
        let no_tags = re.replace_all(html, " ");
        let re_spaces = Regex::new(r"\s+").unwrap();
        re_spaces.replace_all(&no_tags, " ").trim().to_string()
    }

    fn extract_position(text: &str) -> Option<(i32, i32)> {
        let re = Regex::new(r"\[\s*(-?\d+)\s*,\s*(-?\d+)\s*\]").unwrap();
        re.captures_iter(text).last().map(|cap| {
            (
                cap[1].parse::<i32>().unwrap_or(0),
                cap[2].parse::<i32>().unwrap_or(0),
            )
        })
    }

    fn get_special_potion(pos: (i32, i32)) -> Option<&'static str> {
        match pos {
            Self::POS_MILICE_BONTA => Some("potion_bonta"),
            Self::POS_MILICE_BRAKMAR => Some("potion_brakmar"),
            _ => None,
        }
    }

    fn get_blue_spans_raw(html: &str) -> Vec<BlueItem> {
        let mut items = Vec::new();
        let re = Regex::new(r#"(?i)<span[^>]*style="([^"]*)"[^>]*>(.*?)</span>"#).unwrap();
        let strip_tags = Regex::new(r"<[^>]+>").unwrap();

        for cap in re.captures_iter(html) {
            let style = cap[1].to_lowercase();
            let raw_content = &cap[2];
            let full_match = cap.get(0).unwrap();
            let start_pos = full_match.start();

            if !Self::TARGET_COLORS.iter().any(|c| style.contains(&c.to_lowercase())) {
                continue;
            }

            let clean_text = strip_tags.replace_all(raw_content, "").trim().to_string();
            if clean_text.is_empty() { continue; }

            let mut start_ctx = start_pos.saturating_sub(50);
            while !html.is_char_boundary(start_ctx) {
                start_ctx = start_ctx.saturating_sub(1);
            }

            let context = html[start_ctx..start_pos].to_lowercase();
            items.push(BlueItem { text: clean_text, context });
        }
        items
    }

    fn extract_transport_entities(items: &[BlueItem]) -> (Option<String>, Option<String>) {
        let mut final_zaap = None;
        let mut final_zaapi = None;
        let mut skip_next = false;

        for (i, item) in items.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            let lower_text = item.text.to_lowercase();
            let cmd_type = if lower_text.contains("zaapi") {
                Some("zaapi")
            } else if lower_text.contains("zaap") {
                Some("zaap")
            } else {
                None
            };

            if let Some(ctype) = cmd_type {
                if Self::BLACKLIST_CONTEXT.iter().any(|bad| item.context.contains(bad)) {
                    continue;
                }

                let re_clean = Regex::new(r"(?i)zaapi?").unwrap();
                let mut param = re_clean.replace_all(&item.text, "").trim().to_string();

                if param.is_empty() && i + 1 < items.len() {
                    param = items[i + 1].text.clone();
                    skip_next = true;
                }

                if !param.is_empty() {
                    if ctype == "zaap" {
                        final_zaap = Some(param);
                    } else {
                        final_zaapi = Some(param);
                    }
                }
            }
        }
        (final_zaap, final_zaapi)
    }

    fn print_debug_table(res: &GuideResult, raw_text: &str) {
        println!("\n┌──────────────────────────────────────────────────────────────┐");
        println!("│ GUIDE PARSER RESULT (STRICT MODE)                            │");
        println!("├──────────────────────┬───────────────────────────────────────┤");
        
        let pos_str = match res.position {
            Some((x, y)) => format!("[{}, {}]", x, y),
            None => "None".to_string(),
        };
        println!("│ {:<20} │ {:<37} │", "Position", pos_str);
        println!("│ {:<20} │ {:<37} │", "Macro Type", res.macro_type);
        
        let cmd = res.travel_cmd.as_deref().unwrap_or("-");
        let cmd_display = if cmd.len() > 35 { &cmd[0..35] } else { cmd };
        println!("│ {:<20} │ {:<37} │", "Travel Cmd", cmd_display);

        if let Some(arg) = &res.macro_arg {
             println!("│ {:<20} │ {:<37} │", "Macro Arg 1", arg);
        }
        if let Some(arg) = &res.macro_arg2 {
             println!("│ {:<20} │ {:<37} │", "Macro Arg 2", arg);
        }

        println!("├──────────────────────┴───────────────────────────────────────┤");
        let excerpt: String = raw_text.chars().take(52).collect();
        let display_text = if raw_text.chars().count() > 52 { format!("{}...", excerpt) } else { excerpt };
        println!("│ Text: {:<54} │", display_text);
        println!("└──────────────────────────────────────────────────────────────┘\n");
    }
}

struct BlueItem {
    text: String,
    context: String,
}