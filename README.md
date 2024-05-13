# tauri-plugin-decorum

A helper plugin for Tauri v2.0 that gives you convenient ways to:

1. Have a transparent/overlay-style titlebar on Windows and macOS.
2. Inset the macOS traffic light positions.

![demo](./wheeee.gif)

### Why build this?

I'm a designer and I'm _very_ particular about how window decorations should look and behave — this plugin is an opinionated take on titlebars. We retain all the native platform features (including Windows Snap Layout) and use transparent backgrounds to seamlessly sit within your app's UI.

## Usage examples

For a full Tauri app that consumes this plugin, check the [examples folder](examples/tauri-app/).

```rust
use tauri::Manager;
use tauri_plugin_decorum::WebviewWindowExt; // adds the helper methods to WebviewWindow

fn main() {
	tauri::Builder::default()
		.plugin(tauri_plugin_decorum::init())
		.setup(|app| {
			// Create a custom titlebar for main window
			// On Windows this will hide decoration and render custom window controls
			// On macOS it expects a hiddenTitle: true and titleBarStyle: overlay
			let main_window = app.get_webview_window("main").unwrap();
			main_window.create_overlay_titlebar().unwrap();

			// Set an inset to the traffic lights
			#[cfg(target_os = "macos")]
			main_window.set_traffic_lights_inset(12.0, 16.0).unwrap();

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
```

## Can I use it?

Please do! It's still early days and there's some missing features I'm yet to add. In the long run though, hopefully the Tauri team incorporates all the features of this plugin natively in the framework. Looking forward to making this plugin redundant.

Meanwhile, happy building!
