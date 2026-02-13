<script lang="ts">
  import { MapPin } from "@lucide/svelte";
  import { guideStore } from "$lib/stores/guideStore.svelte";

  let { stepIndex, totalSteps, currentStep, tabId } = $props();

  let progressPercentage = $derived(((stepIndex + 1) / totalSteps) * 100);

  function handleStepInput(e: Event) {
    const input = e.target as HTMLInputElement;
    const val = parseInt(input.value);
    if (!isNaN(val) && val >= 1 && val <= totalSteps) {
      guideStore.setStep(tabId, val - 1);
    } else {
      input.value = (stepIndex + 1).toString();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") (e.target as HTMLInputElement).blur();
  }
</script>

<div
  class="flex-none pt-2 pb-0 border-b border-stone-800  bg-stone-900 z-10 flex flex-col gap-2 select-none "
>
  <div class="flex justify-between items-end px-1 ">
    <div
      class="flex items-center gap-1.5 text-orange-500/80 hover:text-orange-500 hover:cursor-pointer  font-mono text-xs px-2 py-0.5 rounded"
    >
      <MapPin class="w-3 h-3" />
      {#if currentStep && (currentStep.pos_x !== 0 || currentStep.pos_y !== 0)}
        [{currentStep.pos_x}, {currentStep.pos_y}]
      {:else}
        [---, ---]
      {/if}
    </div>

    <div
      class="text-xs text-stone-500 font-mono pr-2 shrink-0 flex items-center"
    >
      <span>Étape</span>
      <input
        type="text"
        class="bg-transparent border-none p-0 mx-1 w-[3ch] text-center text-stone-500 font-mono focus:text-stone-200 focus:outline-none focus:bg-stone-800/50 rounded transition-colors cursor-text hover:text-stone-300"
        value={stepIndex + 1}
        onchange={handleStepInput}
        onkeydown={handleKeydown}
      />
      <span>/ {totalSteps}</span>
    </div>
  </div>

  <div class="w-full h-[2px] bg-stone-800 overflow-hidden mb-[-1px]">
    <div
      class="h-full bg-[#8ad47b] transition-all duration-300 ease-out shadow-[0_0_10px_rgba(234,88,12,0.5)]"
      style="width: {progressPercentage}%"
    ></div>
  </div>
</div>
