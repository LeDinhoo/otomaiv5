<script lang="ts">
  let { children } = $props();
</script>

<div
  class="flex items-center bg-stone-900 border-b border-stone-800 h-10 w-full overflow-hidden"
>
  <div class="flex h-full w-full items-center">
    {@render children()}
  </div>
</div>

<style>
  .no-scrollbar::-webkit-scrollbar {
    display: none;
  }
  .no-scrollbar {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
</style>
