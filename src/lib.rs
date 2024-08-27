use std::collections::HashMap;
use std::sync::Arc;

use macos::nswindow_delegates;
use parking_lot::RwLock;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Emitter, Error, Listener, LogicalPosition, Manager, Runtime, WebviewWindow};

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod commands;

#[cfg(target_os = "macos")]
mod macos;

#[repr(u32)]
pub enum NSWindowLevel {
    NSNormalWindowLevel = 0,
    NSFloatingOrSubmenuOrTornOffMenuWindowLevel = 3,
    NSMainMenuWindowLevel = 24,
    NSStatusWindowLevel = 25,
    NSModalPanelWindowLevel = 8,
    NSPopUpMenuWindowLevel = 101,
    NSScreenSaverWindowLevel = 1000,
}

impl From<String> for NSWindowLevel {
    fn from(s: String) -> Self {
        match s.as_str() {
            "NSNormalWindowLevel" => NSWindowLevel::NSNormalWindowLevel,
            "NSFloatingWindowLevel" => NSWindowLevel::NSFloatingOrSubmenuOrTornOffMenuWindowLevel,
            "NSSubmenuWindowLevel" => NSWindowLevel::NSFloatingOrSubmenuOrTornOffMenuWindowLevel,
            "NSTornOffMenuWindowLevel" => {
                NSWindowLevel::NSFloatingOrSubmenuOrTornOffMenuWindowLevel
            }
            "NSMainMenuWindowLevel" => NSWindowLevel::NSMainMenuWindowLevel,
            "NSStatusWindowLevel" => NSWindowLevel::NSStatusWindowLevel,
            "NSModalPanelWindowLevel" => NSWindowLevel::NSModalPanelWindowLevel,
            "NSPopUpMenuWindowLevel" => NSWindowLevel::NSPopUpMenuWindowLevel,
            "NSScreenSaverWindowLevel" => NSWindowLevel::NSScreenSaverWindowLevel,
            _ => panic!("Unknown NSWindowLevel string: {}", s),
        }
    }
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the decorum APIs.
pub trait WebviewWindowExt {
    fn create_overlay_titlebar(&self) -> Result<&WebviewWindow, Error>;
    #[cfg(target_os = "macos")]
    fn set_traffic_lights_inset(
        &self,
        inset: Option<LogicalPosition<f64>>,
    ) -> Result<&WebviewWindow, Error>;
    #[cfg(target_os = "macos")]
    fn make_transparent(&self) -> Result<&WebviewWindow, Error>;
    #[cfg(target_os = "macos")]
    fn set_window_level(&self, level: NSWindowLevel) -> Result<&WebviewWindow, Error>;
}

impl<'a> WebviewWindowExt for WebviewWindow {
    /// Create a custom titlebar overlay.
    /// This will remove the default titlebar and create a draggable area for the titlebar.
    /// On Windows, it will also create custom window controls.
    fn create_overlay_titlebar(&self) -> Result<&WebviewWindow, Error> {
        #[cfg(target_os = "windows")]
        self.set_decorations(false)?;

        let win2 = self.clone();

        self.listen("decorum-page-load", move |_event| {
            // println!("decorum-page-load event received")

            // Create a transparent draggable area for the titlebar
            let script_tb = include_str!("js/titlebar.js");

            win2.eval(script_tb)
                .unwrap_or_else(|e| println!("decorum error: {:?}", e));

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
                        win3.unlisten(_event.id());
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
    fn set_traffic_lights_inset(
        &self,
        inset: Option<LogicalPosition<f64>>,
    ) -> Result<&WebviewWindow, Error> {
        let insets_state = &self.state::<TrafficLightsInsetsState>();
        let mut insets_map = insets_state.0.write();

        let window_label = self.label().to_string();

        match inset {
            Some(inset) => {
                if insets_map.insert(window_label, inset.clone()).is_none() {
                    self.on_window_event(move |event| match event {
                        tauri::WindowEvent::ThemeChanged(_) => {
                            // TODO: Update
                        }
                        _ => (),
                    });
                }

                ensure_main_thread(self, move |win| {
                    // macos::update_traffic_lights_inset(win);
                    Ok(win)
                })
            }
            None => {
                insets_map.remove(&window_label);
                Ok(self)
            }
        }
    }

    /// Set the window background to transparent.
    /// This helper function is different from Tauri's default
    /// as it doesn't use the `transparent` flag or macOS Private APIs.
    #[cfg(target_os = "macos")]
    fn make_transparent(&self) -> Result<&WebviewWindow, Error> {
        use cocoa::{
            base::{id, nil},
            foundation::NSString,
        };

        // Make webview background transparent
        self.with_webview(|webview| unsafe {
            let id = webview.inner();
            let no: id = msg_send![class!(NSNumber), numberWithBool:0];
            let _: id =
                msg_send![id, setValue:no forKey: NSString::alloc(nil).init_str("drawsBackground")];
        })?;

        Ok(self)
    }

    /// Set the window level.
    /// This will set the window level to the specified value.
    /// NSWindowLevel values can be found [here](https://developer.apple.com/documentation/appkit/NSWindowLevel?language=objc).
    /// This is only available on macOS.
    #[cfg(target_os = "macos")]
    fn set_window_level(&self, level: NSWindowLevel) -> Result<&WebviewWindow, Error> {
        ensure_main_thread(self, move |win| {
            let ns_win = win.ns_window()? as cocoa::base::id;
            unsafe {
                let _: () = msg_send![ns_win, setLevel: level];
            }
            Ok(win)
        })
    }
}

#[cfg(target_os = "macos")]
struct TrafficLightsInsetsState(Arc<RwLock<HashMap<String, LogicalPosition<f64>>>>);

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let mut builder = Builder::new("decorum")
        .invoke_handler(tauri::generate_handler![commands::show_snap_overlay])
        .setup(move |app, _api| {
            #[cfg(target_os = "macos")]
            app.manage(TrafficLightsInsetsState(Arc::new(RwLock::new(
                HashMap::new(),
            ))));

            Ok(())
        })
        .on_page_load(|win, _payload: &tauri::webview::PageLoadPayload| {
            match win.emit("decorum-page-load", ()) {
                Ok(_) => {}
                Err(e) => println!("decorum error: {:?}", e),
            }
        });

    #[cfg(target_os = "macos")]
    {
        builder = builder.on_window_ready(|window| {
            // TODO: Only setup if the inset is defined in the config.
            nswindow_delegates::setup(window);
        });
    }

    builder.build()
}

#[cfg(target_os = "macos")]
fn is_main_thread() -> bool {
    std::thread::current().name() == Some("main")
}

#[cfg(target_os = "macos")]
fn ensure_main_thread<F>(
    win: &WebviewWindow,
    main_action: F,
) -> Result<&WebviewWindow, tauri::Error>
where
    F: FnOnce(&WebviewWindow) -> Result<&WebviewWindow, Error> + Send + 'static,
{
    match is_main_thread() {
        true => {
            main_action(win)?;
            Ok(win)
        }
        false => {
            let win2 = win.clone();

            match win.run_on_main_thread(move || {
                main_action(&win2).unwrap();
            }) {
                Ok(_) => Ok(win),
                Err(e) => Err(e),
            }
        }
    }
}
