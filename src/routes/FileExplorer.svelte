<script>
  import { invoke } from "@tauri-apps/api/core";
  import { appState } from "./state.svelte.js";
  import Svg from "./Svg.svelte";
  import MediaItem from "./MediaItem.svelte";

  let asd = $state([]);

  async function openMedia(isFile) {
    asd = await invoke("select_media", { isFile });
    //appState.explorer = [...appState.explorer, ret];
  }
</script>

<div id="file_explorer">
  <div id="buttons">
    <button onclick={() => openMedia(true)}
      ><Svg name="file_open" color="rgb(186, 197, 211)" /><span>Add File</span
      ></button
    >
    <button onclick={() => openMedia(false)}
      ><Svg name="folder_open" color="rgb(186, 197, 211)" /><span
        >Add Folder</span
      ></button
    >
  </div>
  <div id="added_paths">
    <MediaItem contents={asd} />
  </div>
</div>

<style>
  #file_explorer {
    max-width: 300px;
    width: 100%;
    height: 100%;
    margin: 10px;
    border: 1px solid rgb(69, 85, 108);
    border-radius: 10px;
    background-color: rgb(29, 41, 61);
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
    border-bottom: 1px solid rgb(69, 85, 108);
  }

  button {
    background-color: transparent;
    border: 1px solid rgb(69, 85, 108);
    padding: 5px 10px;
    color: rgb(186, 197, 211);
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
      border-color: rgb(186, 197, 211);
      box-shadow: rgb(186, 197, 211) 0px 0px 2px;
    }
  }

  #added_paths {
    display: flex;
    flex-direction: column;
    gap: 5px;
    padding: 5px 0px;
    overflow-y: scroll;
    &::-webkit-scrollbar {
      display: none;
    }
  }
</style>
