use anyhow::Error;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, WebviewWindow,
};

#[cfg(target_os = "macos")]
mod traffic;

mod commands;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

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

            // Create a transparent draggable area for the titlebar
            let script_tb = include_str!("js/titlebar.js");
            win2.eval(script_tb).expect("couldn't run js");

            // On Windows, create custom window controls
            #[cfg(target_os = "windows")]
            {
                let mut controls = Vec::new();

                if win2.is_minimizable().unwrap_or(false) {
                    controls.push("minimize");
                }

                if win2.is_maximizable().unwrap_or(false) && win2.is_resizable().unwrap_or(false) {
                    controls.push("maximize");
                }

                if win2.is_closable().unwrap_or(false) {
                    controls.push("close");
                }

                let script_controls = include_str!("js/controls.js");

                // this line finds ["minimize", "maximize", "close"] in the file
                // and replaces it with only the controls enabled for the window
                let script_controls = script_controls.replacen(
                    "[\"minimize\", \"maximize\", \"close\"]",
                    &controls.join(", "),
                    1,
                );

                win2.eval(script_controls.as_str())
                    .expect("couldn't run js");
            }
        });

        Ok(self)
    }

    #[cfg(target_os = "macos")]
    fn set_traffic_lights_inset(&self, x: f32, y: f32) -> Result<&WebviewWindow, Error> {
        let ns_window = self.ns_window().expect("couldn't get ns_window");
        let ns_window_handle = traffic::UnsafeWindowHandle(ns_window);

        traffic::position_traffic_lights(ns_window_handle, x.into(), y.into());

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
        .on_window_ready(|window| {
            #[cfg(target_os = "macos")]
            traffic::setup_traffic_light_positioner(window);
            return;
        })
        .build()
}
