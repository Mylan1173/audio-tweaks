<script>
  import { appState } from "./state.svelte.js";
  import FileExplorer from "./FileExplorer.svelte";
  import MediaProperties from "./MediaProperties.svelte";

  let isResizing = $state(false);

  function startResizing(e) {
    isResizing = true;
    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseup", stopResizing);
    // Prevent text selection while dragging
    document.body.style.cursor = "col-resize";
    document.body.style.userSelect = "none";
  }

  function handleMouseMove(e) {
    if (!isResizing) return;
    if (e.clientX > 300 && e.clientX < 600) {
      appState.sidebarWidth = e.clientX - 10;
    }
  }

  function stopResizing() {
    isResizing = false;
    window.removeEventListener("mousemove", handleMouseMove);
    window.removeEventListener("mouseup", stopResizing);
    document.body.style.cursor = "default";
    document.body.style.userSelect = "auto";
  }
</script>

<main
  id="main"
  style="grid-template-columns: {appState.sidebarWidth}px 4px 1fr;"
>
  <FileExplorer />
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div id="resizer" role="separator" onmousedown={startResizing}></div>
  <MediaProperties />
</main>

<style>
  /* 
DARK BG: (19, 28, 46)
LIGHT BG: (29, 41, 61)
BTN BORDER: 69, 85, 108
BTN COLOR: 186, 197, 211
TEXT COLOR: 144, 161, 185
*/
  :global(*) {
    margin: 0;
    padding: 0;
    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
    font-size: 12px;
    font-weight: 400;
    box-sizing: border-box;
    user-select: none;
  }

  :root {
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    overflow: hidden;

    color: rgb(255, 255, 255);
    background-color: rgb(19, 28, 46);

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  #main {
    display: grid;
    width: 100vw;
    height: 100vh;
    padding: 10px;
  }

  #resizer {
    width: 4px;
    cursor: col-resize;
    background: transparent;
    transition: background 100ms ease-in-out;
    z-index: 10;
  }
</style>
