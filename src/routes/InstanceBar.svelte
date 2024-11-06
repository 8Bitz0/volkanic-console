<script lang="ts">
  import Icon from '@iconify/svelte';

  import InstanceButton from './InstanceButton.svelte';
  import InstanceStatusIcon from './(instance)/InstanceStatusIcon.svelte';

  import type { AppState } from "../scripts/state";
  import type { Instance } from '../scripts/instance';
  import type { Runner } from '../scripts/runner';

  export let state: AppState;

  function toggleSettingsMenu() {
    state.settingsOpen = !state.settingsOpen;
  }

  function addInstance() {
    state.view = { type: "new-instance", runner: "", name: "" };
  }

  function addRunner() {
    state.newRunnerModal = true;
  }

  function openInstanceGeneral(instance: [string, string]) {
    state.view = { type: "instance-overview", runner: instance[0], instance: instance[1] };
  }

  // [runner ID, instance ID, instance]
  let selectedInstance: [string, string, Instance] | null = null;
  let shownInstances: [string, string, Instance][] = [];
  // [runner ID, runner]
  let shownRunners: [string, Runner][] = [];

  $: updateShownRunners(state.runners);
  $: updateShownInstances(state.selectedInstance, state.runners);
  
  /**
   * Updates and sorts the array of displayed runners based on their connection status and names.
   * 
   * @param {Map<string, Runner>} runners - Map of runner IDs to Runner objects
   *
   * Sorting logic:
   * 1. Connected runners are displayed before disconnected ones
   * 2. Within each connection status group, runners are sorted alphabetically by name
   *
   * This function updates the `shownRunners` array with the sorted entries.
   */
  function updateShownRunners(runners: Map<string, Runner>) {
    shownRunners = Array.from(runners.entries()).sort((a: [string, Runner], b: [string, Runner]) => {
      // Sort by connected status
      if (a[1].connected !== b[1].connected) {
        return b[1].connected ? 1 : -1; // True values first
      }
      // If connected status is the same, sort alphabetically by name
      return a[1].name.localeCompare(b[1].name);
    });
  }

  
  /**
   * Updates the list of shown instances based on the selected instance and available runners.
   * Separates the selected instance from the shown instances list and sorts them alphabetically.
   * 
   * @param {[string, string] | null} selectedId - Tuple of [runnerId, instanceId] for the selected instance, or null if none selected
   * @param {Map<string, Runner>} runners - Map of runners containing instances
   * 
   * @updates {Array} shownInstances - Array of instance entries [runnerId, instanceId, Instance]
   * @updates {Array|null} selectedInstance - Currently selected instance entry [runnerId, instanceId, Instance]
   */
  function updateShownInstances(selectedId: [string, string] | null, runners: Map<string, Runner>) {
    selectedInstance = null;
    shownInstances = [];

    for (let runner of Array.from(runners.entries()) as [string, Runner][]) {
      for (let instance of Array.from(runner[1].instances.entries()) as [string, Instance][]) {
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
</script>

<div class="flex flex-col w-52 min-w-52 bg-zinc-100 dark:bg-zinc-900 overflow-hidden">
  <div class="w-full h-full p-3 overflow-y-auto space-y-2 scrollbar-thumb-rounded-full scrollbar-thin scrollbar-thumb-zinc-200 dark:scrollbar-thumb-zinc-800">
    <!--  -->
    <div class="w-full overflow-y-hidden border-[1px] rounded-lg border-zinc-300 dark:border-zinc-700">
      {#if selectedInstance !== null}
        <InstanceButton disabled={true}>
          <Icon icon="mdi:cube-outline" />
          <p class="max-w-[80%] overflow-hidden text-sm flex-grow text-nowrap text-ellipsis">{selectedInstance[2].name}</p>
          <div class="flex-grow"></div>
          <InstanceStatusIcon status={selectedInstance[2].status} />
        </InstanceButton>
        <!-- Separator between instance label and buttons -->
        <div class="w-full h-[1px] bg-zinc-300 dark:bg-zinc-700"></div>
        <div class="w-full text-zinc-600 dark:text-zinc-400 bg-zinc-200 dark:bg-zinc-800 hover:bg-zinc-300 dark:hover:bg-zinc-700 transition-all duration-100">
          <button
            on:click={() => {selectedInstance && openInstanceGeneral([selectedInstance[0], selectedInstance[1]])}}
            class="flex flex-row w-full h-8 items-center gap-2 p-2 cursor-default"
          >
            <Icon icon="mdi:graph-line" class="w-5 h-5" />
            <p class="text-sm">Overview</p>
          </button>
        </div>
        <!-- Separator between instance category buttons and action buttons -->
        <div class="w-full h-[1px] bg-zinc-300 dark:bg-zinc-700"></div>
        <div class="flex flex-row h-8 gap-[1px] bg-zinc-300 dark:bg-zinc-700">
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
    <!-- Separator between instance selection and instance/runner lists -->
    <div class="h-1"></div>
    <div>
      {#if shownInstances.length > 0}
        <p class="text-zinc-400 dark:text-zinc-600 text-[11px]">INSTANCES</p>
      {/if}
      <div class="flex flex-col w-full gap-1">
        {#each shownInstances as instance}
          <InstanceButton
            class="group"
            onClick={() => {
              state.selectedInstance = [instance[0], instance[1]]

              openInstanceGeneral([instance[0], instance[1]])
            }}
          >
            <Icon icon="mdi:cube-outline" />
            <p class="max-w-[80%] overflow-hidden text-sm flex-grow text-nowrap text-ellipsis">{instance[2].name}</p>
            <div class="flex-grow"></div>
            <div class="relative flex flex-row items-center justify-center">
              <InstanceStatusIcon status={instance[2].status} class="group-hover:opacity-0 group-hover:translate-x-2 transition-all duration-100" />
              <Icon icon="mdi:arrow-right" class="absolute w-4 h-4 opacity-0 -translate-x-2 group-hover:translate-x-0 transition-all duration-100 group-hover:opacity-100 group-hover:transition-all group-hover:duration-100" />
            </div>
          </InstanceButton>
        {/each}
      </div>
    </div>
    <div>
      {#if shownRunners.length > 0}
        <p class="text-zinc-400 dark:text-zinc-600 text-[11px]">RUNNERS</p>
      {/if}
      <div class="flex flex-col w-full gap-1 overflow-y-auto scrollbar-thumb-rounded-full scrollbar-thin scrollbar-thumb-zinc-200 dark:scrollbar-thumb-zinc-800">
        {#each shownRunners as [id, details]}
          <InstanceButton
            onClick={() => {
              state.view = { type: "new-instance", runner: id, name: "" }
            }}
            class="group"
          >
            <Icon icon="mdi:cube-outline" />
            <p class="max-w-[80%] overflow-hidden text-sm flex-grow text-nowrap text-ellipsis">{details.name}</p>
            <div class="flex-grow"></div>
            <div class="relative flex flex-row items-center justify-center">
              {#if details.connected}
                <div class="w-1 h-1 rounded-full group-hover:opacity-0 group-hover:translate-x-2 bg-green-400 dark:bg-green-500 transition-all duration-100 group-hover:transition-all group-hover:duration-100"></div>
              {:else}
                <div class="w-1 h-1 rounded-full group-hover:opacity-0 group-hover:translate-x-2 bg-zinc-400 dark:bg-zinc-500 transition-all duration-100 group-hover:transition-all group-hover:duration-100"></div>
              {/if}
              <Icon icon="mdi:plus" class="absolute w-4 h-4 opacity-0 -translate-x-2 group-hover:translate-x-0 transition-all duration-100 group-hover:opacity-100 group-hover:transition-all group-hover:duration-100" />
            </div>
          </InstanceButton>
        {/each}
      </div>
    </div>
  </div>
  <div class="flex flex-col p-3 gap-2">
    <InstanceButton
      onClick={addRunner}
    >
      <Icon icon="mdi:plus" class="text-zinc-500" />
      <p class="overflow-hidden text-sm flex-grow text-nowrap text-ellipsis text-zinc-500">New Runner</p>
      <div class="flex-grow"></div>
    </InstanceButton>
  </div>
</div>
