<script>
  import { appState } from "./state.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Svg from "./Svg.svelte";

  let selectedFile = $derived(appState.selected_file);

  let fileData = $state(null);
  let loading = $state(false);
  let isVideoPropOpen = $state(false);
  let isAudioPropOpen = $state(false);
  let isSubsPropOpen = $state(true);

  $effect(() => {
    if (!selectedFile) {
      fileData = null;
      return;
    }

    if (appState.media_properties[selectedFile]) {
      fileData = appState.media_properties[selectedFile];
      return;
    }

    loading = true;

    invoke("get_media_streams", { path: selectedFile.path })
      .then((res) => {
        fileData = res;
        appState.media_properties[selectedFile] = fileData;
        loading = false;
      })
      .catch((err) => {
        console.error(err);
        loading = false;
      });
  });
</script>

<div
  class="media_properties"
  class:is_selected={selectedFile}
  class:not_selected={!selectedFile}
>
  {#if selectedFile}
    <div class="file_name">{selectedFile.name}</div>
    {#if !loading && fileData}
      <div class="video properties_cont">
        <button
          class="cont_title"
          onclick={() => (isVideoPropOpen = !isVideoPropOpen)}
        >
          <div class="chevron" class:open_chevron={isVideoPropOpen}>
            <Svg name="chevron_left" size="30" color="rgb(186, 197, 211)" />
          </div>
          <span>Video Properties</span>
        </button>
      </div>
      <div class="audio properties_cont">
        <button
          class="cont_title"
          onclick={() => (isAudioPropOpen = !isAudioPropOpen)}
          ><div class="chevron" class:open_chevron={isAudioPropOpen}>
            <Svg name="chevron_left" size="30" color="rgb(186, 197, 211)" />
          </div>
          <span>Audio Properties</span></button
        >
      </div>
      <div class="subtitles properties_cont">
        <button
          class="cont_title"
          onclick={() => (isSubsPropOpen = !isSubsPropOpen)}
          ><div class="chevron" class:open_chevron={isSubsPropOpen}>
            <Svg name="chevron_left" size="30" color="rgb(186, 197, 211)" />
          </div>
          <span>Subtitle Properties</span></button
        >
        {#if isSubsPropOpen}
          <table>
            <thead
              ><tr
                ><th>Language</th><th>Title</th><th>Default</th><th>Forced</th
                ></tr
              ></thead
            >
            <tbody>
              {#each fileData.streams.filter((x) => x.codec_type === "subtitle") as stream}
                <tr>
                  <td>{stream.tags.language}</td>
                  <td>{stream.tags.title ?? "-"}</td>
                  <td>{stream.disposition.default === 1 ? "Yes" : "No"}</td>
                  <td>{stream.disposition.forced === 1 ? "Yes" : "No"}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        {/if}
      </div>
    {:else}
      <div>Loading...</div>
    {/if}
    <button class="save">Save</button>
    <button
      class="discard"
      onclick={() => {
        appState.media_properties = {};
      }}>Discard Changes</button
    >
  {:else}
    <div class="no_file">No file selected!</div>
  {/if}
</div>

<style>
  .media_properties {
    width: 100%;
    height: 100%;
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

  .file_name {
    width: calc(100% - 20px);
    background-color: rgb(29, 41, 61);
    border: 1px solid rgb(69, 85, 108);
    border-radius: 10px;
    margin: 10px;
    padding: 10px 10px;

    font-size: 15px;
    font-weight: 600;
    text-align: center;
  }

  .properties_cont {
    width: calc(100% - 20px);
    background-color: rgb(29, 41, 61);
    border: 1px solid rgb(69, 85, 108);
    border-radius: 10px;
    margin: 10px 10px;
    padding: 10px 10px;

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
