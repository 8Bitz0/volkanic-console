<script lang="ts">
  import "../app.css";

  import InstanceBar from "./InstanceBar.svelte";

  import type { AppState } from "../scripts/State";

  let state: AppState = {
    runners: [],
    instances: [],
    selectedInstance: ""
  };
</script>

<div class="absolute w-full h-full overflow-hidden bg-white dark:bg-neutral-950 text-black dark:text-gray-50">
  <div class="flex flex-row h-full">
    <InstanceBar bind:state />
  </div>
</div>
