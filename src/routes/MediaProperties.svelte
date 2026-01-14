<script>
  import { appState } from "./state.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Svg from "./Svg.svelte";

  let selectedFile = $derived(appState.selected_file);

  let fileData = $state(null);
  let asd = $state(null);
  let loading = $state(false);
  let isVideoPropOpen = $state(false);
  let isAudioPropOpen = $state(false);
  let isSubsPropOpen = $state(true);

  $effect(() => {
    if (!selectedFile) {
      fileData = null;
      return;
    }

    if (appState.media_properties[selectedFile.path]) {
      fileData = appState.media_properties[selectedFile.path];
      return;
    }

    loading = true;

    invoke("get_media_streams", { path: selectedFile.path })
      .then((res) => {
        fileData = res;
        appState.media_properties[selectedFile.path] = fileData;
        loading = false;
      })
      .catch((err) => {
        console.log(err);
        loading = false;
      });
  });

  function sortSubtitles(a, b) {
    if (a !== b) {
      return a.tags.language.localeCompare(b.tags.language);
    } else {
      return a.tags.title.localeCompare(b.tags.title);
    }
  }

  function changeSelectedSub(index, type) {
    const nonSubStreamCount = fileData.streams.filter(
      (x) => x.codec_type !== "subtitle"
    ).length;
    let lastIndex =
      fileData.streams
        .filter((x) => x.codec_type === "subtitle")
        .findIndex((x) => x.disposition.default === 1) + nonSubStreamCount;
    console.log("Last: " + lastIndex);

    if (type === "default") {
      // At most 1 can be selected
      if (lastIndex === -1) {
        // No selected sub
        fileData.streams[index].disposition[type] = 1;
        console.log(1);
      } else if (lastIndex === index) {
        // Deselect a sub
        fileData.streams[index].disposition[type] = 0;
        console.log(2);
      } else {
        // Change the selected sub
        fileData.streams[lastIndex].disposition[type] = 0;
        fileData.streams[index].disposition[type] = 1;
        console.log(3);
      }
    } else if (type === "forced") {
      // All of them can be selected
      fileData.streams[index].disposition[type] =
        fileData.streams[index].disposition[type] === 0 ? 1 : 0;
    }
  }
</script>

<div
  class="media_properties"
  class:is_selected={selectedFile}
  class:not_selected={!selectedFile}
>
  {#if selectedFile}
    <div class="file_name"><h1>{selectedFile.name}</h1></div>
    {#if !loading && fileData}
      {JSON.stringify(fileData)}
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
          {#if fileData.streams.filter((x) => x.codec_type === "subtitle").length > 0}
            <table>
              <thead
                ><tr
                  ><th>Title</th><th>Language</th><th>Default</th><th>Forced</th
                  ><th>&nbsp;</th></tr
                ></thead
              >
              <tbody>
                {#each fileData.streams
                  .filter((x) => x.codec_type === "subtitle")
                  .sort((a, b) => sortSubtitles(a, b)) as stream}
                  <tr>
                    <td>{stream.tags.title ?? "-"}</td>
                    <td>{stream.tags.language}</td>
                    <td>
                      <button
                        class="checkbox"
                        onclick={() =>
                          changeSelectedSub(stream.index, "default")}
                      >
                        {#if stream.disposition.default === 1}
                          <Svg name="check" color="white" />
                        {:else}
                          <Svg name="uncheck" color="white"></Svg>
                        {/if}
                      </button>
                    </td>
                    <td>
                      <button
                        class="checkbox"
                        onclick={() =>
                          changeSelectedSub(stream.index, "forced")}
                      >
                        {#if stream.disposition.forced === 1}
                          <Svg name="check" color="white" />
                        {:else}
                          <Svg name="uncheck" color="white"></Svg>
                        {/if}
                      </button>
                    </td>
                    <td
                      ><button class="delete"
                        ><Svg name="delete_forever" color="white" /><span
                          >Delete</span
                        >
                      </button></td
                    >
                  </tr>
                {/each}
              </tbody>
            </table>
          {:else}
            <div class="no_file">No subtitle data found</div>
          {/if}
        {/if}
      </div>
    {:else}
      <div>Loading...</div>
    {/if}
    <div class="button_cont">
      <button
        class="discard"
        onclick={() => {
          appState.media_properties = {};
        }}>Discard Changes</button
      >
      <button class="save">Save</button>
    </div>
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

  .file_name {
    height: 70px;
    width: calc(100% - 20px);
    background-color: rgb(29, 41, 61);
    border: 1px solid rgb(69, 85, 108);
    border-radius: 10px;
    margin: 10px;
    padding: 10px;

    display: grid;
    place-items: center;

    h1 {
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
    }
  }

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

  .button_cont {
    width: calc(100%-20px);
    display: flex;
    flex-direction: row;
    gap: 20px;
    justify-content: center;
    align-items: center;
    margin: 10px;
  }

  .save {
    background-color: rgba(19, 46, 21, 0.7);
    border: 2px solid rgb(0, 207, 62);
    border-radius: 10px;
    color: rgb(0, 207, 62);
    width: 100px;
    padding: 10px 20px;
    font-size: 17px;
    font-weight: 700;
    transition: 100ms all ease-in-out;
    text-transform: uppercase;

    &:hover {
      cursor: pointer;
      background-color: rgba(32, 110, 38, 0.7);
    }
    &:active {
      transform: scale(97%);
    }
  }

  .discard {
    background-color: rgba(207, 186, 0, 0.1);
    border: 2px solid rgb(207, 186, 0);
    border-radius: 10px;
    color: rgb(207, 186, 0);
    width: 250px;
    padding: 10px 20px;
    font-size: 17px;
    font-weight: 700;
    transition: 100ms all ease-in-out;
    text-transform: uppercase;

    &:hover {
      cursor: pointer;
      background-color: rgba(145, 134, 31, 0.7);
    }
    &:active {
      transform: scale(97%);
    }
  }
</style>
