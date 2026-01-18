<script>
  // @ts-nocheck
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { Button } from "$lib/components/ui/button/index.js";
  import { X, Minus, Settings, Lock, LockOpen } from "@lucide/svelte";

  let { windowTitle = $bindable() } = $props();
  let isLocked = $state(true);

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
  class="flex items-center justify-between bg-stone-800 h-8 relative select-none"
>
  <div class="absolute inset-0" data-tauri-drag-region></div>

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
