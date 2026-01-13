<script>
  import Self from "./MediaItem.svelte";
  import Svg from "./Svg.svelte";
  import { appState } from "./state.svelte.js";

  let { contents, level = 0 } = $props();

  let currentContents = $derived(contents);

  let isOpened = $state({});

  function openCloseMenu(index) {
    isOpened[index] = !isOpened[index];
  }

  function setSelectedFile(path, name) {
    appState.selected_file = { path, name };
  }
</script>

{#each currentContents as item, index}
  <div class="path_cont">
    {#if item.d_type === "Folder"}
      <button
        onclick={() => openCloseMenu(index)}
        class="path"
        style="padding-left: {5 * level}px;"
      >
        <div class="svg" class:open={isOpened[index]}>
          <Svg name="chevron_left" size="20" />
        </div>
        <div class="svg">
          <Svg
            name={isOpened[index] ? "folder_open" : "folder_closed"}
            size="20"
          />
        </div>
        <span>{item.name}</span>
      </button>
      {#if item.children && item.children.length > 0 && isOpened[index]}
        <Self contents={item.children} level={level + 1} />
      {/if}
    {:else if item.d_type === "File"}
      <button
        class="path"
        class:selected={appState.selected_file?.name === item.name}
        style="padding-left: {5 * level + 25}px;"
        onclick={() => setSelectedFile(item.pb, item.name)}
      >
        <div class="svg"><Svg name="video" size="20" /></div>
        <span>{item.name}</span>
      </button>
    {/if}
  </div>
{/each}

<style>
  .path_cont {
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
    gap: 5px;

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
    }

    .svg.open {
      transform: rotate(90deg);
    }

    &:hover {
      background-color: rgb(19, 28, 46);
    }
  }

  .selected {
    background-color: rgb(19, 28, 46);
  }
</style>
