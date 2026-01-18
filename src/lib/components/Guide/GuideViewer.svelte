<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { ChevronLeft, ChevronRight } from "@lucide/svelte";

  // Props
  let { guide, stepIndex = 0, onPrev, onNext } = $props();

  // Variables dérivées
  let currentStep = $derived(guide.steps[stepIndex]);
</script>

<div class="flex flex-col h-full">
  <div class="flex-none p-2 border-b border-stone-800 bg-stone-900 z-10">
    <div class="flex justify-between items-end">
      <h2 class="text-sm font-bold text-stone-200 truncate pr-4">
        {guide.name}
      </h2>
      <span class="text-xs text-stone-500 font-mono flex-shrink-0">
        Étape {stepIndex + 1} / {guide.steps.length}
      </span>
    </div>
  </div>

  <div
    class="flex-1 overflow-y-auto p-4 custom-scrollbar guide-content bg-stone-950/30"
  >
    {#if currentStep}
      <div class="text-stone-300">
        {@html currentStep.web_text}
      </div>
    {:else}
      <p class="text-red-500">Erreur: Étape introuvable.</p>
    {/if}
  </div>

  <div
    class="flex-none p-4 border-t border-stone-800 bg-stone-900 flex justify-between items-center gap-4"
  >
    <Button
      variant="secondary"
      onclick={onPrev}
      disabled={stepIndex === 0}
      class="w-32"
    >
      <ChevronLeft class="w-4 h-4 mr-1" /> Précédent
    </Button>

    <Button
      variant="default"
      onclick={onNext}
      disabled={stepIndex === guide.steps.length - 1}
      class="w-32 bg-orange-700 hover:bg-orange-600"
    >
      Suivant <ChevronRight class="w-4 h-4 ml-1" />
    </Button>
  </div>
</div>

<style>
  /* ... Colle ici tout le CSS que je t'ai fourni dans la réponse précédente ... */
  /* Je ne le remets pas pour ne pas spammer, mais c'est EXACTEMENT le même bloc */

  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: #57534e;
    border-radius: 0;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.2);
  }

  .guide-content {
    font-size: 1.05rem;
    line-height: 1.6;
    color: #e7e5e4;
  }

  /* ... Suite du CSS (Tags, Images, Checklists, etc.) ... */
  .guide-content :global(p) {
    margin-bottom: 0.8rem;
    display: block;
  }
  .guide-content :global(p:empty) {
    display: none;
  }
  .guide-content :global(img) {
    vertical-align: middle;
    display: inline-block;
    max-width: 100%;
    height: auto;
    border-radius: 10px;
  }
  .guide-content :global(.tag-item),
  .guide-content :global(.tag-quest),
  .guide-content :global(.tag-monster),
  .guide-content :global(.tag-map) {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    text-decoration: none !important;
    vertical-align: middle;
  }
  .guide-content :global(.tag-item:hover),
  .guide-content :global(.tag-quest:hover) {
    cursor: pointer;
  }
  .guide-content :global(.tag-item) {
    color: #facc15;
  }
  .guide-content :global(.tag-quest) {
    color: #f472b6;
  }
  .guide-content :global(.tag-monster) {
    color: #ef4444;
  }
  .guide-content :global(.tag-map) {
    color: #60a5fa;
    font-family: monospace;
    border: none;
    background: transparent;
    padding: 0;
  }
  .guide-content :global(.tag-item img),
  .guide-content :global(.tag-quest img),
  .guide-content :global(.tag-monster img) {
    width: 20px;
    height: 20px;
    object-fit: contain;
    border-radius: 0;
  }
  .guide-content :global(ul[data-type="taskList"]) {
    list-style: none;
    padding: 10px;
    margin: 1rem 0;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }
  .guide-content :global(li[data-type="taskItem"]) {
    display: flex;
    align-items: flex-start;
    margin-bottom: 8px;
    padding-bottom: 8px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }
  .guide-content :global(li[data-type="taskItem"]:last-child) {
    border-bottom: none;
    margin-bottom: 0;
    padding-bottom: 0;
  }
  .guide-content :global(li[data-type="taskItem"] label) {
    display: flex;
    align-items: center;
    margin-right: 12px;
    margin-top: 4px;
    cursor: pointer;
  }
  .guide-content :global(li[data-type="taskItem"] div) {
    flex: 1;
    min-width: 0;
  }
  .guide-content :global(li[data-type="taskItem"] p) {
    margin: 0;
    display: inline-block;
  }
  .guide-content :global(span[style*="rgb(255, 255, 0)"]),
  .guide-content :global(span[style*="#ffff00"]) {
    color: #fde047 !important;
    font-weight: 600;
  }
  .guide-content :global(span[style*="rgb(250, 0, 0)"]) {
    color: #f87171 !important;
    font-weight: 600;
  }
  .guide-content :global(.img-large) {
    display: block;
    margin: 1.5rem auto;
    max-width: 100%;
    box-shadow: none;
    border-radius: 8px;
  }
  .guide-content :global([data-tooltip]) {
    cursor: help;
    border-bottom: 1px dotted #a8a29e;
    position: relative;
    border-radius: 4px;
  }
  .guide-content :global([data-tooltip]:hover::after) {
    content: attr(data-tooltip);
    position: absolute;
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    background: #1c1917;
    border: 1px solid #444;
    color: white;
    padding: 4px 8px;
    font-size: 12px;
    white-space: nowrap;
    z-index: 50;
    border-radius: 0;
    margin-bottom: 5px;
  }
</style>
