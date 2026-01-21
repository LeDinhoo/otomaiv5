<script lang="ts">
  import { onMount } from "svelte";
  import {
    fetchGuideCatalog,
    type GuideSummary,
  } from "$lib/services/guideService";
  import GuideSearch from "./GuideSearch.svelte";
  import GuideList from "./GuideList.svelte";
  import PaginationControls from "./PaginationControls.svelte";

  let { onSelectGuide } = $props();

  // --- États ---
  let allGuides: GuideSummary[] = $state([]);
  let loading = $state(false);
  let error = $state("");
  let searchQuery = $state("");
  let currentPage = $state(1);
  const itemsPerPage = 11;

  async function loadCatalog() {
    loading = true;
    error = "";
    try {
      allGuides = await fetchGuideCatalog();
      currentPage = 1;
    } catch (e) {
      error = "Impossible de charger le catalogue : " + e;
    } finally {
      loading = false;
    }
  }

  onMount(loadCatalog);

  // --- Logique Dérivée ---
  let filteredGuides = $derived(
    allGuides.filter((g) =>
      g.name.toLowerCase().includes(searchQuery.toLowerCase()),
    ),
  );
  let totalPages = $derived(Math.ceil(filteredGuides.length / itemsPerPage));
  let paginatedGuides = $derived(
    filteredGuides.slice(
      (currentPage - 1) * itemsPerPage,
      currentPage * itemsPerPage,
    ),
  );

  function handleSearch() {
    currentPage = 1;
  }
</script>

<div
  class="flex flex-col h-full w-full bg-stone-900 text-stone-200 p-2 animate-in fade-in duration-300"
>
  <div class="flex flex-col gap-4 mb-6">
    <GuideSearch bind:value={searchQuery} onSearch={handleSearch} />
  </div>

  <GuideList
    {loading}
    {error}
    guides={paginatedGuides}
    onSelect={onSelectGuide}
  />

  <PaginationControls
    {currentPage}
    {totalPages}
    onPrev={() => currentPage > 1 && currentPage--}
    onNext={() => currentPage < totalPages && currentPage++}
  >
    <p slot="count" class="text-xs text-stone-500">
      {filteredGuides.length} guides trouvés
    </p>
  </PaginationControls>
</div>
