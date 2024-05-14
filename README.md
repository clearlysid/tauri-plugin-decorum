# tauri-plugin-decorum

Being a designer, I'm _very_ particular about window decorations. This Tauri (v2 only) plugin is an opinionated take on titlebars and solves my gripes with the default one. It does so by:
1. retaining most native features (like Windows Snap Layout)
2. not feeling too _disconnected_ from the rest of the app, by being transparent and blending in better
3. offering custom inset for window controls that are often not aligned well with the rest of the contents

![demo](./wheeee.gif)


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

There's some missing features I'd still like to add, all documented on the [Issues page](https://github.com/clearlysid/tauri-plugin-decorum/issues).
In the long run though I hope the core team incorporates all these within Tauri and I look forward to making this plugin obsolete.

Meanwhile, I hope you find this useful. Happy building! 🥂
