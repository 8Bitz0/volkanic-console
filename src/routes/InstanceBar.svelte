<script lang="ts">
  import Icon from '@iconify/svelte';

  import type { AppState } from "../scripts/State";

  export let state: AppState;

  function toggleSettingsMenu() {
    state.settingsOpen = !state.settingsOpen;
  }
</script>

<div class="flex flex-col w-64 h-full left-0 bg-zinc-100 dark:bg-zinc-900">
  <div class="flex flex-col w-full h-full p-3">
    <div class="flex flex-col w-full h-full gap-3">
      <div class="flex flex-row w-full h-8 rounded-md items-center p-2 gap-1 text-zinc-700 dark:text-zinc-400 hover:bg-zinc-200 dark:hover:bg-zinc-800 hover:transition-all hover:duration-150 active:bg-zinc-300 active:scale-95 dark:active:bg-zinc-700 active:transition-all active:duration-100">
        <Icon icon="mdi:cube-outline" />
        <p class="text-sm flex-grow">Minecraft 1.19.2</p>
        <div class="w-1 h-1 rounded-full bg-green-400 dark:bg-green-500"></div>
      </div>
    </div>
  </div>
  <div class="flex flex-row w-full h-14 min-h-14 bg-zinc-200 dark:bg-zinc-800 items-center justify-center border-t-[1px] border-zinc-200 dark:border-zinc-800 gap-[1px]">
    <button
      class="flex flex-row w-full h-full bg-zinc-100 hover:bg-zinc-200 dark:bg-zinc-900 dark:hover:bg-zinc-800 active:bg-zinc-300 dark:active:bg-zinc-700 items-center justify-center transition-all duration-100 hover:transition-all hover:duration-100"
      on:click={toggleSettingsMenu}
    >
      <Icon class="w-6 h-6 text-zinc-500 dark:text-zinc-300" icon="mdi:settings-outline" />
    </button>
    <button
      class="flex flex-row w-full h-full bg-zinc-100 hover:bg-zinc-200 dark:bg-zinc-900 dark:hover:bg-zinc-800 active:bg-zinc-300 dark:active:bg-zinc-700 items-center justify-center transition-all duration-100 hover:transition-all hover:duration-100"
    >
      <Icon class="w-8 h-8 text-zinc-500 dark:text-zinc-300" icon="mdi:plus" />
    </button>
  </div>
</div>
