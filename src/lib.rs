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
pub trait DecorumExt<R: Runtime> {
    fn transparent_titlebar(&self) -> &Decorum<R>;
}

impl<R: Runtime, T: Manager<R>> crate::DecorumExt<R> for T {
    fn transparent_titlebar(&self) -> &Decorum<R> {
        self.state::<Decorum<R>>().inner()
    }
}

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
