<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
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
    RefreshCw,
    Pencil,
  } from "@lucide/svelte";

  import { windowStore } from "$lib/stores/windowStore.svelte";
  import { showCustomNotification } from "$lib/utils";

  onMount(async () => {
    const unlisten = await listen<string>("sync-status", (event) => {
      const status = event.payload;

      if (status === "synced") {
        windowStore.syncState = "synced";
        windowStore.isLocked = true;
      } else if (status === "recovering") {
        windowStore.syncState = "recovering";
        windowStore.isLocked = true;
      } else if (status === "lost") {
        windowStore.syncState = "lost";
        windowStore.isLocked = true;
      }
    });

    return () => {
      unlisten();
    };
  });

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

  <div class="flex items-center z-40 pl-2 gap-1">
    <Button
      variant="ghost"
      size="icon"
      class="h-6 w-6 text-stone-500 hover:text-stone-200 hover:bg-stone-700"
      onclick={() => windowStore.toggleView("library")}
      title="Bibliothèque de guides"
      onmousedown={(e) => e.stopPropagation()}
    >
      <BookSearch class="w-4 h-4" />
    </Button>

    <Button
      variant="ghost"
      size="icon"
      class="h-6 w-6 text-stone-500 hover:text-stone-200 hover:bg-stone-700"
      onclick={() => windowStore.toggleView("settings")}
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
      onclick={() => showCustomNotification()}
      onmousedown={(e) => e.stopPropagation()}
    >
      <img
        src="archmonster.png"
        alt="Archimonstre"
        class="w-5 h-5 -translate-y-[1.5px] hover:opacity-100 opacity-60"
      />
    </Button>
  </div>

  <div
    class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-20 flex items-center gap-1 group"
  >
    <!-- Bouton éditer/verrouiller le nom -->
    <button
      onclick={() => windowStore.toggleNameLock()}
      class="p-1.5 rounded transition-all z-30 flex items-center justify-center cursor-pointer hover:bg-stone-700"
      title={windowStore.isLocked ? "Modifier le nom" : "Confirmer le nom"}
    >
      {#if windowStore.isLocked}
        <Pencil class="w-3 h-3 text-stone-500 hover:text-stone-300" />
      {:else}
        <Lock class="w-3 h-3 text-yellow-500/80" />
      {/if}
    </button>

    <input
      type="text"
      tabindex={windowStore.isLocked ? -1 : 0}
      bind:value={windowStore.usableTitle}
      readonly={windowStore.isLocked}
      class="bg-transparent border-none text-sm font-bold text-stone-200 text-center
             focus:outline-none focus:bg-stone-700/50 focus:ring-1 focus:ring-stone-600
             rounded px-2 py-0.5 transition-all outline-none min-w-[50px]
             {windowStore.isLocked
        ? 'pointer-events-none cursor-default opacity-80'
        : 'pointer-events-auto cursor-text hover:bg-stone-700/30'}"
      spellcheck="false"
      placeholder="Personnage..."
      onkeydown={(e) => {
        if (e.key === "Enter") {
          windowStore.toggleNameLock();
          windowStore.triggerSync();
        }
      }}
    />

    <!-- Bouton sync/resync séparé -->
    <button
      onclick={() => {
        if (windowStore.syncState === "lost") {
          windowStore.triggerRecovery();
        } else {
          windowStore.triggerSync();
        }
      }}
      disabled={windowStore.syncState === "recovering"}
      class="p-1.5 rounded transition-all relative z-30 flex items-center justify-center
      {windowStore.syncState === 'recovering'
        ? 'cursor-wait'
        : 'cursor-pointer hover:bg-stone-700'}"
      title={windowStore.syncState === "lost"
        ? "Connexion perdue — Cliquer pour relancer"
        : windowStore.syncState === "synced"
          ? "Synchronisé — Cliquer pour resync"
          : windowStore.syncState === "recovering"
            ? "Synchronisation en cours..."
            : "Synchroniser"}
    >
      {#if windowStore.syncState === "recovering"}
        <Loader2 class="w-3.5 h-3.5 text-orange-500 animate-spin" />
      {:else if windowStore.syncState === "lost"}
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
      {:else if windowStore.syncState === "synced"}
        <RefreshCw class="w-3.5 h-3.5 text-green-500" />
      {:else}
        <RefreshCw class="w-3.5 h-3.5 text-stone-500 hover:text-stone-300" />
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
