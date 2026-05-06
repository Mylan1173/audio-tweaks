<script>
  import { invoke } from "@tauri-apps/api/core";
  import { appState } from "../utils/state.svelte.js";
  import Svg from "../utils/Svg.svelte";
  import { LANGUAGES } from "../utils/maps.js";
  import {
    openQuickMenu,
    startModal,
    closeModal,
  } from "../utils/state.svelte.js";
  import { open } from "@tauri-apps/plugin-dialog";

  let isSubsPropOpen = $state(false);

  let isEditing = $state(false);
  let editIndex = $state();
  let editTitle = $state("");
  let editLanguage = $state("");

  async function handleMore(e, idx) {
    const action = await openQuickMenu(e.currentTarget, [
      { label: "Edit", value: "edit", icon: "edit" },
      { label: "Delete", value: "delete", icon: "delete_forever" },
      { label: "Export", value: "subtitles", icon: "export" },
    ]);

    if (!action) return;

    if (action === "delete") {
      const answer = await startModal(
        "Ask",
        "Are you sure you want to delete this subtitle track?",
      );
      if (answer) {
        appState.data.setSubtitle("delete", idx);
      }
    } else if (action === "export") {
      startModal("Console", "Saving Track...");
      try {
        await invoke("export_stream", {
          inputPath: appState.selectedMedia.mediaPath,
          streamType: "subtitle",
          streamIndex: idx,
        });
      } catch (err) {
        console.error(err);
      } finally {
        closeModal();
      }
    } else if (action === "edit") {
      isEditing = true;
      editIndex = idx;
      editTitle = appState.data.getSubtitles()[idx].title;
      editLanguage = appState.data.getSubtitles()[idx].language;
    }
  }

  function applyEdit() {
    isEditing = false;
    if (editTitle !== appState.data.getSubtitles()[editIndex].title)
      appState.data.setSubtitle("title", editIndex, editTitle);
    if (editLanguage !== appState.data.getSubtitles()[editIndex].language)
      appState.data.setSubtitle("language", editIndex, editLanguage);
  }

  function discardEdit() {
    if (!editLanguage) return;
    isEditing = false;
    editIndex = null;
  }

  async function handleImport() {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "SubRip",
          extensions: ["srt"],
        },
      ],
    });
    if (file === null) return;

    const index = appState.data.addSubtitle(file);
    isEditing = true;
    editIndex = index;

    editTitle = appState.data.getSubtitles()[index].title;
    editLanguage = appState.data.getSubtitles()[index].language;
  }
</script>

