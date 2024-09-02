const COMMANDS: &[&str] = &["show_snap_overlay", "set_window_buttons_inset"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
