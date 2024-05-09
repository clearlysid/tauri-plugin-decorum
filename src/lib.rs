use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, Webview, WebviewWindow,
};

#[cfg(desktop)]
mod desktop;

mod commands;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the decorum APIs.

pub fn caller(win: &WebviewWindow) {
    win.eval("console.log('Hello from Rust wala PURANA PLUGIN  bro!')")
        .unwrap();
}

pub trait Foo {
    fn foo(&self);
}

impl<'a> Foo for WebviewWindow {
    fn foo(&self) {
        self.eval("console.log('Hello from Rust wala PLUGIN bro!');")
            .unwrap();
    }
}

// Mac: set titlebar to overlay
// #[cfg(target_os = "macos")]
// {
//     editor_win = editor_win
//         .title_bar_style(tauri::TitleBarStyle::Overlay)
//         .hidden_title(true);
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
