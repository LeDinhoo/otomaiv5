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
  let mirrorClicks = $state(false);
  let combatWatcher = $state(false);
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
      const analysis = currentAnalysis as {
        macro_type?: string;
        travel_cmd?: string | null;
        [key: string]: unknown;
      };
      const isClassic = analysis.macro_type === "classic";
      const titles = isClassic
        ? [windowStore.fullTitle].filter(Boolean)
        : windowStore.allSyncedTitles;
      const isLeader = (title: string) => title === windowStore.fullTitle;

      for (const title of titles) {
        try {
          // Les suiveurs ne font pas le travel final (ils suivent le meneur en jeu)
          const step =
            !isLeader(title) && analysis.travel_cmd
              ? { ...analysis, travel_cmd: null }
              : currentAnalysis;

          await invoke("execute_step_automation", {
            step,
            windowTitle: title,
          });
        } catch (e) {
          console.error(`Erreur Auto-Pilot (${title}):`, e);
        }
      }

      // Refocus + réactiver le suivi seulement après une automation multi-fenêtres
      if (!isClassic && titles.length > 1 && windowStore.fullTitle) {
        try {
          await invoke("focus_window", {
            windowTitle: windowStore.fullTitle,
          });
          await invoke("press_key", { key: "ctrl+z", count: 1 });
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

  // --- Click Mirror ---
  function updateClickMirror() {
    const followerTitles = windowStore.teamMode
      ? windowStore.teamMembers
          .map((m) => windowStore.teamWindows[m])
          .filter((s) => s?.syncState === "synced" && s.fullTitle)
          .map((s) => s.fullTitle)
      : [];

    invoke("set_click_mirror", {
      active: mirrorClicks && followerTitles.length > 0,
      leaderTitle: windowStore.fullTitle || "",
      followerTitles,
    });
  }

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

  async function updateCombatWatcher() {
    if (combatWatcher) {
      await invoke("start_combat_watcher", {
        combatStartImage: "resources/ui/combat_start.png",
        combatEndImages: [
          "resources/ui/combat_end_1.png",
          "resources/ui/combat_end_2.png",
        ],
      });
    } else {
      await invoke("stop_combat_watcher");
    }
  }

  onDestroy(() => {
    invoke("set_key_listener", { active: false, keys: [] });
    invoke("set_click_mirror", {
      active: false,
      leaderTitle: "",
      followerTitles: [],
    });
    invoke("stop_combat_watcher");
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
    bind:mirrorClicks
    bind:combatWatcher
    onToggleListenKeys={() => {
      listenKeys = !listenKeys;
      updateKeyListener();
    }}
    onToggleMirrorClicks={() => {
      mirrorClicks = !mirrorClicks;
      updateClickMirror();
    }}
    onToggleCombatWatcher={() => {
      combatWatcher = !combatWatcher;
      updateCombatWatcher();
    }}
  />
</div>
