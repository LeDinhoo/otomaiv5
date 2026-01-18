<script lang="ts">
  import { onMount } from "svelte";
  import TitleBar from "$lib/components/TitleBar/TitleBar.svelte";
  import TabBar from "$lib/components/TabBar/TabBar.svelte";
  import Tab from "$lib/components/TabBar/Tab.svelte";
  import SettingsPage from "$lib/components/Settings/SettingsPage.svelte";
  import GuideLibrary from "$lib/components/Guide/GuideLibrary.svelte";
  import GuideViewer from "$lib/components/Guide/GuideViewer.svelte";

  import { loadOrDownloadGuide } from "$lib/services/guideService";
  import {
    saveProfile,
    loadProfile,
    type AppProfile,
  } from "$lib/services/profileService";

  // --- États ---
  let status = $state("Initialisation...");
  let windowTitle = $state("Mon Personnage");
  let usableTitle = $state("Mon Personnage");

  let currentView = $state("dashboard"); // "dashboard" | "settings"

  // Fonction pour basculer la vue
  function toggleSettingsView() {
    if (currentView === "dashboard") {
      currentView = "settings";
    } else {
      currentView = "dashboard";
    }
  }

  // Gestion des onglets
  let tabs = $state([]);
  let activeTab = $state("");

  // Données des guides
  let openGuides = $state({});
  let guideProgress = $state({}); // { "guide_552": 2 } (index étape)
  let checkboxStates = $state({}); // { "guide_552": { 0: [true, false] } }

  // Flag pour éviter de sauvegarder pendant le chargement initial
  let isLoaded = $state(false);

  // --- 1. CHARGEMENT AU DÉMARRAGE ---
  onMount(async () => {
    console.log("🚀 Démarrage du chargement du profil...");
    const profile = await loadProfile();

    // Restauration des données simples
    windowTitle = profile.characterName;
    usableTitle = profile.characterName;
    guideProgress = profile.guideProgress || {};
    checkboxStates = profile.checkboxStates || {};

    // Restauration des onglets
    if (profile.openTabIds.length > 0) {
      status = "Restauration de la session...";

      // CORRECTION ICI : On commence toujours avec l'onglet Général
      const loadedTabs = [];

      for (const tabId of profile.openTabIds) {
        try {
          // Extrait l'ID (ex: "guide_552" -> "552")
          const guideId = tabId.replace("guide_", "");
          const guideData = await loadOrDownloadGuide(guideId);

          openGuides[tabId] = guideData;
          loadedTabs.push({ id: tabId, label: guideData.name });
        } catch (e) {
          console.error(`Impossible de restaurer ${tabId}`, e);
        }
      }
      tabs = loadedTabs;
    }

    // Restauration de l'onglet actif
    if (profile.activeTabId && tabs.find((t) => t.id === profile.activeTabId)) {
      activeTab = profile.activeTabId;
    }

    status = "Prêt";
    isLoaded = true;
  });

  // --- 2. SAUVEGARDE AUTOMATIQUE ---
  $effect(() => {
    if (!isLoaded) return;

    const profileToSave: AppProfile = {
      characterName: windowTitle,
      openTabIds: tabs.map((t) => t.id),
      activeTabId: activeTab,
      guideProgress: $state.snapshot(guideProgress),
      checkboxStates: $state.snapshot(checkboxStates),
    };

    saveProfile(profileToSave);
  });

  // --- Actions ---

  async function handleOpenGuide(id: string) {
    status = "Chargement...";
    try {
      const guideData = await loadOrDownloadGuide(id);
      const tabId = `guide_${id}`;

      openGuides[tabId] = guideData;

      // Initialisation progression
      if (guideProgress[tabId] === undefined) {
        guideProgress[tabId] = 0;
      }

      // Initialisation checkboxes
      if (!checkboxStates[tabId]) {
        checkboxStates[tabId] = {};
      }

      // Ajout onglet
      if (!tabs.find((t) => t.id === tabId)) {
        tabs.push({ id: tabId, label: guideData.name });
      }

      activeTab = tabId;
      status = `Guide chargé : ${guideData.name}`;
    } catch (e) {
      status = "Erreur : " + e;
    }
  }

  function closeTab(id: string) {
    tabs = tabs.filter((t) => t.id !== id);
    if (activeTab === id) {
      activeTab = tabs.length > 0 ? tabs[0].id : "general";
    }
  }

  // Navigation dans le guide (Passées au composant Viewer)
  function prevStep(tabId: string) {
    if (guideProgress[tabId] > 0) guideProgress[tabId]--;
  }

  function nextStep(tabId: string, totalSteps: number) {
    if (guideProgress[tabId] < totalSteps - 1) guideProgress[tabId]++;
  }

  // --- NOUVELLE FONCTION : Navigation inter-guides ---
  async function handleNavigate(
    targetGuideId: string,
    targetStepIndex: number,
  ) {
    const tabId = `guide_${targetGuideId}`;

    // 1. Ouvrir le guide (si pas déjà ouvert)
    // On réutilise ta fonction handleOpenGuide mais sans toucher à activeTab tout de suite
    if (!tabs.find((t) => t.id === tabId)) {
      await handleOpenGuide(targetGuideId);
    }

    // 2. Basculer sur l'onglet
    activeTab = tabId;

    // 3. Forcer l'étape cible
    // On attend un micro-tick pour être sûr que l'onglet est monté
    setTimeout(() => {
      guideProgress[tabId] = targetStepIndex;
    }, 50);
  }
</script>

<div
  class="flex flex-col h-screen w-full bg-stone-900 border border-stone-700 rounded-md overflow-hidden text-stone-200"
>
  <div class="flex-none">
    <TitleBar
      bind:windowTitle
      bind:usableTitle
      onToggleSettings={toggleSettingsView}
    />
  </div>

  {#if currentView === "settings"}
    <div class="flex-1 overflow-hidden">
      <SettingsPage
        onSelectGuide={(id) => {
          handleOpenGuide(id);
          currentView = "dashboard";
        }}
      />
    </div>
  {:else}
    <div class="flex flex-col flex-1 min-h-0 overflow-hidden">
      <div class="flex-none">
        <TabBar>
          {#each tabs as tab}
            <Tab
              label={tab.label}
              active={activeTab === tab.id}
              onclick={() => (activeTab = tab.id)}
              onclose={tab.id !== "general"
                ? () => closeTab(tab.id)
                : undefined}
            />
          {/each}
        </TabBar>
      </div>

      <div class="flex-1 min-h-0 flex flex-col relative overflow-hidden">
        {#if tabs.length === 0}
          <div
            class="flex items-center justify-center h-full text-stone-500 italic"
          >
            Aucun onglet ouvert.
          </div>
        {:else if activeTab === "general"}
          <GuideLibrary onOpen={handleOpenGuide} />
        {:else if activeTab.startsWith("guide_")}
          {@const guide = openGuides[activeTab]}

          {#if !checkboxStates[activeTab]}
            {(checkboxStates[activeTab] = {})}
          {/if}

          <GuideViewer
            {guide}
            bind:stepIndex={guideProgress[activeTab]}
            bind:checkboxState={checkboxStates[activeTab]}
            onPrev={() => prevStep(activeTab)}
            onNext={() => nextStep(activeTab, guide.steps.length)}
            onNavigate={handleNavigate}
          />
        {/if}
      </div>
    </div>
  {/if}
</div>
