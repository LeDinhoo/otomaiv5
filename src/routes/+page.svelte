<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import TitleBar from "$lib/components/TitleBar/TitleBar.svelte";

  // Imports des 3 vues
  import LibraryPage from "$lib/components/Settings/LibraryPage.svelte";
  import SettingsPage from "$lib/components/Settings/SettingsPage.svelte"; // <--- Assure-toi du chemin
  import DashboardView from "$lib/components/Dashboard/DashboardView.svelte";

  import { loadOrDownloadGuide } from "$lib/services/guideService";
  import { saveProfile, loadProfile } from "$lib/services/profileService";

  let status = $state("Initialisation...");
  let windowTitle = $state("Mon Personnage");
  let usableTitle = $state("Mon Personnage");

  // GESTION DE LA VUE ACTIVE : "dashboard" | "library" | "settings"
  let currentView = $state("dashboard");

  let isLoaded = $state(false);

  let tabs = $state([]);
  let activeTab = $state("");
  let openGuides = $state({});
  let guideProgress = $state({});
  let checkboxStates = $state({});

  let syncTimeout: number | undefined;

  async function performWindowSync(nameToFind: string) {
    if (!nameToFind) return;
    try {
      const fullTitle = await invoke("sync_window_title", {
        characterName: nameToFind,
      });
      windowTitle = fullTitle as string;
      const cleanPseudo = (fullTitle as string).split(" - ")[0];
      if (usableTitle !== cleanPseudo) usableTitle = cleanPseudo;
      status = "Synchronisé ✅";
    } catch (e) {
      console.error("Erreur synchro:", e);
      status = "Fenêtre introuvable ❌";
    }
  }

  async function handleOpenGuide(id: string) {
    status = "Chargement...";
    try {
      const guideData = await loadOrDownloadGuide(id);
      const tabId = `guide_${id}`;
      openGuides[tabId] = guideData;

      if (guideProgress[tabId] === undefined) guideProgress[tabId] = 0;
      if (!checkboxStates[tabId]) checkboxStates[tabId] = {};

      if (!tabs.find((t) => t.id === tabId)) {
        tabs.push({ id: tabId, label: guideData.name });
      }
      activeTab = tabId;
      status = `Guide chargé : ${guideData.name}`;
      return tabId;
    } catch (e) {
      status = "Erreur : " + e;
    }
  }

  function closeTab(id: string) {
    tabs = tabs.filter((t) => t.id !== id);
    if (activeTab === id) activeTab = tabs.length > 0 ? tabs[0].id : "general";
  }

  onMount(async () => {
    const profile = await loadProfile();
    let savedName = profile.characterName || "Mon Personnage";

    windowTitle = savedName;
    usableTitle = savedName.split(" - ")[0];
    guideProgress = profile.guideProgress || {};
    checkboxStates = profile.checkboxStates || {};

    if (profile.openTabIds && profile.openTabIds.length > 0) {
      const loadedTabs = [];
      for (const tabId of profile.openTabIds) {
        try {
          const guideId = tabId.replace("guide_", "");
          const guideData = await loadOrDownloadGuide(guideId);
          openGuides[tabId] = guideData;
          loadedTabs.push({ id: tabId, label: guideData.name });
        } catch (e) {
          console.error(e);
        }
      }
      tabs = loadedTabs;
    }

    if (profile.activeTabId && tabs.find((t) => t.id === profile.activeTabId)) {
      activeTab = profile.activeTabId;
    }

    isLoaded = true;
    await performWindowSync(usableTitle);
    status = "Prêt";
  });

  $effect(() => {
    if (!isLoaded) return;
    const currentTitle = usableTitle;
    if (syncTimeout) clearTimeout(syncTimeout);
    syncTimeout = setTimeout(() => performWindowSync(currentTitle), 800);
  });

  $effect(() => {
    if (!isLoaded) return;
    saveProfile({
      characterName: windowTitle,
      openTabIds: tabs.map((t) => t.id),
      activeTabId: activeTab,
      guideProgress: $state.snapshot(guideProgress),
      checkboxStates: $state.snapshot(checkboxStates),
    });
  });

  async function handleNavigate(
    targetGuideId: string,
    targetStepIndex: number,
  ) {
    const tabId = `guide_${targetGuideId}`;
    if (!tabs.find((t) => t.id === tabId)) await handleOpenGuide(targetGuideId);
    activeTab = tabId;
    setTimeout(() => {
      guideProgress[tabId] = targetStepIndex;
    }, 50);
  }
</script>

<div
  class="flex flex-col h-screen w-full bg-stone-900 border border-stone-700 rounded-md overflow-hidden text-stone-200"
>
    <TitleBar
      bind:windowTitle
      bind:usableTitle
      statusMessage={status}
      onToggleLibrary={() =>
        (currentView = currentView === "library" ? "dashboard" : "library")}
      onToggleSettings={() =>
        (currentView = currentView === "settings" ? "dashboard" : "settings")}
    />
  {#if currentView === "library"}
    <div class="flex-1 overflow-hidden">
      <LibraryPage
        onSelectGuide={(id) => {
          handleOpenGuide(id);
          currentView = "dashboard";
        }}
      />
    </div>
  {:else if currentView === "settings"}
    <div class="flex-1 overflow-hidden bg-stone-900">
      <SettingsPage />
    </div>
  {:else}
    <DashboardView
      {tabs}
      {openGuides}
      bind:activeTab
      bind:guideProgress
      bind:checkboxStates
      bind:usableTitle
      onCloseTab={closeTab}
      onOpenGuide={handleOpenGuide}
      onNavigate={handleNavigate}
      onPrevStep={(id) => guideProgress[id] > 0 && guideProgress[id]--}
      onNextStep={(id, total) =>
        guideProgress[id] < total - 1 && guideProgress[id]++}
    />
  {/if}
</div>
