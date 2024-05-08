# tauri-plugin-decorum

This is a plugin for Tauri (v2.0.0-beta only, atm) that provides you a transparent/overlay-style titlebar specifically on Windows. MacOS is supported natively within Tauri, but on Windows the titlebar is not that customizable.

![demo](./wheeee.gif)

This plugin covers:

1. native looking controls for windows
2. transparent titlebar
3. windows 11 "Snap Layout" feature! -- this is the one that makes this plugin different than the others.

## How it works?

1. The "transparent titlebar" is created with HTML/CSS elements
2. Javascript connects the controls to Tauri's Maximize/Minimize/Close Window controls
3. For the "Snap Layout" feature, we emulate a few keypresses (Win + Z, followed by Alt) to get the overlay to show.

Note: only windows APIs are used to make these keypresses, this plugin does NOT store any of yours or your end-users data in any way shape or form.

## Can I use it?

Nothing's stopping you from using it right away, though I suggest waiting for a bit until I figure out the right API design for this --it's almost there.

## TODOs

-   [ ] API design
-   [ ] Add alternate maximize icon
-   [ ] Support non-React frontends
-   [ ] Tweet at Microsoft demanding better WinRT integration with Rust
-   [ ] Profit !!!
