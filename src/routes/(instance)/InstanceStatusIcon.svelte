<script lang="ts">
  import Icon from "@iconify/svelte";

  import type { InstanceStatus } from "../../scripts/instance";

  let className: string = "";
  export let status: InstanceStatus;

  export { className as class };
</script>

<div class="relative flex flex-row items-center justify-center {className}">
  {#if status === "inactive"}
    <div class="w-1 h-1 rounded-full bg-zinc-400 dark:bg-zinc-500"></div>
  {:else if status === "running"}
    <div class="w-1 h-1 rounded-full bg-green-400 dark:bg-green-500"></div>
  {:else if typeof status === "object" && "creating" in status}
    {#if status.creating < 100}
      <div class="relative w-3.5 h-3.5">
        <svg class="w-full h-full" viewBox="0 0 100 100">
          <!-- Background circle -->
          <circle
            class="text-zinc-200 dark:text-zinc-800 stroke-current"
            stroke-width="15"
            cx="50"
            cy="50"
            r="40"
            fill="transparent"
          ></circle>
          <!-- Progress circle -->
          <circle
            class="text-emerald-400 dark:text-emerald-600 radial-progress-circle stroke-current"
            stroke-width="15"
            stroke-linecap="round"
            cx="50"
            cy="50"
            r="40"
            fill="transparent"
            stroke-dasharray="251.2" 
            stroke-dashoffset="calc(251.2px - (251.2px * {status.creating}) / 100)"
          ></circle>
        </svg>
      </div>
    {:else}
      <Icon icon="mdi:check" class="w-4 h-4 text-green-900 dark:text-green-300" />
    {/if}
  {:else if status === "starting"}
    <div class="w-1 h-1 rounded-full bg-yellow-400 dark:bg-yellow-500"></div>
  {:else if status === "stopping"}
    <div class="w-1 h-1 rounded-full bg-yellow-400 dark:bg-yellow-500"></div>
  {:else if status === "deleting"}
    <Icon icon="svg-spinners:90-ring-with-bg" class="w-4 h-4 text-red-500 dark:text-red-400" />
  {/if}
</div>
