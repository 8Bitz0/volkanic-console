<script lang="ts">
  import Icon from "@iconify/svelte";
  import { blur } from "svelte/transition";

  import InstanceViewButton from "./InstanceViewButton.svelte";
  import type { AppState } from "../../scripts/state";
  import { delInstance, type Instance } from "../../scripts/instance";

  export let state: AppState;
  // [runner ID, instance ID]
  export let instanceRef: [string, string];

  let openInstance: Instance | undefined;

  let deleteConfirmation: boolean = false;

  async function deleteInstance() {
    if (openInstance) {
      await delInstance(instanceRef[0], instanceRef[1]);
      state.view = { type: "home" };
    }
  }

  $: instanceName = openInstance?.name ?? "Unknown";
  $: state.pageViewPath = [
      { name: instanceName, view: null },
  ];

  $: {
    const runner = state.runners instanceof Map ? state.runners.get(instanceRef[0]) : undefined;
    openInstance = runner?.instances instanceof Map ? runner.instances.get(instanceRef[1]) : undefined;
    if (!openInstance) {
      state.view = { type: "home" };
    }
  }
</script>

<div class="flex flex-col w-full h-full overflow-y-auto gap-12 p-8 text-zinc-700 dark:text-zinc-300">
  {#if openInstance !== undefined}
    <div class="flex flex-row items-center gap-2">
      <h1 class="text-2xl font-medium">{openInstance.name}</h1>
      <button class="flex flex-col w-7 h-7 items-center justify-center p-0.5 text-zinc-300 dark:text-zinc-700 hover:text-zinc-400 dark:hover:text-zinc-600 active:scale-90 transition-all duration-100 cursor-default">
        <Icon icon="mdi:square-edit-outline" class="w-full h-full" />
      </button>
    </div>
    <div class="w-full rounded-lg border-[0.5px] border-red-400">
      <div class="flex flex-row w-full items-center px-4 py-3">
        <div class="flex-grow">
          <p class="font-semibold">Lock Instance</p>
          <p class="text-sm">While an instance is locked, it cannot be modified or started.</p>
        </div>
        <InstanceViewButton danger={true}>Lock</InstanceViewButton>
      </div>
      <div class="relative h-[68px]">
        {#if !deleteConfirmation}
          <div class="absolute inset-0 flex flex-row w-full items-center px-4 py-3" transition:blur>
            <div class="flex-grow">
              <p class="font-semibold">Delete Instance</p>
              <p class="text-sm">Once an instance is deleted, it cannot be undone.</p>
            </div>
            <InstanceViewButton onClick={() => deleteConfirmation = true} danger={true}>Delete</InstanceViewButton>
          </div>
        {:else}
          <div class="absolute inset-0 flex flex-row w-full items-center px-4 py-3" transition:blur>
            <div class="flex-grow">
              <p class="font-semibold">Are you sure?</p>
              <p class="text-sm">This action cannot be undone.</p>
            </div>
            <div class="flex flex-row gap-2">
              <InstanceViewButton onClick={deleteInstance} danger={true}>Confirm</InstanceViewButton>
              <InstanceViewButton onClick={() => deleteConfirmation = false} danger={false}>Cancel</InstanceViewButton>
            </div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
