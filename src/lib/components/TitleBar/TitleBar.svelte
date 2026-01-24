<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import {
    X,
    Minus,
    Lock,
    LockOpen,
    Loader2,
    BookSearch,
    Settings,
    WifiOff,
  } from "@lucide/svelte";

  let {
    usableTitle = $bindable(),
    onToggleLibrary,
    onToggleSettings,
  } = $props();

  // États de la synchronisation
  type SyncStatus = "none" | "synced" | "recovering" | "lost";

  let syncState = $state<SyncStatus>("none");
  let isLocked = $state(false);

  // Initialisation de l'écouteur d'événements
  onMount(async () => {
    const unlisten = await listen<string>("sync-status", (event) => {
      const status = event.payload;

      if (status === "synced") {
        syncState = "synced";
        isLocked = true;
      } else if (status === "recovering") {
        syncState = "recovering";
        isLocked = true;
      } else if (status === "lost") {
        syncState = "lost";
        isLocked = true;
      }
    });

    return () => {
      unlisten();
    };
  });

  // Gestion du Clic principal sur le Cadenas
  const handleLockAction = async () => {
    if (!isLocked) {
      // CAS 1 : On veut VERROUILLER -> Lancer la Synchro initiale
      if (!usableTitle || usableTitle.trim() === "") return;

      isLocked = true;
      syncState = "recovering"; // Feedback immédiat

      try {
        await invoke("sync_window_title", { characterName: usableTitle });
      } catch (err) {
        console.error("Erreur sync:", err);
        syncState = "lost";
      }
    } else {
      // CAS 2 : C'est déjà verrouillé.
      if (syncState === "synced") {
        // Vert -> On déverrouille pour éditer
        isLocked = false;
        syncState = "none";
      } else if (syncState === "lost") {
        // Rouge -> On relance la recherche (Retry)
        try {
          await invoke("trigger_auto_recovery");
        } catch (e) {
          console.error("Erreur trigger recovery:", e);
        }
      }
    }
  };

  const close = async () => {
    await getCurrentWindow().close();
  };

  const minimize = async () => {
    await getCurrentWindow().minimize();
  };
</script>

<div
  class="flex items-center justify-between bg-stone-800 h-8 relative select-none border-b border-stone-700"
>
  <div class="absolute inset-0" data-tauri-drag-region></div>

  <div class="flex items-center z-40 pl-2">
    <Button
      variant="ghost"
      size="icon"
      class="h-6 w-6 text-stone-500 hover:text-stone-200 hover:bg-stone-700 mr-1"
      onclick={onToggleLibrary}
      title="Bibliothèque de guides"
      onmousedown={(e) => e.stopPropagation()}
    >
      <BookSearch class="w-4 h-4" />
    </Button>

    <Button
      variant="ghost"
      size="icon"
      class="h-6 w-6 text-stone-500 hover:text-stone-200 hover:bg-stone-700"
      onclick={onToggleSettings}
      title="Configuration"
      onmousedown={(e) => e.stopPropagation()}
    >
      <Settings class="w-4 h-4" />
    </Button>

    <Button
      variant="ghost"
      size="icon"
      class="h-6 w-6 hover:bg-stone-700"
      title="Archimonstre"
      onmousedown={(e) => e.stopPropagation()}
    >
      <img src="archmonster.png" alt="Archimonstre" class="w-5 h-5 -translate-y-[1.5px] hover:opacity-100 opacity-60" />
    </Button>
  </div>

  <div
    class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-20 flex items-center gap-2 group"
  >
    <input
      type="text"
      tabindex={isLocked ? -1 : 0}
      bind:value={usableTitle}
      readonly={isLocked}
      class="bg-transparent border-none text-sm font-bold text-stone-200 text-center
             focus:outline-none focus:bg-stone-700/50 focus:ring-1 focus:ring-stone-600
             rounded px-2 py-0.5 transition-all outline-none min-w-[50px]
             {isLocked
        ? 'pointer-events-none cursor-default opacity-80'
        : 'pointer-events-auto cursor-text hover:bg-stone-700/30'}"
      spellcheck="false"
      placeholder="Personnage..."
      onkeydown={(e) => e.key === "Enter" && handleLockAction()}
    />

    <button
      onclick={handleLockAction}
      class="p-1.5 rounded transition-all relative z-30 flex items-center justify-center
      {syncState === 'recovering'
        ? 'cursor-wait'
        : 'cursor-pointer hover:bg-stone-700'}"
      title={isLocked
        ? syncState === "lost"
          ? "Perdu ! Cliquer pour relancer"
          : "Déverrouiller"
        : "Verrouiller et Synchroniser"}
    >
      {#if !isLocked}
        <LockOpen
          class="w-3.5 h-3.5 text-yellow-500/80 group-hover:text-yellow-400"
        />
      {:else if syncState === "recovering"}
        <Loader2 class="w-3.5 h-3.5 text-orange-500 animate-spin" />
      {:else if syncState === "lost"}
        <div class="relative">
          <WifiOff class="w-3.5 h-3.5 text-red-500 animate-pulse" />
          <span class="absolute -top-1 -right-1 flex h-2 w-2">
            <span
              class="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-400 opacity-75"
            ></span>
            <span class="relative inline-flex rounded-full h-2 w-2 bg-red-500"
            ></span>
          </span>
        </div>
      {:else}
        <Lock class="w-3.5 h-3.5 text-green-500" />
      {/if}
    </button>
  </div>

  <div class="flex flex-row ml-auto h-full relative z-10">
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
