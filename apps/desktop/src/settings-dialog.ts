const requiredElement = <T extends HTMLElement>(id: string): T => {
  const value = document.getElementById(id);
  if (!value) throw new Error(`設定画面の要素が見つかりません: ${id}`);
  return value as T;
};

export const installSettingsDialog = (): void => {
  const dialog = requiredElement<HTMLDialogElement>("settingsDialog");

  requiredElement("openSettings").addEventListener("click", () => {
    dialog.showModal();
  });
  requiredElement("closeSettings").addEventListener("click", () => {
    dialog.close();
  });
  dialog.addEventListener("click", (event) => {
    if (event.target === dialog) dialog.close();
  });
};
