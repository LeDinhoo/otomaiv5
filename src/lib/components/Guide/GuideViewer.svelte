<script lang="ts">
  import { Button } from "$lib/components/ui/button/";
  import {
    ChevronLeft,
    ChevronRight,
    MapPin,
    Keyboard,
    Waypoints,
  } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  let {
    guide,
    stepIndex = $bindable(0),
    checkboxState = $bindable({}),
    onPrev,
    onNext,
    // On récupère le titre synchronisé pour l'Auto-Pilot
    usableTitle = $bindable(""),
    onNavigate,
  } = $props();

  let currentStep = $derived(guide.steps[stepIndex]);
  let progressPercentage = $derived(
    ((stepIndex + 1) / guide.steps.length) * 100,
  );

  let contentDiv: HTMLElement;

  // --- ÉTATS ---
  let listenKeys = $state(true);
  let autoPilot = $state(false); // État de l'Auto-Pilot
  let unlistenHandle: (() => void) | undefined;

  // État pour les info-bulles de quêtes
  let tooltip = $state({
    visible: false,
    text: "",
  });

  // Stockage de l'analyse Rust pour exécution
  let currentAnalysis = $state(null);

  // --- 1. LOGIQUE AUTO-PILOT (RUST) ---

  // Analyse l'étape au chargement (sans bouger)
  async function analyzeStep(html: string) {
    try {
      const result = await invoke("parse_guide_step", { htmlContent: html });
      currentAnalysis = result;
    } catch (err) {
      console.error("Erreur parsing :", err);
      currentAnalysis = null;
    }
  }

  // Action au clic sur "Suivant" (ou touche D)
  async function handleNextAction() {
    // Si Auto-Pilot est ON, on exécute le voyage
    if (autoPilot && currentAnalysis) {
      console.log(`🚀 Auto-Pilot : Exécution...`);
      invoke("execute_step_automation", {
        step: currentAnalysis,
        windowTitle: usableTitle, // Sécurité
      }).catch((e) => console.error("Erreur Auto-Pilot:", e));
    }
    // On passe à la suite
    onNext();
  }

  // Surveille le changement d'étape pour lancer l'analyse
  $effect(() => {
    if (currentStep && currentStep.web_text) {
      analyzeStep(currentStep.web_text);
    }
  });

  // --- 2. GESTION CLAVIER (Ta version améliorée) ---

  onMount(async () => {
    // 1. On active l'écoute côté Rust
    await invoke("set_key_listener", { active: true, keys: ["a", "d"] });

    // 2. Sécurité : On s'assure qu'on n'écoute pas déjà
    if (!unlistenHandle) {
      unlistenHandle = await listen("key-detected", (event) => {
        if (!listenKeys) return;

        const key = event.payload as string;
        if (key === "a") {
          onPrev();
        } else if (key === "d") {
          handleNextAction(); // <-- MODIFIÉ : Appelle la logique Auto-Pilot
        }
      });
    }
  });

  onDestroy(() => {
    invoke("set_key_listener", { active: false, keys: [] });
    if (unlistenHandle) {
      unlistenHandle();
      unlistenHandle = undefined;
    }
  });

  function handleListenKeysChange() {
    listenKeys = !listenKeys;
    invoke("set_key_listener", {
      active: listenKeys,
      keys: listenKeys ? ["a", "d"] : [],
    });
  }

  // --- 3. LOGIQUE UI (Checkboxes, Tooltips, Navigation) ---

  // Gestion des clics sur les liens (Navigation inter-guides)
  function handleContentClick(event: MouseEvent) {
    const target = event.target as HTMLElement;
    const stepLink = target.closest('[data-type="guide-step"], .guide-step');

    if (stepLink) {
      event.preventDefault();
      const targetGuideId = stepLink.getAttribute("guideid");
      const targetStepNum = parseInt(
        stepLink.getAttribute("stepnumber") || "1",
      );
      const targetIndex = Math.max(0, targetStepNum - 1);

      if (targetGuideId === "0" || targetGuideId == guide.id) {
        stepIndex = targetIndex;
      } else if (onNavigate) {
        onNavigate(targetGuideId, targetIndex);
      }
    }
  }

  // Effet combiné : Checkboxes + Tooltips Quêtes
  $effect(() => {
    if (!contentDiv) return;

    // A. Checkboxes
    const inputs = contentDiv.querySelectorAll('input[type="checkbox"]');
    if (!checkboxState[stepIndex]) checkboxState[stepIndex] = [];

    inputs.forEach((input: HTMLInputElement, index) => {
      input.checked = checkboxState[stepIndex][index] || false;
      input.onchange = () => {
        checkboxState[stepIndex][index] = input.checked;
        checkboxState = { ...checkboxState };
      };
    });

    // B. Tooltips Quêtes (Ta logique)
    const questBlocks = contentDiv.querySelectorAll(
      '[data-type="quest-block"][title]',
    );
    questBlocks.forEach((el) => {
      const titleText = el.getAttribute("title");
      if (titleText) {
        el.setAttribute("data-tooltip-text", titleText);
        el.removeAttribute("title");
        el.addEventListener("mouseenter", () => {
          tooltip.text = titleText;
          tooltip.visible = true;
        });
        el.addEventListener("mouseleave", () => {
          tooltip.visible = false;
        });
      }
    });
  });

  // --- UTILS ---
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

        {#if tooltip.visible}
          <div class="fixed-guide-tooltip">
            <img
              src="https://ganymede-dofus.com/images/icon_quest.png"
              alt="quest"
            />
            <div class="tooltip-body">
              {tooltip.text}
            </div>
          </div>
        {/if}
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

    <div class="flex items-center overflow-hidden rounded-sm h-full">
      <Button
        variant="secondary"
        onclick={() => (autoPilot = !autoPilot)}
        title="Activer l'Auto-Pilot"
        class="{autoPilot
          ? ' bg-[#a09890b9] hover:bg-[#d1c4b7b2] '
          : 'bg-[#615d59] hover:bg-[#968d84]'} size-9 cursor-pointer rounded-none h-full flex items-center justify-center select-none"
      >
        <Waypoints
          class="size-5 drop-shadow-4xl {autoPilot
            ? 'text-[#f7c882]'
            : 'text-stone-300'}"
        />
      </Button>

      <Button
        variant="secondary"
        onclick={handleListenKeysChange}
        title="Raccourcis Clavier (A/D)"
        class="{listenKeys
          ? ' bg-[#a09890b9] hover:bg-[#d1c4b7b2] '
          : 'bg-[#615d59] hover:bg-[#968d84]'} size-9 cursor-pointer rounded-none h-full flex items-center justify-center select-none"
      >
        <Keyboard
          class="size-5 drop-shadow-4xl {listenKeys
            ? 'text-[#f7c882]'
            : 'text-stone-300'}"
        />
      </Button>
    </div>

    <Button
      variant="default"
      onclick={handleNextAction}
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
    font-family: sans-serif;
    font-size: 1.05rem;
    font-weight: 100;
    color: #e7e5e4;
  }

  .guide-content :global(p) {
    margin-bottom: 1rem;
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

  .guide-content :global([data-type="quest-block"]) {
    border: 1px solid #4444447a;
    background: rgba(0, 0, 0, 0.2);
    padding: 10px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding: 16px;
    gap: 8px;
    margin: 1rem 0;
    border-radius: 10px;
  }

  .guide-content :global([data-type="quest-block"]):hover {
    background: rgba(0, 0, 0, 0.3);
    cursor: pointer;
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

  .guide-content :global(.tag-monster img) {
    width: 32px;
    height: 32px;
    object-fit: contain;
    border-radius: 8px;
  }

  .guide-content :global(.title) {
    font-size: 1.5rem;
    font-weight: 700;
    margin: 1rem 0 0.5rem 0;
    color: #fbbf24;
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
    color: #ff3131 !important;
    font-weight: 600;
  }

  .guide-content :global(.img-large) {
    display: block;
    margin: 1.5rem 0;
    max-width: 100%;
    box-shadow: none;
    border-radius: 8px;
  }

  .guide-content :global([data-tooltip]) {
    cursor: help;
    position: relative;
  }

  .guide-content :global([data-type="guide-step"]) {
    color: #b19cd9;
    font-weight: 700;
    display: inline;
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
    color: #34d399;
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
    color: #f97316;
    font-weight: 700;
    cursor: pointer;
    padding: 0 2px;
  }

  /* Style spécifique pour les quest-blocks convertis */

  /* Style de l'infobulle flottante */
  /* Configuration du tooltip fixe */
  .fixed-guide-tooltip {
    position: absolute;
    top: 50px;
    right: 30px;
    z-index: 50;

    background-color: #1c1917; /* stone-900 */
    border: 1px solid #cecece21; /* Bordure rose quête */
    border-radius: 8px;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.6);
    display: flex;

    flex-direction: row;
    align-items: center;

    /* Animation d'entrée douce */
    animation: slideIn 0.2s ease-out;
    pointer-events: none; /* Important : permet de cliquer "au travers" si besoin */
  }

  .fixed-guide-tooltip img {
    height: 20px;
    width: 18px;
    margin-left: 12px;
  }

  .tooltip-body {
    padding: 6px;
    margin-right: 8px;
    color: #e7e5e4;
    font-size: 0.9rem;
    line-height: 1.4;
    white-space: pre-wrap;
    color: #f472b6;
    font-weight: 600;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(10px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  /* RAPPEL : Correction pour enlever la ligne blanche pointillée par défaut */
  .guide-content :global([data-type="quest-block"]) {
    border-bottom: 1px solid #4444447a !important;
  }
</style>
