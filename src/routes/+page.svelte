<script lang="ts">
  import "../app.css";

  import InstanceBar from "./InstanceBar.svelte";
  import NewInstanceModal from "./NewInstanceModal.svelte";
  import NewRunnerModal from "./NewRunnerModal.svelte";
  import PageView from "./PageView.svelte";
  import SettingsPage from "./SettingsPage.svelte";
  import TitleBar from "./TitleBar.svelte";

  import type { AppState } from "../scripts/State";
  import { runnerListener } from "../scripts/Event";
  import { listRunners, type Runner } from "../scripts/Runner";

  let state: AppState = {
    runners: new Map(),
    selectedInstance: null,
    settingsOpen: false,
    newInstanceModal: false,
    newRunnerModal: false,
    titleBarEnabled: false,
  };

  function updateRunners(runners: Map<string, Runner>) {
    state.runners = runners;
  }

  listRunners().then((runners) => {
    updateRunners(runners);
  })

  runnerListener((runners) => {
    updateRunners(runners);
  });

  addEventListener('contextmenu', (e) => {
    e.preventDefault();
  });
</script>

<div class="absolute flex flex-col w-full h-full bg-zinc-100 dark:bg-zinc-900 text-zinc-950 dark:text-zinc-50">
  <TitleBar bind:state />
  <div class="flex flex-row w-full h-full flex-grow">
    {#if state.settingsOpen}
      <div class="w-full h-full">
        <SettingsPage bind:state />
      </div>
    {:else}
      <div class="flex flex-row w-full h-full">
        <InstanceBar bind:state />
        <PageView bind:state />
      </div>
    {/if}
  </div>
  {#if state.newInstanceModal}
    <NewInstanceModal bind:state />
  {/if}
  {#if state.newRunnerModal}
    <NewRunnerModal bind:state />
  {/if}
</div>
