<script lang="ts">
  import Icon from "@iconify/svelte";
  import { sineInOut } from "svelte/easing";
  import { blur, scale } from "svelte/transition";

  import { isValidUrl, newRunner } from "../scripts/Runner";
  import type { AppState } from "../scripts/State";

  export let state: AppState;

  let canConnect: boolean = false;

  let nameValue: string = "";
  let addressValue: string = "";

  let processing: boolean = false;

  $: checkDetails(nameValue, addressValue).then((v) => canConnect = v);

  function closeModal() {
    state.newRunnerModal = false;
  }

  async function connectRunner() {
    try {
      processing = true;

      await newRunner(nameValue, addressValue);

      closeModal();
    } catch(e) {
      console.error("Error while adding runner: " + e);
      processing = false;
    }
  }

  async function checkDetails(name: string, address: string): Promise<boolean> {
    let valid: boolean = true;

    if (name.length <= 0) {
      valid = false;
    }

    if (!await isValidUrl(address)) {
      valid = false;
    }

    return valid;
  }
</script>

<div transition:blur={{ duration: 200 }} class="absolute flex flex-col w-full h-full items-center justify-center bg-[#00000020] backdrop-blur-md">
  <div transition:scale={{ duration: 200, start: 0.5, opacity: 0, easing: sineInOut }} class="flex flex-col w-[384px] border-[1px] border-zinc-300 dark:border-zinc-700 rounded-2xl bg-zinc-50 dark:bg-zinc-800">
    <div class="flex flex-col w-full h-full gap-3 p-6">
      <h1 class="text-xl">Add Runner</h1>
      <div>
        <p class="text-sm">Name:</p>
        <input placeholder="My Runner" bind:value={nameValue} class="w-full h-8 border-[1px] border-zinc-200 dark:border-zinc-700 rounded-lg p-2 bg-zinc-50 dark:bg-zinc-800" />
      </div>
      <div>
        <p class="text-sm">Address:</p>
        <input placeholder="https://example.com:56088" bind:value={addressValue} type="url" class="w-full h-8 border-[1px] border-zinc-200 dark:border-zinc-700 rounded-lg p-2 bg-zinc-50 dark:bg-zinc-800" />
      </div>
    </div>
    <div class="flex flex-row gap-4 px-5 pb-5">
      <button
        class="w-full h-12 rounded-lg text-zinc-600 dark:text-zinc-300 bg-zinc-200 dark:bg-zinc-700 hover:bg-zinc-300 hover:dark:bg-zinc-600 active:scale-95 transition-all duration-100 cursor-default"
        on:click={closeModal}
      >
        Cancel
      </button>
      {#if canConnect && !processing}
        <button
          class="flex flex-row w-full h-12 justify-center items-center rounded-lg text-green-600 dark:text-emerald-400 bg-green-200 dark:bg-emerald-900 hover:bg-green-300 hover:dark:bg-emerald-800 active:scale-95 transition-all duration-100 cursor-default"
          on:click={connectRunner}
        >
          Connect
        </button>
      {:else}
        {#if !processing}
          <button
            class="flex flex-row w-full h-12 justify-center items-center rounded-lg text-zinc-400 dark:text-zinc-400 bg-zinc-100 dark:bg-zinc-900 cursor-default"
          >
            Connect
          </button>
        {:else}
          <button
            class="flex flex-row w-full h-12 justify-center items-center rounded-lg text-zinc-400 dark:text-zinc-400 bg-zinc-100 dark:bg-zinc-900 cursor-default"
          >
            <Icon icon="svg-spinners:3-dots-fade" class="w-6 h-6" />
          </button>
        {/if}
      {/if}
    </div>
  </div>
</div>
