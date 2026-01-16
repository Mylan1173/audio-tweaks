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

  table {
    border-collapse: collapse;
    margin: 10px;

    thead > tr {
      height: 40px;
      th {
        font-size: 15px;
        font-weight: 600;
        color: rgb(186, 197, 211);
      }
    }
    tbody {
      tr {
        margin-top: 5px;
        height: 40px;
        td {
          font-size: 15px;
          font-weight: 600;
        }

        td:nth-child(1) {
          background-color: rgb(19, 28, 46);
        }
        td:nth-child(2) {
          text-transform: uppercase;
          background-color: rgb(19, 28, 46);
        }
        td:nth-child(3),
        td:nth-child(4) {
          width: 10%;
        }
        td:nth-child(5) {
          width: 10%;
        }
        &:hover .delete {
          opacity: 1;
        }
      }
    }
    tr:nth-child(1) td:nth-child(1) {
      border-top-left-radius: 10px;
    }
    tr:nth-child(4) td:nth-child(1) {
      border-bottom-left-radius: 10px;
    }
  }

  .checkbox {
    cursor: pointer;
    background-color: transparent;
    border: none;
  }

  .delete {
    opacity: 0;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    background-color: rgb(202, 0, 0);
    border-radius: 5px;
    margin: 2px auto;
    padding: 5px 10px;
    gap: 10px;
    border: none;
    transition: 100ms all ease-in-out;
    cursor: pointer;

    span {
      font-size: 15px;
      font-weight: 600;
      color: white;
    }
    &:active {
      transform: scale(97%);
    }
  }
</style>
