<script>
  import { appState, openMedia } from "./utils/state.svelte.js";
  import Svg from "./utils/Svg.svelte";
  import MediaItem from "./MediaItem.svelte";
</script>

<div id="file-explorer">
  <div id="buttons">
    <button onclick={() => openMedia(true)}>
      <Svg name="open_file" color="rgb(186, 197, 211)" /><span>Open File</span>
    </button>
    <button onclick={() => openMedia(false)}>
      <Svg name="open_folder" color="rgb(186, 197, 211)" /><span
        >Open Folder</span
      >
    </button>
  </div>

  <div class="env">File Browser</div>

  <div id="added-paths">
    <MediaItem contents={appState.explorer} />
  </div>
</div>

<style>
  #file-explorer {
    width: 100%;
    height: calc(100vh - 20px);
    border: 1px solid var(--border);
    border-radius: 10px;
    background-color: var(--bg-light);
    display: flex;
    flex-direction: column;
  }

  #buttons {
    width: 100%;
    height: 75px;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 20px;
    border-bottom: 1px solid var(--border);
  }

  button {
    background-color: transparent;
    border: 1px solid var(--border);
    padding: 5px 10px;
    color: var(--text-light);
    border-radius: 7px;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: 10px;

    box-shadow: none;
    transition: all ease-in-out 100ms;

    & > span {
      font-weight: 600;
      font-size: 16px;
    }

    &:hover {
      cursor: pointer;
      border-color: var(--text-light);
      box-shadow: var(--text-light) 0px 0px 2px;
    }
  }

  #added-paths {
    display: flex;
    flex-direction: column;
    padding: 0 0 5px 0;
    overflow-y: scroll !important;
    &::-webkit-scrollbar {
      display: none;
    }
  }

  .env {
    width: 100%;
    padding-left: 10px;
    background-color: var(--bg-dark);
    border-bottom: 1px solid var(--border);
    color: var(--text-light);
    font-size: 13px;
    font-weight: bold;
  }
</style>
