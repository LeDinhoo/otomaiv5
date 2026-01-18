<script lang="ts">
  // @ts-nocheck
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { Button } from "$lib/components/ui/button/index.js";
  // Ajout de l'icône Check
  import { X, Minus, Settings, Lock, LockOpen, Check, BookSearch } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";

  let {
    windowTitle = $bindable(),
    usableTitle = $bindable(),
    onToggleSettings,
  } = $props();
  let isLocked = $state(true);
  // Nouvel état pour le succès de la recherche
  let isFound = $state(false);

  const toggleLock = () => {
    isLocked = !isLocked;
    if (isLocked) {
      findWindow();
    } else {
      // Si on déverrouille pour changer le nom, on reset le statut "trouvé"
      isFound = false;
    }
  };

  const close = async () => {
    const window = getCurrentWindow();
    await window.close();
  };

  const minimize = async () => {
    const window = getCurrentWindow();
    await window.minimize();
  };

  async function findWindow() {
    try {
      const foundTitle: string = await invoke("get_game_title_by_name", {
        characterName: windowTitle,
      });
      usableTitle = foundTitle;
      isFound = true; // Succès !
    } catch (error) {
      console.error("Erreur : " + error);
      isFound = false; // Échec
    }
  }

  // Optionnel : Lancer la recherche au démarrage si c'est déjà verrouillé
  $effect(() => {
    if (isLocked && windowTitle) findWindow();
  });
</script>

<div
  class="flex items-center justify-between bg-stone-800 h-8 relative select-none"
>
  <div class="absolute inset-0" data-tauri-drag-region></div>

  <Button
    variant="ghost"
    size="icon"
    class="pl-2 h-6 w-6 text-stone-500 hover:text-stone-200 hover:bg-stone-800 z-40"
    onclick={onToggleSettings}
    onmousedown={(e) => e.stopPropagation()}
  >
    <BookSearch class="w-4 h-4" />
  </Button>

  <div
    class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-20 flex items-center space-x-2 group"
  >
    <input
      type="text"
      tabindex={isLocked ? -1 : 0}
      bind:value={windowTitle}
      readonly={isLocked}
      class="bg-transparent border-none text-md font-bold text-stone-200 text-center
             focus:outline-none focus:bg-stone-700/50 focus:ring-1 focus:ring-stone-600
             rounded px-2 py-0.5 transition-all outline-none
             {isLocked
        ? 'pointer-events-none cursor-default'
        : 'pointer-events-auto cursor-text hover:bg-stone-700/30'}"
      spellcheck="false"
    />

    <div class="flex items-center">
      <button
        onclick={toggleLock}
        class="p-1 rounded hover:bg-stone-700 transition-colors relative z-30 {isLocked
          ? 'opacity-50 group-hover:opacity-100'
          : 'opacity-100 text-yellow-500'}"
        title={isLocked ? "Déverrouiller le titre" : "Verrouiller le titre"}
      >
        {#if isLocked}
          <Lock class="w-3 h-3" />
        {:else}
          <LockOpen class="w-3 h-3" />
        {/if}
      </button>

      {#if isLocked && isFound}
        <Check class="w-4 h-4 text-green-500 ml-1" />
      {/if}
    </div>
  </div>

  <div class="flex flex-row ml-auto h-full relative z-10">
    <Button
      variant="outline"
      class="border-none rounded-none h-full w-8 bg-stone-800 hover:bg-stone-700 transition-colors"
    >
      <Settings class="w-4 h-4 text-white" />
    </Button>
    <Button
      variant="outline"
      onclick={minimize}
      class="border-none rounded-none h-full w-8 bg-stone-800 hover:bg-stone-700 transition-colors"
    >
      <Minus class="w-4 h-4 text-white" />
    </Button>
    <Button
      variant="outline"
      onclick={close}
      class="border-none rounded-none h-full w-8 bg-stone-800 hover:bg-red-700 transition-colors"
    >
      <X class="w-4 h-4 text-white" />
    </Button>
  </div>
</div>

<style>
  input {
    field-sizing: content;
  }
</style>
