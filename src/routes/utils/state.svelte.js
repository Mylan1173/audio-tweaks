export const appState = $state({
  explorer: [],
  selected_file: undefined,
  media_properties: {},
  sidebarWidth: 330,
  modal: null,
  pendingChanges: {},
});

export async function askModal(title, options) {
  if (!appState.modal) return null;
  return await appState.modal.showModal(title, options);
}
