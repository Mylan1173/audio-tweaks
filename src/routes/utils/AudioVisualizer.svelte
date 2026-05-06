<script>
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { emit } from "@tauri-apps/api/event";
  import { appState, closeModal, startModal } from "./state.svelte.js";
  import Svg from "./Svg.svelte";
  import Dropdown from "./Dropdown.svelte";
  import { CHANNEL_LAYOUTS } from "./maps.js";

  let { activeStreamIdx } = $props();

  let lastActiveSteamIdx = $state(null);

  let audioCtx = null;
  let mediaSourceNode = null;
  let splitterNode = null;

  let isLoaded = $state(false);
  let audioDuration = $state(0);
  let assetUrl = $state("");

  let waveformCache = $state.raw([]);
  let NOC = $state(0);

  let containerElement = $state();
  let rulerCanvas = $state();
  let canvases = $state([]);
  let audioElement = $state();

  let scrollLeft = $state(0);
  let viewWidth = $state(0);

  let isPlaying = $state(false);
  let currentTime = $state(0);
  let selectedChannel = $state("all");

  const ZOOM = 10;

  const getChannelCode = (i, total) => {
    return (
      CHANNEL_LAYOUTS[total]?.find((x) => x.id === i)?.code || `CH ${i + 1}`
    );
  };

  const getChannelNames = (total) => {
    const defaultOpt = { code: "all", name: "All Channels" };
    if (!CHANNEL_LAYOUTS[total]) {
      return [
        defaultOpt,
        ...Array.from({ length: total }, (_, i) => ({
          code: i.toString(),
          name: `Channel ${i + 1}`,
        })),
      ];
    }
    return [
      defaultOpt,
      ...CHANNEL_LAYOUTS[total].map((ch) => ({
        code: ch.id.toString(),
        name: ch.name,
      })),
    ];
  };

  $effect(() => {
    if (lastActiveSteamIdx === null) {
      lastActiveSteamIdx = activeStreamIdx;
      return;
    }
    if (activeStreamIdx !== lastActiveSteamIdx) {
      stopAudio();

      isLoaded = false;
      assetUrl = "";
      audioDuration = 0;
      waveformCache = [];
      canvases = [];
      NOC = 0;
      currentTime = 0;
      selectedChannel = "all";

      if (rulerCanvas) {
        const ctx = rulerCanvas.getContext("2d");
        ctx.clearRect(0, 0, rulerCanvas.width, rulerCanvas.height);
      }
      lastActiveSteamIdx = activeStreamIdx;
    }
  });

  $effect(() => {
    if (containerElement) {
      const observer = new ResizeObserver((entries) => {
        viewWidth = entries[0].contentRect.width;
      });
      observer.observe(containerElement);
      return () => observer.disconnect();
    }
  });

  $effect(() => {
    selectedChannel;
    updateAudioRouting();
  });

  async function loadAudio() {
    startModal("Console", "Extracting Audio...");
    try {
      const tempPath = await invoke("get_audio_path", {
        inputPath: appState.selectedMedia.mediaPath,
        streamIndex: activeStreamIdx,
      });

      assetUrl = convertFileSrc(tempPath);

      await emit("app-progress", {
        progress: 0,
        title: "Reading File into Memory...",
      });

      const response = await fetch(assetUrl);
      const arrayBuffer = await response.arrayBuffer();

      await emit("app-progress", {
        progress: 20,
        title: "Decoding Audio Data (Optimized)...",
      });

      const offlineCtx = new window.AudioContext({ sampleRate: 8000 });
      const decodedBuffer = await offlineCtx.decodeAudioData(arrayBuffer);

      NOC = decodedBuffer.numberOfChannels;
      audioDuration = decodedBuffer.duration;

      const cache = [];
      const totalWidth = Math.ceil(audioDuration * ZOOM);
      const sampleRate = decodedBuffer.sampleRate;
      const samplesPerPixel = sampleRate / ZOOM;

      for (let ch = 0; ch < NOC; ch++) {
        await emit("app-progress", {
          progress: 20 + Math.floor(((ch + 1) / NOC) * 80),
          title: `Rendering Channel ${ch + 1} of ${NOC}...`,
        });

        await new Promise((resolve) => setTimeout(resolve, 5));

        const data = decodedBuffer.getChannelData(ch);
        const peaks = new Float32Array(totalWidth * 2);

        for (let x = 0; x < totalWidth; x++) {
          let min = 0;
          let max = 0;
          const start = Math.floor(x * samplesPerPixel);
          const end = Math.floor((x + 1) * samplesPerPixel);

          for (let i = start; i < end; i++) {
            const val = data[i];
            if (val < min) min = val;
            else if (val > max) max = val;
          }
          peaks[x * 2] = min;
          peaks[x * 2 + 1] = max;
        }
        cache.push(peaks);
      }

      waveformCache = cache;
      canvases = new Array(NOC).fill(null);
      isLoaded = true;

      offlineCtx.close();

      if (!audioCtx) {
        audioCtx = new window.AudioContext();
        mediaSourceNode = audioCtx.createMediaElementSource(audioElement);
        updateAudioRouting();
      }
    } catch (e) {
      console.error(e);
    } finally {
      closeModal();
    }
  }

  function updateAudioRouting() {
    if (!mediaSourceNode || !audioCtx) return;

    mediaSourceNode.disconnect();
    if (splitterNode) splitterNode.disconnect();

    if (selectedChannel === "all") {
      mediaSourceNode.connect(audioCtx.destination);
    } else {
      splitterNode = audioCtx.createChannelSplitter(NOC);
      mediaSourceNode.connect(splitterNode);
      splitterNode.connect(audioCtx.destination, parseInt(selectedChannel), 0);
    }
  }

  $effect(() => {
    if (!rulerCanvas || !viewWidth || !isLoaded) return;
    const ctx = rulerCanvas.getContext("2d", { alpha: false });
    const dpr = window.devicePixelRatio || 1;
    rulerCanvas.width = viewWidth * dpr;
    rulerCanvas.height = 30 * dpr;
    ctx.scale(dpr, dpr);

    ctx.fillStyle = "rgb(19, 28, 46)";
    ctx.fillRect(0, 0, viewWidth, 30);
    ctx.fillStyle = "rgb(144, 161, 185)";
    ctx.font = "10px sans-serif";
    ctx.textAlign = "center";

    const startSec = Math.floor(scrollLeft / ZOOM);
    const endSec = Math.ceil((scrollLeft + viewWidth) / ZOOM);

    for (let s = startSec; s <= endSec; s++) {
      const x = s * ZOOM - scrollLeft;
      if (s % 10 === 0) {
        ctx.fillRect(x, 15, 1, 15);
        const mins = Math.floor(s / 60);
        const secs = s % 60;
        ctx.fillText(`${mins}:${secs.toString().padStart(2, "0")}`, x, 10);
      } else if (s % 1 === 0) {
        ctx.fillRect(x, 22, 1, 8);
      }
    }
  });

  $effect(() => {
    if (!viewWidth || waveformCache.length === 0) return;

    waveformCache.forEach((peaks, i) => {
      const canvas = canvases[i];
      if (!canvas) return;

      const ctx = canvas.getContext("2d", { alpha: false });
      const dpr = window.devicePixelRatio || 1;
      canvas.width = viewWidth * dpr;
      canvas.height = 80 * dpr;
      ctx.scale(dpr, dpr);

      ctx.fillStyle = "rgb(29, 41, 61)";
      ctx.fillRect(0, 0, viewWidth, 80);

      ctx.strokeStyle =
        selectedChannel === "all" || parseInt(selectedChannel) === i
          ? "rgb(186, 197, 211)"
          : "rgb(69, 85, 108)";
      ctx.lineWidth = 1;
      const midY = 40;

      ctx.beginPath();
      for (let x = 0; x < viewWidth; x++) {
        const tX = Math.floor(x + scrollLeft);
        const min = peaks[tX * 2] || 0;
        const max = peaks[tX * 2 + 1] || 0;
        if (min !== 0 || max !== 0) {
          ctx.moveTo(x, midY + min * 35);
          ctx.lineTo(x, midY + max * 35);
        }
      }
      ctx.stroke();
    });
  });

  function togglePlay() {
    if (!isLoaded) return;

    if (audioCtx.state === "suspended") {
      audioCtx.resume();
    }

    if (isPlaying) {
      audioElement.pause();
      isPlaying = false;
    } else {
      audioElement.play();
      isPlaying = true;
    }
  }

  function stopAudio() {
    if (audioElement) {
      audioElement.pause();
      audioElement.currentTime = 0;
    }
    isPlaying = false;
    currentTime = 0;
  }

  function handleSeek(e) {
    if (!isLoaded) return;
    const rect = containerElement.getBoundingClientRect();
    const clickX = e.clientX - rect.left + scrollLeft - 50;
    let newTime = clickX / ZOOM;

    if (newTime < 0) newTime = 0;
    if (newTime > audioDuration) newTime = audioDuration;

    currentTime = newTime;
    if (audioElement) audioElement.currentTime = newTime;
  }
