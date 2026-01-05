<script>
// @ts-nocheck

    const props = $props()
    const attrbutes = {
        height: "24px",
        width: "24px",
        viewBox: "0 -960 960 960",
        fill: props.color || "#ffffff"
    }
    const svgs = {
        file_open: `<path d="M240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v240h-80v-200H520v-200H240v640h360v80H240Zm638 15L760-183v89h-80v-226h226v80h-90l118 118-56 57Zm-638-95v-640 640Z" />`,
        folder_open: `<path d="M160-160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h240l80 80h320q33 0 56.5 23.5T880-640H447l-80-80H160v480l96-320h684L837-217q-8 26-29.5 41.5T760-160H160Zm84-80h516l72-240H316l-72 240Zm0 0 72-240-72 240Zm-84-400v-80 80Z" />`
    }
</script>

<svg xmlns="http://www.w3.org/2000/svg" {...attrbutes}>
    {@html svgs[props.name]}
</svg>
