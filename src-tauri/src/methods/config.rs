// Position du Zaap havre-sac
pub const POS_ZAAP_INPUT: (i32, i32) = (725, 497);

// Positions des Zaapi regroupées
pub const POS_ZAAPI_SUFOKIA: (i32, i32) = (590, 880);
pub const POS_ZAAPI_BRAKMAR: (i32, i32) = (1544, 134);
pub const POS_ZAAPI_BONTA: (i32, i32) = (1918, 305);
pub const POS_ZAAPI_FRIGOST: (i32, i32) = (1070, 170);

// Positions des tabs zaapi
pub const POS_TAB_ZAAPI_ATELIER: (i32, i32) = (1119, 471); 
pub const POS_TAB_ZAAPI_HOTEL:   (i32, i32) = (1288, 471);
pub const POS_TAB_ZAAPI_DIVERS:  (i32, i32) = (1458, 471);

//Input Text
pub const POS_INPUT_TEXT_ZAAPI: (i32, i32) = (975, 531);

// Positions des villes
pub const POS_FRIGOST:(i32,i32) = (-78,-41);
pub const POS_SUFOKIA:(i32,i32) = (13,26);
pub const POS_BRAKMAR:(i32,i32) = (-26,37);
pub const POS_BONTA:(i32,i32) = (-31, -56);

// Waiting time for zaapi
pub const WAIT_ZAPPI_BONTA: u64 = 1500;
pub const WAIT_ZAPPI_BRAKMAR: u64 = 2500;
pub const WAIT_ZAPPI_SUFOKIA: u64 = 1500;
pub const WAIT_ZAPPI_FRIGOST: u64 = 1500;

// ============================================================
// --- TABLEAUX REGROUPÉS (EXTRAITS DES IMAGES) ---
// ============================================================

// Liste de tous les noms de Zaaps (Ordre identique aux positions)
pub const ZAAP_NAMES: &[&str] = &[
    "La Bourgade",
    "Cœur immaculé",
    "Plaine des Porkass",
    "Village d'Amakna",
    "Cité d'Astrub",
    "Sufokia",
    "Village enseveli",
    "Entrée du château de Harebourg",
    "Nimotopia",
    "Village de la Canopée",
    "Village côtier",
    "Village des Dopeuls",
    "Champs de Cania",
    "La Cuirasse",
    "Route des Roulottes",
    "Routes Rocailleuses",
    "Plaines Rocheuses",
    "Village des Eleveurs",
    "Terres Désacrées",
    "Massif de Cania",
    "Cimetière primitif",
    "Foire du Trool",
    "Montagne des Craqueleurs",
    "Lac de Cania",
    "Bord de la forêt maléfique",
    "Plaine des Scarafeuilles",
    "Village des Kanigs",
    "Tainéla",
    "Château d'Amakna",
    "Coin des Bouftous",
    "Port de Madrestam",
    "Rivage sufokien",
    "Temple des alliances",
    "Dunes des ossements",
    "Arche de Vili",
    "Village de Pandala",
    "Île de la Cawotte",
    "Laboratoires abandonnés",
    "Plage de la Tortue",
    "Futaie enneigée",
    "Mont des Tombeaux",
    "Crocuzko",
    "Village des Zoths",
    "Village des Brigandins",
    "Route des âmes",
    "Pâturages",
    "Cimetière",
];

