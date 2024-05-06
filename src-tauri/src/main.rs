// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_RWIN,
    };

    unsafe {
        let mut inputs: [INPUT; 4] = std::mem::zeroed();
        inputs[0].r#type = INPUT_KEYBOARD;
        inputs[0].Anonymous.ki.wVk = VK_RWIN;
        inputs[0].Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(0);

        inputs[1].r#type = INPUT_KEYBOARD;
        inputs[1].Anonymous.ki.wVk = VIRTUAL_KEY('Z' as u16);
        inputs[1].Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(0);

        inputs[2].r#type = INPUT_KEYBOARD;
        inputs[2].Anonymous.ki.wVk = VIRTUAL_KEY('Z' as u16);
        inputs[2].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

        // inputs[3].r#type = INPUT_KEYBOARD;
        // inputs[3].Anonymous.ki.wVk = VIRTUAL_KEY(0x12); // VK_MENU (Alt key)
        // inputs[3].Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(0);

        // inputs[4].r#type = INPUT_KEYBOARD;
        // inputs[4].Anonymous.ki.wVk = VIRTUAL_KEY(0x12); // VK_MENU (Alt key)
        // inputs[4].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

        inputs[3].r#type = INPUT_KEYBOARD;
        inputs[3].Anonymous.ki.wVk = VK_RWIN;
        inputs[3].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

        SendInput(&inputs, std::mem::size_of::<INPUT>() as _);
    }

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
