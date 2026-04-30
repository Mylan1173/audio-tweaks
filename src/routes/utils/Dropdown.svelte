<script>
  import { fade } from "svelte/transition";
  import { appState } from "../utils/state.svelte.js";

  let {
    options,
    value,
    setValue,
    id,
    searchable = true,
    showCode = true,
  } = $props();

  let isOpen = $derived(appState.activeDropdownId === id);
  let searchQuery = $state("");

  let displayValue = $derived(
    options.choices.find((x) => x.code === value)?.name ?? options.dft,
  );

  let filteredChoices = $derived(
    !searchable || searchQuery.trim() === ""
      ? options.choices
      : options.choices.filter(
          (choice) =>
            choice.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
            choice.code.toLowerCase().includes(searchQuery.toLowerCase()),
        ),
  );

  function toggle(e) {
    e.stopPropagation();
    if (isOpen) {
      appState.activeDropdownId = null;
    } else {
      appState.activeDropdownId = id;
      searchQuery = "";
    }
  }

  function setOption(code) {
    setValue(code);
    appState.activeDropdownId = null;
  }

  function handleWindowClick() {
    if (isOpen) appState.activeDropdownId = null;
  }

  function autoFocus(node) {
    node.focus();
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div class="custom-select">
  <div
    class="select-trigger"
    role="combobox"
    aria-expanded={isOpen}
    aria-controls="dropdown-list-{id}"
    tabindex="0"
    onclick={toggle}
    onkeydown={(e) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        toggle(e);
      }
    }}
  >
    {#if isOpen && searchable}
      <input
        type="text"
        class="search-input"
        bind:value={searchQuery}
        placeholder="Search..."
        onclick={(e) => e.stopPropagation()}
        use:autoFocus
      />
    {:else}
      <span class="selected-value">{displayValue}</span>
    {/if}

    <span class="arrow" style:transform={isOpen ? "rotate(180deg)" : "none"}
    ></span>
  </div>

  {#if isOpen}
    <div class="select-dropdown" transition:fade={{ duration: 50 }}>
      <div class="options-list" id="dropdown-list-{id}" role="listbox">
        {#each filteredChoices as option}
          <button
            type="button"
            role="option"
            aria-selected={value === option.code}
            onclick={() => setOption(option.code)}
          >
            <span class="opt-name">{option.name}</span>
            {#if showCode}
              <span class="opt-code">{option.code}</span>
            {/if}
          </button>
        {:else}
          <div class="no-results">No matches found</div>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .custom-select {
    position: relative;
    width: 100%;
    height: 100%;
  }

  .select-trigger {
    font-size: 14px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    height: 100%;
    background-color: transparent;
    color: white;
    cursor: pointer;
    border-radius: 4px;
  }

  .selected-value {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-size: 16px;
    font-weight: 500;
  }

  .search-input {
    width: 100%;
    background: transparent;
    border: none;
    color: white;
    font-size: 16px;
    outline: none;
    padding: 0;
    margin-right: 8px;
  }

  .search-input::placeholder {
    color: rgb(186, 197, 211);
  }

  .arrow {
    border-left: 4px solid transparent;
    border-right: 4px solid transparent;
    border-top: 5px solid rgb(186, 197, 211);
    transition: transform 0.2s;
    flex-shrink: 0;
  }

  .select-dropdown {
    z-index: 1000;
    position: absolute;
    top: calc(100% + 5px);
    left: 5%;
    width: 90%;
    border: 1px solid rgb(69, 85, 108);
    border-radius: 8px;
    background-color: rgb(29, 41, 61);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .options-list {
    max-height: 200px;
    overflow-y: auto;
    padding: 5px;
  }

  .options-list button {
    width: 100%;
    border: none;
    padding: 8px 10px;
    background-color: inherit;
    color: white;
    text-align: left;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-radius: 4px;
  }

  .options-list button:hover {
    background-color: rgb(42, 58, 86);
  }

  .opt-code {
    font-size: 11px;
    color: rgb(186, 197, 211);
    text-transform: uppercase;
  }

  .no-results {
    padding: 15px;
    font-size: 13px;
    color: rgb(186, 197, 211);
    text-align: center;
  }

  .options-list::-webkit-scrollbar {
    width: 6px;
  }
  .options-list::-webkit-scrollbar-track {
    background: transparent;
  }
  .options-list::-webkit-scrollbar-thumb {
    background: rgb(69, 85, 108);
    border-radius: 10px;
  }
</style>
