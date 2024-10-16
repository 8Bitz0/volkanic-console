<script lang="ts">
  export let onClick: () => void = () => {};
  let className: string = "";

  export { className as class };
</script>

<button
  class="flex flex-row w-full h-8 rounded-md items-center text-left p-2 gap-1 text-zinc-700 dark:text-zinc-400 hover:bg-zinc-200 dark:hover:bg-zinc-800 transition-all duration-75 hover:transition-all hover:duration-100 active:bg-zinc-300 active:scale-[97%] dark:active:bg-zinc-700 active:transition-all active:duration-75 {className}"
  on:click={onClick}
>
  <slot />
</button>
