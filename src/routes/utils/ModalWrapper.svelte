<script>
  import AskModal from "./ModalTypes/AskModal.svelte";
  import ProgressBarModal from "./ModalTypes/ProgressBarModal.svelte";
  let isShowing = $state(false);
  let title = $state("");
  let type = $state("");
  let options = $state();

  let resolvePromise = $state(null);

  export function showModal(
    modalType,
    modalTitle,
    modalOptions = { cancel: "Cancel", agree: "Yes" },
  ) {
    type = modalType;
    title = modalTitle;
    options = modalOptions;
    isShowing = true;

    return new Promise((resolve) => {
      resolvePromise = resolve;
    });
  }

  export function handleSelect(value) {
    isShowing = false;
    if (resolvePromise) {
      resolvePromise(value);
      resolvePromise = null;
    }
  }
</script>

{#if isShowing}
  <div class="cont">
    <button class="bg" onclick={() => handleSelect(false)} aria-label="bg"
    ></button>
    <div class="modal">
      {#if type === "Ask"}
        <AskModal {handleSelect} {title} {options} />
      {:else if type === "ProgressBar"}
        <ProgressBarModal {handleSelect} {title} />
      {/if}
    </div>
  </div>
{/if}

<style>
  .bg {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    backdrop-filter: brightness(0.5) blur(2px);
    z-index: 11;
    border: none;
    background: none;
  }
  .modal {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);

    height: 200px;
    width: 400px;

    display: flex;
    flex-direction: column;
    border: 1px solid rgb(69, 85, 108);
    border-radius: 10px;
    z-index: 12;
    background-color: rgb(19, 28, 46);
    overflow: hidden;
  }
</style>
