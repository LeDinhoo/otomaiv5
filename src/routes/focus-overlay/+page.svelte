<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, emit } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  interface TeamMember {
    name: string;
    breedId: number | null;
    fullTitle: string;
  }

  let focusedName = $state("");
  let members = $state<TeamMember[]>([]);

  function focusMember(member: TeamMember) {
    if (!member.fullTitle) return;
    invoke("focus_window", { windowTitle: member.fullTitle });
  }

  onMount(async () => {
    const win = getCurrentWindow();

    const el = document.getElementById("drag-area");
    el?.addEventListener("mousedown", (e) => {
      // Ne pas drag si on clique sur un bouton
      if ((e.target as HTMLElement).closest("button")) return;
      win.startDragging();
    });

    const unlistenFocus = await listen<string>("focus-changed", (event) => {
      const title = event.payload;
      focusedName = title ? title.split(" - ")[0] : "";
    });

    const unlistenBreeds = await listen<TeamMember[]>("breed-mapping", (event) => {
      members = event.payload;
    });

    emit("focus-overlay-ready");

    return () => {
      unlistenFocus();
      unlistenBreeds();
    };
  });
</script>

<div
  id="drag-area"
  class="h-screen w-screen overflow-hidden flex items-center justify-center"
>
  <div
    class="w-full h-full bg-[#333328] backdrop-blur-sm text-white rounded-lg shadow-2xl flex items-center px-2 gap-1.5"
  >
    <!-- Icônes de tous les membres -->
    {#each members as member}
      {@const isActive = focusedName === member.name}
      <button
        onclick={() => focusMember(member)}
        class="shrink-0 rounded p-0.5 transition-all cursor-pointer hover:opacity-100 {isActive ? 'ring-1 ring-amber-400 bg-amber-500/20' : 'opacity-40 hover:bg-stone-700/50'}"
        title={member.name}
      >
        {#if member.breedId}
          <img
            src="/breeds/symbol_{member.breedId}.png"
            alt={member.name}
            class="w-5 h-5"
          />
        {:else}
          <div class="w-5 h-5 rounded bg-stone-600 flex items-center justify-center">
            <span class="text-[8px] text-stone-400">{member.name.charAt(0).toUpperCase()}</span>
          </div>
        {/if}
      </button>
    {/each}

    <!-- Nom de la fenêtre active -->
    {#if focusedName}
      <span class="text-sm font-medium truncate ml-1 text-amber-300">{focusedName}</span>
    {:else}
      <span class="text-[10px] text-stone-500 ml-1">—</span>
    {/if}
  </div>
</div>
