<script lang="ts">
  import Icon from '@iconify/svelte';

  import InstanceButton from './InstanceButton.svelte';

  import type { AppState } from "../scripts/State";

  export let state: AppState;

  function toggleSettingsMenu() {
    state.settingsOpen = !state.settingsOpen;
  }
</script>

<div class="flex flex-col w-72 h-full left-0 bg-zinc-100 dark:bg-zinc-900 overflow-hidden">
  <div class="flex flex-col w-full h-full p-3">
    <div class="flex flex-col w-full h-full gap-3">
      <InstanceButton class="group">
        <Icon icon="mdi:cube-outline" class="min-w-max" />
        <p class="max-w-[80%] overflow-hidden text-sm flex-grow text-nowrap text-ellipsis">Minecraft 1.19.2</p>
        <div class="flex-grow" />
        <div class="relative flex flex-row items-center justify-center">
          <div class="w-1 min-w-max h-1 rounded-full group-hover:opacity-0 group-hover:translate-x-2 bg-green-400 dark:bg-green-500 transition-all duration-100 group-hover:transition-all group-hover:duration-100" />
          <Icon icon="mdi:arrow-right" class="absolute w-4 min-w-max h-4 opacity-0 -translate-x-2 group-hover:translate-x-0 transition-all duration-100 group-hover:opacity-100 group-hover:transition-all group-hover:duration-100" />
        </div>
      </InstanceButton>
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
