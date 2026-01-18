<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";

  let { onOpen } = $props(); // Callback quand on clique sur ouvrir
  let guideInputId = $state("552");
</script>

<div class="p-4 space-y-4 max-w-sm">
  <div class="space-y-2 p-3 bg-stone-800/40 border border-stone-700 rounded-md">
    <p class="text-xs font-bold uppercase text-orange-500">Bibliothèque</p>
    <div class="flex space-x-2">
      <Input
        bind:value={guideInputId}
        placeholder="ID Guide"
        class="bg-stone-900 h-9"
      />
      <Button
        onclick={() => onOpen(guideInputId)}
        class="bg-orange-700 hover:bg-orange-600"
      >
        Ouvrir
      </Button>
    </div>
  </div>
</div>
