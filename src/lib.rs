use tauri::{
    plugin::{Builder, TauriPlugin},
    App, Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;

mod commands;

#[cfg(desktop)]
use desktop::TransparentTitlebar;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the transparent-titlebar APIs.
pub trait TransparentTitlebarExt<R: Runtime> {
    fn transparent_titlebar(&self) -> &TransparentTitlebar<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TransparentTitlebarExt<R> for T {
    fn transparent_titlebar(&self) -> &TransparentTitlebar<R> {
        self.state::<TransparentTitlebar<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("transparent-titlebar")
        .on_webview_ready(|webview| {
            println!("webview is ready");
            let label = webview.label();
            let window = webview
                .get_webview_window(label)
                .expect("failed to get webview window");

            // window.set_decorations(false).unwrap();

            // window
            //     .set_title("Transparent Titlebar")
            //     .expect("failed to set title")
            //     .set_decorations(false)
            //     .expect("failed to set decorations");
            webview.eval("console.log('Hello from Rust!')").unwrap();
        })
        .build()
}
