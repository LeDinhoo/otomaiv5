<script lang="ts">
  import { Button } from "$lib/components/ui/button/";
  import {
    ChevronLeft,
    ChevronRight,
    Waypoints,
    Keyboard,
    MousePointerClick,
  } from "@lucide/svelte";

  let {
    canGoPrev,
    canGoNext,
    onPrev,
    onNext,
    autoPilot = $bindable(false),
    listenKeys = $bindable(true),
    mirrorClicks = $bindable(false),
    onToggleListenKeys,
    onToggleMirrorClicks,
  } = $props();
</script>

<div
  class="flex-none p-1 border-t border-stone-700 bg-stone-800  flex justify-between items-center gap-2 select-none"
>
  <Button
    variant="secondary"
    onclick={onPrev}
    disabled={!canGoPrev}
    class="select-none size-8 text-stone-300 bg-[#615d59] hover:bg-[#968d84]"
  >
    <ChevronLeft class="w-4 h-4" />
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

    <Button
      variant="secondary"
      onclick={onToggleMirrorClicks}
      title="Mirror Clics (réplique les clics sur les suiveurs)"
      class="{mirrorClicks
        ? ' bg-[#a09890b9] hover:bg-[#d1c4b7b2] '
        : 'bg-[#615d59] hover:bg-[#968d84]'} size-9 cursor-pointer rounded-none h-full flex items-center justify-center select-none"
    >
      <MousePointerClick
        class="size-5 drop-shadow-4xl {mirrorClicks
          ? 'text-[#f7c882]'
          : 'text-stone-300'}"
      />
    </Button>
  </div>

  <Button
    variant="default"
    onclick={onNext}
    disabled={!canGoNext}
    class=" bg-[#a4713e] size-8 hover:bg-[#b8976f] select-none"
  ><ChevronRight class="w-4 h-4" />
  </Button>
</div>
