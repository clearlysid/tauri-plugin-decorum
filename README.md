# tauri-plugin-decorum

A Tauri plugin (v2.0 only for now) that gives you convenient ways to:

1. Create transparent/overlay-style titlebars on Windows and macOS.
2. Inset the macOS traffic light positions.

![demo](./wheeee.gif)

Being a designer, I'm _very_ particular about how window decorations look and behave. This plugin is an opinionated take on titlebars: it retains the native featuress (like Windows Snap Layout) and uses transparency to seamlessly sit within your app's UI solving my two gripes with the default titlebars. One, they feel too "disconnected" from the rest of the app and two, they window controls often aren't aligned well with the rest of the contents.

## Installation and Usage

Note: For a full example app that uses this plugin, check the [this folder](examples/tauri-app/).

To install the plugin:
```bash
cargo add tauri-plugin-decorum
```

Usage in Tauri:
```rust
use tauri::Manager;

// adds the helper methods to WebviewWindow
use tauri_plugin_decorum::WebviewWindowExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_decorum::init())
        .setup(|app| {
	    // Create a custom titlebar for main window
	    // On Windows this hides decoration and creates custom window controls
	    // On macOS it needs hiddenTitle: true and titleBarStyle: overlay
	    let main_window = app.get_webview_window("main").unwrap();
	    main_window.create_overlay_titlebar().unwrap();

	    // Set a custom inset to the traffic lights
	    #[cfg(target_os = "macos")]
	    main_window.set_traffic_lights_inset(12.0, 16.0).unwrap();

	    Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Development Guide

PRs and issues welcome! Here's a short primer to get you started with development on this:
1. Ensure you have all the [Tauri prerequisites](https://beta.tauri.app/start/prerequisites/) set up
2. Clone this repo
3. Use the [example app](examples/tauri-app) as a test bed for your changes

## Roadmap

It's still early days and there's a lot of missing features I'd like to add. Most of them are documented on the [Issues page](https://github.com/clearlysid/tauri-plugin-decorum/issues).
In the long run though, I hope the Tauri team incorporates all these features natively in the framework and I looking forward to making this plugin redundant someday.

Meanwhile, I hope you find this useful. Happy building! 🥂
