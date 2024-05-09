# tauri-plugin-decorum

This is a plugin for Tauri (v2-beta only, atm) that provides you a transparent/overlay-style titlebar specifically on Windows. MacOS is supported natively within Tauri, but on Windows the titlebar is not that customizable.

![demo](./wheeee.gif)

This plugin covers:

1. Windows: overlay titlebar with snap feature
2. macOS: overlay titlebar
3. macOS: traffic light positions

## Usage examples

1. **Windows overlay titlebar**

```rust
use tauri_plugin_decorum::WebviewWindowExt;

let window: WebviewWindow = app.get_webview_window("main").unwrap();
window.create_overlay_titlebar().unwrap();
```

2. **macOS overlay titlebar**

```rust
use tauri_plugin_decorum::overlay_titlebar_mac;

let mut window_builder = WebviewWindowBuilder::new(app, "test", WebviewUrl::App("/".into())).decorations(true);
overlay_titlebar_mac(window_builder)
```

3. **macOS traffic light inset**

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
