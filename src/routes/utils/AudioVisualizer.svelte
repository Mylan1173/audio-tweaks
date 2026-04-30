<script>
  // @ts-nocheck

  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { emit } from "@tauri-apps/api/event";
  import { appState, closeModal, startModal } from "./state.svelte.js";
  import Svg from "./Svg.svelte";
  import Dropdown from "./Dropdown.svelte";
  import { CHANNEL_LAYOUTS } from "./maps.js";

  let { activeStreamIdx } = $props();

  let lastActiveSteamIdx = $state(null);

  let audioCtx = $state();
  let audioBuffer = $state();
  let waveformCache = $state([]);
  let NOC = $state(0);

  let containerElement = $state();
  let rulerCanvas = $state();
  let canvases = $state([]);

  let scrollLeft = $state(0);
  let viewWidth = $state(0);

  let isPlaying = $state(false);
  let currentTime = $state(0);
  let selectedChannel = $state("all");

  let sourceNode = null;
  let startTime = 0;
  let pauseTime = 0;
  let animationFrame = null;

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

      audioBuffer = null;
      waveformCache = [];
      canvases = [];
      NOC = 0;
      currentTime = 0;
      pauseTime = 0;
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

  async function loadAudio() {
    startModal("ProgressBar", "Extracting Audio...");
    try {
      if (!audioCtx) audioCtx = new window.AudioContext();

      const tempPath = await invoke("get_audio_path", {
        inputPath: appState.selectedMedia.mediaPath,
        streamIndex: activeStreamIdx,
      });

      await emit("app-progress", {
        progress: 0,
        title: "Reading File into Memory...",
      });

      const assetUrl = convertFileSrc(tempPath);
      const response = await fetch(assetUrl);
      const arrayBuffer = await response.arrayBuffer();

      await emit("app-progress", {
        progress: 20,
        title: "Decoding Audio Data...",
      });

      audioBuffer = await audioCtx.decodeAudioData(arrayBuffer);
      NOC = audioBuffer.numberOfChannels;

      const cache = [];
      const totalWidth = Math.ceil(audioBuffer.duration * ZOOM);
      const sampleRate = audioBuffer.sampleRate;
      const samplesPerPixel = sampleRate / ZOOM;

      for (let ch = 0; ch < NOC; ch++) {
        await emit("app-progress", {
          progress: 20 + Math.floor(((ch + 1) / NOC) * 80),
          title: `Rendering Channel ${ch + 1} of ${NOC}...`,
        });

        await new Promise((resolve) => setTimeout(resolve, 5));

        const data = audioBuffer.getChannelData(ch);
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
    } catch (e) {
      console.error(e);
    } finally {
      closeModal();
    }
  }

  $effect(() => {
    if (!rulerCanvas || !viewWidth || !audioBuffer) return;
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

  function updateTime() {
    if (isPlaying && audioCtx) {
      currentTime = audioCtx.currentTime - startTime;
      if (currentTime >= audioBuffer.duration) {
        stopAudio();
        currentTime = audioBuffer.duration;
      } else {
        animationFrame = requestAnimationFrame(updateTime);
      }
    }
  }

  function handleChannelChange() {
    if (isPlaying) {
      stopAudio();
      togglePlay();
    }
  }

  function togglePlay() {
    if (!audioBuffer) return;

    if (audioCtx.state === "suspended") {
      audioCtx.resume();
    }

    if (isPlaying) {
      stopAudio();
      pauseTime = currentTime;
    } else {
      sourceNode = audioCtx.createBufferSource();
      sourceNode.buffer = audioBuffer;

      if (selectedChannel === "all") {
        sourceNode.connect(audioCtx.destination);
      } else {
        const splitter = audioCtx.createChannelSplitter(NOC);
        sourceNode.connect(splitter);
        splitter.connect(audioCtx.destination, parseInt(selectedChannel), 0);
      }

      startTime = audioCtx.currentTime - pauseTime;
      sourceNode.start(0, pauseTime);
      isPlaying = true;
      animationFrame = requestAnimationFrame(updateTime);
    }
  }

  function stopAudio() {
    if (sourceNode) {
      sourceNode.stop();
      sourceNode.disconnect();
      sourceNode = null;
    }
    isPlaying = false;
    cancelAnimationFrame(animationFrame);
  }

  function handleSeek(e) {
    if (!audioBuffer) return;
    const rect = containerElement.getBoundingClientRect();
    const clickX = e.clientX - rect.left + scrollLeft - 50;
    let newTime = clickX / ZOOM;

    if (newTime < 0) newTime = 0;
    if (newTime > audioBuffer.duration) newTime = audioBuffer.duration;

    const wasPlaying = isPlaying;
    if (isPlaying) stopAudio();

    currentTime = newTime;
    pauseTime = newTime;

    if (wasPlaying) togglePlay();
  }
</script>

<div class="visualizer-wrapper">
  <div class="toolbar">
    {#if !audioBuffer}
      <button onclick={loadAudio} class="load-button"
        ><Svg name="waveform" color="rgb(186, 197, 211)" />
        <span> Load Audio Visualizer</span></button
      >
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
    onscroll={(e) => (scrollLeft = e.target.scrollLeft)}
    onclick={handleSeek}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === "Enter" && handleSeek(e)}
  >
    {#if audioBuffer}
      <div
        class="total-content-spacer"
        style:width="{audioBuffer.duration * ZOOM + 50}px"
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
    color: rgb(144, 161, 185);
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
    background: rgb(19, 28, 46);
    position: relative;
    cursor: text;
    border-radius: 10px;
  }

  .scroll-container::-webkit-scrollbar {
    cursor: grab !important;
    height: 12px;
  }
  .scroll-container::-webkit-scrollbar-track {
    background: rgb(19, 28, 46);
  }
  .scroll-container::-webkit-scrollbar-thumb {
    background: rgb(69, 85, 108);
    border-radius: 6px;
    border: 3px solid rgb(19, 28, 46);
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
    background: rgb(19, 28, 46);
    width: 100%;
    z-index: 10;
    display: flex;
  }

  .channel-track-row {
    position: relative;
    height: 80px;
    background: rgb(29, 41, 61);
    width: 100%;
    display: flex;
  }

  .sticky-label {
    position: sticky;
    left: 0;
    width: 50px;
    min-width: 50px;
    background: rgb(19, 28, 46);
    z-index: 30;
    display: flex;
    align-items: center;
    justify-content: center;
    border-right: 1px solid rgb(69, 85, 108);
    font-size: 11px;
    color: rgb(144, 161, 185);
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
    border: 1px solid rgb(69, 85, 108);
    background-color: transparent;
    cursor: pointer;
    gap: 10px;
    transition: 100ms all ease-in-out;

    span {
      font-size: 16px;
      font-weight: 600;
      color: rgb(186, 197, 211);
    }
    &:hover {
      border-color: rgb(186, 197, 211);
      box-shadow: rgb(186, 197, 211) 0px 0px 2px;
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
    border: 1px solid rgb(69, 85, 108);
    transition: 100ms all ease-in-out;
    cursor: pointer;

    &:hover {
      border-color: rgb(186, 197, 211);
      box-shadow: rgb(186, 197, 211) 0px 0px 2px;
    }
    &:active {
      transform: scale(97%);
    }
  }

  .channel-selector {
    border: 1px solid rgb(69, 85, 108);
    border-radius: 10px;
    padding: 0 20px;
    width: 200px;
    height: 40px;
    font-size: 16px;
    font-weight: 500;
    color: rgb(186, 197, 211);
    display: grid;
    place-items: center;
    transition: 100ms all ease-in-out;

    &:hover {
      border-color: rgb(186, 197, 211);
      box-shadow: rgb(186, 197, 211) 0px 0px 2px;
    }
  }
</style>
