<script lang="ts">
  import Icon from '@iconify/svelte';

  import InstanceButton from './InstanceButton.svelte';

  import type { AppState } from "../scripts/State";
  import type { Instance } from '../scripts/Instance';
  import type { Runner } from '../scripts/Runner';

  export let state: AppState;

  function toggleSettingsMenu() {
    state.settingsOpen = !state.settingsOpen;
  }

  function addInstance() {
    state.newInstanceModal = true;
  }

  function addRunner() {
    state.newRunnerModal = true;
  }

  // [runner ID, instance ID, instance]
  let selectedInstance: [string, string, Instance] | null = null;
  let shownInstances: [string, string, Instance][] = [];
  // [runner ID, runner]
  let shownRunners: [string, Runner][] = [];

  $: updateShownInstances(state.selectedInstance, state.runners);
  $: updateShownRunners(state.runners);

  function updateShownInstances(selectedId: [string, string] | null, runners: Map<string, Runner>) {
    shownInstances = [];

    for (let runner of Object.entries(runners) as [string, Runner][]) {
      for (let instance of Object.entries(runner[1].instances) as [string, Instance][]) {
        if (selectedId?.[0] === runner[0] && selectedId?.[1] === instance[0]) {
          selectedInstance = [runner[0], instance[0], instance[1]];
        } else {
          shownInstances.push([runner[0], instance[0], instance[1]]);
          shownInstances = shownInstances;
        }
      }
    }

    shownInstances = shownInstances.sort((a, b) => {
      // Sort alphabetically
      return a[2].name.localeCompare(b[2].name);
    })
  }

  function updateShownRunners(runners: Map<string, Runner>) {
    shownRunners = Object.entries(runners).sort((a: [string, Runner], b: [string, Runner]) => {
      // Sort by connected status
      if (a[1].connected !== b[1].connected) {
        return b[1].connected ? 1 : -1; // True values first
      }
      // If connected status is the same, sort alphabetically by name
      return a[1].name.localeCompare(b[1].name);
    });
  }
</script>

<div class="w-52 min-w-52 bg-zinc-100 dark:bg-zinc-900 overflow-hidden">
  <div class="w-full p-3 h-[95%] overflow-y-auto space-y-2 scrollbar-thumb-rounded-full scrollbar-thin scrollbar-thumb-zinc-200 dark:scrollbar-thumb-zinc-800">
    <div class="w-full overflow-y-hidden border-[1px] rounded-lg border-zinc-300 dark:border-zinc-700">
      {#if selectedInstance !== null}
        <InstanceButton active={false}>
          <Icon icon="mdi:cube-outline" class="min-w-max" />
          <p class="max-w-[80%] overflow-hidden text-sm flex-grow text-nowrap text-ellipsis">{selectedInstance[2].name}</p>
          <div class="flex-grow" />
          <div class="relative flex flex-row items-center justify-center">
            <div class="w-1 min-w-max h-1 rounded-full bg-green-400 dark:bg-green-500" />
          </div>
        </InstanceButton>
        <div class="flex flex-row h-8 gap-[1px] pt-[1px] i bg-zinc-300 dark:bg-zinc-700">
          <button class="flex flex-col w-full h-auto items-center justify-center bg-zinc-100 dark:bg-zinc-900 hover:bg-zinc-200 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-300 active:bg-zinc-300 dark:active:bg-zinc-700 transition-all cursor-default"><Icon icon="mdi:restart" /></button>
          <button class="flex flex-col w-full h-auto items-center justify-center bg-zinc-100 dark:bg-zinc-900 hover:bg-zinc-200 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-300 active:bg-zinc-300 dark:active:bg-zinc-700 transition-all cursor-default"><Icon icon="mdi:power" /></button>
        </div>
      {:else}
        <div class="flex flex-col w-full h-[65px] min-[65px] items-center justify-center text-zinc-400 dark:text-zinc-500">
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
      <div class="flex flex-col w-full gap-1">
        {#each shownInstances as instance}
          <InstanceButton
            class="group"
            onClick={() => state.selectedInstance = [instance[0], instance[1]]}
          >
            <Icon icon="mdi:cube-outline" class="min-w-max" />
            <p class="max-w-[80%] overflow-hidden text-sm flex-grow text-nowrap text-ellipsis">{instance[2].name}</p>
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
    <div>
      <p class="text-zinc-400 dark:text-zinc-600 text-[11px]">RUNNERS</p>
      <div class="flex flex-col w-full gap-1 overflow-y-auto scrollbar-thumb-rounded-full scrollbar-thin scrollbar-thumb-zinc-200 dark:scrollbar-thumb-zinc-800">
        {#each shownRunners as [id, details]}
          <InstanceButton
            class="group"
          >
            <Icon icon="mdi:cube-outline" class="min-w-max" />
            <p class="max-w-[80%] overflow-hidden text-sm flex-grow text-nowrap text-ellipsis">{details.name}</p>
            <div class="flex-grow" />
            <div class="relative flex flex-row items-center justify-center">
              {#if details.connected}
                <div class="w-1 min-w-max h-1 rounded-full group-hover:opacity-0 group-hover:translate-x-2 bg-green-400 dark:bg-green-500 transition-all duration-100 group-hover:transition-all group-hover:duration-100" />
              {:else}
                <div class="w-1 min-w-max h-1 rounded-full group-hover:opacity-0 group-hover:translate-x-2 bg-zinc-400 dark:bg-zinc-500 transition-all duration-100 group-hover:transition-all group-hover:duration-100" />
              {/if}
              <Icon icon="mdi:arrow-right" class="absolute w-4 min-w-max h-4 opacity-0 -translate-x-2 group-hover:translate-x-0 transition-all duration-100 group-hover:opacity-100 group-hover:transition-all group-hover:duration-100" />
            </div>
          </InstanceButton>
        {/each}
      </div>
    </div>
    <InstanceButton
      onClick={addRunner}
    >
      <Icon icon="mdi:plus" class="min-w-max text-zinc-500" />
      <p class="overflow-hidden text-sm flex-grow text-nowrap text-ellipsis text-zinc-500">New Runner</p>
      <div class="flex-grow" />
    </InstanceButton>
  </div>
</div>
