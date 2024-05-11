use anyhow::Error;
use tauri::WebviewWindow;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the decorum APIs.
pub trait WebviewWindowExt {
    #[cfg(target_os = "windows")]
    fn create_overlay_titlebar(self) -> Result<WebviewWindow, Error>;
    #[cfg(target_os = "macos")]
    fn set_traffic_light_inset(self, x: f32, y: f32) -> Result<WebviewWindow, Error>;
}

impl<'a> WebviewWindowExt for WebviewWindow {
    #[cfg(target_os = "windows")]
    fn create_overlay_titlebar(self) -> Result<WebviewWindow, Error> {
        self.set_decorations(false)
            .expect("failed to set decorations");

        // get the file script.js as a string
        // The snippet checks for ab existing elment with data-tauri-decorum-tb
        // and creates a windows "default" titlebar if not found.
        let script = include_str!("script.js");
        self.eval(script).expect("couldn't run js");

        // TODO: ensure this script is re-run on reload

        Ok(self)
    }

    #[cfg(target_os = "macos")]
    fn set_traffic_light_inset(self, x: f32, y: f32) -> Result<WebviewWindow, Error> {
        let ns_window = self.ns_window().expect("couldn't get ns_window");

        Ok(self)
    }
}

// init the plugin, and also handle the onload maybe???
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("decorum")
        .invoke_handler(tauri::generate_handler![commands::show_snap_overlay])
        .on_page_load(|window, _payload| {
            // window.eval("console.warn('RELOAD kyu kiya bkl')").unwrap();

            let script = include_str!("script.js");
            window.eval(script).expect("couldn't run js");
        })
        .build()
}
