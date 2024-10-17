<script lang="ts">
  import Icon from '@iconify/svelte';

  import InstanceButton from './InstanceButton.svelte';

  import type { AppState } from "../scripts/State";

  export let state: AppState;

  function toggleSettingsMenu() {
    state.settingsOpen = !state.settingsOpen;
  }

  function addInstance() {
    state.instances.push({
      id: "12345",
      name: "Minecraft 1.19.2",
      type: {
        volkanic: {
          source: {
            url: ["https://example.com"]
          }
        }
      }
    });

    state.instances = state.instances;
  }
</script>

<div class="flex flex-col w-72 bg-zinc-100 dark:bg-zinc-900 overflow-hidden">
  <div class="flex flex-col w-full max-h-[80%] gap-4 p-3">
    <div class="flex flex-col w-full gap-1 overflow-y-auto border-[1px] border-dashed border-zinc-600 rounded-lg scrollbar-thumb-rounded-full scrollbar-thin scrollbar-thumb-zinc-800">
      {#each state.instances as instance}
        <InstanceButton class="group">
          <Icon icon="mdi:cube-outline" class="min-w-max" />
          <p class="max-w-[80%] overflow-hidden text-sm flex-grow text-nowrap text-ellipsis">{instance.name}</p>
          <div class="flex-grow" />
          <div class="relative flex flex-row items-center justify-center">
            <div class="w-1 min-w-max h-1 rounded-full group-hover:opacity-0 group-hover:translate-x-2 bg-green-400 dark:bg-green-500 transition-all duration-100 group-hover:transition-all group-hover:duration-100" />
            <Icon icon="mdi:arrow-right" class="absolute w-4 min-w-max h-4 opacity-0 -translate-x-2 group-hover:translate-x-0 transition-all duration-100 group-hover:opacity-100 group-hover:transition-all group-hover:duration-100" />
          </div>
        </InstanceButton>
      {/each}
    </div>
    <InstanceButton
      onClick={addInstance}
    >
      <Icon icon="mdi:plus" class="min-w-max" />
        <p class="overflow-hidden text-sm flex-grow text-nowrap text-ellipsis">New Instance</p>
        <div class="flex-grow" />
    </InstanceButton>
  </div>
</div>
