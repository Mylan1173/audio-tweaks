<script>
  import { listen } from "@tauri-apps/api/event";

  let modalProps = $props();
  let saveProgress = $state(0);
  let displayTitle = $state(modalProps.title);

  listen("ffmpeg-progress", (event) => {
    saveProgress = event.payload;
    if (event.payload < 100) {
      displayTitle = modalProps.title;
    }
  });

  listen("app-progress", (event) => {
    saveProgress = event.payload.progress;
    if (event.payload.title) {
      displayTitle = event.payload.title;
    }
  });
</script>

<div class="title">
  <h1>{displayTitle}</h1>
</div>
<div class="progress-bar">
  <div class="progress-value" style:width={`${saveProgress}%`}>
    {saveProgress}%
  </div>
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
      color: rgb(144, 161, 185);
    }
  }
  .progress-bar {
    background: rgba(255, 255, 255, 0.1);
    justify-content: flex-start;
    border-radius: 100px;
    align-items: center;
    position: relative;
    padding: 0 5px;
    display: flex;
    height: 35px;
    margin: auto 10px;
    width: calc(100% - 20px);
  }

  .progress-value {
    transition: width 200ms linear;
    box-shadow: 0 10px 40px -10px linear-gradient(to right, rgb(83, 204, 255), rgb(5, 255, 117));
    border-radius: 100px;
    background: linear-gradient(to right, rgb(83, 204, 255), rgb(5, 255, 117));
    height: 25px;
    width: 0;
    font-size: 16px;
    text-align: end;
    padding-right: 5px;
    font-weight: 700;
    color: rgb(19, 28, 46);
  }
</style>
