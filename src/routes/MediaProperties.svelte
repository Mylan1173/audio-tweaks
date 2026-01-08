<script>
  import { appState } from "./state.svelte";
</script>

<div id="media_properties">
  {@html appState.selected_file ?? "<p>No selected media!</p>"}
</div>

<style>
  #media_properties {
    width: 100%;
    height: 100%;
    display: grid;
    place-items: center;
  }
</style>
