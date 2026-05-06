<script>
  import { listen } from "@tauri-apps/api/event";
  import { onMount, tick } from "svelte";

  let { title, handleSelect } = $props();

  let logs = $state([]);
  let logContainer = $state();

  async function addLog(text, type = "normal") {
    const cleanText = text.replace(/[\r\n]+/g, "").trim();
    if (!cleanText) return;

    logs.push({ text: cleanText, type });

    await tick();
    if (logContainer) {
      logContainer.scrollTop = logContainer.scrollHeight;
    }
  }

  onMount(() => {
    let unlistenLog;
    let unlistenProg;

    listen("ffmpeg-log", (event) => {
      const text = event.payload;
      let type = "normal";

      if (text.startsWith("> ffmpeg")) type = "command";
      else if (
        text.toLowerCase().includes("error") ||
        text.toLowerCase().includes("failed")
      )
        type = "error";
      else if (text.startsWith("frame=") || text.startsWith("size="))
        type = "progress";

      addLog(text, type);
    }).then((u) => (unlistenLog = u));

    listen("app-progress", (event) => {
      if (event.payload.title) {
        addLog(`[System] ${event.payload.title}`, "system");
      }
    }).then((u) => (unlistenProg = u));

    return () => {
      if (unlistenLog) unlistenLog();
      if (unlistenProg) unlistenProg();
    };
  });
</script>

<div class="console-wrapper">
  <div class="header">
    <h1>{title}</h1>
  </div>

  <div class="terminal-body" bind:this={logContainer}>
    {#each logs as log}
      <div class="log-line {log.type}">
        {#if log.type === "command"}
          <span class="prompt">$</span>
        {/if}
        {log.text}
      </div>
    {/each}
    <div class="cursor"></div>
  </div>
</div>

<style>
  .console-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background-color: #0d1117;
  }

  .header {
    height: 60px;
    background-color: var(--bg-light);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    padding: 0 15px;
    position: relative;
  }

  h1 {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    font-size: 14px;
    font-weight: 600;
    color: var(--text-dark);
  }

  .terminal-body {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
    font-family: "Courier New", Courier, monospace;
    font-size: 14px;
    line-height: 1.5;
    color: #c9d1d9;
    display: flex;
    flex-direction: column;
  }

  .terminal-body::-webkit-scrollbar {
    width: 8px;
  }
  .terminal-body::-webkit-scrollbar-track {
    background: transparent;
  }
  .terminal-body::-webkit-scrollbar-thumb {
    background: var(--border);
    border-radius: 4px;
  }

  .log-line {
    word-break: break-all;
    margin-bottom: 2px;
  }

  .prompt {
    color: rgb(83, 204, 255);
    margin-right: 5px;
    font-weight: bold;
  }

  .command {
    color: rgb(83, 204, 255);
    font-weight: bold;
    margin-bottom: 10px;
  }
  .system {
    color: rgb(5, 255, 117);
    font-weight: bold;
    margin-top: 5px;
  }
  .error {
    color: #ff5f56;
    font-weight: bold;
  }
  .progress {
    color: #8b949e;
  }
  .normal {
    color: #c9d1d9;
  }

  .cursor {
    display: inline-block;
    width: 8px;
    height: 15px;
    background-color: #c9d1d9;
    animation: blink 1s step-end infinite;
    margin-top: 5px;
  }

  @keyframes blink {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0;
    }
  }
</style>
