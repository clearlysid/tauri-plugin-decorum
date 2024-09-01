use std::collections::HashMap;
use std::sync::Arc;

use config::DecorumConfig;
use parking_lot::RwLock;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Emitter, Listener, LogicalPosition, Manager, Result, Runtime, WebviewWindow};

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod commands;
mod config;

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
    fn create_overlay_titlebar(&self) -> Result<()>;

    #[cfg(target_os = "macos")]
    fn set_window_buttons_inset(&self, options: Option<LogicalPosition<f64>>) -> Result<()>;

    #[cfg(target_os = "macos")]
    fn make_transparent(&self) -> Result<()>;

    #[cfg(target_os = "macos")]
    fn set_window_level(&self, level: NSWindowLevel) -> Result<()>;
}

impl<R: Runtime> WebviewWindowExt for WebviewWindow<R> {
    /// Create a custom titlebar overlay.
    /// This will remove the default titlebar and create a draggable area for the titlebar.
    ///
    /// ## Platform-specific:
    ///
    /// - **Windows:** On Windows, it will also create custom window controls.
    fn create_overlay_titlebar(&self) -> Result<()> {
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

    /// Sets the window controls (Traffic lights) inset
    ///
    /// ## Platform-specific:
    ///
    /// - **macOS:** Only supported on macOS.
    // TODO: Also Implement for Windows 11 (>=22H2)
    #[cfg(target_os = "macos")]
    fn set_window_buttons_inset(&self, inset_option: Option<LogicalPosition<f64>>) -> Result<()> {
        let insets_state = &self.state::<WindowButtonsInsetsState>();
        let mut insets_map = insets_state.0.write();

        let window_label = self.label().to_string();

        match inset_option {
            Some(inset) => {
                if insets_map
                    .insert(window_label, Some(inset.clone()))
                    .is_none()
                {
                    let c_insets_map = insets_map.clone();
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
                insets_map.remove(&window_label);
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

    /// Makes the background of the WKWebView layer transparent.
    /// This differs from Tauri's implementation as it does not change the window background which causes performance performance issues and artifacts when shadows are enabled on the window.
    /// Use Tauri's implementation to make the window itself transparent.
    ///
    /// ## Platform-specific:
    ///
    /// - **macOS:** Only supported on macOS.
    /// This is a private API on macOS, so you cannot use this if your application will be published on the App Store.
    #[cfg(target_os = "macos")]
    fn make_transparent(&self) -> Result<()> {
        use cocoa::{
            base::{id, nil},
            foundation::NSString,
        };

        self.with_webview(|webview| unsafe {
            let wkwebview = webview.inner();
            // `NO` is disallowed, use [NSNumber numberWithBool:NO]
            let no: id = msg_send![class!(NSNumber), numberWithBool:0];
            // Deprecated since OS X 10.14
            // [https://developer.apple.com/documentation/webkit/webview/1408486-drawsbackground]
            let _: id =
                msg_send![wkwebview, setValue:no forKey: NSString::alloc(nil).init_str("drawsBackground")];
        })
    }

    /// Set the window level.   
    /// This will set the window level to the specified value.
    /// NSWindowLevel values can be found [here](https://developer.apple.com/documentation/appkit/NSWindowLevel).
    ///
    /// ## Platform-specific:
    ///
    /// - **macOS:** Only supported on macOS.
    #[cfg(target_os = "macos")]
    fn set_window_level(&self, level: NSWindowLevel) -> Result<()> {
        ensure_main_thread(self, move |win| unsafe {
            let ns_win = win.ns_window()? as cocoa::base::id;
            let _: () = msg_send![ns_win, setLevel: level];
            Ok(())
        })
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
struct WindowButtonsInsetsState(Arc<RwLock<HashMap<String, Option<LogicalPosition<f64>>>>>);

// #[allow(dead_code)]
// struct DecorumConfigState(DecorumConfig);

pub fn init<R: Runtime>() -> TauriPlugin<R, DecorumConfig> {
    let mut builder = Builder::<R, DecorumConfig>::new("decorum")
        .invoke_handler(tauri::generate_handler![
            commands::set_window_buttons_inset,
            commands::show_snap_overlay,
        ])
        .setup(move |app, api| {
            let decorum_config = api.config().clone();

            #[cfg(target_os = "macos")]
            {
                let insets_map: HashMap<String, Option<LogicalPosition<f64>>> = decorum_config
                    .merged
                    .iter()
                    .filter_map(|(label, config)| {
                        // Make sure there's at least one inset defined.
                        config.window_buttons.as_ref().and_then(|buttons| {
                            if buttons.inset_x.is_some() || buttons.inset_y.is_some() {
                                Some((
                                    label.clone(),
                                    Some(LogicalPosition::new(
                                        buttons.inset_x.unwrap_or_default(),
                                        buttons.inset_y.unwrap_or_default(),
                                    )),
                                ))
                            } else {
                                None
                            }
                        })
                    })
                    .collect();

                app.manage(WindowButtonsInsetsState(Arc::new(RwLock::new(insets_map))));
            }

            // app.manage(DecorumConfigState(c_config));
            Ok(())
        })
        .on_page_load(|win, _payload: &tauri::webview::PageLoadPayload| {
            match win.emit("decorum-page-load", ()) {
                Ok(_) => {}
                Err(e) => println!("decorum error: {:?}", e),
            }
        });

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        builder = builder.on_window_ready(|window| {
            let insets_state = window.state::<WindowButtonsInsetsState>();
            let insets_map = insets_state.0.read();

            if let Some(Some(insets)) = insets_map.get(window.label()) {
                #[cfg(target_os = "macos")]
                macos::nswindow_delegates::setup(window.clone(), insets.to_owned());
            }
        });
    }

    builder.build()
}

#[cfg(target_os = "macos")]
pub(crate) fn ensure_main_thread<F, R: Runtime>(
    win: &WebviewWindow<R>,
    main_action: F,
) -> Result<()>
where
    F: FnOnce(&WebviewWindow<R>) -> Result<()> + Send + 'static,
{
    match std::thread::current().name() == Some("main") {
        true => main_action(win),
        false => {
            let c_win = win.clone();
            win.run_on_main_thread(move || main_action(&c_win).unwrap())
        }
    }
}
