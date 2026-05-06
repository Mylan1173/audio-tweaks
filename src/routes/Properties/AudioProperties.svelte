<script>
  import { invoke } from "@tauri-apps/api/core";
  import { appState } from "../utils/state.svelte.js";
  import Svg from "../utils/Svg.svelte";
  import Dropdown from "../utils/Dropdown.svelte";
  import AudioVisualizer from "../utils/AudioVisualizer.svelte";
  import ChannelRemapper from "../utils/ChannelRemapper.svelte";
  import { AUDIO_CODECS, SAMPLE_RATES, LANGUAGES } from "../utils/maps.js";
  import { startModal, closeModal } from "../utils/state.svelte.js";
  import { open } from "@tauri-apps/plugin-dialog";

  let isAudioPropOpen = $state(true);
  let activeStreamIdx = $state(0);
  let activeStream = $derived(appState.data.getAudio(activeStreamIdx));

  let bitrate = $derived(activeStream.bitRate);
  let bitdepth = $derived(activeStream.bitDepth);

  function setActiveStreamIdx(index) {
    activeStreamIdx = index;
  }
  function setCodec(code) {
    appState.data.setAudio("codec", activeStreamIdx, code);
  }
  function setBitrate() {
    appState.data.setAudio("bitrate", activeStreamIdx, bitrate);
  }
  function setBitDepth() {
    appState.data.setAudio("bitdepth", activeStreamIdx, bitdepth);
  }
  function setSampleRate(code) {
    appState.data.setAudio("samplerate", activeStreamIdx, code);
  }
  function setLanguage(code) {
    appState.data.setAudio("language", activeStreamIdx, code);
  }

  async function handleImport() {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Audio",
          extensions: [
            "mp3",
            "wav",
            "flac",
            "aac",
            "m4a",
            "ogg",
            "mka",
            "ac3",
            "eac3",
          ],
        },
      ],
    });
    if (file === null) return;

    startModal("Console", "Analyzing imported file...");

    try {
      const res = await invoke("get_media_streams", { path: file });

      const rawAudio = res.streams.find((s) => s.codec_type === "audio") || {};

      const newIndex = appState.data.addAudio(file, rawAudio);
      activeStreamIdx = newIndex;
    } catch (err) {
      console.error(err);
    } finally {
      closeModal();
    }
  }

  async function handleExport() {
    startModal("Console", "Saving Track...");
    try {
      await invoke("export_stream", {
        inputPath: appState.selectedMedia.mediaPath,
        streamType: "audio",
        streamIndex: activeStreamIdx,
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
    onclick={() => (isAudioPropOpen = !isAudioPropOpen)}
    ><div class="chevron" class:open-chevron={isAudioPropOpen}>
      <Svg name="chevron" size={30} color="rgb(186, 197, 211)" />
    </div>
    <span>Audio Properties</span></button
  >
  {#if isAudioPropOpen}
    {#if appState.data.isAudio}
      <div class="selectors">
        <p>Stream:</p>
        <div class="stream-dropdown">
          <Dropdown
            id="audio-stream"
            options={{
              dft: "Select Audio Stream",
              choices: appState.data.getAudioTracks(),
            }}
            value={activeStreamIdx}
            setValue={setActiveStreamIdx}
            searchable={false}
            showCode={false}
          />
        </div>
        <button class="selector-button" onclick={handleImport}
          ><Svg name="add" color="rgb(186, 197, 211)" /><span
            >Add Audio Track</span
          ></button
        >
        <button class="selector-button" onclick={handleExport}
          ><Svg name="audio" color="rgb(186, 197, 211)" /><span
            >Export Audio Track</span
          ></button
        >
      </div>
      {#if activeStreamIdx !== null}
        <hr />
        <div class="settings">
          <div class="setting-container">
            <div class="setting-name">Delete Track</div>
            <div class="setting-value">
              <button
                class="toggle-btn {activeStream.isDeleted
                  ? 'toggle-active'
                  : ''}"
                onclick={() =>
                  appState.data.setAudio("delete", activeStreamIdx)}
              >
                {activeStream.isDeleted ? "True" : "False"}
              </button>
            </div>
          </div>
          {#if !activeStream.isDeleted}
            <div class="setting-container">
              <div class="setting-name">Default Track</div>
              <div class="setting-value">
                <button
                  class="toggle-btn {activeStream.default
                    ? 'toggle-active'
                    : ''}"
                  onclick={() =>
                    appState.data.setAudio("default", activeStreamIdx, true)}
                  disabled={activeStream.default}
                >
                  {activeStream.default ? "True" : "False"}
                </button>
              </div>
            </div>
            <div class="setting-container">
              <div class="setting-name">Audio Codec</div>
              <div class="setting-value">
                <Dropdown
                  id="audio-codec"
                  options={{
                    choices: Object.entries(AUDIO_CODECS).map((x) => ({
                      code: x[0],
                      name: x[1],
                    })),
                  }}
                  value={activeStream.codecName}
                  setValue={setCodec}
                />
              </div>
            </div>
            <div class="setting-container">
              <div class="setting-name">Bitrate (bit/s)</div>
              <div class="setting-value">
                <input
                  class="setting-input"
                  type="number"
                  bind:value={bitrate}
                  oninput={setBitrate}
                />
              </div>
            </div>
            <div class="setting-container">
              <div class="setting-name">Sample Rate</div>
              <div class="setting-value">
                <Dropdown
                  id="audio-srate"
                  options={{
                    choices: SAMPLE_RATES,
                  }}
                  value={activeStream.sampleRate}
                  setValue={setSampleRate}
                />
              </div>
            </div>
            <div class="setting-container">
              <div class="setting-name">Language</div>
              <div class="setting-value">
                <Dropdown
                  id="audio-language"
                  options={{
                    choices: LANGUAGES,
                  }}
                  value={activeStream.language}
                  setValue={setLanguage}
                />
              </div>
            </div>
            {#if ["flac", "alac", "pcm_s16le", "pcm_s24le", "truehd", "mlp"].includes(activeStream.codecName)}
              <div class="setting-container">
                <div class="setting-name">Bit Depth</div>
                <div class="setting-value">
                  <input
                    class="setting-input"
                    type="number"
                    bind:value={bitdepth}
                    oninput={setBitDepth}
                  />
                </div>
              </div>
            {/if}
          {/if}
        </div>
        {#if !activeStream.isDeleted}
          <hr />
          <AudioVisualizer {activeStreamIdx} />

          <ChannelRemapper {activeStreamIdx} />
        {/if}
      {/if}
    {:else}
      <div class="no_file">No audio data found</div>
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

  .selectors {
    margin: 10px;
    width: calc(100% - 20px);
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    flex-wrap: wrap;
    column-gap: 20px;
    row-gap: 10px;

    p {
      color: var(--text-light);
      font-size: 15px;
      font-weight: 600;
      margin-right: 15px;
    }

    .stream-dropdown {
      width: 400px;
      height: 40px;
      border: 1px solid var(--border);
      padding: 0px 20px;
      display: grid;
      place-items: center;
      border-radius: 10px;
    }
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

    span {
      font-size: 16px;
      font-weight: 600;
      color: var(--text-light);
    }
    &:hover {
      border-color: var(--text-light);
      box-shadow: var(--text-light) 0px 0px 2px;
    }
    &:active {
      transform: scale(97%);
    }
  }

  .settings {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(600px, 1fr));
    gap: 10px;
    margin: 10px;

    .setting-container {
      height: 50px;
      width: 600px;
      border-radius: 10px;
      border: 1px solid var(--border);
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
        background-color: var(--bg-dark);
        color: var(--text-light);
        border-right: 1px solid var(--border);
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
  .toggle-btn {
    width: 100%;
    height: 100%;
    background-color: var(--bg-light);
    color: var(--text-light);
    font-weight: 600;
    font-size: 15px;
    cursor: pointer;
    transition: all 0.15s ease;
    border: none;
  }

  .toggle-btn:disabled {
    cursor: not-allowed;
  }

  .toggle-active {
    color: white;
  }

  hr {
    height: 2px;
    background-color: var(--border);
    border: none;
    margin: 10px 10px;
  }
</style>
