<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { windowStore } from "$lib/stores/windowStore.svelte";
  import { guideStore } from "$lib/stores/guideStore.svelte";

  let {
    currentStep,
    guideId,
    tabId,
    stepIndex,
    onNavigate,
  } = $props();

  let contentDiv: HTMLElement | undefined = $state();
  let tooltip = $state({ visible: false, text: "" });

  let formattedText = $derived.by(() => {
    if (!currentStep?.web_text) return "";
    const posRegex = /\[(-?\d+)\s*,\s*(-?\d+)\]/g;
    return currentStep.web_text.replace(
      posRegex,
      (match: string) => `<span class="inline-pos">${match}</span>`,
    );
  });

  function handleContentClick(event: MouseEvent) {
    const target = event.target as HTMLElement;

    if (target.matches(".inline-pos")) {
      event.preventDefault();
      event.stopPropagation();

      const rawText = target.textContent || "";
      const coords = rawText.replace(/[\[\]]/g, "").trim();

      if (coords) {
        invoke("send_chat_command", {
          command: `/travel ${coords}`,
          windowTitle: windowStore.fullTitle,
        }).catch((err) => {
          console.error("Erreur travel:", err);
        });
      }
      return;
    }

    const stepLink = target.closest('[data-type="guide-step"], .guide-step');
    if (stepLink && onNavigate) {
      event.preventDefault();
      const targetGuideId = stepLink.getAttribute("guideid");

      const stepIdAttr = stepLink.getAttribute("stepid");
      const stepNumAttr = stepLink.getAttribute("stepnumber");

      let targetIndex = -1;

      if (stepIdAttr && stepIdAttr !== "0" && stepNumAttr) {
        const parsedStep = parseInt(stepNumAttr);
        if (!isNaN(parsedStep)) {
          targetIndex = Math.max(0, parsedStep - 1);
        }
      }

      const finalGuideId =
        targetGuideId === "0" || targetGuideId == guideId
          ? guideId
          : targetGuideId;

      onNavigate(finalGuideId, targetIndex);
    }
  }

  $effect(() => {
    if (!contentDiv) return;

    const inputs = contentDiv.querySelectorAll('input[type="checkbox"]');

    if (!guideStore.checkboxStates[tabId]) guideStore.checkboxStates[tabId] = {};
    const checkboxState = guideStore.checkboxStates[tabId];

    inputs.forEach((inputElement, index) => {
      const input = inputElement as HTMLInputElement;

      input.checked = checkboxState[stepIndex]?.[index] || false;

      input.onchange = () => {
        if (!checkboxState[stepIndex]) checkboxState[stepIndex] = [];
        checkboxState[stepIndex][index] = input.checked;
      };
    });

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
</script>

<div
  bind:this={contentDiv}
  onclick={handleContentClick}
  role="button"
  tabindex="0"
  onkeydown={() => {}}
  class="flex-1 overflow-y-auto p-2 custom-scrollbar guide-content bg-stone-950/30 text-left cursor-auto"
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
          <div class="tooltip-body">{tooltip.text}</div>
        </div>
      {/if}
    </div>
  {:else}
    <p class="text-red-500">Erreur: Étape introuvable.</p>
  {/if}
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
    font-size: 0.9rem;
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
    padding: 10px;
    gap: 8px;
    margin: 1rem 0;
    border-radius: 10px;
  }

  .guide-content :global([data-type="quest-block"] p) {
    margin-bottom: 0;
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

  .fixed-guide-tooltip {
    position: absolute;
    top: 50px;
    right: 30px;
    z-index: 50;

    background-color: #1c1917;
    border: 1px solid #cecece21;
    border-radius: 8px;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.6);
    display: flex;

    flex-direction: row;
    align-items: center;

    animation: slideIn 0.2s ease-out;
    pointer-events: none;
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

  .guide-content :global([data-type="quest-block"]) {
    border-bottom: 1px solid #4444447a !important;
  }
</style>
