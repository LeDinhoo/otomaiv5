<script>
  import { invoke } from "@tauri-apps/api/core";
  import TitleBar from "$lib/components/TitleBar/TitleBar.svelte";
  import { Button } from "$lib/components/ui/button/index.js";

  let status = $state("Statut : En attente d'action...");
  let windowTitle = $state("Mon Personnage");

  async function bringToFront() {
    try {
      // Appel au Rust
      status = await invoke("focus_window", { windowTitle: windowTitle });
    } catch (error) {
      status = "Erreur : " + error;
    }
  }

  async function clickCenter() {
    try {
      // Appel au Rust
      status = await invoke("click_window_center", {
        title: windowTitle,
      });
    } catch (error) {
      status = "Erreur : " + error;
    }
  }
</script>

<main
  class="border bg-stone-900 rounded-md overflow-hidden border-stone-700 flex flex-col h-full w-full text-stone-200"
>
  <TitleBar bind:windowTitle />
  <div class="p-4 flex flex-col space-y-4 flex-grow">
      <Button variant="secondary" onclick={bringToFront}
        >Mettre au premier plan</Button
      >
      <Button variant="secondary" onclick={clickCenter}>Cliquer au centre</Button>
      <p>{status}</p>
  </div>
</main>
