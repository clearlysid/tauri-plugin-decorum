use tauri::Manager;

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

    Ok(())
}

#[tauri::command]
async fn show_snap_overlay() -> Result<(), String> {
    emulate_win_z().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show_snap_overlay])
        .plugin(tauri_plugin_decorum::init())
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                let window = app.get_webview_window("main").unwrap();

                window
                    .set_decorations(false)
                    .expect("couldn't set decorations");
                window.set_shadow(true).expect("couldn't set shadow");

                window_vibrancy::apply_mica(window, Some(true)).expect("couldn't set mica");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
