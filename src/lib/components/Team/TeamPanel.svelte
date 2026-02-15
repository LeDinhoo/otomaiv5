<script lang="ts">
  import {
    Plus,
    Trash2,
    RefreshCw,
    Crown,
    Loader2,
    WifiOff,
    Users,
    Keyboard,
  } from "@lucide/svelte";
  import { windowStore } from "$lib/stores/windowStore.svelte";

  let newMemberName = $state("");
  let breedPickerFor = $state<string | null>(null);
  let keybindListeningFor = $state<string | null>(null);

  const BREED_COUNT = 18;

  function handleAddMember() {
    if (!newMemberName.trim()) return;
    windowStore.addTeamMember(newMemberName.trim());
    newMemberName = "";
  }

  function openBreedPicker(target: string) {
    breedPickerFor = breedPickerFor === target ? null : target;
    keybindListeningFor = null;
  }

  function selectBreed(target: string, breedId: number | null) {
    if (target === "__leader__") {
      windowStore.setBreedIcon(breedId);
    } else {
      windowStore.setMemberBreedIcon(target, breedId);
    }
    breedPickerFor = null;
  }

  function startKeybindCapture(target: string) {
    keybindListeningFor = keybindListeningFor === target ? null : target;
    breedPickerFor = null;
  }

  function handleKeybindCapture(e: KeyboardEvent, target: string) {
    if (keybindListeningFor !== target) return;
    e.preventDefault();
    e.stopPropagation();

    // Ignorer les touches modificateurs seules
    if (["Shift", "Control", "Alt", "Meta"].includes(e.key)) return;

    // Normaliser le nom de la touche
    let keyName = e.key.toUpperCase();
    if (e.code.startsWith("Digit")) keyName = e.code.replace("Digit", "");
    if (e.code.startsWith("Numpad")) keyName = e.code;
    if (e.code.startsWith("F") && /^F\d+$/.test(e.code)) keyName = e.code;

    if (target === "__leader__") {
      windowStore.setLeaderKeybind(keyName);
    } else {
      windowStore.setMemberKeybind(target, keyName);
    }
    keybindListeningFor = null;
  }

  function clearKeybind(target: string) {
    if (target === "__leader__") {
      windowStore.setLeaderKeybind(null);
    } else {
      windowStore.setMemberKeybind(target, null);
    }
    keybindListeningFor = null;
  }

  function getKeybind(target: string): string | null {
    if (target === "__leader__") return windowStore.leaderKeybind;
    return windowStore.teamMemberKeybinds[target] ?? null;
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
      <div class="relative">
        <div
          class="flex items-center gap-3 px-3 py-2.5 rounded-lg bg-amber-500/10 border border-amber-500/30"
        >
          <!-- Breed icon -->
          <button
            onclick={() => openBreedPicker("__leader__")}
            class="w-6 h-6 shrink-0 rounded cursor-pointer hover:ring-2 hover:ring-amber-500/50 transition-all"
            title="Choisir la classe"
          >
            {#if windowStore.breedIcon}
              <img
                src="/breeds/symbol_{windowStore.breedIcon}.png"
                alt="Classe"
                class="w-6 h-6"
              />
            {:else}
              <Crown class="w-4 h-4 text-amber-400 mx-auto" />
            {/if}
          </button>

          <span class="text-sm font-semibold text-amber-300 flex-1">
            {windowStore.usableTitle}
          </span>

          <!-- Keybind button -->
          <button
            onclick={() => startKeybindCapture("__leader__")}
            onkeydown={(e) => handleKeybindCapture(e, "__leader__")}
            class="px-1.5 py-0.5 rounded text-xs cursor-pointer transition-colors shrink-0
              {keybindListeningFor === '__leader__'
                ? 'bg-amber-500/30 text-amber-300 ring-2 ring-amber-500 animate-pulse'
                : getKeybind('__leader__')
                  ? 'bg-stone-700 text-stone-300 hover:bg-stone-600'
                  : 'bg-stone-700/50 text-stone-500 hover:bg-stone-600 hover:text-stone-300'}"
            title={keybindListeningFor === "__leader__" ? "Appuie sur une touche..." : "Bind une touche de focus"}
          >
            {#if keybindListeningFor === "__leader__"}
              ...
            {:else if getKeybind("__leader__")}
              {getKeybind("__leader__")}
            {:else}
              <Keyboard class="w-3.5 h-3.5" />
            {/if}
          </button>

          {#if getKeybind("__leader__") && keybindListeningFor !== "__leader__"}
            <button
              onclick={() => clearKeybind("__leader__")}
              class="text-stone-500 hover:text-red-400 cursor-pointer text-xs"
              title="Supprimer le raccourci"
            >
              &times;
            </button>
          {/if}

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

        <!-- Breed picker for leader -->
        {#if breedPickerFor === "__leader__"}
          <div
            class="absolute z-10 top-full left-0 mt-1 p-2 bg-stone-800 border border-stone-600 rounded-lg shadow-xl grid grid-cols-6 gap-1"
          >
            {#each Array.from({ length: BREED_COUNT }, (_, i) => i + 1) as id}
              <button
                onclick={() => selectBreed("__leader__", id)}
                class="w-8 h-8 rounded hover:bg-stone-600 cursor-pointer transition-colors {windowStore.breedIcon === id ? 'ring-2 ring-amber-500 bg-stone-600' : ''}"
              >
                <img
                  src="/breeds/symbol_{id}.png"
                  alt="Classe {id}"
                  class="w-7 h-7 mx-auto"
                />
              </button>
            {/each}
            {#if windowStore.breedIcon}
              <button
                onclick={() => selectBreed("__leader__", null)}
                class="w-8 h-8 rounded hover:bg-stone-600 cursor-pointer transition-colors text-xs text-stone-400 col-span-6 mt-1 border border-stone-600"
              >
                Retirer
              </button>
            {/if}
          </div>
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
            {@const memberBreed = windowStore.teamMemberBreeds[member] ?? null}
            {@const memberKeybind = windowStore.teamMemberKeybinds[member] ?? null}
            <div class="relative">
              <div
                class="flex items-center gap-3 px-3 py-2 rounded-lg bg-stone-700/30 border border-stone-700 hover:border-stone-600 transition-colors group"
              >
                <!-- Breed icon -->
                <button
                  onclick={() => openBreedPicker(member)}
                  class="w-6 h-6 shrink-0 rounded cursor-pointer hover:ring-2 hover:ring-amber-500/50 transition-all"
                  title="Choisir la classe"
                >
                  {#if memberBreed}
                    <img
                      src="/breeds/symbol_{memberBreed}.png"
                      alt="Classe"
                      class="w-6 h-6"
                    />
                  {:else}
                    {#if state?.syncState === "synced"}
                      <span
                        class="block w-2 h-2 rounded-full bg-green-500 mx-auto"
                      ></span>
                    {:else if state?.syncState === "recovering"}
                      <Loader2
                        class="w-3.5 h-3.5 text-orange-500 animate-spin mx-auto"
                      />
                    {:else}
                      <span
                        class="block w-2 h-2 rounded-full bg-red-500 mx-auto"
                      ></span>
                    {/if}
                  {/if}
                </button>

                <!-- Name -->
                <span class="text-sm text-stone-300 flex-1">{member}</span>

                <!-- Keybind button -->
                <button
                  onclick={() => startKeybindCapture(member)}
                  onkeydown={(e) => handleKeybindCapture(e, member)}
                  class="px-1.5 py-0.5 rounded text-xs cursor-pointer transition-colors shrink-0
                    {keybindListeningFor === member
                      ? 'bg-amber-500/30 text-amber-300 ring-2 ring-amber-500 animate-pulse'
                      : memberKeybind
                        ? 'bg-stone-700 text-stone-300 hover:bg-stone-600'
                        : 'bg-stone-700/50 text-stone-500 hover:bg-stone-600 hover:text-stone-300'}"
                  title={keybindListeningFor === member ? "Appuie sur une touche..." : "Bind une touche de focus"}
                >
                  {#if keybindListeningFor === member}
                    ...
                  {:else if memberKeybind}
                    {memberKeybind}
                  {:else}
                    <Keyboard class="w-3.5 h-3.5" />
                  {/if}
                </button>

                {#if memberKeybind && keybindListeningFor !== member}
                  <button
                    onclick={() => clearKeybind(member)}
                    class="text-stone-500 hover:text-red-400 cursor-pointer text-xs"
                    title="Supprimer le raccourci"
                  >
                    &times;
                  </button>
                {/if}

                <!-- Sync indicator (when breed icon is set) -->
                {#if memberBreed}
                  {#if state?.syncState === "synced"}
                    <span class="w-2 h-2 rounded-full bg-green-500 shrink-0"
                    ></span>
                  {:else if state?.syncState === "recovering"}
                    <Loader2
                      class="w-3.5 h-3.5 text-orange-500 animate-spin shrink-0"
                    />
                  {:else}
                    <span class="w-2 h-2 rounded-full bg-red-500 shrink-0"
                    ></span>
                  {/if}
                {/if}

                <!-- Actions -->
                <div
                  class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
                >
                  <button
                    onclick={() => windowStore.setAsLeader(member)}
                    class="p-1 hover:bg-stone-600 rounded cursor-pointer"
                    title="Promouvoir meneur"
                  >
                    <Crown
                      class="w-3.5 h-3.5 text-amber-400/60 hover:text-amber-400"
                    />
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
                    <Trash2
                      class="w-3.5 h-3.5 text-stone-400 hover:text-red-400"
                    />
                  </button>
                </div>
              </div>

              <!-- Breed picker for member -->
              {#if breedPickerFor === member}
                <div
                  class="absolute z-10 top-full left-0 mt-1 p-2 bg-stone-800 border border-stone-600 rounded-lg shadow-xl grid grid-cols-6 gap-1"
                >
                  {#each Array.from({ length: BREED_COUNT }, (_, i) => i + 1) as id}
                    <button
                      onclick={() => selectBreed(member, id)}
                      class="w-8 h-8 rounded hover:bg-stone-600 cursor-pointer transition-colors {memberBreed === id ? 'ring-2 ring-amber-500 bg-stone-600' : ''}"
                    >
                      <img
                        src="/breeds/symbol_{id}.png"
                        alt="Classe {id}"
                        class="w-7 h-7 mx-auto"
                      />
                    </button>
                  {/each}
                  {#if memberBreed}
                    <button
                      onclick={() => selectBreed(member, null)}
                      class="w-8 h-8 rounded hover:bg-stone-600 cursor-pointer transition-colors text-xs text-stone-400 col-span-6 mt-1 border border-stone-600"
                    >
                      Retirer
                    </button>
                  {/if}
                </div>
              {/if}
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
