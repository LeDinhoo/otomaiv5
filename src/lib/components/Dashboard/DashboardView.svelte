<script lang="ts">
  import TabBar from "$lib/components/TabBar/TabBar.svelte";
  import Tab from "$lib/components/TabBar/Tab.svelte";
  import GuideLibrary from "$lib/components/Guide/GuideLibrary.svelte";
  import GuideViewer from "$lib/components/Guide/GuideViewer.svelte";
  import EmptyDashboard from "./EmptyDashboard.svelte";
  import { flip } from "svelte/animate";

  import { tabStore } from "$lib/stores/tabStore.svelte";
  import { guideStore } from "$lib/stores/guideStore.svelte";

  // S'assurer que la progression existe quand on change d'onglet
  $effect(() => {
    if (tabStore.activeTab.startsWith("guide_")) {
      guideStore.ensureProgress(tabStore.activeTab);
    }
  });

  // --- LOGIQUE DRAG & DROP ---
  let draggingIndex: number | null = $state(null);
  let hoveringIndex: number | null = $state(null);

  function handleDragStart(e: DragEvent, index: number) {
    draggingIndex = index;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "move";
      e.dataTransfer.dropEffect = "move";
      e.dataTransfer.setData("text/plain", index.toString());
    }
  }

  function handleDragOver(e: DragEvent, index: number) {
    e.preventDefault();
    hoveringIndex = index;
  }

  function handleDrop(e: DragEvent, targetIndex: number) {
    e.preventDefault();
    if (draggingIndex === null || draggingIndex === targetIndex) return;
    tabStore.reorderTabs(draggingIndex, targetIndex);
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
      {#each tabStore.tabs as tab, index (tab.id)}
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
            active={tabStore.activeTab === tab.id}
            onclick={() => tabStore.setActiveTab(tab.id)}
            onclose={() => tabStore.closeTab(tab.id)}
            closable={true}
          />
        </div>
      {/each}
    </TabBar>
  </div>

  <div class="flex-1 min-h-0 flex flex-col relative overflow-hidden">
    {#if tabStore.tabs.length === 0 && tabStore.activeTab !== "general"}
      <EmptyDashboard />
    {:else if tabStore.activeTab === "general"}
      <GuideLibrary onOpen={(id) => tabStore.openGuide(id)} />
    {:else if tabStore.activeTab.startsWith("guide_")}
      {@const guide = tabStore.openGuides[tabStore.activeTab]}

      {#if guide}
        <GuideViewer
          {guide}
          tabId={tabStore.activeTab}
        />
      {/if}
    {/if}
  </div>
</div>
