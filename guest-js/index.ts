import { invoke } from "@tauri-apps/api/core";
import { LogicalPosition } from "@tauri-apps/api/dpi";

export async function show_snap_overlay() {
  await invoke("plugin:decorum|show_snap_overlay");
}

export async function setWindowButtonsInset(
  inset: LogicalPosition | null,
  targetLabel: string | null = null
) {
  await invoke("plugin:decorum|set_window_buttons_inset", {
    inset,
    target_label: targetLabel,
  });
}
