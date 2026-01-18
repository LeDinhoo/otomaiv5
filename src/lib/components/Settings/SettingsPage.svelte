<script lang="ts">
  import { onMount } from "svelte";
  import { fetchGuideCatalog, type GuideSummary } from "$lib/services/guideService";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Loader2, Search, ChevronLeft, ChevronRight, RefreshCw, BookOpen } from "@lucide/svelte";

  // On reçoit une fonction pour ouvrir le guide choisi
  let { onSelectGuide } = $props();

  // --- États ---
  let allGuides: GuideSummary[] = $state([]);
  let loading = $state(false);
  let error = $state("");
  let searchQuery = $state("");
  let currentPage = $state(1);
  const itemsPerPage = 11;

  // --- Chargement ---
  async function loadCatalog() {
    loading = true;
    error = "";
    try {
      allGuides = await fetchGuideCatalog();
      // On reset la page quand on recharge
      currentPage = 1;
    } catch (e) {
      error = "Impossible de charger le catalogue : " + e;
    } finally {
      loading = false;
    }
  }

  // Charger au démarrage
  onMount(() => {
    loadCatalog();
  });

  // --- Logique Dérivée (Runes) ---
  
  // 1. Filtrage
  let filteredGuides = $derived(
    allGuides.filter(g => 
      g.name.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  // 2. Pagination
  let totalPages = $derived(Math.ceil(filteredGuides.length / itemsPerPage));
  
  let paginatedGuides = $derived(
    filteredGuides.slice(
      (currentPage - 1) * itemsPerPage, 
      currentPage * itemsPerPage
    )
  );

  // Reset page quand on cherche
  function handleSearch(e) {
    searchQuery = e.target.value;
    currentPage = 1; 
  }

  // Navigation
  function nextPage() {
    if (currentPage < totalPages) currentPage++;
  }

  function prevPage() {
    if (currentPage > 1) currentPage--;
  }
</script>

<div class="flex flex-col h-full w-full bg-stone-900 text-stone-200 p-2 animate-in fade-in duration-300">
  
  <div class="flex flex-col gap-4 mb-6">
    <div class="flex justify-between items-center">
    </div>

    <div class="relative">
      <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 text-stone-500 w-4 h-4" />
      <Input 
        type="text" 
        placeholder="Rechercher un guide..." 
        class="pl-10 bg-stone-800 border-stone-700 h-10 w-full"
        value={searchQuery}
        oninput={handleSearch}
      />
    </div>
  </div>

  <div class="flex-1 overflow-y-auto">
    {#if loading && allGuides.length === 0}
      <div class="flex h-full items-center justify-center text-stone-500 gap-2">
        <Loader2 class="animate-spin" /> Chargement...
      </div>
    
    {:else if error}
      <div class="flex h-full items-center justify-center text-red-400 p-4 text-center">
        {error}
      </div>

    {:else if paginatedGuides.length === 0}
      <div class="flex h-full items-center justify-center text-stone-500 italic">
        Aucun guide trouvé.
      </div>

    {:else}
      <div class="flex flex-col gap-2">
        {#each paginatedGuides as guide}
          <button 
            onclick={() => onSelectGuide(guide.id)}
            class="flex items-center justify-between p-3 rounded-lg bg-stone-800/40 border border-stone-800 hover:bg-stone-800 hover:border-orange-500/50 transition-all text-left group"
          >
            <span class="font-medium text-stone-300 group-hover:text-white truncate pr-4">
              {guide.name}
            </span>
            <span class="text-xs text-stone-600 font-mono group-hover:text-orange-400">
              Ouvrir →
            </span>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <div class="flex items-center justify-between mt-4 pt-2 border-t border-stone-800">
    <p class="text-xs text-stone-500">
      {filteredGuides.length} guides trouvés
    </p>

    <div class="flex items-center gap-2">
      <Button variant="secondary" size="sm" onclick={prevPage} disabled={currentPage === 1}>
        <ChevronLeft class="w-4 h-4" />
      </Button>
      
      <span class="text-sm font-mono text-stone-400 w-16 text-center">
        {currentPage} / {totalPages || 1}
      </span>

      <Button variant="secondary" size="sm" onclick={nextPage} disabled={currentPage >= totalPages}>
        <ChevronRight class="w-4 h-4" />
      </Button>
    </div>
  </div>
</div>