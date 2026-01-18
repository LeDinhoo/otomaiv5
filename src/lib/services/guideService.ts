import { fetch } from "@tauri-apps/plugin-http";
import {
  writeFile,
  BaseDirectory,
  mkdir,
  exists,
  readTextFile,
} from "@tauri-apps/plugin-fs";

export async function loadOrDownloadGuide(id: string): Promise<any> {
  const filePath = `guides/guide_${id}.json`;
  let content: string;

  // 1. Vérifier si local
  const fileExists = await exists(filePath, { baseDir: BaseDirectory.AppData });

  if (!fileExists) {
    // 2. Télécharger si absent
    const response = await fetch(
      `https://ganymede-app.com/guides/${id}/export`,
    );

    if (!response.ok) throw new Error(`Erreur HTTP: ${response.status}`);

    const buffer = await response.arrayBuffer();
    const uint8 = new Uint8Array(buffer);

    await mkdir("guides", { baseDir: BaseDirectory.AppData, recursive: true });
    await writeFile(filePath, uint8, { baseDir: BaseDirectory.AppData });

    content = new TextDecoder().decode(uint8);
  } else {
    // 3. Lire si présent
    content = await readTextFile(filePath, { baseDir: BaseDirectory.AppData });
  }

  return JSON.parse(content);
}
