<script>
  import { invoke } from "@tauri-apps/api/core";
  import { appState } from "./utils/state.svelte.js";
  import Svg from "./utils/Svg.svelte";
  import MediaItem from "./MediaItem.svelte";

  let files = $state([]);
  let enviroment = $state({ name: undefined });

  async function openMedia(isFile) {
    appState.selected_file = undefined;
    appState.media_properties = {};

    const ret = await invoke("select_media", { isFile });
    files = [];
    enviroment = { name: undefined };
    if (isFile) {
      files.push(ret);
      enviroment = { name: "New Project" };
    } else {
      files = ret.children;
      enviroment = { name: ret.name };
    }
  }
</script>

<div id="file_explorer">
  <div id="buttons">
    <button onclick={() => openMedia(true)}
      ><Svg name="file_open" color="rgb(186, 197, 211)" /><span>Open File</span
      ></button
    >
    <button onclick={() => openMedia(false)}
      ><Svg name="folder_open_outline" color="rgb(186, 197, 211)" /><span
        >Open Folder</span
      ></button
    >
  </div>
  <div class="env">{enviroment.name}</div>
  <div id="added_paths">
    <MediaItem contents={files} />
  </div>
</div>

<style>
  #file_explorer {
    width: 100%;
    height: calc(100vh - 20px);
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
    padding: 0 0 5px 0;
    overflow-y: scroll !important;
    &::-webkit-scrollbar {
      display: none;
    }
  }

  .env {
    width: 100%;
    padding-left: 10px;
    background-color: rgb(19, 28, 46);
    border-bottom: 1px solid rgb(69, 85, 108);
    color: rgb(186, 197, 211);
    font-size: 13px;
    font-weight: bold;
  }
</style>
