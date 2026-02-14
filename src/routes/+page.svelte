<script lang="ts">
  import { onMount } from "svelte";
  import TitleBar from "$lib/components/TitleBar/TitleBar.svelte";
  import LibraryPage from "$lib/components/Settings/LibraryPage.svelte";
  import SettingsPage from "$lib/components/Settings/SettingsPage.svelte";
  import DashboardView from "$lib/components/Dashboard/DashboardView.svelte";
  import TeamPanel from "$lib/components/Team/TeamPanel.svelte";

  import { tabStore } from "$lib/stores/tabStore.svelte";
  import { guideStore } from "$lib/stores/guideStore.svelte";
  import { windowStore } from "$lib/stores/windowStore.svelte";
  import { profileStore } from "$lib/stores/profileStore.svelte";

  let syncTimeout: number | undefined;

  onMount(async () => {
    await profileStore.init();
  });

  // Auto-sync quand usableTitle change
  $effect(() => {
    if (!windowStore.isLoaded) return;
    const currentTitle = windowStore.usableTitle;
    if (syncTimeout) clearTimeout(syncTimeout);
    syncTimeout = setTimeout(
      () => windowStore.performWindowSync(currentTitle),
      800,
    );
  });

  // Auto-save du profil actif
  $effect(() => {
    if (!windowStore.isLoaded) return;
    profileStore.saveCurrentProfile();
  });
</script>

<div
  class="flex flex-col h-screen w-full rounded-lg overflow-hidden bg-stone-800 border border-stone-700 text-stone-200"
>
  <TitleBar />
  {#if windowStore.currentView === "library"}
    <div class="flex-1 overflow-hidden">
      <LibraryPage
        onSelectGuide={(id) => {
          tabStore.openGuide(id);
          windowStore.setView("dashboard");
        }}
      />
    </div>
  {:else if windowStore.currentView === "settings"}
    <div class="flex-1 overflow-hidden bg-stone-900">
      <SettingsPage />
    </div>
  {:else if windowStore.currentView === "team"}
    <div class="flex-1 overflow-hidden bg-stone-900">
      <TeamPanel />
    </div>
  {:else}
    <DashboardView />
  {/if}
</div>