<div class="subtitles properties-container">
  <button
    class="properties-container-title"
    onclick={() => (isSubsPropOpen = !isSubsPropOpen)}
    ><div class="chevron" class:open-chevron={isSubsPropOpen}>
      <Svg name="chevron" size={30} color="rgb(186, 197, 211)" />
    </div>
    <span>Subtitle Properties</span></button
  >
  {#if isSubsPropOpen}
    {#if appState.data.isSubtitle}
      <table>
        <thead
          ><tr
            ><th>Title</th><th>Language</th><th>Default</th><th>Forced</th><th
              >&nbsp;</th
            ></tr
          ></thead
        >
        <tbody>
          {#each appState.data.getSubtitles() as subtitle, index}
            {#if !subtitle.isDeleted}
              <tr>
                {#if isEditing && editIndex === index}
                  <td><input type="text" bind:value={editTitle} /></td>
                  <td
                    ><select bind:value={editLanguage}>
                      {#each LANGUAGES as lang}
                        <option value={lang.code}>{lang.name}</option>
                      {/each}
                    </select></td
                  >
                {:else}
                  <td>{subtitle.title}</td>
                  <td
                    >{subtitle.language
                      ? LANGUAGES.find((x) => x.code === subtitle.language)
                          ?.name
                      : ""}</td
                  >
                {/if}
                <td>
                  <button
                    class="checkbox"
                    onclick={() => appState.data.setSubtitle("default", index)}
                  >
                    {#if subtitle.default}
                      <Svg name="check" color="white" />
                    {:else}
                      <Svg name="uncheck" color="white"></Svg>
                    {/if}
                  </button>
                </td>
                <td>
                  <button
                    class="checkbox"
                    onclick={() => appState.data.setSubtitle("forced", index)}
                  >
                    {#if subtitle.forced}
                      <Svg name="check" color="white" />
                    {:else}
                      <Svg name="uncheck" color="white"></Svg>
                    {/if}
                  </button>
                </td>
                {#if isEditing && editIndex === index}
                  <td>
                    <button class="more" color="blue" onclick={applyEdit}
                      ><Svg name="tick" /></button
                    >
                    <button class="more" color="red" onclick={discardEdit}
                      ><Svg name="x" /></button
                    >
                  </td>
                {:else}
                  <td
                    ><button class="more" onclick={(e) => handleMore(e, index)}
                      ><Svg name="more" color="white" />
                    </button></td
                  >
                {/if}
              </tr>
            {/if}
          {/each}
        </tbody>
      </table>

      <button class="add-stream" onclick={handleImport}
        ><Svg name="add" color="rgb(186, 197, 211)" /><span
          >Add Subtitle Track</span
        ></button
      >
    {:else}
      <div class="no_file">No subtitle data found</div>
    {/if}
  {/if}
</div>

<style>
  .properties-container {
    width: calc(100% - 40px);
    background-color: var(--bg-light);
    border: 1px solid var(--border);
    border-radius: 10px;
    margin: 10px 20px;
    padding: 10px;

    font-size: 15px;
    font-weight: 600;
    text-align: center;
    display: flex;
    flex-direction: column;

    .properties-container-title {
      background-color: inherit;
      border: none;
      display: flex;
      flex-direction: row;
      align-items: center;
      width: fit-content;

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
        color: var(--text-light);
        font-size: 15px;
        font-weight: 600;
      }
    }
  }

  .open-chevron {
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
        color: var(--text-light);
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
          width: 50%;
          background-color: var(--bg-dark);
        }
        td:nth-child(2) {
          width: 20%;
          background-color: var(--bg-dark);
        }
        td:nth-child(3),
        td:nth-child(4) {
          width: 10%;
        }
        td:nth-child(5) {
          display: flex;
          flex-direction: row;
          justify-content: center;
          align-items: center;
        }
      }
    }
    tr:first-child td:first-child {
      border-top-left-radius: 10px;
    }
    tr:first-child td:nth-child(2) {
      border-top-right-radius: 10px;
    }
    tr:last-child td:first-child {
      border-bottom-left-radius: 10px;
    }
    tr:last-child td:nth-child(2) {
      border-bottom-right-radius: 10px;
    }
  }

  td input[type="text"],
  td select {
    width: calc(100% - 10px);
    height: calc(100% - 10px);
    box-sizing: border-box;
    background-color: var(--bg-light);
    color: white;
    border: 1px solid var(--border);
    border-radius: 5px;
    font-size: 15px;
    font-weight: 600;
    outline: none;
    transition: border-color 0.2s;

    text-align: center;
    padding: 5px;
  }

  td input[type="text"]:focus,
  td select:focus {
    border-color: var(--border);
    background-color: var(--bg-light);
  }

  select option {
    background-color: var(--bg-dark);
    color: white;
    text-align: left;
    border-radius: 10px;
  }

  .add-stream {
    margin: 0 auto;
    width: fit-content;
    height: 40px;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    padding: 10px;
    border-radius: 10px;
    border: 1px solid var(--border);
    background-color: transparent;
    cursor: pointer;
    gap: 10px;
    transition: 100ms all ease-in-out;

    span {
      font-size: 16px;
      font-weight: 600;
      color: var(--text-light);
    }
    &:hover {
      border-color: var(--text-light);
      box-shadow: var(--text-light) 0px 0px 2px;
    }
    &:active {
      transform: scale(97%);
    }
  }

  .checkbox {
    cursor: pointer;
    background-color: transparent;
    border: none;
  }

  .more {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    background-color: transparent;
    border-radius: 10px;
    margin: 2px auto;
    padding: 5px 10px;
    gap: 10px;
    border: none;
    transition: 100ms all ease-in-out;
    cursor: pointer;
    &:hover {
      background-color: var(--bg-dark);
    }
    &:active {
      transform: scale(97%);
    }
  }
</style>
