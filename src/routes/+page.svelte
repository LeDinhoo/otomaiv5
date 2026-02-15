<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, emit } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
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
    await windowStore.restoreMainWindow();

    const unlistenCombatStart = await listen("combat-detected", () => {
      invoke("pause_click_mirror", { paused: true });
      if (!windowStore.isMini) {
        windowStore.toggleMini();
      }
    });

    const unlistenCombatEnd = await listen("combat-ended", () => {
      invoke("pause_click_mirror", { paused: false });
      if (windowStore.isMini) {
        windowStore.toggleMini();
      }
    });

    // Quand l'overlay focus est prêt, renvoyer le breed mapping
    const unlistenOverlayReady = await listen("focus-overlay-ready", () => {
      emitBreedMapping();
    });

    // Sauvegarder le layout et fermer l'overlay quand la fenêtre principale se ferme
    const unlistenClose = await getCurrentWindow().onCloseRequested(async () => {
      await windowStore.saveLayout();
      await windowStore.closeFocusOverlay();
    });

    return () => {
      unlistenCombatStart();
      unlistenCombatEnd();
      unlistenOverlayReady();
      unlistenClose();
    };
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

  // Sync focus cycler titles
  $effect(() => {
    const titles = windowStore.allSyncedTitles;
    invoke("set_click_mirror", { titles });
  });

  // Sync focus keybinds
  $effect(() => {
    const keybinds = windowStore.focusKeybinds;
    invoke("set_focus_keybinds", { keybinds });
  });

  // Construit et émet le breed mapping vers l'overlay
  function emitBreedMapping() {
    if (!windowStore.teamMode) return;
    const members: { name: string; breedId: number | null; fullTitle: string }[] = [];
    // Leader en premier
    const leaderName = windowStore.usableTitle;
    if (leaderName) {
      members.push({ name: leaderName, breedId: windowStore.breedIcon, fullTitle: windowStore.fullTitle });
    }
    // Puis les membres
    for (const name of windowStore.teamMembers) {
      const memberState = windowStore.teamWindows[name];
      members.push({
        name,
        breedId: windowStore.teamMemberBreeds[name] ?? null,
        fullTitle: memberState?.fullTitle ?? "",
      });
    }
    emit("breed-mapping", members);
  }

  // Sync breed mapping vers l'overlay focus
  $effect(() => {
    if (!windowStore.teamMode) return;
    // Accéder aux dépendances réactives pour que l'effect se re-déclenche
    windowStore.usableTitle;
    windowStore.breedIcon;
    windowStore.fullTitle;
    windowStore.teamMembers;
    windowStore.teamMemberBreeds;
    windowStore.teamWindows;
    emitBreedMapping();
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
  {#if windowStore.isMini}
    <!-- Mode mini : titlebar uniquement -->
  {:else if windowStore.currentView === "library"}
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
