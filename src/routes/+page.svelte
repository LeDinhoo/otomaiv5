<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import TitleBar from "$lib/components/TitleBar/TitleBar.svelte";
  import { Button } from "$lib/components/ui/button/index.js";

  let status = $state("Statut : En attente d'action...");
  let windowTitle = $state("Mon Personnage");
  let usableTitle = $state("Mon Personnage");

  async function bringToFront() {
    try {
      status = await invoke("focus_window", { windowTitle: usableTitle });
    } catch (error) {
      status = "Erreur : " + error;
    }
  }

  async function clickCenter() {
    try {
      status = await invoke("click_window_center", {
        title: usableTitle,
      });
    } catch (error) {
      status = "Erreur : " + error;
    }
  }
</script>

<div
  class="border bg-stone-900 rounded-md overflow-hidden border-stone-700 flex flex-col h-full w-full text-stone-200"
>
  <TitleBar bind:windowTitle bind:usableTitle />
  <div class="p-4 flex flex-col space-y-4 grow">
    <Button variant="secondary" onclick={bringToFront} class=""
      >Mettre au premier plan</Button
    >
    <Button variant="secondary" onclick={clickCenter}>Cliquer au centre</Button>
    <p>{status}</p>
  </div>
</div>
