use tauri;

#[cfg(target_os = "windows")]
fn emulate_win_z() -> Result<(), anyhow::Error> {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_MENU,
        VK_RWIN,
    };
    // Press Win + Z
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

    // Wait 50 ms
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Press Alt to hide the ugly numbers
    unsafe {
        let mut inputs: [INPUT; 2] = std::mem::zeroed();
        inputs[0].r#type = INPUT_KEYBOARD;
        inputs[0].Anonymous.ki.wVk = VK_MENU;
        inputs[0].Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(0);

        inputs[1].r#type = INPUT_KEYBOARD;
        inputs[1].Anonymous.ki.wVk = VK_MENU;
        inputs[1].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

        SendInput(&inputs, std::mem::size_of::<INPUT>() as _);
    }

    Ok(())
}

#[tauri::command]
pub async fn show_snap_overlay() -> Result<(), String> {
    #[cfg(not(target_os = "windows"))]
    return Ok(());

    #[cfg(target_os = "windows")]
    emulate_win_z().map_err(|e| e.to_string())
}
