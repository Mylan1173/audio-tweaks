<script>
  import Self from "./MediaItem.svelte";
  import Svg from "./utils/Svg.svelte";
  import { appState, setSelectedMedia } from "./utils/state.svelte.js";

  let { contents, level = 0 } = $props();

  let currentContents = $derived(contents);

  let isOpened = $state({});

  function openCloseMenu(index) {
    isOpened[index] = !isOpened[index];
  }
</script>

{#each currentContents as item, index (index)}
  <div class="path-container">
    {#if item.data_type === "Folder"}
      <div
        class="path"
        class:selected={appState.selectedMedia?.mediaPath === item.data_path}
      >
        <button
          onclick={() => openCloseMenu(index)}
          style="padding-left: {5 * level}px;"
        >
          <div class="svg" class:open={isOpened[index]}>
            <Svg name="chevron" size={20} />
          </div>
        </button>
        <button
          class="folder-name"
          onclick={() =>
            setSelectedMedia(item.data_path, item.data_name, "folder")}
        >
          <div class="svg">
            <Svg
              name={isOpened[index] ? "folder_open" : "folder_closed"}
              size={20}
            />
          </div>
          <span>{item.data_name}</span>
        </button>
      </div>
      {#if item.children && item.children.length > 0 && isOpened[index]}
        <Self contents={item.children} level={level + 1} />
      {/if}
    {:else if item.data_type === "File"}
      <button
        class="path"
        class:selected={appState.selectedMedia?.mediaPath === item.data_path}
        style="padding-left: {5 * level + 25}px;"
        onclick={() => setSelectedMedia(item.data_path, item.data_name, "file")}
      >
        <div class="svg"><Svg name="file" size={20} /></div>
        <span>{item.data_name}</span>
      </button>
    {/if}
  </div>
{/each}

<style>
  .path-container {
    width: 100%;
    display: flex;
    flex-direction: column;
  }
  button {
    border: none;
    background-color: inherit;
    color: white;
    font-size: 12px;

    &:hover {
      cursor: pointer;
    }
  }
  .path {
    height: 25px;
    display: flex;
    flex-direction: row;
    align-items: center;
    transition: 100ms all ease-in-out;

    .svg {
      transform: rotate(0deg);
      display: grid;
      place-items: center;
    }

    span {
      text-align: left;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;

      min-width: 0;
      width: 100%;
      margin-left: 5px;
    }

    .svg.open {
      transform: rotate(90deg);
    }

    &:hover {
      background-color: var(--bg-dark);
    }

    .folder-name {
      display: flex;
      flex-direction: row;
      align-items: center;
      flex: 1;
    }
  }

  .selected {
    background-color: var(--bg-dark);
  }
</style>
