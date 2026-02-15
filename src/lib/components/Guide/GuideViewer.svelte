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
    if (windowStore.autoPilot && currentAnalysis) {
      const titles = windowStore.allSyncedTitles;

      try {
        if (titles.length > 1) {
          // Mode chaîné : envoie les commandes à tous les persos
          // puis attend UNE SEULE FOIS (chargement parallèle)
          await invoke("execute_step_automation_chained", {
            step: currentAnalysis,
            windowTitles: titles,
          });
        } else if (titles.length === 1) {
          await invoke("execute_step_automation", {
            step: currentAnalysis,
            windowTitle: titles[0],
          });
        }
      } catch (e) {
        console.error("Erreur Auto-Pilot:", e);
      }

      // Refocus après une automation multi-fenêtres
      if (titles.length > 1 && windowStore.fullTitle) {
        try {
          await invoke("focus_window", {
            windowTitle: windowStore.fullTitle,
          });
        } catch {}
      }
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
      active: windowStore.listenKeys,
      keys: windowStore.listenKeys ? ["left", "right"] : [],
    });
  }

  onMount(async () => {
    updateKeyListener();
    if (!unlistenHandle) {
      unlistenHandle = await listen("key-detected", (event) => {
        if (!windowStore.listenKeys) return;
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
    autoPilot={windowStore.autoPilot}
    listenKeys={windowStore.listenKeys}
    combatWatcher={windowStore.combatWatcherActive}
    onToggleAutoPilot={() => (windowStore.autoPilot = !windowStore.autoPilot)}
    onToggleListenKeys={() => {
      windowStore.listenKeys = !windowStore.listenKeys;
      updateKeyListener();
    }}
    onToggleCombatWatcher={() => windowStore.toggleCombatWatcher()}
  />
</div>
