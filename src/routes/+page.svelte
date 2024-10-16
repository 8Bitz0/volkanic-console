<script lang="ts">
  import "../app.css";

  import InstanceBar from "./InstanceBar.svelte";
  import PageContainer from "./PageView.svelte";
  import SettingsPage from "./SettingsPage.svelte";
  import TitleBar from "./TitleBar.svelte";

  import type { AppState } from "../scripts/State";

  let state: AppState = {
    runners: [],
    instances: [],
    selectedInstance: "",
    settingsOpen: false
  };
</script>

<div class="absolute flex flex-col w-full h-full overflow-hidden bg-zinc-100 dark:bg-zinc-900 text-black dark:text-gray-50">
  <TitleBar />
  {#if state.settingsOpen}
    <div class="absolute w-full h-full">
      <SettingsPage bind:state />
    </div>
  {/if}
  <div class="flex flex-row h-full">
    <InstanceBar bind:state />
    <PageContainer />
  </div>
</div>
