<script lang="ts">
  import Icon from "@iconify/svelte";
  import { sineInOut } from "svelte/easing";
  import { blur, scale } from "svelte/transition";

  import type { AppState } from "../scripts/State";

  export let state: AppState;

  function closeModal() {
    state.newInstanceModal = false;
  }
</script>

<div transition:blur={{ duration: 200 }} class="absolute flex flex-col w-full h-full items-center justify-center bg-[#00000020] backdrop-blur-md">
  <div transition:scale={{ duration: 200, start: 0.5, opacity: 0, easing: sineInOut }} class="flex flex-col w-[384px] border-[1px] border-zinc-300 dark:border-zinc-700 rounded-2xl bg-zinc-50 dark:bg-zinc-800">
    <div class="flex flex-col w-full h-full gap-3 p-6">
      <h1 class="text-xl">New Instance</h1>
      <div>
        <p class="text-sm">Name:</p>
        <input placeholder="My Instance" class="w-full h-8 border-[1px] border-zinc-200 dark:border-zinc-700 rounded-lg p-2 bg-zinc-50 dark:bg-zinc-800" />
      </div>
      <div class="flex flex-col w-full rounded-xl border border-zinc-200 dark:border-zinc-700">
        <div class="flex flex-row w-full rounded-xl gap-[1px] overflow-hidden bg-zinc-200 dark:bg-zinc-700">
          <button class="flex flex-col w-full items-center justify-center p-4 bg-zinc-50 dark:bg-zinc-800 hover:bg-zinc-100 dark:hover:bg-zinc-700 transition-all duration-100 cursor-default">
            <Icon icon="mdi:cube-outline" class="w-4 h-4" />
            <h1 class="text-center text-sm">Volkanic Construct</h1>
          </button>
          <button class="flex flex-col w-full items-center justify-center p-4 bg-zinc-50 dark:bg-zinc-800 hover:bg-zinc-100 dark:hover:bg-zinc-700 transition-all duration-100 cursor-default">
            <Icon icon="mdi:terminal" class="w-4 h-4" />
            <h1 class="text-center text-sm">Shell Script</h1>
          </button>
        </div>
      </div>
    </div>
    <div class="flex flex-row gap-4 px-5 pb-5">
      <button
        class="w-full h-12 rounded-lg text-zinc-600 dark:text-zinc-300 bg-zinc-200 dark:bg-zinc-700 hover:bg-zinc-300 hover:dark:bg-zinc-600 active:scale-95 transition-all duration-100"
        on:click={closeModal}
      >
        Cancel
      </button>
      <button class="w-full h-12 rounded-lg text-green-600 dark:text-green-400 bg-green-200 dark:bg-green-900 hover:bg-green-300 hover:dark:bg-green-800 active:scale-95 transition-all duration-100">Create</button>
    </div>
  </div>
</div>
