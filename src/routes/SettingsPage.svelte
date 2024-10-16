<script lang="ts">
  import Icon from "@iconify/svelte";

  import RoundButton from "./RoundButton.svelte";

  import { type AppState } from "../scripts/State";

  export let state: AppState;

  function toggleSettingsMenu() {
    state.settingsOpen = !state.settingsOpen;
  }
</script>

<div class="w-full h-full bg-zinc-50 dark:bg-zinc-950 p-6">
  <RoundButton
    class="w-16 h-16 bg-zinc-200 dark:bg-zinc-800"
    onClick={toggleSettingsMenu}
  >
    <Icon icon="mdi:arrow-left" class="w-8 h-8" />
  </RoundButton>
</div>
