import { invoke } from "@tauri-apps/api/core";

export const appState = $state({
  explorer: [],
  enviroment: {},
  selected_file: undefined,
  media_properties: {},
  sidebarWidth: 330,
  modal: null,
  pendingChanges: {},
  quickMenu: {
    isOpen: false,
    coords: { top: 0, left: 0 },
    options: [],
    resolve: null,
  },
});

export async function startModal(type, title, options) {
  if (!appState.modal) return null;
  return await appState.modal.showModal(type, title, options);
}

export async function closeModal() {
  appState.modal.handleSelect(false);
}

export async function openMedia(isFile, doRefresh = false) {
  appState.explorer = [];

  if (doRefresh) {
    const ret = await invoke("select_media", {
      isFile,
      refreshPath: appState.enviroment.dataPath,
    });

    appState.explorer = ret.data_type ? [ret] : ret.children;
  } else {
    appState.selected_file = undefined;
    appState.media_properties = {};
    appState.enviroment = {};

    const ret = await invoke("select_media", {
      isFile,
      refreshPath: undefined,
    });

    if (isFile) {
      appState.explorer = [ret];
      appState.enviroment.name = "New Project";
      appState.enviroment.dataPath = ret.data_path;
      appState.enviroment.isFile = true;
    } else {
      appState.explorer = ret.children;
      appState.enviroment.name = ret.data_name;
      appState.enviroment.dataPath = ret.data_path;
      appState.enviroment.isFile = false;
    }
  }
}

export async function setSelectedFile(path, name) {
  if (Object.keys(appState.pendingChanges).length !== 0) {
    const answer = await startModal(
      "Ask",
      "By changing files you discard the changes! Continue?",
      { cancel: "Cancel", agree: "Yes" },
    );
    if (answer) {
      appState.selected_file = {};
      appState.selected_file = { path, name };
      appState.pendingChanges = {};
    }
  } else {
    appState.selected_file = {};
    appState.selected_file = { path, name };
    appState.pendingChanges = {};
  }
}

export function openQuickMenu(target, options) {
  const rect = target.getBoundingClientRect();

  return new Promise((resolve) => {
    setTimeout(() => {
      appState.quickMenu = {
        isOpen: true,
        options,
        resolve,
        coords: {
          left: rect.left + rect.width / 2,
          top: rect.bottom,
          height: rect.height,
        },
      };
    }, 0);
  });
}

export function closeQuickMenu(value = null) {
  if (appState.quickMenu.resolve) {
    appState.quickMenu.resolve(value);
  }
  console.log(124);

  appState.quickMenu.isOpen = false;
  appState.quickMenu.resolve = null;
}
