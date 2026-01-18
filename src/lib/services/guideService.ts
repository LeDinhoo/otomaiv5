import { fetch } from "@tauri-apps/plugin-http";
import { writeFile, BaseDirectory, mkdir, exists, readTextFile } from "@tauri-apps/plugin-fs";

export interface GuideSummary {
    id: number;
    name: string;
}

// --- CHARGEMENT D'UN GUIDE UNIQUE (Rien ne change ici) ---
export async function loadOrDownloadGuide(id: string): Promise<any> {
    const filePath = `guides/guide_${id}.json`;
    let content: string;

    const fileExists = await exists(filePath, { baseDir: BaseDirectory.AppData });

    if (!fileExists) {
        const response = await fetch(`https://ganymede-app.com/guides/${id}/export`);
        if (!response.ok) throw new Error(`Erreur HTTP: ${response.status}`);
        
        const buffer = await response.arrayBuffer();
        const uint8 = new Uint8Array(buffer);

        await mkdir("guides", { baseDir: BaseDirectory.AppData, recursive: true });
        await writeFile(filePath, uint8, { baseDir: BaseDirectory.AppData });

        content = new TextDecoder().decode(uint8);
    } else {
        content = await readTextFile(filePath, { baseDir: BaseDirectory.AppData });
    }

    return JSON.parse(content);
}

// --- CHARGEMENT DU CATALOGUE (NOUVELLE VERSION ROBUSTE) ---
export async function fetchGuideCatalog(): Promise<GuideSummary[]> {
    // 1. On fait une requête simple, comme un navigateur qui arrive sur la page
    const response = await fetch("https://ganymede-app.com/guides", {
        method: "GET",
        headers: {
            // On se fait passer pour un vrai navigateur pour éviter les blocages
            "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        }
    });
    
    if (!response.ok) {
        throw new Error(`Erreur HTTP: ${response.status}`);
    }

    // 2. On récupère le HTML (puisque c'est ce que Inertia envoie au premier chargement)
    const htmlText = await response.text();

    // 3. On utilise le parseur du navigateur pour lire le HTML
    const parser = new DOMParser();
    const doc = parser.parseFromString(htmlText, "text/html");

    // 4. Inertia stocke tout le JSON de la page dans une div avec l'ID "app" et l'attribut "data-page"
    const appDiv = doc.getElementById("app");

    if (!appDiv || !appDiv.dataset.page) {
        console.error("HTML reçu (début):", htmlText.slice(0, 500));
        throw new Error("Impossible de trouver les données Inertia (div #app data-page) dans la page.");
    }

    // 5. On parse ce JSON caché
    const inertiaData = JSON.parse(appDiv.dataset.page);

    // 6. On navigue dans la structure d'Inertia pour trouver nos guides
    // La structure est généralement : props -> guides -> data (si paginé) ou props -> guides
    const props = inertiaData.props;
    
    // Sécurité pour trouver le bon tableau
    let guidesList = [];
    
    if (props.guides && Array.isArray(props.guides.data)) {
        // Cas pagination Laravel standard
        guidesList = props.guides.data;
    } else if (Array.isArray(props.guides)) {
        // Cas tableau simple
        guidesList = props.guides;
    } else {
        console.error("Structure Inertia reçue:", inertiaData);
        throw new Error("Structure des données inconnue (pas de props.guides).");
    }

    // 7. On nettoie et on retourne
    return guidesList.map((g: any) => ({
        id: g.id,
        name: g.name || "Guide sans nom"
    }));
}