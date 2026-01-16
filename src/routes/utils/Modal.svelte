<script>
  let isShowing = $state(false);
  let title = $state("");
  let options = $state({ cancel: "Cancel", agree: "Confirm" });

  let resolvePromise = $state(null);

  export function showModal(m_title, m_options) {
    title = m_title;
    options = m_options;
    isShowing = true;

    return new Promise((resolve) => {
      resolvePromise = resolve;
    });
  }

  function handleSelect(value) {
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
      <div class="title">
        <h1>{title}</h1>
      </div>
      <div class="buttons">
        <button class="left" onclick={() => handleSelect(false)}
          >{options.cancel}</button
        >
        <button class="right" onclick={() => handleSelect(true)}
          >{options.agree}</button
        >
      </div>
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
  .title {
    padding: 20px;
    width: 100%;
    height: fit-content;
    border-bottom: 1px solid rgb(69, 85, 108);
    background-color: rgb(29, 41, 61);
    display: grid;
    place-items: center;

    h1 {
      font-size: 20px;
      font-weight: 600;
      text-align: center;
    }
  }
  .buttons {
    flex: 1;
    height: calc(100% - 75px);
    padding: 20px;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-evenly;
    gap: 20px;

    button {
      cursor: pointer;
      border: 1px solid rgb(69, 85, 108);
      background-color: rgb(29, 41, 61);
      font-size: 18px;
      font-weight: 600;
      padding: 10px 20px;
      color: white;
      border-radius: 10px;
      transition: 100ms all ease-in-out;
      width: 40%;
      height: 50px;

      &:hover {
        border-color: rgb(186, 197, 211);
        transform: scale(110%);
      }
      &:active {
        transform: scale(100%);
      }
    }
  }
</style>
