<script lang="ts">
  import { fetch } from "@tauri-apps/plugin-http";
  import {
    writeFile,
    BaseDirectory,
    mkdir,
    exists,
  } from "@tauri-apps/plugin-fs"; // Ajout de 'exists'
  import { Button } from "$lib/components/ui/button/index.js";

  let guideId = $state("552");
  let status = $state("Prêt");

  async function downloadGuide() {
    try {
      const filePath = `guides/guide_${guideId}.json`;

      // 1. Vérifier si le fichier existe déjà
      const alreadyExists = await exists(filePath, {
        baseDir: BaseDirectory.AppData,
      });

      if (alreadyExists) {
        status = `Le guide ${guideId} est déjà présent localement.`;
        return; // On arrête la fonction ici
      }

      status = "Téléchargement...";
      const url = `https://ganymede-app.com/guides/${guideId}/export`;

      // 2. Requête HTTP
      const response = await fetch(url, { method: "GET" });

      if (!response.ok) throw new Error(`Status: ${response.status}`);

      const buffer = await response.arrayBuffer();
      const uint8 = new Uint8Array(buffer);

      // 3. Création du dossier et écriture
      await mkdir("guides", {
        baseDir: BaseDirectory.AppData,
        recursive: true,
      });

      await writeFile(filePath, uint8, {
        baseDir: BaseDirectory.AppData,
      });

      status = `Guide ${guideId} téléchargé et sauvegardé !`;
    } catch (e) {
      status = "Erreur: " + e;
      console.error(e);
    }
  }
</script>

<div
  class="p-4 bg-stone-800 rounded-md border border-stone-700 flex flex-col space-y-2"
>
  <div class="flex space-x-2">
    <input
      type="text"
      bind:value={guideId}
      class="bg-stone-900 border border-stone-700 p-1 rounded grow text-sm"
    />
    <Button onclick={downloadGuide}>Vérifier/Télécharger</Button>
  </div>
  <p class="text-xs text-stone-400 italic">{status}</p>
</div>
