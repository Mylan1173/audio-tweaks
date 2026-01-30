<script>
  let modalProps = $props();
</script>

<div class="title">
  <h1>{modalProps.title}</h1>
</div>
<div class="buttons">
  <button class="left" onclick={() => modalProps.handleSelect(false)}
    >{modalProps.options.cancel}</button
  >
  <button class="right" onclick={() => modalProps.handleSelect(true)}
    >{modalProps.options.agree}</button
  >
</div>

<style>
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
