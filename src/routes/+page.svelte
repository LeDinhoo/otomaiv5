<script lang="ts">
  import TitleBar from "$lib/components/TitleBar/TitleBar.svelte";
  import TabBar from "$lib/components/TabBar/TabBar.svelte";
  import Tab from "$lib/components/TabBar/Tab.svelte";
  import SettingsPage from "$lib/components/Settings/SettingsPage.svelte";

  // Nos nouveaux composants
  import GuideLibrary from "$lib/components/Guide/GuideLibrary.svelte";
  import GuideViewer from "$lib/components/Guide/GuideViewer.svelte";
  import { loadOrDownloadGuide } from "$lib/services/guideService";

  // --- États ---
  let status = $state("Prêt");
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
  let activeTab = $state("general");

  // Données des guides
  let openGuides = $state({});
  let guideProgress = $state({}); // { "guide_552": 2 } (index étape)

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
        handleOpenGuide(id);     // 1. Charge le guide
        currentView = "dashboard"; // 2. Revient à l'affichage principal
      }}/>
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

          <GuideViewer
            {guide}
            stepIndex={guideProgress[activeTab]}
            onPrev={() => prevStep(activeTab)}
            onNext={() => nextStep(activeTab, guide.steps.length)}
          />
        {/if}
      </div>
    </div>
  {/if}

  <!-- <div
    class="flex-none p-2 px-4 bg-stone-950 border-t border-stone-800 flex justify-between items-center"
  >
    <p class="text-[10px] italic text-stone-500">{status}</p>
  </div> -->
</div>
