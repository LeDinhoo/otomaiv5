<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  import GuideHeader from "./GuideHeader.svelte";
  import GuideContent from "./GuideContent.svelte";
  import GuideControls from "./GuideControls.svelte";

  import { guideStore } from "$lib/stores/guideStore.svelte";
  import { windowStore } from "$lib/stores/windowStore.svelte";
  import { tabStore } from "$lib/stores/tabStore.svelte";

  let { guide, tabId } = $props();

  // --- États Logiques ---
  let listenKeys = $state(false);
  let autoPilot = $state(false);
  let unlistenHandle: (() => void) | undefined;
  let currentAnalysis = $state(null);

  let stepIndex = $derived(guideStore.guideProgress[tabId] ?? 0);
  let currentStep = $derived(guide.steps[stepIndex]);

  // --- Logique Métier (Parsing & Autopilot) ---
  async function analyzeStep(html: string) {
    try {
      currentAnalysis = await invoke("parse_guide_step", { htmlContent: html });
    } catch (err) {
      console.error("Erreur parsing :", err);
      currentAnalysis = null;
    }
  }

  async function handleNextAction() {
    if (autoPilot && currentAnalysis) {
      invoke("execute_step_automation", {
        step: currentAnalysis,
        windowTitle: windowStore.fullTitle,
      }).catch((e) => console.error("Erreur Auto-Pilot:", e));
    }
    guideStore.nextStep(tabId, guide.steps.length);
  }

  function handlePrev() {
    guideStore.prevStep(tabId);
  }

  $effect(() => {
    if (currentStep?.web_text) analyzeStep(currentStep.web_text);
  });

  // --- Gestion des Raccourcis Clavier (Tauri Events) ---
  function updateKeyListener() {
    invoke("set_key_listener", {
      active: listenKeys,
      keys: listenKeys ? ["left", "right"] : [],
    });
  }

  onMount(async () => {
    updateKeyListener();
    if (!unlistenHandle) {
      unlistenHandle = await listen("key-detected", (event) => {
        if (!listenKeys) return;
        const key = event.payload as string;
        if (key === "left") handlePrev();
        else if (key === "right") handleNextAction();
      });
    }
  });

  onDestroy(() => {
    invoke("set_key_listener", { active: false, keys: [] });
    if (unlistenHandle) unlistenHandle();
  });

  function handleInternalNavigate(targetId: string, targetIndex: number) {
    if (targetId == guide.id) {
      if (targetIndex !== -1) {
        guideStore.setStep(tabId, targetIndex);
      }
    } else {
      // Ouvrir un autre guide et naviguer
      tabStore.openGuide(targetId).then((newTabId) => {
        if (newTabId && targetIndex !== -1) {
          setTimeout(() => {
            guideStore.setStep(newTabId, targetIndex);
          }, 50);
        }
      });
    }
  }
</script>

<div class="flex flex-col h-full">
  <GuideHeader
    {stepIndex}
    totalSteps={guide.steps.length}
    {currentStep}
    {tabId}
  />

  <GuideContent
    {currentStep}
    guideId={guide.id}
    {tabId}
    {stepIndex}
    onNavigate={handleInternalNavigate}
  />

  <GuideControls
    canGoPrev={stepIndex > 0}
    canGoNext={stepIndex < guide.steps.length - 1}
    onPrev={handlePrev}
    onNext={handleNextAction}
    bind:autoPilot
    bind:listenKeys
    onToggleListenKeys={() => {
      listenKeys = !listenKeys;
      updateKeyListener();
    }}
  />
</div>
