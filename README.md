# tauri-plugin-decorum

A Tauri plugin (v2.0 only for now) that gives you convenient ways to:

1. Create transparent/overlay-style titlebars on Windows and macOS.
2. Inset the macOS traffic light positions.

![demo](./wheeee.gif)

Being a designer, I'm _very_ particular about window decorations. This plugin is an opinionated take on titlebars and solves my three gripes with the default ones:
1. it retains most native features (like Windows Snap Layout)
2. they doesn't feel _disconnected_ from the rest of your app and uses transparency to blend in
3. they window controls often aren't aligned well with the rest of the contents

## Installation and Usage

Note: For a full example app that uses this plugin, check this [folder](examples/tauri-app/).

To install the plugin:
```bash
cargo add tauri-plugin-decorum
```

Usage in Tauri:
```rust
use tauri::Manager;

use tauri_plugin_decorum::WebviewWindowExt; // adds helper methods to WebviewWindow

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_decorum::init()) // initialize the decorum plugin
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

You'll also need to set these permissions for your window in `src-tauri/capabilities/default.json`
```
"window:allow-close",
"window:allow-center",
"window:allow-minimize",
"window:allow-maximize",
"window:allow-set-size",
"window:allow-set-focus",
"window:allow-start-dragging",
"decorum:allow-show-snap-overlay",
```

*there's likely a better way to handle plugin permissions that I haven't found, please let me know if you have!


## Development Guide

PRs and issues welcome! Here's a short primer to get you started with development on this:
1. Ensure you have all the [Tauri prerequisites](https://beta.tauri.app/start/prerequisites/) set up
2. Clone this repo
3. Use the [example app](examples/tauri-app) as a test bed for your changes

## Roadmap

It's still early days and there's a lot of missing features I'd like to add. Most of them are documented on the [Issues page](https://github.com/clearlysid/tauri-plugin-decorum/issues).
In the long run though, I hope the Tauri team incorporates all these features within the framework and look forward to making this plugin redundant.

Meanwhile, I hope you find this useful. Happy building! 🥂
