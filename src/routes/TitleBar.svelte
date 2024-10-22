<script lang="ts">
  import Icon from "@iconify/svelte";
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from "@tauri-apps/api/core";

  import type { AppState } from "../scripts/State";

  export let state: AppState;

  let os: Promise<string> = invoke("host_platform");

  const window = getCurrentWindow();

  let enabled = false;
  let height: number = 0;
  let buttonStyle: string = "";
  let additionalClasses: string = "";
  let additionalLogoClasses: string = "";
  let additionalIconClasses: string = "";
  let additionalTextClasses: string = "";

  os.then((os) => {
    if (os === "macos") {
      enabled = true;
      height = 26;
      additionalLogoClasses = additionalLogoClasses + "gap-[2px] justify-center";
      additionalIconClasses = additionalIconClasses + "w-[15px] h-[15px]";
      additionalTextClasses = additionalTextClasses + "text-[13px] font-extrabold";

      state.titleBarEnabled = true;
    } else if (os === "windows") {
      enabled = true;
      buttonStyle = "windows"
      height = 32;
      additionalLogoClasses = additionalLogoClasses + "gap-[2px] px-2";
      additionalIconClasses = additionalIconClasses + "w-[15px] h-[15px]";
      additionalTextClasses = additionalTextClasses + "text-[13px] font-medium";

      state.titleBarEnabled = true;
    } else {
      state.titleBarEnabled = false;
    }
  });

  async function minimizeWindow() {
    window.minimize();
  }

  async function maximizeWindow() {
    window.toggleMaximize();
  }

  async function closeWindow() {
    window.close();
  }

  let maximized: boolean = false;

  window.isMaximized().then((isMaximized) => maximized = isMaximized);

  window.onResized(({payload: size}) => {
    window.isMaximized().then((isMaximized) => maximized = isMaximized);
  });
</script>

<div class="bg-zinc-100 dark:bg-zinc-900 overflow-hidden" style="height: {height}px;">
  {#if enabled}
    <div data-tauri-drag-region class="absolute flex flex-row w-full justify-end right-0 translate-x-5 overflow-hidden" style="height: {height - 2}px;">
      <div data-tauri-drag-region class="w-64 h-full blur-[128px] bg-green-500" />
    </div>
    {#if buttonStyle == "windows"}
      <div class="absolute flex flex-row right-0" style="height: {height}px;">
        <button
          class="flex flex-col w-12 z-40 items-center justify-center bg-black dark:bg-white bg-opacity-0 dark:bg-opacity-0 hover:bg-opacity-10 transition-all duration-75 cursor-default"
          style="height: {height - 2}px;"
          on:click={minimizeWindow}
        >
          <Icon icon="fluent:minimize-12-regular" class="w-4" />
        </button>
        <button
          class="flex flex-col w-12 z-40 items-center justify-center bg-black dark:bg-white bg-opacity-0 dark:bg-opacity-0 hover:bg-opacity-10 transition-all duration-75 cursor-default"
          style="height: {height - 2}px;"

          on:click={maximizeWindow}
        >
          {#if maximized}
            <Icon icon="fluent:restore-16-regular" class="w-4" />
          {:else}
            <Icon icon="fluent:maximize-16-regular" class="w-4" />
          {/if}
        </button>
        <button
          class="flex flex-col w-12 z-40 group items-center justify-center hover:bg-red-500 transition-all duration-75 cursor-default"
          style="height: {height - 2}px;"
          on:click={closeWindow}
        >
          <Icon icon="fluent:dismiss-12-regular" class="w-4 group-hover:text-white" />
        </button>
      </div>
    {/if}
    <div data-tauri-drag-region class="{additionalClasses}" style="height: {height}px;">
      <div data-tauri-drag-region class="flex flex-row w-full h-full items-center {additionalLogoClasses}">
        <img
          data-tauri-drag-region
          src="/icons/icon-128x128.png"
          alt="Logo"
          class="{additionalIconClasses}"
        />
        <h1 data-tauri-drag-region class="text-zinc-600 dark:text-zinc-400 tracking-tight font-windowTitle {additionalTextClasses}">Console</h1>
      </div>
    </div>
  {/if}
</div>
