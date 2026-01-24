<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  // Imports des nouveaux composants
  import GuideHeader from "./GuideHeader.svelte";
  import GuideContent from "./GuideContent.svelte";
  import GuideControls from "./GuideControls.svelte";

  let {
    guide,
    stepIndex = $bindable(0),
    checkboxState = $bindable({}),
    onPrev,
    onNext,
    fullTitle = $bindable(),
    onNavigate,
  } = $props();

  // --- États Logiques ---
  let listenKeys = $state(false);
  let autoPilot = $state(false);
  let unlistenHandle: (() => void) | undefined;
  let currentAnalysis = $state(null);

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
        windowTitle: fullTitle,
      }).catch((e) => console.error("Erreur Auto-Pilot:", e));
    }
    onNext();
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
        if (key === "left") onPrev();
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
      stepIndex = targetIndex;
    } else if (onNavigate) {
      onNavigate(targetId, targetIndex);
    }
  }
</script>

<div class="flex flex-col h-full ">
  <GuideHeader bind:stepIndex totalSteps={guide.steps.length} {currentStep} />

  <GuideContent
    {currentStep}
    guideId={guide.id}
    bind:checkboxState={checkboxState[stepIndex]}
    onNavigate={handleInternalNavigate}
    {fullTitle}
  />

  <GuideControls
    canGoPrev={stepIndex > 0}
    canGoNext={stepIndex < guide.steps.length - 1}
    {onPrev}
    onNext={handleNextAction}
    bind:autoPilot
    bind:listenKeys
    onToggleListenKeys={() => {
      listenKeys = !listenKeys;
      updateKeyListener();
    }}
  />
</div>
