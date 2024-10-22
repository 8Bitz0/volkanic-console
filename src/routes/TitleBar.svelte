<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let os: Promise<string> = invoke("host_platform");

  let enabled = false;
  let height: number = 0;
  let additionalClasses: string = "";
  let additionalLogoClasses: string = "";
  let additionalIconClasses: string = "";
  let additionalTextClasses: string = "";

  os.then((os) => {
    if (os === "macos") {
      enabled = true;
      height = 26;
      additionalLogoClasses = additionalLogoClasses + "gap-[2px]";
      additionalIconClasses = additionalIconClasses + "w-[15px] h-[15px]";
      additionalTextClasses = additionalTextClasses + "text-[13px]";
    }
  });
</script>

<div class="bg-zinc-100 dark:bg-zinc-900 overflow-hidden">
  {#if enabled}
    <div data-tauri-drag-region class="absolute flex flex-row w-full justify-end right-0 translate-x-5 overflow-hidden" style="height: {height}px;">
      <div data-tauri-drag-region class="w-64 h-full blur-[128px] bg-green-500" />
    </div>
    <div data-tauri-drag-region class="{additionalClasses}" style="height: {height}px;">
      <div data-tauri-drag-region class="flex flex-row w-full h-full items-center justify-center {additionalLogoClasses}">
        <img
          data-tauri-drag-region
          src="/icons/icon-128x128.png"
          alt="Logo"
          class="{additionalIconClasses}"
        />
        <h1 data-tauri-drag-region class="text-zinc-600 dark:text-zinc-400 tracking-tight font-extrabold font-windowTitle {additionalTextClasses}">Console</h1>
      </div>
    </div>
  {/if}
</div>