// Liste de toutes les positions de Zaaps (Ordre identique aux noms)
pub const ZAAP_POSITIONS: &[(i32, i32)] = &[
    (-78, -41), // La Bourgade
    (-31, -56), // Cœur immaculé
    (-5, -23),  // Plaine des Porkass
    (-2, 0),    // Village d'Amakna
    (5, -18),   // Cité d'Astrub
    (13, 26),   // Sufokia
    (-77, -73), // Village enseveli
    (-67, -77), // Entrée du château de Harebourg
    (-67, 29),  // Nimotopia
    (-54, 16),  // Village de la Canopée
    (-46, 18),  // Village côtier
    (-34, -8),  // Village des Dopeuls
    (-27, -36), // Champs de Cania
    (-26, 37),  // La Cuirasse
    (-25, 12),  // Route des Roulottes
    (-20, -20), // Routes Rocailleuses
    (-17, -47), // Plaines Rocheuses
    (-16, 1),   // Village des Eleveurs
    (-15, 25),  // Terres Désacrées
    (-13, -28), // Massif de Cania
    (-12, 19),  // Cimetière primitif
    (-11, -36), // Foire du Trool
    (-5, -8),   // Montagne des Craqueleurs
    (-3, -42),  // Lac de Cania
    (-1, 13),   // Bord de la forêt maléfique
    (-1, 24),   // Plaine des Scarafeuilles
    (0, -56),   // Village des Kanigs
    (1, -32),   // Tainéla
    (3, -5),    // Château d'Amakna
    (5, 7),     // Coin des Bouftous
    (7, -4),    // Port de Madrestam
    (10, 22),   // Rivage sufokien
    (13, 35),   // Temple des alliances
    (15, -58),  // Dunes des ossements
    (15, -20),  // Arche de Vili
    (20, -29),  // Village de Pandala
    (25, -4),   // Île de la Cawotte
    (27, -14),  // Laboratoires abandonnés
    (35, 12),   // Plage de la Tortue
    (39, -82),  // Futaie enneigée
    (40, -44),  // Mont des Tombeaux
    (-83, -15), // Crocuzko
    (-53, 18),  // Village des Zoths
    (-16, -24), // Village des Brigandins
    (-1, -3),   // Route des âmes
    (2, -5),    // Pâturages
    (3, 0),     // Cimetière
    (-29, -56), // Atelier des sculpteurs (Bonta)
    (-30, -56), // Atelier des forgerons (Bonta)
    (-33, -55), // Atelier des bijoutiers (Bonta)
    (-33, -54), // Atelier des tailleurs (Bonta)
    (-30, -52), // Atelier des chasseurs (Bonta)
    (-31, -52), // Atelier des paysans (Bonta)
    (-28, -56), // Atelier des bûcherons (Bonta)
    (-32, -54), // Atelier des bricoleurs (Bonta)
    (-34, -54), // Atelier des pêcheurs (Bonta)
    (-28, -55), // Atelier des mineurs (Bonta)
    (-29, -54), // Atelier des alchimistes (Bonta)
    (-32, -53), // Atelier des façonneurs (Bonta)
    (-29, -57), // Atelier des forgemages (Bonta)
    (-30, -57), // Atelier des cordonniers (Bonta)
    (-32, -59), // Hôtel de vente des âmes (Bonta)
    (-31, -55), // Hôtel de vente des équipements (Bonta)
    (-30, -54), // Hôtel de vente des ressources (Bonta)
    (-31, -53), // Hôtel de vente des consommables (Bonta)
    (-30, -59), // Hôtel de vente des créatures (Bonta)
    (-35, -60), // Hôtel de vente des cosmétiques (Bonta)
    (-32, -57), // Milice (Bonta)
    (-31, -57), // Banque (Bonta)
    (-28, 34),  // Atelier des tailleurs (Brakmar)
    (-28, 33),  // Atelier des mineurs (Brakmar)
    (-28, 32),  // Atelier des sculpteurs (Brakmar)
    (-29, 36),  // Atelier des bricoleurs (Brakmar)
    (-22, 35),  // Atelier des alchimistes (Brakmar)
    (-23, 35),  // Atelier des paysans (Brakmar)
    (-29, 33),  // Atelier des cordonniers (Brakmar)
    (-28, 36),  // Atelier des façonneurs (Brakmar)
    (-29, 34),  // Atelier des bijoutiers (Brakmar)
    (-22, 37),  // Atelier des chasseurs (Brakmar)
    (-26, 38),  // Atelier des forgemages (Brakmar)
    (-22, 39),  // Atelier des pêcheurs (Brakmar)
    (-26, 33),  // Hôtel de vente des ressources (Brakmar)
    (-28, 35),  // Hôtel de vente des équipements (Brakmar)
    (-23, 36),  // Hôtel de vente des consommables (Brakmar)
    (-25, 38),  // Hôtel de vente des créatures (Brakmar)
    (-29, 38),  // Hôtel de vente des cosmétiques (Brakmar)
    (-25, 36),  // Hôtel de vente des âmes (Brakmar)
    (-25, 33),  // Milice (Brakmar)
    (-26, 36),  // Banque (Brakmar)
    (12, 29),   // Hôtel de vente des consommables (Sufokia)
    (22, 23),   // Hôtel de vente des cosmétiques (Sufokia)
    (15, 22),   // Hôtel de vente des créatures (Sufokia)
    (8, 25),    // Quai de la bricole
    (18, 24),   // Quai des marchands
    (16, 28),   // Quai du port
];