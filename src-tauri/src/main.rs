// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use tauri::Manager;

#[cxx::bridge(namespace = "farzi::tauri")]
pub mod ffi {
    unsafe extern "C++" {
        include!("src/hello.h");
        // Functions implemented in C++.
        pub fn print_hello_world();
    }
}

fn emulate_win_z() -> Result<(), anyhow::Error> {
    let mut enigo = Enigo::new(&Settings::default())?;

    enigo.key(Key::RWin, Press)?;
    enigo.key(Key::Unicode('z'), Click)?;

    // wait 100ms  and click alt
    std::thread::sleep(std::time::Duration::from_millis(100));
    enigo.key(Key::Alt, Click)?;

    enigo.key(Key::RWin, Release)?;

    Ok(())
}

#[tauri::command]
async fn show_snap_overlay() -> Result<(), String> {
    emulate_win_z().map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                let window = app.get_webview_window("main").unwrap();

                window
                    .set_decorations(false)
                    .expect("couldn't set decorations");
                window.set_shadow(true).expect("couldn't set shadow");

                window_vibrancy::apply_mica(window, Some(true)).expect("couldn't set mica");

                // Step 1: get the HWND of the window

                // let hwnd = window.hwnd().expect("couldn't get HWND");
                // println!("HWND: {:?}", hwnd);
                // // Step 2: pass HWND to our C++/C# code and run AppWindow methods.

                // // TODO: call some C++/C# code here
                // ffi::print_hello_world();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![show_snap_overlay])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
