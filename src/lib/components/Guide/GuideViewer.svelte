<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { ChevronLeft, ChevronRight, MapPin } from "@lucide/svelte";

  // Props
  let {
    guide,
    stepIndex = $bindable(0),
    checkboxState = $bindable({}),
    onPrev,
    onNext,
    // NOUVEAU : Callback pour demander au parent de changer de guide
    onNavigate,
  } = $props();

  let currentStep = $derived(guide.steps[stepIndex]);
  let progressPercentage = $derived(
    ((stepIndex + 1) / guide.steps.length) * 100,
  );

  let contentDiv: HTMLElement;

  // --- GESTION DES CLICS (NAVIGATION) ---
  function handleContentClick(event: MouseEvent) {
    const target = event.target as HTMLElement;

    // On cherche si l'élément cliqué (ou son parent) est un lien d'étape
    // On supporte ta classe .guide-step ET l'attribut data-type="guide-step"
    const stepLink = target.closest('[data-type="guide-step"], .guide-step');

    if (stepLink) {
      event.preventDefault(); // On empêche le comportement par défaut si c'était un lien

      const targetGuideId = stepLink.getAttribute("guideid");
      const targetStepNum = parseInt(
        stepLink.getAttribute("stepnumber") || "1",
      );

      // Calcul de l'index (Step 1 = Index 0)
      const targetIndex = Math.max(0, targetStepNum - 1);

      // CAS 1 : C'est le guide actuel (ID "0" ou ID identique)
      // (Note: on compare en string car les attributs HTML sont des strings)
      if (targetGuideId === "0" || targetGuideId == guide.id) {
        console.log("Navigation locale vers étape", targetIndex);
        stepIndex = targetIndex;
      }
      // CAS 2 : C'est un autre guide -> On prévient le parent
      else if (onNavigate) {
        console.log(
          "Navigation externe vers guide",
          targetGuideId,
          "étape",
          targetIndex,
        );
        onNavigate(targetGuideId, targetIndex);
      }
    }
  }

  // --- GESTION DES CHECKBOXES ---
  $effect(() => {
    if (!contentDiv) return;

    const inputs = contentDiv.querySelectorAll('input[type="checkbox"]');
    if (!checkboxState[stepIndex]) checkboxState[stepIndex] = [];

    inputs.forEach((input: HTMLInputElement, index) => {
      input.checked = checkboxState[stepIndex][index] || false;
      input.onchange = () => {
        checkboxState[stepIndex][index] = input.checked;
        checkboxState = { ...checkboxState };
      };
    });
  });

  // --- Fonctions utilitaires (Texte, Input, etc.) ---
  function handleStepInput(e: Event) {
    const input = e.target as HTMLInputElement;
    const val = parseInt(input.value);
    if (!isNaN(val) && val >= 1 && val <= guide.steps.length)
      stepIndex = val - 1;
    else input.value = (stepIndex + 1).toString();
  }

  let formattedText = $derived.by(() => {
    if (!currentStep?.web_text) return "";
    const posRegex = /\[(-?\d+)\s*,\s*(-?\d+)\]/g;
    return currentStep.web_text.replace(
      posRegex,
      (match) => `<span class="inline-pos">${match}</span>`,
    );
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") (e.target as HTMLInputElement).blur();
  }
</script>

<div class="flex flex-col h-full">
  <div
    class="flex-none pt-2 pb-0 border-b border-stone-800 bg-stone-900 z-10 flex flex-col gap-2 select-none"
  >
    <div class="flex justify-between items-end px-1">
      <div
        class="flex items-center gap-1.5 text-orange-500/80 hover:text-orange-500 hover:cursor-pointer font-mono text-xs px-2 py-0.5 rounded"
      >
        <MapPin class="w-3 h-3 " />
        {#if currentStep && (currentStep.pos_x !== 0 || currentStep.pos_y !== 0)}
          [{currentStep.pos_x}, {currentStep.pos_y}]
        {:else}
          [---, ---]
        {/if}
      </div>
      <div
        class="text-xs text-stone-500 font-mono pr-2 shrink-0 flex items-center"
      >
        <span>Étape</span>
        <input
          type="text"
          class="bg-transparent border-none p-0 mx-1 w-[3ch] text-center text-stone-500 font-mono focus:text-stone-200 focus:outline-none focus:bg-stone-800/50 rounded transition-colors cursor-text hover:text-stone-300"
          value={stepIndex + 1}
          onchange={handleStepInput}
          onkeydown={handleKeydown}
        />
        <span>/ {guide.steps.length}</span>
      </div>
    </div>
    <div class="w-full h-[2px] bg-stone-800 overflow-hidden mb-[-1px]">
      <div
        class="h-full bg-[#d4b07b] transition-all duration-300 ease-out shadow-[0_0_10px_rgba(234,88,12,0.5)]"
        style="width: {progressPercentage}%"
      ></div>
    </div>
  </div>

  <div
    bind:this={contentDiv}
    onclick={handleContentClick}
    role="button"
    tabindex="0"
    onkeydown={() => {}}
    class="flex-1 overflow-y-auto p-4 custom-scrollbar guide-content bg-stone-950/30 text-left cursor-auto"
  >
    {#if currentStep}
      <div class="text-stone-300">
        {@html formattedText}
      </div>
    {:else}
      <p class="text-red-500">Erreur: Étape introuvable.</p>
    {/if}
  </div>

  <div
    class="flex-none p-2 border-t border-stone-800 bg-stone-900 flex justify-between items-center gap-4 select-none"
  >
    <Button
      variant="secondary"
      onclick={onPrev}
      disabled={stepIndex === 0}
      class="w-28 select-none text-stone-300 bg-[#615d59] hover:bg-[#968d84]"
    >
      <ChevronLeft class="w-4 h-4 mr-1" /> Précédent
    </Button>
    <Button
      variant="default"
      onclick={onNext}
      disabled={stepIndex === guide.steps.length - 1}
      class="w-28 bg-[#a4713e] hover:bg-[#b8976f] select-none"
    >
      Suivant <ChevronRight class="w-4 h-4 ml-1" />
    </Button>
  </div>
</div>

<style>
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
  .guide-content :global(.tag-dungeon img),
  .guide-content :global(.tag-monster img) {
    width: 24px;
    height: 24px;
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

  .guide-content :global([data-type="guide-step"]) {
    color: #b19cd9; /* Violet clair (Couleur historique Dofus pour les étapes) */
    font-weight: 700;
    display: inline; /* S'assure qu'il reste dans le flux du texte */
  }

  .guide-content :global([data-type="guide-step"] img) {
    width: 20px;
    height: 20px;
    object-fit: contain;
    vertical-align: middle;
    margin-right: 4px;
  }

  .guide-content :global([data-type="guide-step"]:hover) {
    cursor: pointer;
  }

  .guide-content :global(.tag-dungeon) {
    color: #34d399; /* Vert clair */
    font-weight: 600;
    margin-right: 8px;
  }

  .guide-content :global(.tag-dungeon:hover) {
    cursor: pointer;
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

  .guide-content :global(.inline-pos) {
    color: #f97316; /* orange-500 */
    font-weight: 700;
    cursor: pointer;
    padding: 0 2px;
  }
</style>
