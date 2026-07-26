import { getCurrentWindow } from "@tauri-apps/api/window";

const requiredElement = <T extends HTMLElement>(id: string): T => {
  const value = document.getElementById(id);
  if (!value) throw new Error(`Missing window element: ${id}`);
  return value as T;
};

const runWindowAction = (action: () => Promise<void>): void => {
  void action().catch((error: unknown) => {
    console.error("Window operation failed", error);
  });
};

export const installWindowControls = (): void => {
  if (!("__TAURI_INTERNALS__" in window)) return;
  const appWindow = getCurrentWindow();
  const titlebar = requiredElement<HTMLElement>("titlebar");

  requiredElement("windowMinimize").addEventListener("click", () => {
    runWindowAction(() => appWindow.minimize());
  });
  requiredElement("windowMaximize").addEventListener("click", () => {
    runWindowAction(() => appWindow.toggleMaximize());
  });
  requiredElement("windowClose").addEventListener("click", () => {
    runWindowAction(() => appWindow.close());
  });
  titlebar.addEventListener("dblclick", (event) => {
    if (!(event.target as Element).closest(".window-controls")) {
      runWindowAction(() => appWindow.toggleMaximize());
    }
  });
};
