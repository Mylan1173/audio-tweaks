<script>
  import FileExplorer from "./FileExplorer.svelte";
  import MediaProperties from "./MediaProperties.svelte";
</script>

<main id="main">
  <FileExplorer />
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
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
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
  height: calc(100vh - 20px);
  width: 100vw;
  display: flex;
  flex-direction: row;
}



</style>
