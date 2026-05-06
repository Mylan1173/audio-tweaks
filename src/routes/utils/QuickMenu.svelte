<script>
  import { appState, closeQuickMenu } from "./state.svelte.js";
  import { fade } from "svelte/transition";
  import Svg from "./Svg.svelte";

  let menu = $derived(appState.quickMenu);

  let shouldFlipAbove = $derived(menu.coords.top > window.innerHeight * 0.7);
</script>

<svelte:window
  onkeydown={(e) => e.key === "Escape" && closeQuickMenu()}
  onclick={(e) => {
    if (!menu.isOpen) return;

    if (e.target instanceof Element && !e.target.closest(".menu-content")) {
      closeQuickMenu();
    }
  }}
/>

{#if menu.isOpen}
  <div class="quick-menu-overlay" transition:fade={{ duration: 60 }}>
    <div
      class="menu-content"
      class:flipped={shouldFlipAbove}
      style:top="{menu.coords.top}px"
      style:left="{menu.coords.left}px"
    >
      {#each menu.options as option (option.value)}
        <button onclick={() => closeQuickMenu(option.value)} class="menu-item">
          {#if option.icon}
            <Svg name={option.icon} size={18} />
          {/if}
          <span>{option.label}</span>
        </button>
      {/each}
    </div>
  </div>
{/if}

<style>
  .quick-menu-overlay {
    position: fixed;
    inset: 0;
    z-index: 9999;
    pointer-events: auto;
  }

  .menu-content {
    position: fixed;
    background-color: var(--bg-light);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 4px;
    min-width: 160px;
    width: max-content;
    display: flex;
    flex-direction: column;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);

    transform: translate(-50%, 8px);
  }

  .flipped {
    transform: translate(-50%, calc(-100% - 45px));
  }

  .menu-item {
    background: transparent;
    border: none;
    color: var(--text-light);
    padding: 8px 12px;
    text-align: left;
    display: flex;
    align-items: center;
    gap: 10px;
    border-radius: 5px;
    cursor: pointer;

    span {
      font-size: 14px;
      font-weight: 600;
    }

    &:hover {
      background-color: rgb(45, 60, 85);
      color: white;
    }
    &:active {
      transform: scale(97%);
    }
  }
</style>
