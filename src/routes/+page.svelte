<script lang="ts">
  import { onMount } from "svelte";
  import TitleBar from "$lib/components/TitleBar/TitleBar.svelte";
  import LibraryPage from "$lib/components/Settings/LibraryPage.svelte";
  import SettingsPage from "$lib/components/Settings/SettingsPage.svelte";
  import DashboardView from "$lib/components/Dashboard/DashboardView.svelte";

  import { saveProfile, loadProfile } from "$lib/services/profileService";

  import { tabStore } from "$lib/stores/tabStore.svelte";
  import { guideStore } from "$lib/stores/guideStore.svelte";
  import { windowStore } from "$lib/stores/windowStore.svelte";

  let syncTimeout: number | undefined;

  onMount(async () => {
    const profile = await loadProfile();

    windowStore.restoreFromProfile(profile.characterName || "Mon Personnage");
    guideStore.restoreFromProfile(
      profile.guideProgress || {},
      profile.checkboxStates || {},
    );
    await tabStore.restoreFromProfile(
      profile.openTabIds || [],
      profile.activeTabId || "",
    );

    windowStore.isLoaded = true;
    await windowStore.performWindowSync();
    windowStore.status = "Prêt";
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

  // Auto-save du profil
  $effect(() => {
    if (!windowStore.isLoaded) return;
    saveProfile({
      characterName: windowStore.windowTitle,
      openTabIds: tabStore.tabs.map((t) => t.id),
      activeTabId: tabStore.activeTab,
      guideProgress: $state.snapshot(guideStore.guideProgress),
      checkboxStates: $state.snapshot(guideStore.checkboxStates),
    });
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
  {:else}
    <DashboardView />
  {/if}
</div>
