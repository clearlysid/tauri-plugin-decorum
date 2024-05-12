use anyhow::Error;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
use tauri::{Manager, WebviewWindow};

mod commands;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the decorum APIs.
pub trait WebviewWindowExt {
    fn create_overlay_titlebar(&self) -> Result<&WebviewWindow, Error>;
    #[cfg(target_os = "macos")]
    fn set_traffic_lights_inset(&self, x: f32, y: f32) -> Result<&WebviewWindow, Error>;
}

impl<'a> WebviewWindowExt for WebviewWindow {
    fn create_overlay_titlebar(&self) -> Result<&WebviewWindow, Error> {
        #[cfg(target_os = "windows")]
        self.set_decorations(false)
            .expect("failed to hide decorations");

        let win2 = self.clone();

        self.listen("decorum-page-load", move |_| {
            // println!("decorum-page-load event received")

            // get the file script.js as a string
            let script = include_str!("script.js");
            win2.eval(script).expect("couldn't run js");
            // The snippet checks for an existing elment with data-tauri-decorum-tb
            // and creates a windows "default" titlebar if not found.
        });

        Ok(self)
    }

    #[cfg(target_os = "macos")]
    fn set_traffic_lights_inset(&self, x: f32, y: f32) -> Result<&WebviewWindow, Error> {
        let ns_window = self.ns_window().expect("couldn't get ns_window");

        Ok(self)
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("decorum")
        .invoke_handler(tauri::generate_handler![commands::show_snap_overlay])
        .on_page_load(|window, _payload| {
            window
                .emit("decorum-page-load", ())
                .expect("couldn't fire decorum-page-load event");
        })
        .build()
}
