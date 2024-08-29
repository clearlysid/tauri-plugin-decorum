use std::collections::HashMap;
use std::sync::Arc;

use macos::nswindow_delegates;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Emitter, Listener, LogicalPosition, Manager, Runtime, WebviewWindow};

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod commands;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
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

#[cfg(target_os = "macos")]
impl From<u32> for NSWindowLevel {
    fn from(n: u32) -> Self {
        match n {
            0 => NSWindowLevel::NSNormalWindowLevel,
            3 => NSWindowLevel::NSFloatingOrSubmenuOrTornOffMenuWindowLevel,
            8 => NSWindowLevel::NSModalPanelWindowLevel,
            24 => NSWindowLevel::NSMainMenuWindowLevel,
            25 => NSWindowLevel::NSStatusWindowLevel,
            101 => NSWindowLevel::NSPopUpMenuWindowLevel,
            1000 => NSWindowLevel::NSScreenSaverWindowLevel,
            _ => NSWindowLevel::NSNormalWindowLevel,
        }
    }
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the decorum APIs.
pub trait WebviewWindowExt {
    fn create_overlay_titlebar(&self) -> tauri::Result<()>;

    #[cfg(target_os = "macos")]
    fn set_window_buttons_inset(&self, options: Option<LogicalPosition<f64>>) -> tauri::Result<()>;

    #[cfg(target_os = "macos")]
    fn make_transparent(&self) -> tauri::Result<()>;

    #[cfg(target_os = "macos")]
    fn set_window_level(&self, level: NSWindowLevel) -> tauri::Result<()>;
}

impl<R: Runtime> WebviewWindowExt for WebviewWindow<R> {
    /// Create a custom titlebar overlay.
    /// This will remove the default titlebar and create a draggable area for the titlebar.
    /// On Windows, it will also create custom window controls.
    fn create_overlay_titlebar(&self) -> tauri::Result<()> {
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

        Ok(())
    }

    /// Set the window background to transparent.
    /// This helper function is different from Tauri's default
    /// as it doesn't use the `transparent` flag or macOS Private APIs.
    #[cfg(target_os = "macos")]
    fn make_transparent(&self) -> tauri::Result<()> {
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

        Ok(())
    }

    /// Set the inset of the traffic lights.
    /// This will move the traffic lights to the specified position.
    /// This is only available on macOS.
    /// TODO: Also Implement for Windows 11 (>=22H2)
    #[cfg(target_os = "macos")]
    fn set_window_buttons_inset(
        &self,
        inset_option: Option<LogicalPosition<f64>>,
    ) -> tauri::Result<()> {
        let styles_state = &self.state::<WindowButtonsInsetsState>();
        let mut styles_map = styles_state.0.write();

        let window_label = self.label().to_string();

        match inset_option {
            Some(inset) => {
                if styles_map
                    .insert(window_label, Some(inset.clone()))
                    .is_none()
                {
                    let c_insets_map = styles_map.clone();
                    let c_win = self.clone();

                    self.on_window_event(move |event| match event {
                        tauri::WindowEvent::ThemeChanged(..) => {
                            if c_insets_map.contains_key(c_win.label()) {
                                let _ = ensure_main_thread(&c_win, move |win| {
                                    macos::update_window_controls_inset(&win.as_ref().window());
                                    Ok(())
                                });
                            }
                        }
                        _ => (),
                    });
                }
            }
            None => {
                styles_map.remove(&window_label);
            }
        }

        ensure_main_thread(self, move |win| {
            let inset = inset_option.unwrap_or(macos::DEFAULT_TRAFFIC_LIGHTS_INSET);

            macos::position_window_controls(
                macos::nswindow_delegates::UnsafeWindowHandle(
                    win.ns_window().expect("Failed to create window handle"),
                ),
                inset.x,
                inset.y,
            );

            Ok(())
        })
    }

    /// Set the window level.   
    /// This will set the window level to the specified value.
    /// NSWindowLevel values can be found [here](https://developer.apple.com/documentation/appkit/NSWindowLevel?language=objc).
    /// This is only available on macOS.
    #[cfg(target_os = "macos")]
    fn set_window_level(&self, level: NSWindowLevel) -> tauri::Result<()> {
        ensure_main_thread(self, move |win| unsafe {
            let ns_win = win.ns_window()? as cocoa::base::id;
            let _: () = msg_send![ns_win, setLevel: level];
            Ok(())
        })
    }
}

#[cfg(not(target_os = "linux"))]
struct WindowButtonsInsetsState(
    Arc<parking_lot::RwLock<HashMap<String, Option<LogicalPosition<f64>>>>>,
);

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let mut builder = Builder::new("decorum")
        .invoke_handler(tauri::generate_handler![
            commands::set_window_buttons_inset,
            commands::show_snap_overlay,
        ])
        .setup(move |app, _api| {
            #[cfg(not(target_os = "linux"))]
            app.manage(WindowButtonsInsetsState(Arc::new(
                parking_lot::RwLock::new(HashMap::new()),
            )));

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
pub(crate) fn ensure_main_thread<F, R: Runtime>(
    win: &WebviewWindow<R>,
    main_action: F,
) -> tauri::Result<()>
where
    F: FnOnce(&WebviewWindow<R>) -> tauri::Result<()> + Send + 'static,
{
    match std::thread::current().name() == Some("main") {
        true => main_action(win),
        false => {
            let c_win = win.clone();
            win.run_on_main_thread(move || main_action(&c_win).unwrap())
        }
    }
}
