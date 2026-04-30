<script>
  // @ts-nocheck

  import { appState } from "../utils/state.svelte.js";
  import Dropdown from "../utils/Dropdown.svelte";
  import Svg from "../utils/Svg.svelte";
  import {
    VIDEO_CODECS,
    ASPECT_RATIOS,
    PIXEL_FORMATS,
    FIELD_ORDER,
    AUDIO_CODECS,
    SAMPLE_RATES,
    VIDEO_PROFILES,
    CHANNEL_LAYOUTS,
  } from "../utils/maps.js";

  let comparer = $derived(appState.contentData);
  let isPropOpen = $state({ video: true, audio: true, subtitle: true });

  const formatChoices = (data) => {
    if (!data) return [];
    if (Array.isArray(data)) return data;
    return Object.entries(data).map(([code, name]) => ({ code, name }));
  };

  const getFlatVideoProfiles = () => {
    let flat = [];
    for (const codec in VIDEO_PROFILES)
      flat = flat.concat(VIDEO_PROFILES[codec]);
    const unique = [];
    const map = new Map();
    for (const item of flat) {
      if (!map.has(item.code)) {
        map.set(item.code, true);
        unique.push(item);
      }
    }
    return unique;
  };

  const getFlatChannels = () =>
    Object.entries(CHANNEL_LAYOUTS).map(([code, _]) => ({
      code: parseInt(code),
      name: `${code} Channels`,
    }));

  let audioStreamChoices = $derived(
    comparer
      .getUniqueValues("audio", "fullTrack")
      .map((x) => ({ code: x, name: x })),
  );
  let subtitleStreamChoices = $derived(
    comparer
      .getUniqueValues("subtitle", "fullTrack")
      .map((x) => ({ code: x, name: x })),
  );

  let schemas = $derived({
    video: [
      {
        id: "width",
        type: "number",
        label: "Force Width (px)",
        sourceId: "width",
      },
      {
        id: "height",
        type: "number",
        label: "Force Height (px)",
        sourceId: "height",
      },
      {
        id: "codecName",
        type: "dropdown",
        label: "Force Video Codec",
        options: formatChoices(VIDEO_CODECS),
        sourceId: "codecName",
      },
      {
        id: "aspectRatio",
        type: "dropdown",
        label: "Force Aspect Ratio",
        options: formatChoices(ASPECT_RATIOS),
        sourceId: "aspectRatio",
      },
      {
        id: "pixelFormat",
        type: "dropdown",
        label: "Force Pixel Format",
        options: formatChoices(PIXEL_FORMATS),
        sourceId: "pixelFormat",
      },
      {
        id: "fieldOrder",
        type: "dropdown",
        label: "Force Scan Type",
        options: formatChoices(FIELD_ORDER),
        sourceId: "fieldOrder",
      },
      {
        id: "profile",
        type: "dropdown",
        label: "Force Video Profile",
        options: getFlatVideoProfiles(),
        sourceId: "profile",
      },
    ],
    audio: [
      {
        id: "forceCodec",
        type: "dropdown",
        label: "Force Audio Codec",
        options: formatChoices(AUDIO_CODECS),
        sourceId: "codecName",
      },
      {
        id: "forceSampleRate",
        type: "dropdown",
        label: "Force Sample Rate",
        options: formatChoices(SAMPLE_RATES),
        sourceId: "sampleRate",
      },
      {
        id: "forceChannels",
        type: "dropdown",
        label: "Force Channels",
        options: getFlatChannels(),
        sourceId: "channels",
      },
      {
        id: "forceBitrate",
        type: "number",
        label: "Force Bitrate (bps)",
        sourceId: "bitRate",
      },
      {
        id: "setDefaultStream",
        type: "dropdown",
        label: "Set Default Stream",
        options: audioStreamChoices,
      },
      {
        id: "setForcedStream",
        type: "dropdown",
        label: "Set Forced Stream",
        options: audioStreamChoices,
      },
      {
        id: "keepOnlyStream",
        type: "dropdown",
        label: "Keep ONLY this Stream",
        options: audioStreamChoices,
      },
      {
        id: "deleteStream",
        type: "dropdown",
        label: "Delete ONLY this Stream",
        options: audioStreamChoices,
      },
      { id: "deleteAll", type: "boolean", label: "Delete ALL Audio Streams" },
    ],
    subtitle: [
      {
        id: "setDefaultStream",
        type: "dropdown",
        label: "Set Default Stream",
        options: subtitleStreamChoices,
      },
      {
        id: "setForcedStream",
        type: "dropdown",
        label: "Set Forced Stream",
        options: subtitleStreamChoices,
      },
      {
        id: "keepOnlyStream",
        type: "dropdown",
        label: "Keep ONLY this Stream",
        options: subtitleStreamChoices,
      },
      {
        id: "deleteStream",
        type: "dropdown",
        label: "Delete ONLY this Stream",
        options: subtitleStreamChoices,
      },
      { id: "deleteAll", type: "boolean", label: "Delete ALL Subtitles" },
    ],
  });

  function handleExport() {
    const data = comparer.exportProfile();
    const blob = new Blob([data], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "batch_profile.json";
    a.click();
    URL.revokeObjectURL(url);
  }

  function handleImport(e) {
    const file = e.target.files[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (event) => comparer.importProfile(event.target.result);
    reader.readAsText(file);
    e.target.value = "";
  }

  function toggleSection(sec) {
    isPropOpen[sec] = !isPropOpen[sec];
  }

  function getFoundLabel(val, options) {
    if (!options) return val;
    const match = options.find((o) => o.code == val);
    return match ? match.name : val;
  }
</script>

<div class="comparer-wrapper">
  <div class="profile-toolbar">
    <div class="toolbar-title">Batch Profile Editor</div>
    <div class="toolbar-actions">
      <button class="action-btn" onclick={handleExport}
        ><Svg name="export" color="rgb(186, 197, 211)" />Export Profile</button
      >
      <label class="action-btn">
        <input
          type="file"
          accept=".json"
          onchange={handleImport}
          class="hidden-input"
        />
        <Svg name="import" color="rgb(186, 197, 211)" />Import Profile
      </label>
    </div>
  </div>

  {#each Object.entries(schemas) as [category, schema]}
    <div class="properties_cont">
      <button class="cont_title" onclick={() => toggleSection(category)}>
        <div class="chevron" class:open_chevron={isPropOpen[category]}>
          <Svg name="chevron_left" size={30} color="rgb(186, 197, 211)" />
        </div>
        <span style="text-transform: capitalize;">{category} Batch Rules</span>
      </button>

      {#if isPropOpen[category]}
        <div class="settings-list">
          {#each schema as item}
            <div class="rule-block">
              <div class="rule-core">
                <div class="rule-name">{item.label}</div>
                <div class="rule-input">
                  {#if item.type === "dropdown"}
                    <Dropdown
                      id="{category}-{item.id}"
                      options={{ choices: item.options }}
                      value={comparer.getTarget(category, item.id)}
                      setValue={(val) =>
                        comparer.setTarget(category, item.id, val)}
                      showCode={false}
                    />
                  {:else if item.type === "number"}
                    <input
                      class="custom-input"
                      type="number"
                      placeholder="Blank = Keep Original"
                      value={comparer.getTarget(category, item.id) || ""}
                      oninput={(e) =>
                        comparer.setTarget(category, item.id, e.target.value)}
                    />
                  {:else if item.type === "boolean"}
                    <button
                      class="toggle-btn {comparer.getTarget(category, item.id)
                        ? 'toggle-active'
                        : ''}"
                      onclick={() =>
                        comparer.setTarget(
                          category,
                          item.id,
                          !comparer.getTarget(category, item.id),
                        )}
                    >
                      {comparer.getTarget(category, item.id)
                        ? "Enabled"
                        : "Disabled"}
                    </button>
                  {/if}
                </div>
              </div>

              {#if item.sourceId}
                <div class="rule-detected">
                  <div class="detected-label">Detected in files:</div>
                  <div class="detected-tags">
                    {#each comparer.getUniqueValues(category, item.sourceId) as foundVal}
                      <span class="tag-item"
                        >{item.options
                          ? getFoundLabel(foundVal, item.options)
                          : foundVal}</span
                      >
                    {/each}
                    {#if comparer.getUniqueValues(category, item.sourceId).length === 0}
                      <span class="tag-empty">No data found</span>
                    {/if}
                  </div>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  .comparer-wrapper {
    width: 100%;
    max-width: 1100px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 15px;
    padding-bottom: 30px;
  }

  .profile-toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin: 10px 20px;
    padding: 15px 25px;
    background-color: rgb(29, 41, 61);
    border: 1px solid rgb(69, 85, 108);
    border-radius: 7px;
  }

  .toolbar-title {
    color: white;
    font-size: 15px;
    font-weight: 600;
  }
  .toolbar-actions {
    display: flex;
    gap: 15px;
  }

  .action-btn {
    background-color: rgb(19, 28, 46);
    border: 1px solid rgb(69, 85, 108);
    padding: 5px 10px;
    color: rgb(186, 197, 211);
    border-radius: 7px;
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 15px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .action-btn:hover {
    background-color: rgb(39, 51, 71);
    color: white;
    border-color: rgb(186, 197, 211);
  }
  .hidden-input {
    display: none;
  }

  .properties_cont {
    width: calc(100% - 40px);
    background-color: rgb(29, 41, 61);
    border: 1px solid rgb(69, 85, 108);
    border-radius: 7px;
    margin: 0 20px;
    padding: 15px;
    display: flex;
    flex-direction: column;
  }

  .cont_title {
    background-color: transparent;
    border: none;
    display: flex;
    flex-direction: row;
    align-items: center;
    width: fit-content;
    cursor: pointer;
    padding-bottom: 10px;
  }
  .chevron {
    display: grid;
    place-items: center;
    transition: 150ms transform ease-in-out;
    transform: rotate(0);
  }
  .open_chevron {
    transform: rotate(90deg) !important;
  }
  .cont_title span {
    color: rgb(186, 197, 211);
    font-size: 15px;
    font-weight: 600;
    margin-left: 5px;
  }

  .settings-list {
    display: flex;
    flex-direction: column;
    gap: 20px;
    margin: 10px;
  }

  .rule-block {
    display: flex;
    flex-direction: column;
    background-color: rgb(19, 28, 46);
    border: 1px solid rgb(69, 85, 108);
    border-radius: 7px;
  }

  .rule-core {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    padding: 15px 25px;
    border-bottom: 1px solid rgb(40, 52, 72);
    border-radius: 7px 7px 0 0;
  }

  .rule-core:only-child {
    border-radius: 7px;
    border-bottom: none;
  }

  .rule-name {
    font-size: 15px;
    font-weight: 600;
    color: white;
  }
  .rule-input {
    width: 350px;
    height: 45px;
    border: 1px solid rgb(69, 85, 108);
    padding: 5px 10px;
    border-radius: 7px;
  }

  .custom-input {
    width: 100%;
    height: 100%;
    background-color: rgb(29, 41, 61);
    border: none;
    border-radius: 7px;
    color: white;
    font-size: 15px;
    font-weight: 500;
    padding: 0 15px;
    outline: none;
    box-sizing: border-box;
  }
  .custom-input::placeholder {
    color: rgb(110, 125, 145);
    font-style: italic;
  }
  .custom-input::-webkit-outer-spin-button,
  .custom-input::-webkit-inner-spin-button {
    -webkit-appearance: none;
  }

  .toggle-btn {
    width: 100%;
    height: 100%;
    background-color: rgb(29, 41, 61);
    border: 1px solid rgb(69, 85, 108);
    color: rgb(186, 197, 211);
    border-radius: 7px;
    font-weight: 600;
    font-size: 15px;
    cursor: pointer;
    transition: all 0.15s ease;
  }
  .toggle-active {
    color: white;
  }

  .rule-detected {
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 15px 25px;
    background-color: rgb(19, 28, 46);
    border-bottom-left-radius: 7px;
    border-bottom-right-radius: 7px;
  }
  .detected-label {
    font-size: 14px;
    font-weight: 600;
    color: rgb(144, 161, 185);
    margin-right: 15px;
    white-space: nowrap;
  }
  .detected-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }
  .tag-item {
    background-color: rgb(29, 41, 61);
    border: 1px solid rgb(69, 85, 108);
    color: white;
    padding: 6px 14px;
    border-radius: 7px;
    font-size: 14px;
    font-weight: 600;
    letter-spacing: 0.5px;
  }
  .tag-empty {
    color: rgb(144, 161, 185);
    font-style: italic;
    font-size: 14px;
    padding: 6px 0;
  }
</style>
