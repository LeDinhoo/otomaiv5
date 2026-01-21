<script lang="ts">
  import { Loader2 } from "@lucide/svelte";

  let { loading, error, guides, onSelect } = $props();
</script>

<div class="flex-1 overflow-y-auto">
  {#if loading && guides.length === 0}
    <div class="flex h-full items-center justify-center text-stone-500 gap-2">
      <Loader2 class="animate-spin" /> Chargement...
    </div>
  {:else if error}
    <div
      class="flex h-full items-center justify-center text-red-400 p-4 text-center"
    >
      {error}
    </div>
  {:else if guides.length === 0}
    <div class="flex h-full items-center justify-center text-stone-500 italic">
      Aucun guide trouvé.
    </div>
  {:else}
    <div class="flex flex-col gap-2">
      {#each guides as guide (guide.id)}
        <button
          onclick={() => onSelect(guide.id)}
          class="flex items-center justify-between p-3 rounded-lg bg-stone-800/40 border border-stone-800 hover:bg-stone-800 hover:border-orange-500/50 transition-all text-left group"
        >
          <span
            class="font-medium text-stone-300 group-hover:text-white truncate pr-4"
          >
            {guide.name}
          </span>
          <span
            class="text-xs text-stone-600 font-mono group-hover:text-orange-400"
          >
            Ouvrir →
          </span>
        </button>
      {/each}
    </div>
  {/if}
</div>
