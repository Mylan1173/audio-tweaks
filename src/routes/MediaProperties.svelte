<script>
  import {
    appState,
    startModal,
    closeModal,
    openMedia,
    loadMediaProperties,
    loadContentMediaProperties,
  } from "./utils/state.svelte.js";
  import { invoke } from "@tauri-apps/api/core";
  import VideoProperties from "./Properties/VideoProperties.svelte";
  import AudioProperties from "./Properties/AudioProperties.svelte";
  import SubtitleProperties from "./Properties/SubtitleProperties.svelte";
  import PropertiesComparer from "./Properties/PropertiesComparer.svelte";
  import Svg from "./utils/Svg.svelte";

  let selectedMedia = $derived(appState.selectedMedia);
  let loading = $state(false);

  $effect.pre(() => {
    if (!selectedMedia) {
      appState.data.reset();
      return;
    }

    loading = true;

    if (selectedMedia.mediaType === "file") {
      appState.data.reset();
      loadMediaProperties(selectedMedia.mediaPath)
        .then((res) => {
          appState.data.init(res);
          appState.data.fileName = selectedMedia.mediaName;
        })
        .finally(() => {
          loading = false;
        });
    } else if (selectedMedia.mediaType === "folder") {
      loadContentMediaProperties().then(() => (loading = false));
    }
  });

  async function handleSave(saveAs) {
    if (!saveAs) {
      const answer = await startModal(
        "Ask",
        "Are you sure you want to write changes to the file(s)?",
        { cancel: "Cancel", agree: "Yes" },
      );
      if (!answer) return;
    }

    try {
      if (selectedMedia.mediaType === "file") {
        startModal("Console", "Saving file...");
        await invoke("save_media_props", {
          filePath: selectedMedia.mediaPath,
          changes: appState.data.getPendingChanges(),
          saveAs,
        });
      } else if (selectedMedia.mediaType === "folder") {
        const batchPayload = appState.contentData.getBatchPayload();

        for (let i = 0; i < batchPayload.length; i++) {
          startModal(
            "Console",
            `Processing file ${i + 1} of ${batchPayload.length}...`,
          );
          await invoke("save_media_props", {
            filePath: batchPayload[i].filePath,
            changes: batchPayload[i].changes,
            saveAs,
          });
        }
      }

      await closeModal();

      appState.data.reset();
      appState.contentData.reset();

      await openMedia(appState.enviroment.isFile, true);
    } catch (error) {
      await closeModal();
      console.error(error);
      const again = await startModal("Ask", `File(s) could not be saved!`, {
        cancel: "Cancel",
        agree: "Try Again",
      });
      if (again) {
        handleSave(saveAs);
      }
    }
  }
</script>

<div
  class="media-properties"
  class:is_selected={selectedMedia}
  class:not_selected={!selectedMedia}
>
  {#if selectedMedia && !$effect.pending()}
    <div class="header">
      <div class="media-type">
        {#if selectedMedia.mediaType === "folder"}
          <Svg name="folder_edit" color="rgb(186, 197, 211)" />
        {:else}
          <Svg name="file_edit" color="rgb(186, 197, 211)" />
        {/if}
      </div>
      <div class="media-details">
        <h1 class="file_name">{selectedMedia.mediaName}</h1>
      </div>

      <div class="button-container">
        {#if selectedMedia.mediaType === "folder"}
          <button class="save" onclick={() => handleSave(false)}>
            <Svg name="save" color="rgb(186, 197, 211)" />
            <span>Apply Profile</span>
          </button>
        {:else}
          <button
            class="save"
            class:hidden={!appState.data.isPendingChanges ||
              appState.data.hasError()}
            onclick={() => handleSave(false)}
          >
            <Svg name="save" color="rgb(186, 197, 211)" />
            <span>Save</span>
          </button>
          <button
            class="save"
            class:hidden={!appState.data.isPendingChanges ||
              appState.data.hasError()}
            onclick={() => handleSave(true)}
          >
            <Svg name="save_as" color="rgb(186, 197, 211)" />
            <span>Save As</span>
          </button>
        {/if}
      </div>
    </div>
    {#if !loading}
      <div class="scroll-container">
        {#if appState.selectedMedia.mediaType === "file" && appState.data.initialized}
          {#if appState.data.isVideo}
            <VideoProperties />
          {/if}
          {#if appState.data.isAudio}
            <AudioProperties />
          {/if}
          {#if appState.data.isSubtitle}
            <SubtitleProperties />
          {/if}
        {:else if appState.selectedMedia.mediaType === "folder" && appState.contentData.initialized}
          <PropertiesComparer />
        {/if}
      </div>
    {:else}
      <div class="loader"></div>
    {/if}
  {:else}
    <div class="no_file">No file selected</div>
  {/if}
</div>

<style>
  .media-properties {
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
    background-color: var(--bg-light);
    border: 1px solid var(--border);
    color: var(--text-light);
    border-radius: 10px;
    padding: 10px 20px;
    font-size: 16px;
    font-weight: 600;
    text-align: center;
  }

  .scroll-container {
    overflow: scroll;
    scrollbar-width: none;
  }

  .header {
    min-height: 58px;
    height: auto;
    width: calc(100% - 20px);
    background-color: var(--bg-light);
    border: 1px solid var(--border);
    border-radius: 10px;
    margin: 0 10px 10px 10px;

    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 20px;

    .media-details {
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

    .media-type {
      width: 58px;
      height: 100%;
      overflow: hidden;
      justify-self: flex-start;
      display: grid;
      place-items: center;
      border-right: 1px solid var(--border);
    }

    .button-container {
      display: flex;
      flex-direction: row;
      gap: 20px;
      justify-content: center;
      align-items: center;
      width: max-content;
      margin-right: 10px;
    }
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
    }
    &:active {
      transform: scale(97%);
    }
  }

  .hidden {
    visibility: hidden;
  }

  .loader {
    margin: auto;
    width: 60px;
    aspect-ratio: 4;
    background: radial-gradient(circle closest-side, #ffffff 90%, #ffffff00) 0 /
      calc(100% / 3) 100% no-repeat;
    animation: l2 1s steps(3) infinite;
  }
  @keyframes l2 {
    to {
      background-position: 150%;
    }
  }
</style>
