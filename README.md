# tauri-plugin-decorum

This is a plugin for Tauri (v2.0.0-beta only, atm) that provides you a transparent/overlay-style titlebar specifically on Windows. MacOS is supported natively within Tauri, but on Windows the titlebar is not that customizable.

![demo](./wheeee.gif)

This plugin covers:

1. Windows: transparent titlebar
2. Windows: snap overlay
3. macOS: overlay titlebar
4. macOS: traffic light positions

## Usage examples

### Windows native titlebar

```rust
use tauri_plugin_decorum::WebviewWindowExt;

let window: WebviewWindow = app.get_webview_window("main").unwrap();
window.create_overlay_titlebar().unwrap();
```

### macOS overlay titlebar

```rust
use tauri_plugin_decorum::overlay_titlebar_mac;

let mut window_builder = WebviewWindowBuilder::new(app, "test", WebviewUrl::App("/".into())).decorations(true);
overlay_titlebar_mac(window_builder)
```

### macOS traffic light positions

```rust
use tauri_plugin_decorum::WebviewWindowExt;

let window: WebviewWindow = app.get_webview_window("main").unwrap();
window.set_traffic_light_inset((10.0, 10.0)).unwrap()
```

## Can I use it?

Nothing's stopping you from using it right away, though I suggest waiting for a bit until I figure out the right API design for this --it's almost there.

## TODOs

-   [ ] API design
-   [ ] Add alternate maximize icon
-   [ ] Support non-React frontends
-   [ ] Tweet at Microsoft demanding better WinRT integration with Rust
-   [ ] Profit !!!
