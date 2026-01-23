<script lang="ts">
  import { Button } from "$lib/components/ui/button/";
  import {
    ChevronLeft,
    ChevronRight,
    Waypoints,
    Keyboard,
  } from "@lucide/svelte";

  let {
    canGoPrev,
    canGoNext,
    onPrev,
    onNext,
    autoPilot = $bindable(false),
    listenKeys = $bindable(true),
    onToggleListenKeys,
  } = $props();
</script>

<div
  class="flex-none p-2 border-t border-stone-800 bg-[#373737]  flex justify-between items-center gap-4 select-none"
>
  <Button
    variant="secondary"
    onclick={onPrev}
    disabled={!canGoPrev}
    class="w-28 select-none text-stone-300 bg-[#615d59] hover:bg-[#968d84]"
  >
    <ChevronLeft class="w-4 h-4 mr-1" /> Précédent
  </Button>

  <div class="flex items-center overflow-hidden rounded-sm h-full">
    <Button
      variant="secondary"
      onclick={() => (autoPilot = !autoPilot)}
      title="Activer l'Auto-Pilot"
      class="{autoPilot
        ? ' bg-[#a09890b9] hover:bg-[#d1c4b7b2] '
        : 'bg-[#615d59] hover:bg-[#968d84]'} size-9 cursor-pointer rounded-none h-full flex items-center justify-center select-none"
    >
      <Waypoints
        class="size-5 drop-shadow-4xl {autoPilot
          ? 'text-[#f7c882]'
          : 'text-stone-300'}"
      />
    </Button>

    <Button
      variant="secondary"
      onclick={onToggleListenKeys}
      title="Raccourcis Clavier (A/D)"
      class="{listenKeys
        ? ' bg-[#a09890b9] hover:bg-[#d1c4b7b2] '
        : 'bg-[#615d59] hover:bg-[#968d84]'} size-9 cursor-pointer rounded-none h-full flex items-center justify-center select-none"
    >
      <Keyboard
        class="size-5 drop-shadow-4xl {listenKeys
          ? 'text-[#f7c882]'
          : 'text-stone-300'}"
      />
    </Button>
  </div>

  <Button
    variant="default"
    onclick={onNext}
    disabled={!canGoNext}
    class="w-28 bg-[#a4713e] hover:bg-[#b8976f] select-none"
  >
    Suivant <ChevronRight class="w-4 h-4 ml-1" />
  </Button>
</div>
