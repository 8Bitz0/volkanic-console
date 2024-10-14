<script lang="ts">
  import Icon from '@iconify/svelte';

  import type { AppState } from "../scripts/State";

  export let state: AppState;
</script>

<div class="flex flex-col w-52 h-full left-0 bg-neutral-100 dark:bg-neutral-900 border-r-[1px] border-neutral-200 dark:border-neutral-800">
  <div class="flex flex-col w-full h-full p-4">
    <h1 class="text-xl font-bold">Instances</h1>
    <div class="flex flex-col w-full h-full">
      <div class="flex flex-row w-full h-8 hover:bg-neutral-800 rounded-md">
        
      </div>
    </div>
  </div>
  <div class="flex flex-row w-full h-14 min-h-14 bg-neutral-200 dark:bg-neutral-800 items-center justify-center border-t-[1px] border-neutral-200 dark:border-neutral-800 gap-[1px]">
    <div class="flex flex-row w-full h-full bg-neutral-100 hover:bg-neutral-200 dark:bg-neutral-900 dark:hover:bg-neutral-800 active:bg-neutral-300 dark:active:bg-neutral-700 items-center justify-center transition-all duration-100 hover:transition-all hover:duration-100">
      <Icon class="w-6 h-6 text-neutral-300" icon="mdi:settings-outline" />
    </div>
    <div class="flex flex-row w-full h-full bg-neutral-100 hover:bg-neutral-200 dark:bg-neutral-900 dark:hover:bg-neutral-800 active:bg-neutral-300 dark:active:bg-neutral-700 items-center justify-center transition-all duration-100 hover:transition-all hover:duration-100">
      <Icon class="w-8 h-8 text-neutral-300" icon="mdi:plus" />
    </div>
  </div>
</div>
