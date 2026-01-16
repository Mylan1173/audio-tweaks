<script>
  import { appState } from "../utils/state.svelte.js";
  import Svg from "../utils/Svg.svelte";

  let selectedFile = $derived(appState.selected_file);
  let fileData = $derived(appState.media_properties[selectedFile.path]);
  let sortedSubtitles = $derived(
    fileData.streams
      .filter((x) => x.codec_type === "subtitle")
      .toSorted(sortSubtitles)
  );

  let oldFileData = $state(null);

  // Whenever the selected path changes, take a new deep-copy snapshot
  $effect(() => {
    if (!oldFileData) {
      oldFileData = JSON.parse(
        JSON.stringify(appState.media_properties[selectedFile.path])
      );
    }
  });
  let isSubsPropOpen = $state(true);

  function sortSubtitles(a, b) {
    if (a !== b) {
      return a.tags.language.localeCompare(b.tags.language);
    } else {
      return a.tags.title.localeCompare(b.tags.title);
    }
  }

  function changeSelectedSub(index, type) {
    if (!oldFileData) return;

    if (!appState.pendingChanges.subs) {
      appState.pendingChanges.subs = {};
    }

    if (type === "default") {
      // 1. Find the current default stream globally
      const currentDefault = fileData.streams.find(
        (s) => s.codec_type === "subtitle" && s.disposition.default === 1
      );

      // 2. Logic for "at most 1"
      if (currentDefault) {
        // If user clicks the one that's already default, toggle it off
        if (currentDefault.index === index) {
          currentDefault.disposition.default = 0;
        } else {
          // Switch default from old to new
          currentDefault.disposition.default = 0;
          fileData.streams[index].disposition.default = 1;
        }
      } else {
        // No current default, just set the new one
        fileData.streams[index].disposition.default = 1;
      }
    } else if (type === "forced") {
      // Toggle logic for forced
      const val = fileData.streams[index].disposition.forced === 1 ? 0 : 1;
      fileData.streams[index].disposition.forced = val;
    }

    // Update pendingChanges accordingly
    const originalSubs = oldFileData.streams.filter(
      (s) => s.codec_type === "subtitle"
    );
    const currentSubs = fileData.streams.filter(
      (s) => s.codec_type === "subtitle"
    );
    // 2. Check if Default has changed
    const oldDefaultIdx = originalSubs.findIndex(
      (s) => s.disposition.default === 1
    );
    const newDefaultIdx = currentSubs.findIndex(
      (s) => s.disposition.default === 1
    );

    // 3. Check if Forced has changed
    const oldForced = originalSubs
      .filter((s) => s.disposition.forced === 1)
      .map((x) => getSubTypeIndex(x.index));
    const newForced = currentSubs
      .filter((s) => s.disposition.forced === 1)
      .map((x) => getSubTypeIndex(x.index));

    if (
      JSON.stringify(oldForced) === JSON.stringify(newForced) &&
      oldDefaultIdx === newDefaultIdx
    ) {
      delete appState.pendingChanges.subs;
    } else {
      appState.pendingChanges.subs.default =
        newDefaultIdx === -1 ? null : newDefaultIdx;
      appState.pendingChanges.subs.forced = newForced;
    }
  }

  function getSubTypeIndex(globalIndex) {
    return fileData.streams
      .filter((s) => s.codec_type === "subtitle")
      .findIndex((s) => s.index === globalIndex);
  }
</script>

<div class="subtitles properties_cont">
  <button class="cont_title" onclick={() => (isSubsPropOpen = !isSubsPropOpen)}
    ><div class="chevron" class:open_chevron={isSubsPropOpen}>
      <Svg name="chevron_left" size={30} color="rgb(186, 197, 211)" />
    </div>
    <span>Subtitle Properties</span></button
  >
  {#if isSubsPropOpen}
    {#if fileData.streams.filter((x) => x.codec_type === "subtitle").length > 0}
      <table>
        <thead
          ><tr
            ><th>Title</th><th>Language</th><th>Default</th><th>Forced</th><th
              >&nbsp;</th
            ></tr
          ></thead
        >
        <tbody>
          {#each sortedSubtitles as stream (stream.index)}
            <tr>
              <td>{stream.tags.title ?? "-"}</td>
              <td>{stream.tags.language}</td>
              <td>
                <button
                  class="checkbox"
                  onclick={() => changeSelectedSub(stream.index, "default")}
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
                  onclick={() => changeSelectedSub(stream.index, "forced")}
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
                  ><Svg name="delete_forever" color="white" /><span>Delete</span
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
