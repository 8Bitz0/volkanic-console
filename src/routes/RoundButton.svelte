<script lang="ts">
  export let onClick: () => void = () => {};
  let className: string = "";

  export { className as class };
</script>

<button
  class="flex flex-row rounded-full items-center justify-center hover:scale-105 hover:transition-all hover:duration-200 active:scale-95 active:transition-all active:duration-200 {className}"
  on:click={onClick}
>
  <slot />
</button>
