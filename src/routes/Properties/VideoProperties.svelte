<script>
  import { invoke } from "@tauri-apps/api/core";
  import { appState } from "../utils/state.svelte.js";
  import Svg from "../utils/Svg.svelte";
  import Dropdown from "../utils/Dropdown.svelte";
  import {
    VIDEO_CODECS,
    ASPECT_RATIOS,
    PIXEL_FORMATS,
    FIELD_ORDER,
    VIDEO_PROFILES,
    VIDEO_FORMATS,
  } from "../utils/maps.js";
  import { startModal, closeModal } from "../utils/state.svelte.js";

  let isVideoPropOpen = $state(false);
  let videoStream = $derived(appState.data.getVideo());

  let width = $derived(videoStream.width);
  let height = $derived(videoStream.height);

  let allowedCodecs = $derived(
    VIDEO_CODECS.filter((c) => {
      const format = videoStream.format || "mkv";
      if (format === "mp4")
        return ["h264", "hevc", "mpeg4", "vp9", "av1", "copy"].includes(c.code);
      if (format === "webm")
        return ["vp8", "vp9", "av1", "copy"].includes(c.code);
      if (format === "avi") return ["mpeg4", "h264", "copy"].includes(c.code);
      if (format === "mov")
        return ["h264", "hevc", "prores", "mpeg4", "copy"].includes(c.code);
      return true;
    }),
  );

  let availableProfiles = $derived(
    Array.isArray(VIDEO_PROFILES)
      ? VIDEO_PROFILES
      : VIDEO_PROFILES[videoStream.codecName] || [],
  );

  $effect(() => {
    if (videoStream.format && videoStream.codecName) {
      const validCodes = allowedCodecs.map((c) => c.code);
      if (!validCodes.includes(videoStream.codecName)) {
        appState.data.setVideo(
          "codec",
          validCodes.includes("copy") ? "copy" : validCodes[0],
        );
        appState.data.setVideo("profile", "");
      }
    }
  });

  $effect(() => {
    if (availableProfiles.length > 0 && videoStream.profile) {
      const validProfileCodes = availableProfiles.map((p) => p.code);
      if (!validProfileCodes.includes(videoStream.profile)) {
        appState.data.setVideo("profile", "");
      }
    }
  });

  function setWidth() {
    if (width % 2 !== 0) {
      appState.data.addError("widthNotEven");
      return;
    } else if (appState.data.hasError("widthNotEven"))
      appState.data.removeError("widthNotEven");

    if (width <= 0) {
      appState.data.addError("widthNotValid");
      return;
    } else if (appState.data.hasError("widthNotValid"))
      appState.data.removeError("widthNotValid");

    appState.data.setVideo("width", width);
  }

  function setHeight() {
    if (height % 2 !== 0) {
      appState.data.addError("heightNotEven");
      return;
    } else if (appState.data.hasError("heightNotEven"))
      appState.data.removeError("heightNotEven");

    if (height <= 0) {
      appState.data.addError("heightNotValid");
      return;
    } else if (appState.data.hasError("heightNotValid"))
      appState.data.removeError("heightNotValid");

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
  function setFormat(code) {
    appState.data.setVideo("format", code);
  }
  function setFieldOrder(code) {
    appState.data.setVideo("fieldOrder", code);
  }

  async function handleExport() {
    startModal("Console", "Saving Track...");
    try {
      await invoke("export_stream", {
        inputPath: appState.selectedMedia.mediaPath,
        streamType: "video",
        streamIndex: 0,
      });
    } catch (err) {
      console.error(err);
    } finally {
      closeModal();
    }
  }
</script>

<div class="properties-container">
  <button
    class="properties-container-title"
    onclick={() => (isVideoPropOpen = !isVideoPropOpen)}
    ><div class="chevron" class:open-chevron={isVideoPropOpen}>
      <Svg name="chevron" size={30} color="rgb(186, 197, 211)" />
    </div>
    <span>Video Properties</span></button
  >
  {#if isVideoPropOpen}
    {#if appState.data.isVideo}
      <div class="settings">
        <div class="setting-wrapper">
          <div
            class="setting-container"
            class:error={appState.data.hasError("width")}
          >
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
          {#each appState.data.getErrorMessages("width") as message}
            <div class="error-text">
              <Svg name="error" size={20} color="#ff4d4d" />
              {message}
            </div>
          {/each}
        </div>

        <div class="setting-wrapper">
          <div
            class="setting-container"
            class:error={appState.data.hasError("height")}
          >
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
          {#each appState.data.getErrorMessages("height") as message}
            <div class="error-text">
              <Svg name="error" size={20} color="#ff4d4d" />
              {message}
            </div>
          {/each}
        </div>

        <div class="setting-wrapper">
          <div class="setting-container">
            <div class="setting-name">Output Format</div>
            <div class="setting-value">
              <Dropdown
                id="video-format"
                options={{ choices: VIDEO_FORMATS }}
                value={videoStream.format}
                setValue={setFormat}
                showCode={false}
                searchable={false}
              />
            </div>
          </div>
        </div>

        <div class="setting-wrapper">
          <div class="setting-container">
            <div class="setting-name">Video Codec</div>
            <div class="setting-value">
              <Dropdown
                id="video-codec"
                options={{ choices: allowedCodecs }}
                value={videoStream.codecName}
                setValue={setCodec}
              />
            </div>
          </div>
        </div>
        {#if availableProfiles.length > 0}
          <div class="setting-wrapper">
            <div class="setting-container">
              <div class="setting-name">Codec Profile</div>
              <div class="setting-value">
                <Dropdown
                  id="video-profile"
                  options={{ choices: availableProfiles }}
                  value={videoStream.profile}
                  setValue={setProfile}
                />
              </div>
            </div>
          </div>
        {/if}

        <div class="setting-wrapper">
          <div class="setting-container">
            <div class="setting-name">Aspect Ratio</div>
            <div class="setting-value">
              <Dropdown
                id="video-aspect-ratio"
                options={{ choices: ASPECT_RATIOS }}
                value={videoStream.aspectRatio}
                setValue={setAspectRatio}
                showCode={false}
              />
            </div>
          </div>
        </div>

        <div class="setting-wrapper">
          <div class="setting-container">
            <div class="setting-name">Pixel Format</div>
            <div class="setting-value">
              <Dropdown
                id="video-pixel-format"
                options={{ choices: PIXEL_FORMATS }}
                value={videoStream.pixelFormat}
                setValue={setPixelFormat}
              />
            </div>
          </div>
        </div>

        <div class="setting-wrapper">
          <div class="setting-container">
            <div class="setting-name">Scan Type</div>
            <div class="setting-value">
              <Dropdown
                id="video-field-order"
                options={{ choices: FIELD_ORDER }}
                value={videoStream.fieldOrder}
                setValue={setFieldOrder}
                searchable={false}
                showCode={false}
              />
            </div>
          </div>
        </div>
      </div>

      <button class="selector-button" onclick={handleExport}>
        <Svg name="video" color={"rgb(186, 197, 211)"} />
        <span>Export Video</span>
      </button>
    {:else}
      <div class="no_file">No video data found</div>
    {/if}
  {/if}
</div>

<style>
  .properties-container {
    width: calc(100% - 40px);
    background-color: var(--bg-light);
    border: 1px solid var(--border);
    border-radius: 10px;
    margin: 10px 20px;
    padding: 10px;
    font-size: 15px;
    font-weight: 600;
    text-align: center;
    display: flex;
    flex-direction: column;
  }

  .properties-container-title {
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
      color: var(--text-light);
      font-size: 15px;
      font-weight: 600;
    }
  }

  .open-chevron {
    transform: rotate(90deg) !important;
  }

  .settings {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(600px, 1fr));
    gap: 15px 10px;
    margin: 10px;

    .setting-wrapper {
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      width: 100%;
    }

    .setting-container {
      height: 50px;
      width: 600px;
      border-radius: 10px;
      border: 1px solid var(--border);
      display: flex;
      flex-direction: row;
      transition: border-color 150ms ease;

      .setting-name {
        border-top-left-radius: 10px;
        border-bottom-left-radius: 10px;
        padding: 10px;
        font-size: 17px;
        height: 100%;
        font-weight: 600;
        min-width: 200px;
        width: fit-content;
        background-color: var(--bg-dark);
        color: var(--text-light);
        border-right: 1px solid var(--border);
        transition: background-color 150ms ease;
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

  .error {
    border: 1px solid #ff4d4d !important;
    box-shadow: 0 0 6px rgba(255, 77, 77, 0.2);
  }

  .error-text {
    color: #ff4d4d;
    font-size: 13px;
    font-weight: 500;
    margin-top: 6px;
    margin-left: 10px;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .selector-button {
    width: fit-content;
    height: 40px;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    padding: 10px 10px;
    border-radius: 10px;
    border: 1px solid var(--border);
    background-color: transparent;
    cursor: pointer;
    gap: 10px;
    transition: 100ms all ease-in-out;
    margin: 10px auto;

    span {
      font-size: 16px;
      font-weight: 600;
      color: var(--text-light);
      transition: color 100ms ease;
    }
    &:hover:not(.disabled-btn) {
      border-color: var(--text-light);
      box-shadow: var(--text-light) 0px 0px 2px;
    }
    &:active:not(.disabled-btn) {
      transform: scale(97%);
    }
  }

  .no_file {
    margin: 20px 0;
    color: var(--text-dark);
  }
</style>