</script>

<div class="visualizer-wrapper">
  <audio
    bind:this={audioElement}
    src={assetUrl}
    crossorigin="anonymous"
    ontimeupdate={() => (currentTime = audioElement.currentTime)}
    onended={() => {
      isPlaying = false;
      stopAudio();
    }}
  ></audio>

  <div class="toolbar">
    {#if !isLoaded}
      <button onclick={loadAudio} class="load-button">
        <Svg name="waveform" color="rgb(186, 197, 211)" />
        <span> Load Audio Visualizer</span>
      </button>
    {:else}
      <button onclick={togglePlay} class="play-pause">
        {#if isPlaying}
          <Svg name="pause" color="rgb(186, 197, 211)" />
        {:else}
          <Svg name="play" color="rgb(186, 197, 211)" />
        {/if}
      </button>

      {#if NOC > 0}
        <div class="channel-selector">
          <Dropdown
            id="channel-selector"
            options={{ choices: getChannelNames(NOC), dft: "All Channels" }}
            value={selectedChannel}
            setValue={(code) => (selectedChannel = code)}
            searchable={false}
            showCode={false}
          />
        </div>
      {/if}

      <span class="time-display">
        {Math.floor(currentTime / 60)}:{Math.floor(currentTime % 60)
          .toString()
          .padStart(2, "0")}.{Math.floor((currentTime % 1) * 100)
          .toString()
          .padStart(2, "0")}
      </span>
    {/if}
  </div>

  <div
    class="scroll-container"
    bind:this={containerElement}
    onscroll={(e) => (scrollLeft = e.currentTarget.scrollLeft)}
    onclick={handleSeek}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === "Enter" && handleSeek(e)}
  >
    {#if isLoaded}
      <div
        class="total-content-spacer"
        style:width="{audioDuration * ZOOM + 50}px"
      >
        <div class="ruler-row">
          <div class="sticky-label">Time</div>
          <div
            class="canvas-drawing-area"
            style:transform="translateX({scrollLeft}px)"
          >
            <canvas bind:this={rulerCanvas}></canvas>
          </div>
        </div>

        {#each waveformCache as peaks, i}
          <div class="channel-track-row">
            <div class="sticky-label">{getChannelCode(i, NOC)}</div>
            <div
              class="canvas-drawing-area"
              style:transform="translateX({scrollLeft}px)"
            >
              <canvas bind:this={canvases[i]}></canvas>
            </div>
          </div>
        {/each}

        <div
          class="playhead"
          style:transform="translateX({currentTime * ZOOM + 50}px)"
        ></div>
      </div>
    {/if}
  </div>
</div>

<style>
  .visualizer-wrapper {
    margin: 0 10px;
    display: flex;
    flex-direction: column;
    width: calc(100% - 20px);
  }

  .toolbar {
    display: flex;
    gap: 20px;
    align-items: center;
    padding: 10px 0;
    color: var(--text-dark);
  }

  .time-display {
    font-family: monospace;
    font-size: 16px;
    margin-left: auto;
  }

  .scroll-container {
    width: 100%;
    max-width: 100vw;
    overflow-x: auto;
    display: block;
    background: var(--bg-dark);
    position: relative;
    cursor: text;
    border-radius: 10px;
  }

  .scroll-container::-webkit-scrollbar {
    cursor: grab !important;
    height: 12px;
  }
  .scroll-container::-webkit-scrollbar-track {
    background: var(--bg-dark);
  }
  .scroll-container::-webkit-scrollbar-thumb {
    background: var(--border);
    border-radius: 6px;
    border: 3px solid var(--bg-dark);
  }

  .total-content-spacer {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding-bottom: 20px;
    position: relative;
  }

  .ruler-row {
    position: sticky;
    top: 0;
    height: 30px;
    background: var(--bg-dark);
    width: 100%;
    z-index: 10;
    display: flex;
  }

  .channel-track-row {
    position: relative;
    height: 80px;
    background: var(--bg-light);
    width: 100%;
    display: flex;
  }

  .sticky-label {
    position: sticky;
    left: 0;
    width: 50px;
    min-width: 50px;
    background: var(--bg-dark);
    z-index: 30;
    display: flex;
    align-items: center;
    justify-content: center;
    border-right: 1px solid var(--border);
    font-size: 11px;
    color: var(--text-dark);
    font-weight: bold;
  }

  .canvas-drawing-area {
    position: absolute;
    top: 0;
    left: 50px;
    will-change: transform;
    z-index: 10;
  }

  .playhead {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 2px;
    background: rgb(58, 138, 243);
    z-index: 50;
    pointer-events: none;
    box-shadow: 0 0 4px rgba(58, 138, 243, 0.5);
  }

  .load-button {
    margin: 0 auto;
    width: fit-content;
    height: 40px;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    padding: 10px;
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

  .play-pause {
    background-color: transparent;
    height: 40px;
    width: 40px;
    display: grid;
    place-items: center;
    border-radius: 10px;
    border: 1px solid var(--border);
    transition: 100ms all ease-in-out;
    cursor: pointer;

    &:hover {
      border-color: var(--text-light);
      box-shadow: var(--text-light) 0px 0px 2px;
    }
    &:active {
      transform: scale(97%);
    }
  }

  .channel-selector {
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 0 20px;
    width: 200px;
    height: 40px;
    font-size: 16px;
    font-weight: 500;
    color: var(--text-light);
    display: grid;
    place-items: center;
    transition: 100ms all ease-in-out;

    &:hover {
      border-color: var(--text-light);
      box-shadow: var(--text-light) 0px 0px 2px;
    }
  }
</style>
