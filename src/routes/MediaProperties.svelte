<script>
  import {
    appState,
    startModal,
    openMedia,
    setSelectedFile,
  } from "./utils/state.svelte.js";
  import { invoke } from "@tauri-apps/api/core";
  import VideoProperties from "./Properties/VideoProperties.svelte";
  import AudioProperties from "./Properties/AudioProperties.svelte";
  import SubtitleProperties from "./Properties/SubtitleProperties.svelte";
  import Svg from "./utils/Svg.svelte";

  let selectedFile = $derived(appState.selected_file);
  let changes = $derived(appState.pendingChanges);

  let fileData = $state(null);
  let loading = $state(false);

  $effect.pre(() => {
    if (!selectedFile) {
      fileData = null;
      return;
    }

    const cachedData = appState.media_properties[selectedFile.path];

    if (cachedData) {
      fileData = cachedData;
      loading = false;
    } else {
      loading = true;
      invoke("get_media_streams", { path: selectedFile.path })
        .then((res) => {
          fileData = res;
          appState.media_properties[selectedFile.path] = res;
          loading = false;
        })
        .catch((err) => {
          console.error(err);
          loading = false;
        });
    }
  });

  async function handleSave(saveAs) {
    if (!saveAs) {
      const answer = await startModal(
        "Ask",
        "Are you sure you want to write changes to the file?",
        { cancel: "Cancel", agree: "Yes" },
      );
      if (!answer) return;
    }

    const targetPath = appState.selected_file.path;

    try {
      startModal("ProgressBar", "Saving file...");
      await invoke("save_media_props", {
        filePath: targetPath,
        changes: $state.snapshot(appState.pendingChanges),
        saveAs,
      });

      appState.pendingChanges = {};
      appState.media_properties[targetPath] = null;
      fileData = null;

      await openMedia(appState.enviroment.isFile, true);
      setSelectedFile(appState.selected_file.path, appState.selected_file.name);
    } catch (error) {
      appState.modal = null;
      console.error(error);
      const again = await startModal("Ask", `File could not be saved!`, {
        cancel: "Cancel",
        agree: "Try Again",
      });

      if (again) {
        handleSave();
      }
    }
  }
</script>

<div
  class="media_properties"
  class:is_selected={selectedFile}
  class:not_selected={!selectedFile}
>
  {#if selectedFile && !$effect.pending()}
    <div class="header">
      <h1 class="file_name">{selectedFile.name}</h1>
      <div
        class="button_cont"
        class:hidden={Object.keys(appState.pendingChanges).length === 0}
      >
        <button
          class="discard"
          onclick={() => {
            loading = true;
            appState.media_properties = {};
            appState.pendingChanges = {};
          }}><Svg name="mop" color="rgb(186, 197, 211)" /></button
        >
        <button class="save" onclick={() => handleSave(false)}
          ><Svg name="save" color="rgb(186, 197, 211)" /><span>Save</span
          ></button
        >
        <button class="save_as" onclick={() => handleSave(true)}
          ><Svg name="save_as" color="rgb(186, 197, 211)" /><span>Save As</span
          ></button
        >
      </div>
    </div>
    {#if !loading && fileData}
      <VideoProperties />
      <AudioProperties />
      <SubtitleProperties />
      {JSON.stringify(fileData)}
      <br /><br />
      {JSON.stringify(changes)}
    {:else}
      <div>Loading...</div>
    {/if}
  {:else}
    <div class="no_file">No file selected</div>
  {/if}
</div>

<style>
  .media_properties {
    width: 100%;
    height: 100%;
    overflow-y: scroll;
    &::-webkit-scrollbar {
      display: none;
    }
  }

  .is_selected {
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
  }

  .not_selected {
    display: grid;
    place-items: center;
  }

  .no_file {
    background-color: rgb(29, 41, 61);
    border: 1px solid rgb(69, 85, 108);
    color: rgb(186, 197, 211);
    border-radius: 10px;
    padding: 10px 20px;
    font-size: 16px;
    font-weight: 600;
    text-align: center;
  }

  .header {
    height: fit-content;
    width: calc(100% - 20px);
    background-color: rgb(29, 41, 61);
    border: 1px solid rgb(69, 85, 108);
    border-radius: 10px;
    margin: 0 10px 10px 10px;
    padding: 10px 10px 10px 20px;

    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 20px;

    h1 {
      height: fit-content;
      font-size: 20px;
      font-weight: 700;
      background: linear-gradient(
        to right,
        rgb(83, 204, 255),
        rgb(5, 255, 117)
      );
      -webkit-background-clip: text;
      background-clip: text;
      color: transparent;
      text-align: center;
      width: fit-content;
    }
  }

  .button_cont {
    display: flex;
    flex-direction: row;
    gap: 20px;
    justify-content: center;
    align-items: center;
    width: max-content;
  }

  .hidden {
    display: none;
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
    }
    &:active {
      transform: scale(97%);
    }
  }
</style>
