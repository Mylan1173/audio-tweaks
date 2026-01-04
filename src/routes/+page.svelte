<script>
  import {onMount, onDestroy} from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from "@tauri-apps/api/core";
  import { convertFileSrc } from '@tauri-apps/api/core';
  import {audioDir} from "@tauri-apps/api/path"
  import WaveSurfer from 'wavesurfer.js'

  let name = $state("");
  let greetMsg = $state("");

  let audioPath = $state("")
  let audioFile = $state("swatch.mp3");
  let audioUrl = $derived(convertFileSrc(`${audioPath}/${audioFile}`));

  let folderPath = $state("E:/Sounds")
  let folderUrl = $derived(convertFileSrc(folderPath))
  let folderContents = $state([])

  /**
   * @type {HTMLDivElement}
   */
  let waveformContainer;
  /**
   * @type {WaveSurfer}
   */
  let wavesurfer;

  async function greet(event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
  // Customizable properties
  
  let waveColor = '#4b5563'; // Tailwind Gray-600
  let progressColor = '#3b82f6'; // Tailwind Blue-500

  onMount(async () => {
    wavesurfer = WaveSurfer.create({
      container: waveformContainer,
      waveColor: waveColor,
      progressColor: progressColor,
      cursorColor: '#ffffff',
     /*  barWidth: 2,
      barGap: 3,
      barRadius: 3, */
      height: 128,
      normalize: true, // Auto-scales volume for small waveforms
    });
    audioPath = await audioDir()
    if (audioUrl) {
      wavesurfer.load(audioUrl);
    }
  });

  onDestroy(() => {
    if (wavesurfer) wavesurfer.destroy();
  });

  const getContents = async () => {
    folderContents = await invoke("list_audio_files", { app: folderPath })
  }

  const playPause = () => wavesurfer.playPause();
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="waveform-box">
  <div bind:this={waveformContainer}></div>
  <div class="controls">
    <button onclick={playPause}>Play / Pause</button>
  </div>
</div>

  <form class="row">
    <input id="greet-input" placeholder="Enter a audio file..." bind:value={audioFile} autocomplete="off"/>
    <button type="submit" onclick={() => wavesurfer.load(audioUrl)}>Greet</button>
  </form>
  <!-- <p>{greetMsg}asdawdawd</p> -->
  <p>{folderUrl}</p>

  <input type="text" bind:value={folderPath}>
  <button onclick={getContents}>Get Folder Contents</button>
  {#each folderContents as cont}
    <p>{cont}</p>
  {/each}
</main>

<style>

.waveform-box {
    background: #111827; /* Dark background for professional feel */
    padding: 20px;
    border-radius: 8px;
  }

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
</style>
