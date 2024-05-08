use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;

mod commands;

#[cfg(desktop)]
use desktop::Decorum;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the decorum APIs.
pub trait WebviewWindowExt<R: Runtime> {
    fn transparent_titlebar(&self) -> &Decorum<R>;
    fn set_native_titlebar(&self) -> Result<(), ()>;
}

impl<R: Runtime, T: Manager<R>> crate::WebviewWindowExt<R> for T {
    fn transparent_titlebar(&self) -> &Decorum<R> {
        println!("transparent_titlebar called");
        self.state::<Decorum<R>>().inner()
    }
    fn set_native_titlebar(&self) -> Result<(), ()> {
        // Windows

        // TODO: self is Window type

        // Get webview of this window
        // Run JS
        // JS:
        // 1. check for presence of titlebar-elements via data
        // 2. if found, attach events
        // 3. if not found, create custom elements and attach events

        // Mac: set titlebar to overlay
        // #[cfg(target_os = "macos")]
        // {
        //     editor_win = editor_win
        //         .title_bar_style(tauri::TitleBarStyle::Overlay)
        //         .hidden_title(true);
        // }

        Ok(())
    }
}

// pub trait WebviewWindowExt {
//     fn set_decorations(&self, decorations: bool) -> Result<(), ()>;
// }

// impl WebviewWindowExt for tauri::WebviewWindow {
//     fn set_decorations(&self, decorations: bool) -> Result<(), ()> {
//         println!("set_decorations called");
//         Ok(())
//     }
// }

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("decorum")
        .on_webview_ready(|webview| {
            println!("webview is ready");
            let label = webview.label();
            webview
                .get_webview_window(label)
                .expect("failed to get webview window");

            // window.set_decorations(false).unwrap();

            // window
            //     .set_title("Transparent Titlebar")
            //     .expect("failed to set title")
            //     .set_decorations(false)
            //     .expect("failed to set decorations");
            // webview.eval("console.log('Hello from Rust!')").unwrap();
        })
        .build()
}
