<script lang="ts">
  export let disabled: boolean = false;
  export let forceActive: boolean = false;
  export let onClick: () => void = () => {};
  let className: string = "";

  export { className as class };

  let buttonClasses: string = "";

  $: if (!disabled) {
    buttonClasses = "hover:bg-zinc-200 dark:hover:bg-zinc-800 active:bg-zinc-300 active:scale-[97%] dark:active:bg-zinc-700 transition-all duration-75";
  }

  $: if (forceActive) {
    buttonClasses = "bg-zinc-200 dark:bg-zinc-800";
  }
</script>

<button
  class="flex flex-row w-full h-8 rounded-md items-center text-left p-2 gap-1 text-zinc-700 dark:text-zinc-400 transition-all duration-75 cursor-default {buttonClasses} {className}"
  on:click={() => {if (!disabled && !forceActive) {onClick()}}}
>
  <slot />
</button>
