<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import { Loader2, Save } from "@lucide/svelte";

  // Correspond exactement à la struct Rust
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

  onMount(async () => {
    try {
      // Charger depuis Rust
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
      // Convertir en nombres entiers (au cas où l'input renvoie des strings)
      const payload = Object.fromEntries(
        Object.entries(settings).map(([k, v]) => [k, Number(v)]),
      );
      await invoke("save_settings_cmd", { newSettings: payload });
    } catch (e) {
      console.error(e);
    }
    // Petit délai visuel pour montrer que c'est fait
    setTimeout(() => (saving = false), 500);
  }
</script>

<div
  class="p-6 space-y-6 text-stone-200 h-full overflow-y-auto custom-scrollbar"
>
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
