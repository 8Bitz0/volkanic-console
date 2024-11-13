<script lang="ts">
  import "../app.css";

  import InstanceBar from "./InstanceBar.svelte";
  import NewRunnerModal from "./NewRunnerModal.svelte";
  import PageView from "./PageView.svelte";
  import SettingsPage from "./SettingsPage.svelte";
  import TitleBar from "./TitleBar.svelte";

  import type { AppState } from "../scripts/state";
  import { runnerListener } from "../scripts/event";
  import { listRunners, type Runner } from "../scripts/runner";

  let state: AppState = {
    runners: new Map(),
    selectedInstance: null,
    view: { type: "home" },
    pageViewPath: [],
    settingsOpen: false,
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

<div class="absolute w-full h-full bg-zinc-100 dark:bg-zinc-900 text-zinc-950 dark:text-zinc-50 flex flex-col">
  <TitleBar bind:state />
  <div class="flex-1">
    {#if state.settingsOpen}
      <div class="h-full">
        <SettingsPage bind:state />
      </div>
    {:else}
      <div class="flex flex-row h-full">
        <InstanceBar bind:state />
        <PageView bind:state />
      </div>
    {/if}
  </div>
  {#if state.newRunnerModal}
    <div class="fixed inset-0 z-40">
      <NewRunnerModal bind:state />
    </div>
  {/if}
</div>
