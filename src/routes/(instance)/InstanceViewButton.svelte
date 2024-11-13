<script lang="ts">
  export let onClick: () => void = () => {};
  export let danger: boolean = false;
  export let disabled: boolean = false;
</script>

{#if !danger}
  {#if !disabled}
    <button on:click={onClick} class="rounded-lg px-2.5 py-1.5 border-[1px] text-sm font-semibold border-zinc-300 dark:border-zinc-600 text-zinc-800 dark:text-zinc-200 bg-zinc-200 dark:bg-zinc-800 bg-opacity-75 hover:border-zinc-300 dark:hover:border-zinc-600 hover:text-zinc-900 dark:hover:text-zinc-100 hover:bg-zinc-300 dark:hover:bg-zinc-700 hover:bg-opacity-80 active:border-zinc-400 dark:active:border-zinc-500 active:border-opacity-80 active:text-zinc-900 dark:active:text-zinc-100 active:bg-zinc-300 dark:active:bg-zinc-600 transition-all duration-100 cursor-default">
      <slot />
    </button>
  {:else}
    <button class="rounded-lg px-2.5 py-1.5 border-[1px] text-sm font-semibold border-zinc-200 dark:border-zinc-800 text-zinc-400 dark:text-zinc-500 bg-zinc-100 dark:bg-zinc-800 bg-opacity-50 dark:bg-opacity-50 cursor-not-allowed">
      <slot />
    </button>
  {/if}
{:else}
  <button on:click={onClick} class="rounded-lg px-2.5 py-1.5 border-[1px] text-sm font-semibold border-zinc-300 dark:border-zinc-600 text-red-600 dark:text-red-400 bg-zinc-200 dark:bg-zinc-800 bg-opacity-75 hover:border-red-700 dark:hover:border-red-500 hover:text-zinc-50 hover:bg-red-600 dark:hover:bg-red-700 active:text-zinc-50 active:border-red-800 dark:active:border-red-500 active:bg-red-700 dark:active:bg-red-600 transition-all duration-100 cursor-default">
    <slot />
  </button>
{/if}
