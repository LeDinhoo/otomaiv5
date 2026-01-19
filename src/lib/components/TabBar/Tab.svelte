<script lang="ts">
  import { X } from "@lucide/svelte";

  let { label, active = false, onclick, onclose } = $props();

  function handleClose(e: MouseEvent) {
    e.stopPropagation();
    if (onclose) onclose();
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === " ") {
      onclick();
    }
  }
</script>

<div
  role="button"
  tabindex="0"
  {onclick}
  onkeydown={handleKeyDown}
  class="group flex-1 flex items-center justify-between px-2 h-full text-sm font-medium transition-all relative border-r border-stone-800/50 outline-none min-w-0
         {active
    ? 'text-[#d4b07b] bg-stone-700/50'
    : 'text-stone-400 hover:text-stone-200 hover:bg-stone-800'}"
>
  <span class="truncate select-none">{label}</span>

  <button
    type="button"
    onclick={handleClose}
    class=" p-0.5 rounded-full hover:bg-stone-500/30 opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0"
    title="Fermer l'onglet"
  >
    <X class="w-3.5 h-3.5" />
  </button>

  {#if active}
    <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-[#d4b07b]"></div>
  {/if}
</div>
