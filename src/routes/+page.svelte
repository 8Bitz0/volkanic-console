<script lang="ts">
  import "../app.css";

  import InstanceBar from "./InstanceBar.svelte";
  import NewInstanceModal from "./NewInstanceModal.svelte";
  import NewRunnerModal from "./NewRunnerModal.svelte";
  import PageView from "./PageView.svelte";
  import SettingsPage from "./SettingsPage.svelte";
  import TitleBar from "./TitleBar.svelte";

  import type { AppState } from "../scripts/State";

  let state: AppState = {
    runners: [],
    instances: [
      {
        id: "1234",
        name: "Minecraft 1.19.2",
        type: {
          volkanic: {
            source: {
              url: ["https://example.com"]
            }
          }
        }
      }
    ],
    selectedInstance: "",
    settingsOpen: false,
    newInstanceModal: false,
    newRunnerModal: false,
    titleBarEnabled: false,
  };
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
