// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::WebviewUrl;
use tauri::WebviewWindowBuilder;
use tauri_plugin_decorum::WebviewWindowExt;

fn emulate_win_z() -> Result<(), anyhow::Error> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::Input::KeyboardAndMouse::{
            SendInput, INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VIRTUAL_KEY,
            VK_RWIN,
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

            inputs[3].r#type = INPUT_KEYBOARD;
            inputs[3].Anonymous.ki.wVk = VK_RWIN;
            inputs[3].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

            SendInput(&inputs, std::mem::size_of::<INPUT>() as _);
        }

        // wait 100 ms
        std::thread::sleep(std::time::Duration::from_millis(100));

        unsafe {
            let mut inputs: [INPUT; 2] = std::mem::zeroed();
            inputs[0].r#type = INPUT_KEYBOARD;
            inputs[0].Anonymous.ki.wVk = VIRTUAL_KEY(0x12); // VK_MENU (Alt key)
            inputs[0].Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(0);

            inputs[1].r#type = INPUT_KEYBOARD;
            inputs[1].Anonymous.ki.wVk = VIRTUAL_KEY(0x12); // VK_MENU (Alt key)
            inputs[1].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

            SendInput(&inputs, std::mem::size_of::<INPUT>() as _);
        }
    }

    Ok(())
}

#[tauri::command]
async fn show_snap_overlay() -> Result<(), String> {
    emulate_win_z().map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show_snap_overlay])
        .plugin(tauri_plugin_decorum::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.create_overlay_titlebar().unwrap();

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
