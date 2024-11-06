<script lang="ts">
  import InstanceViewButton from "./InstanceViewButton.svelte";
  import type { AppState } from "../../scripts/state";
  import { newInstance } from "../../scripts/instance";
  import type { Runner } from "../../scripts/runner";

  export let state: AppState;
  export let runner: string;
  export let name: string = "";
  export let volkanicBase64: string = "";

  let nameInput: HTMLInputElement;
  let baseInput: HTMLInputElement;

  let currentRunner: Runner | undefined = state.runners.get(runner);

  let canCreate: boolean = false;

  $: state.pageViewPath = [
    { name: currentRunner?.name ?? "Unknown", view: null },
    { name: "New Instance", view: null },
  ];

  $: canCreate = (name !== "" && volkanicBase64 !== "");

  async function createInstance() {
    await newInstance(runner, {
      name,
      type: {
        volkanic: {
          source: {
            base64: volkanicBase64,
          }
        }
      },
    });

    state.view = { type: "home" };
  }
</script>

<div class="w-full h-full flex-grow overflow-y-auto p-8 text-zinc-700 dark:text-zinc-300">
  <div class="flex flex-col w-full flex-grow gap-6">
    <div class="space-y-1">
      <h1 class="text-2xl font-medium">New Instance</h1>
      <p class="text-sm text-zinc-600 dark:text-zinc-400">The instance will be available after the runner has finished getting everything ready. Setup time varies by runner.</p>
      <div class="h-2"></div>
      <div class="flex flex-row text-xs  italic space-x-2 text-zinc-600 dark:text-zinc-400">
        <p class="text-red-400">*</p>
        <p>- indicates a required question</p>
      </div>
    </div>
    <div class="w-full rounded-lg overflow-hidden gap-[1px]">
      <div class="flex flex-row w-full items-center px-4 py-3">
        <div class="flex flex-row flex-grow gap-0.5">
          <p>Name</p>
          <p class="text-red-400">*</p>
        </div>
        <input
          bind:this={nameInput}
          placeholder="My Instance"
          bind:value={name}
          type="text"
          autocomplete="off"
          autocorrect="off"
          autocapitalize="off"
          spellcheck="false"
          class="w-56 h-8 border-[1px] border-zinc-200 dark:border-zinc-700 rounded-lg p-2 bg-zinc-50 dark:bg-zinc-900"
        />
      </div>
      <div class="flex flex-row w-full items-center gap-2 px-4 py-3">
        <div class="flex flex-col flex-grow gap-1">
          <div class="flex flex-row gap-0.5">
            <p>Volkanic Template</p>
            <p class="text-red-400">*</p>
          </div>
          <p class="text-sm">Base64 encoded Volkanic Template</p>
        </div>
        <input
          bind:this={baseInput}
          bind:value={volkanicBase64}
          type="text"
          class="w-56 h-8 border-[1px] border-zinc-200 dark:border-zinc-700 rounded-lg p-2 bg-zinc-50 dark:bg-zinc-900"
        />
      </div>
    </div>
    <div class="flex flex-row w-full justify-end px-6">
      <InstanceViewButton
        onClick={createInstance}
        disabled={!canCreate}
      >
        Create
      </InstanceViewButton>
    </div>
  </div>
</div>
