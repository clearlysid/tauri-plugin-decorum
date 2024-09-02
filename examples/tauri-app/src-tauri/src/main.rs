// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_decorum::WebviewWindowExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_decorum::init())
        .setup(|app| {
            // Create a custom titlebar for main window
            // On Windows this will hide decoration and render custom window controls
            // On macOS it expects a hiddenTitle: true and titleBarStyle: overlay
            let main_window = app.get_webview_window("main").unwrap();
            // main_window.create_overlay_titlebar().unwrap();

            #[cfg(target_os = "macos")]
            {
                use tauri_plugin_decorum::NSWindowLevel;

                let _ = main_window.make_transparent();
                let _ = main_window.create_overlay_titlebar();
                // let _ =
                //     main_window.set_window_buttons_inset(Some(LogicalPosition::new(15.0, 20.0)));

                let _ = main_window.set_window_level(NSWindowLevel::NSNormalWindowLevel);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
