<script lang="ts">
  export let active: boolean = true;
  export let onClick: () => void = () => {};
  let className: string = "";

  export { className as class };

  let buttonClasses: string = "";

  $: if (active) {
    buttonClasses = "hover:bg-zinc-200 dark:hover:bg-zinc-800 hover:transition-all hover:duration-100 active:bg-zinc-300 active:scale-[97%] dark:active:bg-zinc-700 active:transition-all active:duration-75"
  } else {
    buttonClasses = "";
  }
</script>

<button
  class="flex flex-row w-full h-8 rounded-md items-center text-left p-2 gap-1 text-zinc-700 dark:text-zinc-400 transition-all duration-75 cursor-default {buttonClasses} {className}"
  on:click={onClick}
>
  <slot />
</button>
