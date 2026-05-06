import { invoke } from "@tauri-apps/api/core";
import { MediaData, MediaDataComparer } from "./classes.svelte.js";

/** @typedef {import('../../types').AppState} AppState */
/** @typedef {import('../../types').ExplorerNode} ExplorerNode */

export const appState = $state(
  /** @type {AppState} */ ({
    explorer: [],
    enviroment: {},
    selectedMedia: undefined,
    sidebarWidth: 330,
    modal: null,
    data: new MediaData(),
    contentData: new MediaDataComparer(),
    quickMenu: {
      isOpen: false,
      coords: { top: 0, left: 0 },
      options: [],
      resolve: null,
    },
    activeDropdownId: null,
  }),
);

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
    appState.selectedMedia = undefined;
    const ret = await invoke("select_media", {
      isFile,
      refreshPath: appState.enviroment.dataPath,
    });

    appState.explorer = [ret];
  } else {
    appState.selectedMedia = undefined;
    appState.enviroment = {};

    const ret = await invoke("select_media", {
      isFile,
    });
    appState.explorer = [ret];
    appState.enviroment.dataPath = ret.data_path;
    appState.enviroment.isFile = isFile;
  }
}

export async function loadMediaProperties(mediaPath) {
  return invoke("get_media_streams", { path: mediaPath });
}

export async function setSelectedMedia(mediaPath, mediaName, mediaType) {
  if (appState.data.isPendingChanges) {
    const answer = await startModal(
      "Ask",
      "By changing files you discard the changes! Continue?",
      { cancel: "Cancel", agree: "Yes" },
    );
    if (answer) {
      appState.selectedMedia = { mediaPath, mediaName, mediaType };
      appState.data.reset();
    }
  } else {
    appState.selectedMedia = { mediaPath, mediaName, mediaType };
    appState.data.reset();
  }
}

export async function reloadMedia() {
  appState.data.reset();

  await openMedia(appState.enviroment.isFile, true);
  setSelectedMedia(
    appState.selectedMedia.mediaPath,
    appState.selectedMedia.mediaName,
  );
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

  appState.quickMenu.isOpen = false;
  appState.quickMenu.resolve = null;
}

export async function loadContentMediaProperties() {
  appState.contentData.initialized = false;
  appState.contentData.mediaDataFiles = [];

  const fileList = getAllMediaFiles(appState.selectedMedia.mediaPath);

  for (const file of fileList) {
    await loadMediaProperties(file.data_path).then((resp) => {
      resp.path = file.data_path;
      appState.contentData.addMediaData(resp);
    });
  }

  appState.contentData.initialized = true;
}

/**
 * @param {string} folderPath
 * @returns {ExplorerNode[]}
 */

function getAllMediaFiles(folderPath) {
  /** @type {ExplorerNode[]} */
  let fileList = [];

  /**
   * @param {ExplorerNode[]} nodes
   * @param {string} path
   * @returns {ExplorerNode[] | null}
   */
  function findFolder(nodes, path) {
    for (const node of nodes) {
      if (node.data_path === path) return node.children;
      if (node.data_type === "Folder" && node.children) {
        const found = findFolder(node.children, path);
        if (found) return found;
      }
    }
    return null;
  }

  const targetChildren = findFolder(appState.explorer, folderPath);

  if (!targetChildren) return [];

  /**
   * @param {ExplorerNode[]} children
   */
  function iterate(children) {
    for (const media of children) {
      if (media.data_type === "File") {
        fileList.push(media);
      } else if (media.data_type === "Folder" && media.children) {
        iterate(media.children);
      }
    }
  }

  iterate(targetChildren);
  return fileList;
}

class ToastManager {
  messages = $state([]);

  /**
   * Adds a toast message to the screen
   * @param {string} text - The message to display
   * @param {"error" | "success" | "info"} type - The style of the toast
   * @param {number} duration - Time in ms before it disappears (default: 4000)
   */
  add(text, type = "error", duration = 4000) {
    const id = crypto.randomUUID();

    this.messages.push({ id, text, type });

    if (duration > 0) {
      setTimeout(() => {
        this.remove(id);
      }, duration);
    }
  }

  remove(id) {
    this.messages = this.messages.filter((msg) => msg.id !== id);
  }
}

export const toastState = new ToastManager();
