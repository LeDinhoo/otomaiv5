<script lang="ts">
  // @ts-nocheck
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { Button } from "$lib/components/ui/button/index.js";
  import {
    X,
    Minus,
    Settings,
    Lock,
    LockOpen,
    Check,
    BookSearch,
  } from "@lucide/svelte";

  let {
    windowTitle = $bindable(), // Le titre technique (pour info, si besoin)
    usableTitle = $bindable(), // Le titre affiché (celui qu'on modifie)
    statusMessage = "", // L'état de la synchro venant de App.svelte
    onToggleSettings,
  } = $props();

  // Par défaut verrouillé si on a déjà un titre, sinon ouvert
  let isLocked = $state(usableTitle && usableTitle !== "Mon Personnage");

  // On déduit si c'est trouvé grâce au message du parent (App.svelte)
  let isFound = $derived(statusMessage.includes("✅"));
  let isError = $derived(statusMessage.includes("❌"));

  const toggleLock = () => {
    isLocked = !isLocked;
  };

  const close = async () => {
    const window = getCurrentWindow();
    await window.close();
  };

  const minimize = async () => {
    const window = getCurrentWindow();
    await window.minimize();
  };
</script>

<div
  class="flex items-center justify-between bg-stone-800 h-8 relative select-none border-b border-stone-700"
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
    class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-20 flex items-center gap-1 group"
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
        ? 'pointer-events-none cursor-default'
        : 'pointer-events-auto cursor-text hover:bg-stone-700/30'}"
      spellcheck="false"
      placeholder="Personnage..."
    />

    <div class="flex items-center">
      <button
        onclick={toggleLock}
        class="p-1 rounded hover:bg-stone-700 transition-colors relative z-30
        {isLocked
          ? 'opacity-50 group-hover:opacity-100'
          : 'opacity-100 text-yellow-500'}"
        title={isLocked
          ? "Déverrouiller pour modifier"
          : "Verrouiller le titre"}
      >
        {#if isLocked}
          <Lock class="w-3 h-3" />
        {:else}
          <LockOpen class="w-3 h-3" />
        {/if}
      </button>

      {#if isLocked && isFound}
        <Check
          class="w-4 h-4 text-green-500 ml-1 animate-in fade-in zoom-in duration-300"
        />
      {/if}

      {#if isLocked && isError}
        <X
          class="w-4 h-4 text-red-500 ml-1 animate-in fade-in zoom-in duration-300"
        />
      {/if}
    </div>
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
  /* Cette propriété CSS magique permet à l'input de s'adapter à la largeur du texte */
  input {
    field-sizing: content;
  }
</style>
