<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import TitleBar from "$lib/components/TitleBar/TitleBar.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js"; // Assurez-vous d'avoir le composant Input

  let status = $state("Statut : En attente d'action...");
  let windowTitle = $state("Mon Personnage");
  let usableTitle = $state("Mon Personnage");

  // Nouveaux états pour la touche
  let selectedKey = $state("Enter");
  let pressCount = $state(1);

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

  // Nouvelle fonction pour tester la touche
  async function testKeyPress() {
    try {
      // Optionnel : on met la fenêtre au premier plan avant de presser la touche
      await invoke("focus_window", { windowTitle: usableTitle });

      status = await invoke("press_key", {
        key: selectedKey,
        count: pressCount,
      });
    } catch (error) {
      status = "Erreur : " + error;
    }
  }

  let clickX = $state(100);
  let clickY = $state(100);

  async function testClick() {
    try {
      status = await invoke("click_window_at", {
        title: usableTitle,
        x: clickX,
        y: clickY,
      });
    } catch (error) {
      status = "Erreur : " + error;
    }
  }

  let textToType = $state("");

  async function sendFastText() {
    try {
      status = await invoke("quick_type", {
        text: textToType,
        windowTitle: usableTitle,
      });
    } catch (error) {
      status = "Erreur : " + error;
    }
  }

  let destination = $state("");

  async function startTravel() {
    status = "Voyage en cours...";
    try {
      status = await invoke("travel_with_zaap", {
        destination: destination,
        windowTitle: usableTitle,
      });
    } catch (error) {
      status = "Erreur : " + error;
    }
  }

  async function useBonta() {
    try {
      status = await invoke("use_potion_bonta", { windowTitle: usableTitle });
    } catch (error) {
      status = "Erreur : " + error;
    }
  }

  async function useBrakmar() {
    try {
      status = await invoke("use_potion_brakmar", { windowTitle: usableTitle });
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
    <div class="space-y-2 border-b border-stone-800 pb-4">
      <p class="text-xs font-bold uppercase text-stone-500">Actions Fenêtre</p>
      <Button variant="secondary" onclick={bringToFront} class="w-full">
        Mettre au premier plan
      </Button>
    </div>

    <div class="space-y-2 border-b border-stone-800 pb-4">
      <p class="text-xs font-bold uppercase text-stone-500">Potions de cité</p>
      <div class="grid grid-cols-2 gap-2">
        <Button 
          variant="outline" 
          onclick={useBonta} 
          class="border-blue-900/50 bg-blue-950/20 hover:bg-blue-900/40 text-blue-200"
        >
          Bonta (-)
        </Button>
        <Button 
          variant="outline" 
          onclick={useBrakmar} 
          class="border-red-900/50 bg-red-950/20 hover:bg-red-900/40 text-red-200"
        >
          Brakmar (=)
        </Button>
      </div>
    </div>

    <div class="space-y-2 p-2 border border-stone-700 rounded bg-stone-800/50">
      <p class="text-xs font-bold text-orange-500 uppercase">Havre-Sac Zaap</p>
      <div class="flex space-x-2">
        <input
          type="text"
          bind:value={destination}
          placeholder="Ex: Tainéla"
          class="bg-stone-900 border-stone-700 rounded px-2 py-1 text-sm grow"
        />
        <Button
          variant="default"
          onclick={startTravel}
          class="bg-blue-700 hover:bg-blue-600"
        >
          Voyager
        </Button>
      </div>
    </div>

    <div class="mt-auto pt-4 border-t border-stone-800">
      <p class="text-sm italic text-stone-400">{status}</p>
    </div>
  </div>
</div>
