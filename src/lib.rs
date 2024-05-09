use anyhow::Error;
use tauri::WebviewWindow;

mod commands;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the decorum APIs.
pub trait WebviewWindowExt {
    fn create_overlay_titlebar(self) -> Result<WebviewWindow, Error>;
}

impl<'a> WebviewWindowExt for WebviewWindow {
    fn create_overlay_titlebar(self) -> Result<WebviewWindow, Error> {
        self.set_decorations(false)
            .expect("failed to set decorations");

        // get the file script.js as a string
        let script = include_str!("script.js");

        self.eval(script).expect("couldn't run js");
        Ok(self)
    }
}

// Initializes the plugin.
// pub fn init<R: Runtime>() -> TauriPlugin<R> {
//     Builder::new("decorum")
//         // .on_webview_ready(|webview| {
//         // println!("webview is ready");
//         // let label = webview.label();
//         // webview
//         //     .get_webview_window(label)
//         //     .expect("failed to get webview window");
//         // window.set_decorations(false).unwrap();
//         // window
//         //     .set_title("Transparent Titlebar")
//         //     .expect("failed to set title")
//         //     .set_decorations(false)
//         //     .expect("failed to set decorations");
//         // webview.eval("console.log('Hello from Rust!')").unwrap();
//         // })
//         .build()
// }
