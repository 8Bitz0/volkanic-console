<script lang="ts">
  import Icon from '@iconify/svelte';

  import InstanceButton from './InstanceButton.svelte';

  import type { AppState } from "../scripts/State";
  import type { Instance } from '../scripts/Instance';

  export let state: AppState;

  function toggleSettingsMenu() {
    state.settingsOpen = !state.settingsOpen;
  }

  function addInstance() {
    state.newInstanceModal = true;
  }

  let selectedInstance: Instance | null = null;
  let shownInstances: Instance[] = [];

  $: updateShownInstances(state.selectedInstance, state.instances);

  function updateShownInstances(selectedId: string | undefined, instances: Instance[]) {
    shownInstances = [];

    for (let instance of instances) {
      if (selectedId === instance.id) {
        selectedInstance = instance;
      } else {
        shownInstances.push(instance);
        shownInstances = shownInstances;
      }
    }
  }
</script>

<div class="flex flex-col w-72 bg-zinc-100 dark:bg-zinc-900 overflow-hidden">
  <div class="flex flex-col w-full max-h-[90%] gap-2 p-3">
    <div class="flex flex-col w-full gap-1 overflow-y-hidden border-[1px] rounded-lg border-zinc-200 dark:border-zinc-800">
      {#if selectedInstance !== null}
        <InstanceButton active={false}>
          <Icon icon="mdi:cube-outline" class="min-w-max" />
          <p class="max-w-[80%] overflow-hidden text-sm flex-grow text-nowrap text-ellipsis">{selectedInstance.name}</p>
          <div class="flex-grow" />
          <div class="relative flex flex-row items-center justify-center">
            <div class="w-1 min-w-max h-1 rounded-full bg-green-400 dark:bg-green-500" />
          </div>
        </InstanceButton>
        <div class="flex flex-row h-8 gap-[1px] pt-[1px] i bg-zinc-200 dark:bg-zinc-800">
          <button class="flex flex-col w-full h-auto items-center justify-center bg-zinc-100 dark:bg-zinc-900 hover:bg-zinc-200 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-300 active:bg-zinc-300 dark:active:bg-zinc-700 transition-all cursor-default"><Icon icon="mdi:restart" /></button>
          <button class="flex flex-col w-full h-auto items-center justify-center bg-zinc-100 dark:bg-zinc-900 hover:bg-zinc-200 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-300 active:bg-zinc-300 dark:active:bg-zinc-700 transition-all cursor-default"><Icon icon="mdi:power" /></button>
        </div>
      {:else}
        <div class="flex flex-col w-full h-[65px] min-[65px] items-center justify-center text-zinc-400 dark:text-zinc-600">
          <Icon icon="mdi:cube-outline" class="w-5 h-5" />
          <p class="text-sm">No instance selected</p>
        </div>
      {/if}
    </div>
    {#if shownInstances.length > 0}
      <div class="h-2" />
    {/if}
    <div>
      {#if shownInstances.length > 0}
        <p class="text-zinc-400 dark:text-zinc-600 text-[11px]">INSTANCES</p>
      {/if}
      <div class="flex flex-col w-full gap-1 overflow-y-auto scrollbar-thumb-rounded-full scrollbar-thin scrollbar-thumb-zinc-200 dark:scrollbar-thumb-zinc-800">
        {#each shownInstances as instance}
          <InstanceButton
            class="group"
            onClick={() => state.selectedInstance = instance.id}
          >
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
