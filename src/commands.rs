use tauri::{LogicalPosition, Manager, Result, Runtime, WebviewWindow};

use crate::WebviewWindowExt;

#[tauri::command]
pub async fn show_snap_overlay() {
    #[cfg(target_os = "windows")]
    {
        use enigo::{Enigo, Key, KeyboardControllable};

        // press win + z using enigo
        let mut enigo = Enigo::new();
        enigo.key_down(Key::Meta);
        enigo.key_click(Key::Layout('z'));
        enigo.key_up(Key::Meta);

        // Wait 50 ms
        std::thread::sleep(std::time::Duration::from_millis(50));

        // Press Alt to hide the ugly numbers
        enigo.key_click(Key::Alt);
    }
}

#[tauri::command]
pub async fn set_window_buttons_inset<R: Runtime>(
    window: WebviewWindow<R>,
    inset: Option<LogicalPosition<f64>>,
    target_label: Option<String>,
) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        let target = match target_label {
            Some(label) => window
                .get_webview_window(&label)
                .ok_or_else(|| tauri::Error::WindowNotFound)?,
            None => window,
        };

        target.set_window_buttons_inset(inset)
    }

    #[cfg(not(target_os = "macos"))]
    "This command is only supported on macOS."
}
