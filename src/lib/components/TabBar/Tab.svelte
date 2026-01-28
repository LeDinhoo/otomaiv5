<script lang="ts">
  import { X } from "@lucide/svelte"; // Attention à l'import lucide-svelte vs @lucide/svelte selon ta version

  let { 
    label, 
    active = false, 
    onclick, 
    onclose, 
    closable = true // Nouvelle prop pour masquer la croix proprement
  } = $props();

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
  class="group flex items-center justify-between px-2 pr-1 h-full text-sm font-medium transition-colors relative border-r border-stone-800/50 outline-none select-none cursor-pointer whitespace-nowrap w-full
         {active
    ? 'text-[#d4b07b] bg-stone-700/50'
    : 'text-stone-400 hover:text-stone-200 hover:bg-stone-800'}"
>
  <span class="truncate mr-2">{label}</span>

  {#if closable}
    <button
      type="button"
      onclick={handleClose}
      class="p-0.5 rounded-full hover:bg-stone-500/30 text-stone-500 hover:text-stone-300 opacity-0 group-hover:opacity-100 transition-opacity shrink-0"
      title="Fermer l'onglet"
    >
      <X class="w-3.5 h-3.5" />
    </button>
  {:else}
    <div class="w-3.5"></div> 
  {/if}

  {#if active}
    <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-[#d4b07b]"></div>
  {/if}
</div>