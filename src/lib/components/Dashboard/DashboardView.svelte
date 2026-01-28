<script lang="ts">
  import TabBar from "$lib/components/TabBar/TabBar.svelte";
  import Tab from "$lib/components/TabBar/Tab.svelte";
  import GuideLibrary from "$lib/components/Guide/GuideLibrary.svelte";
  import GuideViewer from "$lib/components/Guide/GuideViewer.svelte";
  import EmptyDashboard from "./EmptyDashboard.svelte";
  
  // Importer l'animation pour que les onglets glissent
  import { flip } from "svelte/animate";

  let {
    tabs = $bindable([]),
    activeTab = $bindable(),
    openGuides,
    guideProgress = $bindable(),
    checkboxStates = $bindable(),
    usableTitle = $bindable(),
    fullTitle = $bindable(),
    onCloseTab,
    onOpenGuide,
    onNavigate,
    onPrevStep,
    onNextStep,
  } = $props();

  // --- LOGIQUE DRAG & DROP ---
  let draggingIndex: number | null = $state(null);
  let hoveringIndex: number | null = $state(null);

  function handleDragStart(e: DragEvent, index: number) {
    draggingIndex = index;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "move";
      e.dataTransfer.dropEffect = "move";
      // Nécessaire pour Firefox
      e.dataTransfer.setData("text/plain", index.toString());
    }
  }

  function handleDragOver(e: DragEvent, index: number) {
    e.preventDefault(); // Autorise le drop
    hoveringIndex = index;
  }

  function handleDrop(e: DragEvent, targetIndex: number) {
    e.preventDefault();
    if (draggingIndex === null || draggingIndex === targetIndex) return;

    // Réorganiser le tableau
    const itemToMove = tabs[draggingIndex];
    tabs.splice(draggingIndex, 1);
    tabs.splice(targetIndex, 0, itemToMove);

    // Reset
    draggingIndex = null;
    hoveringIndex = null;
  }

  function handleDragEnd() {
    draggingIndex = null;
    hoveringIndex = null;
  }
</script>

<div class="flex flex-col flex-1 min-h-0 overflow-hidden">
  <div class="flex-none">
    <TabBar>
      {#each tabs as tab, index (tab.id)}
        <div
          role="listitem"
          draggable="true"
          ondragstart={(e) => handleDragStart(e, index)}
          ondragover={(e) => handleDragOver(e, index)}
          ondrop={(e) => handleDrop(e, index)}
          ondragend={handleDragEnd}
          animate:flip={{ duration: 300 }}
          class="h-full flex w-full min-w-0 transition-opacity duration-200"
          class:opacity-40={draggingIndex === index}
        >
          <Tab
            label={tab.label}
            active={activeTab === tab.id}
            onclick={() => (activeTab = tab.id)}
            onclose={() => onCloseTab(tab.id)}
            closable={true}
          />
        </div>
      {/each}
    </TabBar>
  </div>

  <div class="flex-1 min-h-0 flex flex-col relative overflow-hidden">
    {#if tabs.length === 0 && activeTab !== "general"}
      <EmptyDashboard />
    {:else if activeTab === "general"}
      <GuideLibrary onOpen={onOpenGuide} />
    {:else if activeTab.startsWith("guide_")}
      {@const guide = openGuides[activeTab]}

      {#if guide}
        {#if !checkboxStates[activeTab]}
          {(checkboxStates[activeTab] = {})}
        {/if}

        <GuideViewer
          {guide}
          bind:stepIndex={guideProgress[activeTab]}
          bind:checkboxState={checkboxStates[activeTab]}
          onPrev={() => onPrevStep(activeTab)}
          onNext={() => onNextStep(activeTab, guide.steps.length)}
          {onNavigate}
          bind:fullTitle
        />
      {/if}
    {/if}
  </div>
</div>