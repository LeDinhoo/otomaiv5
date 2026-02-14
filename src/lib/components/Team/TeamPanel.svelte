<script lang="ts">
  import {
    Plus,
    Trash2,
    RefreshCw,
    Crown,
    Loader2,
    WifiOff,
    Users,
  } from "@lucide/svelte";
  import { windowStore } from "$lib/stores/windowStore.svelte";

  let newMemberName = $state("");

  function handleAddMember() {
    if (!newMemberName.trim()) return;
    windowStore.addTeamMember(newMemberName.trim());
    newMemberName = "";
  }
</script>

<div
  class="p-6 space-y-6 text-stone-200 h-full overflow-y-auto custom-scrollbar"
>
  <!-- Header -->
  <div class="flex items-center justify-between">
    <h2 class="text-xl font-bold text-amber-400 flex items-center gap-2">
      <Users class="w-5 h-5" /> Gestion Team
    </h2>
    <button
      onclick={() => windowStore.toggleTeamMode()}
      class="text-xs px-3 py-1 rounded cursor-pointer transition-colors
      {windowStore.teamMode
        ? 'bg-amber-500/20 text-amber-400 hover:bg-amber-500/30'
        : 'bg-stone-700 text-stone-400 hover:bg-stone-600'}"
    >
      {windowStore.teamMode ? "Actif" : "Inactif"}
    </button>
  </div>

  {#if !windowStore.teamMode}
    <div class="text-stone-500 text-sm text-center py-8">
      Active le mode Team pour gérer plusieurs fenêtres avec un seul guide.
    </div>
  {:else}
    <!-- Meneur -->
    <section class="space-y-2">
      <h3 class="text-stone-400 font-semibold uppercase text-xs">Meneur</h3>
      <div
        class="flex items-center gap-3 px-3 py-2.5 rounded-lg bg-amber-500/10 border border-amber-500/30"
      >
        <Crown class="w-4 h-4 text-amber-400 shrink-0" />
        <span class="text-sm font-semibold text-amber-300 flex-1">
          {windowStore.usableTitle}
        </span>
        <!-- Sync status -->
        {#if windowStore.syncState === "synced"}
          <span class="w-2 h-2 rounded-full bg-green-500"></span>
        {:else if windowStore.syncState === "recovering"}
          <Loader2 class="w-3.5 h-3.5 text-orange-500 animate-spin" />
        {:else if windowStore.syncState === "lost"}
          <WifiOff class="w-3.5 h-3.5 text-red-500" />
        {:else}
          <span class="w-2 h-2 rounded-full bg-stone-500"></span>
        {/if}
      </div>
    </section>

    <!-- Membres -->
    <section class="space-y-2">
      <div class="flex items-center justify-between">
        <h3 class="text-stone-400 font-semibold uppercase text-xs">Membres</h3>
        {#if windowStore.teamMembers.length > 0}
          <button
            onclick={() => windowStore.syncAllTeam()}
            class="flex items-center gap-1 text-xs text-stone-500 hover:text-stone-300 cursor-pointer"
          >
            <RefreshCw class="w-3 h-3" /> Tout sync
          </button>
        {/if}
      </div>

      {#if windowStore.teamMembers.length === 0}
        <div class="text-stone-500 text-xs text-center py-4">
          Aucun membre. Ajoute des personnages ci-dessous.
        </div>
      {:else}
        <div class="space-y-1">
          {#each windowStore.teamMembers as member}
            {@const state = windowStore.teamWindows[member]}
            <div
              class="flex items-center gap-3 px-3 py-2 rounded-lg bg-stone-700/30 border border-stone-700 hover:border-stone-600 transition-colors group"
            >
              <!-- Sync indicator -->
              {#if state?.syncState === "synced"}
                <span class="w-2 h-2 rounded-full bg-green-500 shrink-0"
                ></span>
              {:else if state?.syncState === "recovering"}
                <Loader2
                  class="w-3.5 h-3.5 text-orange-500 animate-spin shrink-0"
                />
              {:else}
                <span class="w-2 h-2 rounded-full bg-red-500 shrink-0"></span>
              {/if}

              <!-- Name -->
              <span class="text-sm text-stone-300 flex-1">{member}</span>

              <!-- Actions -->
              <div
                class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
              >
                <button
                  onclick={() => windowStore.setAsLeader(member)}
                  class="p-1 hover:bg-stone-600 rounded cursor-pointer"
                  title="Promouvoir meneur"
                >
                  <Crown class="w-3.5 h-3.5 text-amber-400/60 hover:text-amber-400" />
                </button>
                <button
                  onclick={() => windowStore.syncTeamMember(member)}
                  class="p-1 hover:bg-stone-600 rounded cursor-pointer"
                  title="Re-sync"
                >
                  <RefreshCw class="w-3.5 h-3.5 text-stone-400" />
                </button>
                <button
                  onclick={() => windowStore.removeTeamMember(member)}
                  class="p-1 hover:bg-stone-600 rounded cursor-pointer"
                  title="Retirer"
                >
                  <Trash2 class="w-3.5 h-3.5 text-stone-400 hover:text-red-400" />
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}

      <!-- Ajouter -->
      <div class="flex items-center gap-2 pt-2">
        <input
          type="text"
          bind:value={newMemberName}
          placeholder="Nom du personnage..."
          class="flex-1 bg-stone-900 border border-stone-700 rounded px-3 py-1.5 text-sm text-stone-200
                 focus:outline-none focus:border-amber-500/50 placeholder:text-stone-500"
          spellcheck="false"
          onkeydown={(e) => {
            if (e.key === "Enter") handleAddMember();
          }}
        />
        <button
          onclick={handleAddMember}
          disabled={!newMemberName.trim()}
          class="p-1.5 rounded bg-amber-500/20 hover:bg-amber-500/30 cursor-pointer transition-colors
                 disabled:opacity-30 disabled:cursor-default"
          title="Ajouter"
        >
          <Plus class="w-4 h-4 text-amber-400" />
        </button>
      </div>
    </section>
  {/if}
</div>
