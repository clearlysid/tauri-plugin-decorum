import { invoke } from "@tauri-apps/api/core";
import { LogicalPosition } from "@tauri-apps/api/dpi";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Window } from "@tauri-apps/api/window";

export async function show_snap_overlay() {
  await invoke("plugin:decorum|show_snap_overlay");
}

/**
 * Sets the window controls inset.
 *
 * ## Platform-specific:
 *
 * - **macOS:** Only supported on macOS.
 *
 * @param inset - The inset position for the window buttons.
 * @param target - The target window. Can be a window label (string), a Window object, or null for the current window.
 * @returns Promise<void>
 */
export async function setWindowButtonsInset(
  inset: LogicalPosition | null,
  target: string | Window | null = null
) {
  let targetLabel: string | null = null;

  if (typeof target === "string") {
    targetLabel = target;
  } else if (target instanceof Window) {
    targetLabel = target.label;
  }

  await invoke("plugin:decorum|set_window_buttons_inset", {
    inset,
    target_label: targetLabel,
  }).catch((e) => {
    console.error("Failed to set window buttons inset:", e);
  });
}

/**
 * Representation of [NSWindowLevel](https://developer.apple.com/documentation/appkit/NSWindowLevel)
 */
export enum NSWindowLevel {
  NSNormalWindowLevel = 0,
  NSFloatingWindowLevel = 3,
  NSSubmenuWindowLevel = 3,
  NSTornOffMenuWindowLevel = 3,
  NSMainMenuWindowLevel = 24,
  NSStatusWindowLevel = 25,
  NSModalPanelWindowLevel = 8,
  NSPopUpMenuWindowLevel = 101,
  NSScreenSaverWindowLevel = 1000,
}

/**
 * Set the window level.
 * This will set the window level to the specified value.
 *
 * ## Platform-specific:
 *
 * - **macOS:** Only supported on macOS.
 *
 * @see {@link NSWindowLevel} for the available window levels.
 * @param level - The window level to set.
 * @returns Promise<void>
 */
export async function setWindowLevel(
  level: NSWindowLevel,
  target: string | Window | null = null
) {
  let targetLabel: string | null = null;

  if (typeof target === "string") {
    targetLabel = target;
  } else if (target instanceof WebviewWindow || target instanceof Window) {
    targetLabel = target.label;
  }

  await invoke("plugin:decorum|set_window_level", {
    level,
    target_label: targetLabel,
  }).catch((e) => {
    console.error("Failed to set window level:", e);
  });
}
