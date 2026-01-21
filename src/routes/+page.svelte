<script lang="ts">
  import { onMount } from "svelte";
  import TitleBar from "$lib/components/TitleBar/TitleBar.svelte";
  import TabBar from "$lib/components/TabBar/TabBar.svelte";
  import Tab from "$lib/components/TabBar/Tab.svelte";
  import SettingsPage from "$lib/components/Settings/SettingsPage.svelte";
  import GuideLibrary from "$lib/components/Guide/GuideLibrary.svelte";
  import GuideViewer from "$lib/components/Guide/GuideViewer.svelte";
  import { invoke } from "@tauri-apps/api/core";

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

  let currentView = $state("dashboard");

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
  let guideProgress = $state({});
  let checkboxStates = $state({});

  let isLoaded = $state(false);
  let syncTimeout: number | undefined; // Pour le debounce

  // --- NOUVELLE FONCTION : SYNCHRONISATION CENTRALISÉE ---
  async function performWindowSync(nameToFind: string) {
    if (!nameToFind) return;

    // status = `Recherche de "${nameToFind}"...`;
    // console.log(`🔄 Tentative de synchronisation pour : [${nameToFind}]`);

    try {
      // 1. On récupère le titre COMPLET depuis Rust (ex: "Miguel - Dofus - Release")
      const fullTitle = await invoke("sync_window_title", {
        characterName: nameToFind,
      });

      console.log("✅ Rust a trouvé :", fullTitle);

      // 2. On met à jour la variable technique (celle utilisée pour les screenshots, etc.)
      windowTitle = fullTitle;

      // 3. NETTOYAGE VISUEL : On ne garde que le pseudo pour l'affichage
      const cleanPseudo = fullTitle.split(" - ")[0];

      // On ne change l'affichage que si nécessaire (évite les sauts de curseur)
      if (usableTitle !== cleanPseudo) {
        usableTitle = cleanPseudo;
      }

      status = "Synchronisé ✅";
    } catch (e) {
      console.error("Erreur synchro fenêtre:", e);
      status = "Fenêtre introuvable ❌";
    }
  }

  // --- 1. CHARGEMENT AU DÉMARRAGE ---
  onMount(async () => {
    console.log("🚀 Démarrage du chargement du profil...");
    const profile = await loadProfile();

    // Restauration des données simples avec NETTOYAGE
    let savedName = profile.characterName || "Mon Personnage";

    // On stocke le nom complet technique
    windowTitle = savedName;
    // On nettoie direct pour l'affichage (au cas où un nom long a été sauvegardé)
    usableTitle = savedName.split(" - ")[0];

    guideProgress = profile.guideProgress || {};
    checkboxStates = profile.checkboxStates || {};

    // Restauration des onglets
    if (profile.openTabIds && profile.openTabIds.length > 0) {
      status = "Restauration de la session...";
      const loadedTabs = [];

      for (const tabId of profile.openTabIds) {
        try {
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

    // On marque comme chargé AVANT de lancer la première synchro
    isLoaded = true;

    // Première tentative immédiate avec le titre nettoyé
    await performWindowSync(usableTitle);

    status = "Prêt";
  });

  // --- 2. SURVEILLANCE DES CHANGEMENTS DE TITRE ($effect) ---
  // Dès que 'usableTitle' change (via l'input utilisateur), on relance la synchro
  $effect(() => {
    // On ne fait rien tant que le profil n'est pas chargé
    if (!isLoaded) return;

    // Dépendance explicite pour Svelte 5
    const currentTitle = usableTitle;

    // Debounce : on attend 800ms que l'utilisateur finisse de taper
    if (syncTimeout) clearTimeout(syncTimeout);

    syncTimeout = setTimeout(() => {
      performWindowSync(currentTitle);
    }, 800);
  });

  // --- 3. SAUVEGARDE AUTOMATIQUE ---
  $effect(() => {
    if (!isLoaded) return;

    const profileToSave: AppProfile = {
      characterName: windowTitle, // On sauvegarde le titre COMPLET (technique) pour la prochaine fois
      openTabIds: tabs.map((t) => t.id),
      activeTabId: activeTab,
      guideProgress: $state.snapshot(guideProgress),
      checkboxStates: $state.snapshot(checkboxStates),
    };

    saveProfile(profileToSave);
  });

  // --- Actions (Reste inchangé) ---
  async function handleOpenGuide(id: string) {
    status = "Chargement...";
    try {
      const guideData = await loadOrDownloadGuide(id);
      const tabId = `guide_${id}`;

      openGuides[tabId] = guideData;

      if (guideProgress[tabId] === undefined) guideProgress[tabId] = 0;
      if (!checkboxStates[tabId]) checkboxStates[tabId] = {};
      if (!tabs.find((t) => t.id === tabId))
        tabs.push({ id: tabId, label: guideData.name });

      activeTab = tabId;
      status = `Guide chargé : ${guideData.name}`;
    } catch (e) {
      status = "Erreur : " + e;
    }
  }

  function closeTab(id: string) {
    tabs = tabs.filter((t) => t.id !== id);
    if (activeTab === id) activeTab = tabs.length > 0 ? tabs[0].id : "general";
  }

  function prevStep(tabId: string) {
    if (guideProgress[tabId] > 0) guideProgress[tabId]--;
  }

  function nextStep(tabId: string, totalSteps: number) {
    if (guideProgress[tabId] < totalSteps - 1) guideProgress[tabId]++;
  }

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
  <div class="flex-none">
    <TitleBar
      bind:windowTitle
      bind:usableTitle
      onToggleSettings={toggleSettingsView}
      statusMessage={status}
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
            bind:usableTitle
          />
        {/if}
      </div>
    </div>
    <!-- <div
      class="fixed bottom-0 right-0 p-1 text-xs text-stone-500 opacity-50 pointer-events-none"
    >
      Target: {usableTitle} | {status}
    </div> -->
  {/if}
</div>
