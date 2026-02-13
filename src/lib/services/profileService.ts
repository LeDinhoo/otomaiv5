import {
  writeFile,
  readTextFile,
  BaseDirectory,
  exists,
  mkdir,
  remove,
} from "@tauri-apps/plugin-fs";

// --- Types ---

export interface AppProfile {
  characterName: string;
  openTabIds: string[];
  activeTabId: string;
  guideProgress: Record<string, number>;
  checkboxStates: Record<string, Record<number, boolean[]>>;
}

export interface ProfileEntry {
  id: string;
  name: string;
}

export interface ProfileIndex {
  activeProfileId: string;
  profiles: ProfileEntry[];
}

// --- Constantes ---

const INDEX_FILE = "profiles.json";
const PROFILES_DIR = "profiles";
const LEGACY_FILE = "user_profile.json";

const DEFAULT_PROFILE: AppProfile = {
  characterName: "Mon Personnage",
  openTabIds: [],
  activeTabId: "general",
  guideProgress: {},
  checkboxStates: {},
};

const DEFAULT_INDEX: ProfileIndex = {
  activeProfileId: "default",
  profiles: [{ id: "default", name: "Défaut" }],
};

// --- Helpers ---

function profilePath(profileId: string): string {
  return `${PROFILES_DIR}/${profileId}.json`;
}

function slugify(name: string): string {
  return (
    name
      .toLowerCase()
      .normalize("NFD")
      .replace(/[\u0300-\u036f]/g, "")
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-|-$/g, "") || "profil"
  );
}

async function ensureProfilesDir() {
  await mkdir(PROFILES_DIR, {
    baseDir: BaseDirectory.AppData,
    recursive: true,
  });
}

async function writeJson(path: string, data: unknown) {
  const content = new TextEncoder().encode(JSON.stringify(data, null, 2));
  await writeFile(path, content, { baseDir: BaseDirectory.AppData });
}

async function readJson<T>(path: string): Promise<T | null> {
  try {
    const fileExists = await exists(path, { baseDir: BaseDirectory.AppData });
    if (!fileExists) return null;
    const content = await readTextFile(path, {
      baseDir: BaseDirectory.AppData,
    });
    return JSON.parse(content) as T;
  } catch {
    return null;
  }
}

// --- Index ---

export async function loadProfileIndex(): Promise<ProfileIndex> {
  await ensureProfilesDir();

  // Toujours vérifier si l'ancien fichier existe et migrer
  const legacy = await readJson<AppProfile>(LEGACY_FILE);

  const index = await readJson<ProfileIndex>(INDEX_FILE);
  if (index) {
    // Migration : si l'ancien fichier existe encore, importer ses données dans le profil défaut
    if (legacy) {
      await writeJson(profilePath("default"), legacy);
      // Mettre à jour le nom du profil défaut si besoin
      const defaultEntry = index.profiles.find((p) => p.id === "default");
      if (defaultEntry && legacy.characterName) {
        defaultEntry.name = legacy.characterName;
        await writeJson(INDEX_FILE, index);
      }
      // Supprimer l'ancien fichier pour ne plus re-migrer
      try {
        await remove(LEGACY_FILE, { baseDir: BaseDirectory.AppData });
      } catch {
        // pas grave si ça échoue
      }
    }
    return index;
  }

  // Premier lancement avec ancien fichier
  if (legacy) {
    await writeJson(profilePath("default"), legacy);
    const newIndex: ProfileIndex = {
      activeProfileId: "default",
      profiles: [{ id: "default", name: legacy.characterName || "Défaut" }],
    };
    await writeJson(INDEX_FILE, newIndex);
    try {
      await remove(LEGACY_FILE, { baseDir: BaseDirectory.AppData });
    } catch {
      // pas grave
    }
    return newIndex;
  }

  // Aucun fichier existant : créer depuis zéro
  await writeJson(profilePath("default"), DEFAULT_PROFILE);
  await writeJson(INDEX_FILE, DEFAULT_INDEX);
  return DEFAULT_INDEX;
}

export async function saveProfileIndex(index: ProfileIndex) {
  await writeJson(INDEX_FILE, index);
}

// --- Profils individuels ---

export async function saveProfile(profileId: string, profile: AppProfile) {
  await ensureProfilesDir();
  await writeJson(profilePath(profileId), profile);
}

export async function loadProfile(profileId: string): Promise<AppProfile> {
  const data = await readJson<AppProfile>(profilePath(profileId));
  if (data) return { ...DEFAULT_PROFILE, ...data };
  return { ...DEFAULT_PROFILE };
}

export async function deleteProfileFile(profileId: string) {
  try {
    await remove(profilePath(profileId), { baseDir: BaseDirectory.AppData });
  } catch {
    // Fichier déjà supprimé ou inexistant
  }
}

// --- Utilitaire pour générer un ID unique ---

export function generateProfileId(
  name: string,
  existingIds: string[],
): string {
  const base = slugify(name);
  if (!existingIds.includes(base)) return base;
  let i = 2;
  while (existingIds.includes(`${base}-${i}`)) i++;
  return `${base}-${i}`;
}
