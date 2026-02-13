<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import {
    Loader2,
    Save,
    Plus,
    Trash2,
    Pencil,
    Check,
    X,
  } from "@lucide/svelte";
  import { profileStore } from "$lib/stores/profileStore.svelte";

  // --- Settings Rust ---
  let settings = $state({
    ui_open_delay: 1200,
    input_react_delay: 100,
    map_load_delay: 2000,
    chat_type_delay: 50,
    chat_validate_delay: 200,
    potion_anim_delay: 4000,
    walk_bonta: 1500,
    walk_brakmar: 2500,
    walk_sufokia: 1500,
    walk_frigost: 1500,
  });

  let loading = $state(true);
  let saving = $state(false);

  // --- Profils ---
  let creatingProfile = $state(false);
  let newProfileName = $state("");
  let renamingId = $state<string | null>(null);
  let renameValue = $state("");
  let confirmDeleteId = $state<string | null>(null);

  onMount(async () => {
    try {
      const res = await invoke("get_settings");
      // @ts-ignore
      settings = res;
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  async function save() {
    saving = true;
    try {
      const payload = Object.fromEntries(
        Object.entries(settings).map(([k, v]) => [k, Number(v)]),
      );
      await invoke("save_settings_cmd", { newSettings: payload });
    } catch (e) {
      console.error(e);
    }
    setTimeout(() => (saving = false), 500);
  }

  async function handleCreateProfile() {
    if (!newProfileName.trim()) return;
    await profileStore.createProfile(newProfileName.trim());
    newProfileName = "";
    creatingProfile = false;
  }

  async function handleRename(id: string) {
    if (!renameValue.trim()) return;
    await profileStore.renameProfile(id, renameValue.trim());
    renamingId = null;
    renameValue = "";
  }

  async function handleDelete(id: string) {
    await profileStore.deleteProfile(id);
    confirmDeleteId = null;
  }
</script>

<div
  class="p-6 space-y-6 text-stone-200 h-full overflow-y-auto custom-scrollbar"
>
  <!-- SECTION PROFILS -->
  <section class="space-y-3 border-b border-stone-700 pb-4">
    <div class="flex items-center justify-between">
      <h3 class="text-stone-400 font-semibold uppercase text-xs">Profils</h3>
      {#if !creatingProfile}
        <button
          onclick={() => (creatingProfile = true)}
          class="flex items-center gap-1 text-xs text-yellow-500 hover:text-yellow-400 cursor-pointer"
        >
          <Plus class="w-3 h-3" /> Nouveau
        </button>
      {/if}
    </div>

    <!-- Création d'un nouveau profil -->
    {#if creatingProfile}
      <div class="flex items-center gap-2">
        <input
          type="text"
          bind:value={newProfileName}
          placeholder="Nom du profil..."
          class="flex-1 bg-stone-900 border border-stone-600 rounded px-2 py-1 text-sm focus:outline-none focus:border-yellow-500"
          onkeydown={(e) => e.key === "Enter" && handleCreateProfile()}
        />
        <button
          onclick={handleCreateProfile}
          class="p-1 text-green-500 hover:text-green-400 cursor-pointer"
        >
          <Check class="w-4 h-4" />
        </button>
        <button
          onclick={() => {
            creatingProfile = false;
            newProfileName = "";
          }}
          class="p-1 text-stone-500 hover:text-stone-300 cursor-pointer"
        >
          <X class="w-4 h-4" />
        </button>
      </div>
    {/if}

    <!-- Liste des profils -->
    <div class="space-y-1">
      {#each profileStore.profiles as profile (profile.id)}
        <div
          class="flex items-center gap-2 px-2 py-1.5 rounded transition-colors
          {profileStore.activeProfileId === profile.id
            ? 'bg-yellow-500/10 border border-yellow-500/30'
            : 'hover:bg-stone-700/50 border border-transparent'}"
        >
          {#if renamingId === profile.id}
            <!-- Mode renommage -->
            <input
              type="text"
              bind:value={renameValue}
              class="flex-1 bg-stone-900 border border-stone-600 rounded px-2 py-0.5 text-sm focus:outline-none focus:border-yellow-500"
              onkeydown={(e) => e.key === "Enter" && handleRename(profile.id)}
            />
            <button
              onclick={() => handleRename(profile.id)}
              class="p-1 text-green-500 hover:text-green-400 cursor-pointer"
            >
              <Check class="w-3.5 h-3.5" />
            </button>
            <button
              onclick={() => (renamingId = null)}
              class="p-1 text-stone-500 hover:text-stone-300 cursor-pointer"
            >
              <X class="w-3.5 h-3.5" />
            </button>
          {:else if confirmDeleteId === profile.id}
            <!-- Confirmation suppression -->
            <span class="flex-1 text-sm text-red-400"
              >Supprimer "{profile.name}" ?</span
            >
            <button
              onclick={() => handleDelete(profile.id)}
              class="px-2 py-0.5 text-xs bg-red-600 hover:bg-red-500 rounded text-white cursor-pointer"
            >
              Oui
            </button>
            <button
              onclick={() => (confirmDeleteId = null)}
              class="px-2 py-0.5 text-xs bg-stone-700 hover:bg-stone-600 rounded cursor-pointer"
            >
              Non
            </button>
          {:else}
            <!-- Mode normal -->
            <button
              onclick={() => profileStore.switchProfile(profile.id)}
              class="flex-1 text-left text-sm cursor-pointer
              {profileStore.activeProfileId === profile.id
                ? 'text-yellow-500 font-semibold'
                : 'text-stone-300'}"
            >
              {profile.name}
            </button>

            <button
              onclick={() => {
                renamingId = profile.id;
                renameValue = profile.name;
              }}
              class="p-1 text-stone-500 hover:text-stone-300 opacity-0 group-hover:opacity-100 cursor-pointer"
              class:opacity-100={profileStore.activeProfileId === profile.id}
              title="Renommer"
            >
              <Pencil class="w-3 h-3" />
            </button>

            {#if profileStore.profiles.length > 1}
              <button
                onclick={() => (confirmDeleteId = profile.id)}
                class="p-1 text-stone-500 hover:text-red-400 opacity-0 group-hover:opacity-100 cursor-pointer"
                class:opacity-100={profileStore.activeProfileId === profile.id}
                title="Supprimer"
              >
                <Trash2 class="w-3 h-3" />
              </button>
            {/if}
          {/if}
        </div>
      {/each}
    </div>
  </section>

  <!-- SECTION SETTINGS -->
  <h2 class="text-xl font-bold text-yellow-500 mb-4">
    Configuration Délais (ms)
  </h2>

  {#if loading}
    <div class="flex justify-center"><Loader2 class="animate-spin" /></div>
  {:else}
    <section class="space-y-4 border-b border-stone-700 pb-4">
      <h3 class="text-stone-400 font-semibold uppercase text-xs">
        Interface & Maps
      </h3>
      <div class="grid grid-cols-2 gap-4">
        <label class="flex flex-col gap-1">
          <span class="text-sm">Ouverture Menus (H)</span>
          <input
            type="number"
            bind:value={settings.ui_open_delay}
            class="bg-stone-900 border border-stone-700 rounded p-2"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-sm">Chargement Map (Zaap)</span>
          <input
            type="number"
            bind:value={settings.map_load_delay}
            class="bg-stone-900 border border-stone-700 rounded p-2"
          />
        </label>
      </div>
    </section>

    <section class="space-y-4 border-b border-stone-700 pb-4">
      <h3 class="text-stone-400 font-semibold uppercase text-xs">
        Chat & Commandes
      </h3>
      <div class="grid grid-cols-2 gap-4">
        <label class="flex flex-col gap-1">
          <span class="text-sm">Vitesse frappe</span>
          <input
            type="number"
            bind:value={settings.chat_type_delay}
            class="bg-stone-900 border border-stone-700 rounded p-2"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-sm">Validation (Entrée)</span>
          <input
            type="number"
            bind:value={settings.chat_validate_delay}
            class="bg-stone-900 border border-stone-700 rounded p-2"
          />
        </label>
      </div>
    </section>

    <section class="space-y-4">
      <h3 class="text-stone-400 font-semibold uppercase text-xs">
        Marche vers Zaapi
      </h3>
      <div class="grid grid-cols-2 gap-4">
        <label class="flex flex-col gap-1">
          <span class="text-sm">Bonta</span>
          <input
            type="number"
            bind:value={settings.walk_bonta}
            class="bg-stone-900 border border-stone-700 rounded p-2"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-sm">Brakmar</span>
          <input
            type="number"
            bind:value={settings.walk_brakmar}
            class="bg-stone-900 border border-stone-700 rounded p-2"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-sm">Sufokia</span>
          <input
            type="number"
            bind:value={settings.walk_sufokia}
            class="bg-stone-900 border border-stone-700 rounded p-2"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-sm">Frigost</span>
          <input
            type="number"
            bind:value={settings.walk_frigost}
            class="bg-stone-900 border border-stone-700 rounded p-2"
          />
        </label>
      </div>
    </section>

    <div class="pt-4 sticky bottom-0 bg-stone-900/90 backdrop-blur pb-2">
      <Button
        onclick={save}
        disabled={saving}
        class="w-full bg-yellow-600 hover:bg-yellow-500 text-white"
      >
        {#if saving}
          <Loader2 class="mr-2 h-4 w-4 animate-spin" /> Sauvegarde...
        {:else}
          <Save class="mr-2 h-4 w-4" /> Enregistrer la configuration
        {/if}
      </Button>
    </div>
  {/if}
</div>
