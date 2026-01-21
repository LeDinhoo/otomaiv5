<script lang="ts">
  import TabBar from "$lib/components/TabBar/TabBar.svelte";
  import Tab from "$lib/components/TabBar/Tab.svelte";
  import GuideLibrary from "$lib/components/Guide/GuideLibrary.svelte";
  import GuideViewer from "$lib/components/Guide/GuideViewer.svelte";
  import EmptyDashboard from "./EmptyDashboard.svelte";

  let {
    tabs,
    activeTab = $bindable(),
    openGuides,
    guideProgress = $bindable(),
    checkboxStates = $bindable(),
    usableTitle = $bindable(),
    onCloseTab,
    onOpenGuide,
    onNavigate,
    onPrevStep,
    onNextStep,
  } = $props();
</script>

<div class="flex flex-col flex-1 min-h-0 overflow-hidden">
  <div class="flex-none">
    <TabBar>
      {#each tabs as tab (tab.id)}
        <Tab
          label={tab.label}
          active={activeTab === tab.id}
          onclick={() => (activeTab = tab.id)}
          onclose={tab.id !== "general" ? () => onCloseTab(tab.id) : undefined}
        />
      {/each}
    </TabBar>
  </div>

  <div class="flex-1 min-h-0 flex flex-col relative overflow-hidden">
    {#if tabs.length === 0}
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
          bind:usableTitle
        />
      {/if}
    {/if}
  </div>
</div>
