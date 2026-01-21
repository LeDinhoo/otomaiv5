<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { ChevronLeft, ChevronRight } from "@lucide/svelte";

  let { currentPage, totalPages, onPrev, onNext } = $props();
</script>

<div
  class="flex items-center justify-between mt-4 pt-2 border-t border-stone-800"
>
  <slot name="count" />
  <div class="flex items-center gap-2">
    <Button
      variant="secondary"
      size="sm"
      onclick={onPrev}
      disabled={currentPage === 1}
    >
      <ChevronLeft class="w-4 h-4" />
    </Button>

    <span class="text-sm font-mono text-stone-400 w-16 text-center">
      {currentPage} / {totalPages || 1}
    </span>

    <Button
      variant="secondary"
      size="sm"
      onclick={onNext}
      disabled={currentPage >= totalPages}
    >
      <ChevronRight class="w-4 h-4" />
    </Button>
  </div>
</div>
