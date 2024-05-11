// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::WebviewUrl;
use tauri::WebviewWindowBuilder;
use tauri_plugin_decorum::WebviewWindowExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_decorum::init())
        .setup(|app| {
            use tauri::window::{Color, Effect, EffectState};

            let window = app.get_webview_window("main").unwrap();
            window
                .clone()
                .create_overlay_titlebar()
                .unwrap()
                .show()
                .unwrap();
            window.set_effects(
                tauri::window::EffectsBuilder::new()
                    .effect(Effect::Tabbed)
                    .build(),
            );

            #[cfg(target_os = "macos")]
            {
                let mut test_win =
                    WebviewWindowBuilder::new(app, "test", WebviewUrl::App("/".into()))
                        .decorations(true);
                test_win = test_win
                    .title_bar_style(tauri::TitleBarStyle::Overlay)
                    .hidden_title(true);
                let test_win = test_win.build().expect("Failed to build test window");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
