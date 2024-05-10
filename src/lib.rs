use anyhow::Error;
use tauri::WebviewWindow;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
  };

mod commands;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the decorum APIs.
#[cfg(target_os = "windows")]
pub trait WebviewWindowExt {
    fn create_overlay_titlebar(self) -> Result<WebviewWindow, Error>;
}

#[cfg(not(target_os = "windows"))]
pub trait WebviewWindowExt {
    fn set_traffic_light_inset(self, x: f32, y: f32) -> Result<WebviewWindow, Error>;
}

#[cfg(target_os = "windows")]
impl<'a> WebviewWindowExt for WebviewWindow {
    fn create_overlay_titlebar(self) -> Result<WebviewWindow, Error> {
        self.set_decorations(false)
            .expect("failed to set decorations");

        // get the file script.js as a string
        let script = include_str!("script.js");

        self.eval(script).expect("couldn't run js");

        // The snippet checks for ab existing elment with data-tauri-decorum-tb
        // and creates a windows "default" titlebar if not found.

        // TODO: attach window control events to the buttons
        // TODO: ensure this script is re-run on reload
        // currently it will disappear if one reloads the page

        Ok(self)
    }

}

#[cfg(not(target_os = "windows"))]
impl<'a> WebviewWindowExt for WebviewWindow {
    fn set_traffic_light_inset(self, x: f32, y: f32) -> Result<WebviewWindow, Error> {
        let ns_window = self.ns_window().expect("couldn't get ns_window");

        Ok(self)
    }
}

// init the plugin, and also handle the onload maybe???
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("decorum")
    //   .invoke_handler(tauri::generate_handler![commands::execute])
      .on_page_load(|window, _payload| {
        window.eval("console.warn('RELOAD kyu kiya bkl')").unwrap();
        // TODO: self will be window here i think
        //self.set_decorations(false)
        // .expect("failed to set decorations");
        let script = include_str!("script.js");
        window.eval(script).expect("couldn't run js");
      })
      .build()
  }
  
