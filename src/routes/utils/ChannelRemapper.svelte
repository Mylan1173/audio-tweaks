<script>
  import { appState } from "../utils/state.svelte.js";
  import { CHANNEL_LAYOUTS, CHANNEL_LAYOUT_TYPES } from "../utils/maps.js";
  import Dropdown from "./Dropdown.svelte";
  import Svg from "./Svg.svelte";
  let { activeStreamIdx } = $props();
  let activeStream = $derived(appState.data.getAudio(activeStreamIdx));

  let channelMap = $derived(activeStream.channelMap);
  let channelVolumes = $derived(activeStream.channelVolumes);

  function setChannels(code) {
    let tempChannelMap = {};
    let tempChannelVolumes = {};
    for (let i = 0; i < code; i++) {
      if (i < activeStream.channels) tempChannelMap[i] = i;
      else tempChannelMap[i] = null;

      if (channelVolumes[i] == 0 || !channelVolumes[i])
        tempChannelVolumes[i] = 0;
      else tempChannelVolumes[i] = channelVolumes[i];
    }

    appState.data.setAudio("channel", activeStreamIdx, code);
    appState.data.setAudio("channelMap", activeStreamIdx, tempChannelMap);
    appState.data.setAudio(
      "channelVolume",
      activeStreamIdx,
      tempChannelVolumes,
    );
  }

  function setChannelMap(channelCode, targetCode) {
    // x : y
    let tempChannelMap = { ...channelMap };
    tempChannelMap[targetCode] =
      channelCode !== activeStream.channels ? channelCode : null;
    appState.data.setAudio("channelMap", activeStreamIdx, tempChannelMap);
  }

  function setChannelVolume(targetCode) {
    let tempChannelVolumes = { ...channelVolumes };
    if (channelVolumes[targetCode] === null) tempChannelVolumes[targetCode] = 0;
    else tempChannelVolumes[targetCode] = channelVolumes[targetCode];

    appState.data.setAudio(
      "channelVolume",
      activeStreamIdx,
      tempChannelVolumes,
    );
  }
</script>

<div class="channel-settings">
  <div class="channel-selectors">
    <p>Remap Channel Layout:</p>
    <div class="current-layout">
      {CHANNEL_LAYOUT_TYPES.find((x) => x.code === activeStream.channels).name}
    </div>
    <Svg name="forward" color="rgb(186, 197, 211)" />
    <div class="channels-dropdown">
      <Dropdown
        id="channel-types"
        options={{
          choices: CHANNEL_LAYOUT_TYPES,
        }}
        value={activeStream.newChannels}
        setValue={setChannels}
        searchable={false}
        showCode={false}
      />
    </div>
  </div>
  <div class="channel-remapper">
    <div class="input">Input</div>
    <div class="output-container">
      <div class="output">Output</div>

      <table>
        <thead>
          <tr>
            <td></td>
            {#each { length: activeStream.channels }, channelIndex}
              <td
                >{CHANNEL_LAYOUTS[activeStream.channels].find(
                  (x) => x.id == channelIndex,
                ).code}</td
              >
            {/each}
            <td>None</td>
            <td>Volume</td>
          </tr>
        </thead>
        <tbody>
          {#each { length: activeStream.newChannels }, newChannelIndex}
            <tr>
              <td
                >{CHANNEL_LAYOUTS[activeStream.newChannels].find(
                  (x) => x.id == newChannelIndex,
                ).code}</td
              >
              {#each { length: activeStream.channels + 1 }, channelIndex}
                <td>
                  <button
                    onclick={() => setChannelMap(channelIndex, newChannelIndex)}
                  >
                    {#if channelMap[newChannelIndex] === channelIndex || (channelMap[newChannelIndex] === null && channelIndex === activeStream.channels)}
                      <Svg name="radio_checked" color="white" />
                    {:else}
                      <Svg name="radio_unchecked" color="white" />
                    {/if}
                  </button>
                </td>
              {/each}
              <td>
                <div class="volume-cont">
                  <input
                    type="number"
                    bind:value={channelVolumes[newChannelIndex]}
                    oninput={() => setChannelVolume(newChannelIndex)}
                  />
                  <div>dB</div>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>

<style>
  .channel-settings {
    width: 100%;
    display: grid;
    place-items: center;
  }
  .channel-selectors {
    margin: 10px;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    gap: 10px;

    p {
      color: var(--text-light);
      font-size: 15px;
      font-weight: 600;
    }

    .current-layout {
      height: 40px;
      border: 1px solid var(--border);
      padding: 0px 20px;
      display: grid;
      place-items: center;
      border-radius: 10px;
      font-size: 16px;
      font-weight: 600;
    }

    .channels-dropdown {
      width: 200px;
      height: 40px;
      border: 1px solid var(--border);
      padding: 0px 20px;
      display: grid;
      place-items: center;
      border-radius: 10px;
    }
  }

  .channel-remapper {
    width: max-content;
    padding: 10px;
    margin: 0 10px;
    border: 1px solid var(--border);
    border-radius: 10px;
  }

  .input {
    font-size: 17px;
    font-weight: 600;
    color: var(--text-light);
  }

  .output-container {
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .output {
    text-orientation: upright;
    writing-mode: sideways-lr;
    font-size: 17px;
    font-weight: 600;
    color: var(--text-light);
  }

  table {
    width: max-content;
    height: fit-content;
    border-collapse: collapse;
    margin: 10px;
  }

  tr {
    height: 50px;
  }

  tbody tr td:not(:last-child) {
    border-top: 1px solid var(--border);
  }

  td:not(:last-child, :nth-last-child(1), :nth-last-child(2)) {
    border-right: 1px solid var(--border);
  }

  td:last-child {
    width: 150px;
  }

  td {
    width: 50px;

    color: var(--text-light);
    font-size: 15px;
    font-weight: 600;
  }

  td button {
    height: 24px;
    width: 24px;
    border: none;
    margin: auto;
    background-color: transparent;
    &:hover {
      cursor: pointer;
    }
  }

  .volume-cont {
    margin: 5px;
    overflow: hidden;
    height: 40px;
    width: 100%;
    border: 1px solid var(--border);
    border-radius: 10px;
    display: flex;
    flex-direction: row;

    input {
      width: calc(100% - 30px);
      background-color: transparent;
      border: none;
      border-right: 1px solid var(--border);
      color: white;
      font-size: 16px;
      font-weight: 600;
      text-align: center;
      padding: 0 10px;
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

    div {
      background-color: var(--bg-dark);
      color: var(--text-light);
      font-weight: 600;
      font-size: 15px;
      width: 30px;
      display: grid;
      place-content: center;
    }
  }
</style>
