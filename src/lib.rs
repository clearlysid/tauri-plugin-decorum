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
    #[cfg(target_os = "macos")]
    fn set_transparent(&self) -> Result<&WebviewWindow, Error>;
    #[cfg(target_os = "macos")]
    fn set_window_level(&self, level: u32) -> Result<&WebviewWindow, Error>;
}

impl<'a> WebviewWindowExt for WebviewWindow {
    /// Create a custom titlebar overlay.
    /// This will remove the default titlebar and create a draggable area for the titlebar.
    /// On Windows, it will also create custom window controls.
    fn create_overlay_titlebar(&self) -> Result<&WebviewWindow, Error> {
        #[cfg(target_os = "windows")]
        self.set_decorations(false)?;

        let win2 = self.clone();

        self.listen("decorum-page-load", move |event| {
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
                let controls = format!("{:?}", controls);

                // this line finds ["minimize", "maximize", "close"] in the file
                // and replaces it with only the controls enabled for the window
                let script_controls = script_controls.replacen(
                    "[\"minimize\", \"maximize\", \"close\"]",
                    &controls,
                    1,
                );

                win2.eval(script_controls.as_str())
                    .expect("couldn't run js");

                let win3 = win2.clone();
                win2.on_window_event(move |eve| match eve {
                    tauri::WindowEvent::CloseRequested { .. } => {
                        win3.unlisten(event.id());
                    }
                    _ => {}
                });
            }
        });

        Ok(self)
    }

    /// Set the inset of the traffic lights.
    /// This will move the traffic lights to the specified position.
    /// This is only available on macOS.
    #[cfg(target_os = "macos")]
    fn set_traffic_lights_inset(&self, x: f32, y: f32) -> Result<&WebviewWindow, Error> {
        let ns_window = self.ns_window()?;
        let ns_window_handle = traffic::UnsafeWindowHandle(ns_window);

        traffic::position_traffic_lights(ns_window_handle, x.into(), y.into());

        Ok(self)
    }

    /// Set the window background to transparent.
    /// This helper function is different from Tauri's default
    /// as it doesn't make use of the `transparent` window attribute.
    /// and doesn't need macOS Private APIs.
    #[cfg(target_os = "macos")]
    fn set_transparent(&self) -> Result<&WebviewWindow, Error> {
        use cocoa::{appkit::NSColor, base::nil, foundation::NSString};

        let ns_win = self.ns_window()? as cocoa::base::id;

        unsafe {
            // Make window background transparent
            let win_bg_color = NSColor::colorWithSRGBRed_green_blue_alpha_(nil, 0.0, 0.0, 0.0, 0.0);
            let _: cocoa::base::id = msg_send![ns_win, setBackgroundColor: win_bg_color];
        }

        self.with_webview(|webview| unsafe {
            let id = webview.inner();
            let no: cocoa::base::id = msg_send![class!(NSNumber), numberWithBool:0];
            let _: cocoa::base::id =
                msg_send![id, setValue:no forKey: NSString::alloc(nil).init_str("drawsBackground")];
        })
        .ok();

        Ok(self)
    }

    /// Set the window level.
    /// This will set the window level to the specified value.
    /// This is only available on macOS.
    #[cfg(target_os = "macos")]
    fn set_window_level(&self, level: u32) -> Result<&WebviewWindow, Error> {
        let ns_win = self.ns_window()? as cocoa::base::id;
        unsafe {
            let _: () = msg_send![ns_win, setLevel: level];
        }
        Ok(self)
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("decorum")
        .invoke_handler(tauri::generate_handler![commands::show_snap_overlay])
        .on_page_load(
            |window, _payload| match window.emit("decorum-page-load", ()) {
                Ok(_) => {}
                Err(e) => println!("decorum error: {:?}", e),
            },
        )
        .on_window_ready(|window| {
            #[cfg(target_os = "macos")]
            traffic::setup_traffic_light_positioner(window);
            return;
        })
        .build()
}
