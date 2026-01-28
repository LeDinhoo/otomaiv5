<script lang="ts">
  let { children } = $props();
</script>

<div
  class="flex items-center bg-stone-900 border-b border-stone-800 h-10 w-full"
>
  <div class="flex h-full w-full items-center overflow-x-auto custom-scrollbar">
    {@render children()}
  </div>
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    height: 4px; 
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: #44403c;
    border-radius: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: #57534e;
  }
</style>