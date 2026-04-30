<script>
  // @ts-nocheck

  import { appState } from "../utils/state.svelte.js";
  import Svg from "../utils/Svg.svelte";
  import Dropdown from "../utils/Dropdown.svelte";
  import {
    VIDEO_CODECS,
    ASPECT_RATIOS,
    PIXEL_FORMATS,
    VIDEO_PROFILES,
    FIELD_ORDER,
  } from "../utils/maps.js";

  let isVideoPropOpen = $state(true);
  let videoStream = $derived(appState.data.getVideo());

  let width = $derived(videoStream.width);
  let height = $derived(videoStream.height);

  function setWidth() {
    appState.data.setVideo("width", width);
  }
  function setHeight() {
    appState.data.setVideo("height", height);
  }
  function setCodec(code) {
    appState.data.setVideo("codec", code);
  }
  function setAspectRatio(code) {
    appState.data.setVideo("aspectRatio", code);
  }
  function setPixelFormat(code) {
    appState.data.setVideo("pixelFormat", code);
  }
  function setProfile(code) {
    appState.data.setVideo("profile", code);
  }
</script>

<div class="properties_cont">
  <button
    class="cont_title"
    onclick={() => (isVideoPropOpen = !isVideoPropOpen)}
    ><div class="chevron" class:open_chevron={isVideoPropOpen}>
      <Svg name="chevron_left" size={30} color="rgb(186, 197, 211)" />
    </div>
    <span>Video Properties</span></button
  >
  {#if isVideoPropOpen}
    {#if appState.data.isVideo}
      <div class="settings">
        <div class="setting-container">
          <div class="setting-name">Width (px)</div>
          <div class="setting-value">
            <input
              class="setting-input"
              type="number"
              bind:value={width}
              oninput={setWidth}
            />
          </div>
        </div>
        <div class="setting-container">
          <div class="setting-name">Height (px)</div>
          <div class="setting-value">
            <input
              class="setting-input"
              type="number"
              bind:value={height}
              oninput={setHeight}
            />
          </div>
        </div>
        <div class="setting-container">
          <div class="setting-name">Video Codec</div>
          <div class="setting-value">
            <Dropdown
              id="video-codec"
              options={{
                choices: VIDEO_CODECS,
              }}
              value={videoStream.codecName}
              setValue={setCodec}
            />
          </div>
        </div>
        <div class="setting-container">
          <div class="setting-name">Aspect Ratio</div>
          <div class="setting-value">
            <Dropdown
              id="video-aspect-ratio"
              options={{
                choices: ASPECT_RATIOS,
              }}
              value={videoStream.aspectRatio}
              setValue={setAspectRatio}
              showCode={false}
            />
          </div>
        </div>
        <div class="setting-container">
          <div class="setting-name">Pixel Format</div>
          <div class="setting-value">
            <Dropdown
              id="video-pixel-format"
              options={{
                choices: PIXEL_FORMATS,
              }}
              value={videoStream.pixelFormat}
              setValue={setPixelFormat}
            />
          </div>
        </div>
        <div class="setting-container">
          <div class="setting-name">Scan Type</div>
          <div class="setting-value">
            <Dropdown
              id="video-field-order"
              options={{
                choices: FIELD_ORDER,
              }}
              value={videoStream.fieldOrder}
              setValue={(code) => appState.data.setVideo("fieldOrder", code)}
              searchable={false}
              showCode={false}
            />
          </div>
        </div>
      </div>
    {:else}
      <div class="no_file">No video data found</div>
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
  }

  .cont_title {
    background-color: inherit;
    border: none;
    display: flex;
    flex-direction: row;
    align-items: center;
    width: fit-content;

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

  .open_chevron {
    transform: rotate(90deg) !important;
  }

  .settings {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(600px, 1fr));
    gap: 10px;
    margin: 10px;

    .setting-container {
      height: 50px;
      /* width: 100%; */
      width: 600px;
      border-radius: 10px;
      border: 1px solid rgb(69, 85, 108);
      display: flex;
      flex-direction: row;

      .setting-name {
        border-top-left-radius: 10px;
        border-bottom-left-radius: 10px;
        padding: 10px;
        font-size: 17px;
        height: 100%;
        font-weight: 600;
        min-width: 200px;
        width: fit-content;
        background-color: rgb(19, 28, 46);
        color: rgb(186, 197, 211);
        border-right: 1px solid rgb(69, 85, 108);
      }

      .setting-value {
        padding: 10px 20px;
        height: 100%;
        width: 100%;
        font-size: 16px;
        font-weight: 500;
        color: white;
        display: grid;
        place-items: center;

        .setting-input {
          border: none;
          width: 100%;
          background-color: transparent;
          color: white;
          font-size: 16px;
          font-weight: 500;
          height: 100%;
          appearance: textfield;
          -moz-appearance: textfield;

          &:focus {
            outline: none;
          }
          &::-webkit-outer-spin-button,
          &::-webkit-inner-spin-button {
            -webkit-appearance: none;
          }
        }
      }
    }
  }
</style>
