<script>
  import { appState } from "../utils/state.svelte.js";
  import Svg from "../utils/Svg.svelte";

  let selectedFile = $derived(appState.selected_file);
  let fileData = $derived(appState.media_properties[selectedFile.path]);
  let isAudioPropOpen = $state(false);
</script>

<div class="audio properties_cont">
  <button
    class="cont_title"
    onclick={() => (isAudioPropOpen = !isAudioPropOpen)}
    ><div class="chevron" class:open_chevron={isAudioPropOpen}>
      <Svg name="chevron_left" size={30} color="rgb(186, 197, 211)" />
    </div>
    <span>Audio Properties</span></button
  >
</div>

<style>
  .properties_cont {
    width: calc(100% - 40px);
    background-color: rgb(29, 41, 61);
    border: 1px solid rgb(69, 85, 108);
    border-radius: 10px;
    margin: 10px 20px;
    padding: 10px;

    font-size: 15px;
    font-weight: 600;
    text-align: center;
    display: flex;
    flex-direction: column;

    .cont_title {
      background-color: inherit;
      border: none;
      display: flex;
      flex-direction: row;
      align-items: center;

      &:hover {
        cursor: pointer;
      }

      .chevron {
        display: grid;
        place-items: center;
        transition: 100ms all ease-in-out;

        transform: rotate(0);
      }

      span {
        color: rgb(186, 197, 211);
        font-size: 15px;
        font-weight: 600;
      }
    }
  }

  .open_chevron {
    transform: rotate(90deg) !important;
  }
</style>
