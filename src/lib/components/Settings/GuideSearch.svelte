<script lang="ts">
  import { Input } from "$lib/components/ui/input/index.js";
  import { Search } from "@lucide/svelte";

  let { value = $bindable(), onSearch } = $props();
</script>

<div class="relative">
  <Search
    class="absolute left-3 top-1/2 transform -translate-y-1/2 text-stone-500 w-4 h-4"
  />
  <Input
    type="text"
    placeholder="Rechercher un guide..."
    class="pl-10 bg-stone-800 border-stone-700 h-10 w-full"
    bind:value
    oninput={onSearch}
  />
</div>
