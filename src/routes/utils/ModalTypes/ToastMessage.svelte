<script>
  import { toastState } from "../state.svelte.js";
  import { fly, fade } from "svelte/transition";
  import { flip } from "svelte/animate";
  import Svg from "../Svg.svelte";
</script>

<div class="toast-container">
  {#each toastState.messages as toast (toast.id)}
    <div
      class="toast {toast.type}"
      animate:flip={{ duration: 300 }}
      in:fly={{ y: 50, duration: 300 }}
      out:fade={{ duration: 200 }}
    >
      <div class="toast-icon">
        {#if toast.type === "error"}
          <Svg name="error" color="#ff4d4d" size={20} />
        {:else if toast.type === "success"}
          <Svg name="success" color="#4caf50" size={20} />
        {:else}
          <Svg name="info" color="#3a8af3" size={20} />
        {/if}
      </div>

      <div class="toast-content">
        {toast.text}
      </div>

      <button class="toast-close" onclick={() => toastState.remove(toast.id)}>
        <Svg name="close" color="var(--text-light)" size={16} />
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: 20px;
    right: 20px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    z-index: 9999;
    pointer-events: none;
  }

  .toast {
    display: flex;
    flex-direction: row;
    align-items: center;
    background-color: var(--bg-dark, #131c2e);
    border: 1px solid var(--border, #2a3750);
    border-radius: 10px;
    padding: 12px 16px;
    min-width: 250px;
    max-width: 400px;
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.4);
    pointer-events: auto;
    gap: 12px;
  }

  .toast.error {
    border-left: 4px solid #ff4d4d;
  }
  .toast.success {
    border-left: 4px solid #4caf50;
  }
  .toast.info {
    border-left: 4px solid #3a8af3;
  }

  .toast-icon {
    display: grid;
    place-items: center;
  }

  .toast-content {
    flex: 1;
    color: white;
    font-size: 14px;
    font-weight: 500;
    line-height: 1.4;
    word-break: break-word;
  }

  .toast-close {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 4px;
    display: grid;
    place-items: center;
    border-radius: 4px;
    transition: background-color 150ms ease;
  }

  .toast-close:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }
</style>
