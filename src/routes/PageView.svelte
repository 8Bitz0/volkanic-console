<script lang="ts">
  import Icon from "@iconify/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-shell";

  import InstanceOverview from "./(instance)/InstanceOverview.svelte";
  import NewInstance from "./(instance)/NewInstance.svelte";
  import type { AppState } from "../scripts/state";

  export let state: AppState;

  let appVersion: string;

  let additionalClasses: string = "";

  $: if (state.titleBarEnabled) {
    additionalClasses = "rounded-tl-xl";
  } else {
    additionalClasses = "";
  }

  invoke("app_version").then((version) => {
    appVersion = version as string;
  });
</script>

<div class="flex flex-col w-full h-full bg-zinc-50 dark:bg-[#121214] border-l-[1px] border-t-[1px] border-zinc-200 dark:border-zinc-700 {additionalClasses}">
  {#if state.view.type !== "home"}
    <div class="absolute w-[512px]">
      <div class="flex flex-row text-sm items-center gap-1.5 px-4 py-2 text-zinc-700 dark:text-zinc-300">
        <button on:click={() => {state.view = { type: "home" }}} class="flex flex-row items-center justify-center rounded-md px-2 py-1 group hover:bg-zinc-200 dark:hover:bg-zinc-800 active:scale-95 transition-all duration-100 cursor-default">
          <p class="group-hover:underline">Root</p>
        </button>
        {#each state.pageViewPath as segment}
          <p class="font-bold">⁄</p>
          {#if segment.view !== null }
            <button class="flex flex-row items-center justify-center rounded-md px-2 py-1 group hover:bg-zinc-200 dark:hover:bg-zinc-800 active:scale-95 transition-all duration-100 cursor-default">
              <p class="group-hover:underline">{segment.name}</p>
            </button>
          {:else}
            <div class="flex flex-row items-center justify-center rounded-md px-2 py-1 cursor-default">
              <p>{segment.name}</p>
            </div>
          {/if}
        {/each}
      </div>
      <div class="w-full h-[1px] bg-gradient-to-r from-zinc-300 dark:from-zinc-700 to-transparent" />
    </div>
    <div class="h-12" />
  {/if}
  {#if state.view.type === "home"}
    <div class="flex flex-col w-full h-full items-center gap-1 p-4 font-medium text-zinc-600 dark:text-zinc-400">
      <div class="flex-1 flex flex-col items-center justify-center">
        <h1 class="text-xl">Welcome to Volkanic Console!</h1>
        <h2 class="text-md opacity-80">Use the sidebar to get started</h2>
      </div>
      <div class="flex flex-row self-end items-center justify-center gap-2">
        <p class="text-xs">v{appVersion}</p>
        <div class="w-[1px] h-full bg-gradient-to-b from-transparent via-zinc-400 dark:via-zinc-600 to-transparent" />
        <button on:click={() => open('https://github.com/8Bitz0/volkanic-console')} class="hover:opacity-80 active:scale-95 transition-all">
          <Icon icon="akar-icons:github-fill" class="w-6 h-6" />
        </button>
      </div>
    </div>
  {:else if state.view.type === "instance-overview"}
    <InstanceOverview bind:state instanceRef={[state.view.runner, state.view.instance]} />
  {:else if state.view.type === "new-instance"}
    <NewInstance bind:state bind:runner={state.view.runner} />
  {/if}
</div>
