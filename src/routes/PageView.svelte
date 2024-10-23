<script lang="ts">
  import type { AppState } from "../scripts/State";

  export let state: AppState;

  let additionalClasses: string = "";

  $: if (state.titleBarEnabled) {
    additionalClasses = "rounded-tl-xl";
  } else {
    additionalClasses = "";
  }
</script>

<div class="w-full h-full bg-zinc-50 dark:bg-zinc-950 border-l-[1px] border-t-[1px] border-zinc-200 dark:border-zinc-700 {additionalClasses}">

</div>