import {
  writeFile,
  readTextFile,
  BaseDirectory,
  exists,
  mkdir,
} from "@tauri-apps/plugin-fs";

// Structure de nos données sauvegardées
export interface AppProfile {
  characterName: string; // "Mon Personnage"
  openTabIds: string[]; // ["guide_552", "guide_12"]
  activeTabId: string; // "guide_552"
  guideProgress: Record<string, number>;
  checkboxStates: Record<string, Record<number, boolean[]>>;
}

const PROFILE_FILE = "user_profile.json";

// Profil par défaut
const DEFAULT_PROFILE: AppProfile = {
  characterName: "Mon Personnage",
  openTabIds: [],
  activeTabId: "general",
  guideProgress: {},
  checkboxStates: {},
};

export async function saveProfile(profile: AppProfile) {
  try {
    console.log("📂 Création dossier...");
    await mkdir("", { baseDir: BaseDirectory.AppData, recursive: true });

    console.log("📝 Écriture fichier...");
    const content = new TextEncoder().encode(JSON.stringify(profile, null, 2));
    await writeFile("user_profile.json", content, {
      baseDir: BaseDirectory.AppData,
    });

    console.log("✅ SUCCÈS ECRITURE DISQUE");
  } catch (e) {
    console.error("❌ ERREUR FATALE:", e);
    // Ajoute ceci pour voir l'erreur directement dans l'appli si la console est capricieuse
    alert("Erreur sauvegarde : " + JSON.stringify(e));
  }
}

export async function loadProfile(): Promise<AppProfile> {
  try {
    const fileExists = await exists(PROFILE_FILE, {
      baseDir: BaseDirectory.AppData,
    });
    if (!fileExists) return DEFAULT_PROFILE;

    const content = await readTextFile(PROFILE_FILE, {
      baseDir: BaseDirectory.AppData,
    });
    const data = JSON.parse(content);

    // On fusionne avec le défaut pour éviter les bugs si on rajoute des champs plus tard
    return { ...DEFAULT_PROFILE, ...data };
  } catch (e) {
    console.error("Erreur chargement profil:", e);
    return DEFAULT_PROFILE;
  }
}
