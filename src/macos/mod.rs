use nswindow_delegates::UnsafeWindowHandle;
// use objc::{msg_send, sel, sel_impl};
// use rand::{distributions::Alphanumeric, Rng};
use tauri::{LogicalPosition, Manager, Runtime, Window};

use crate::TrafficLightsInsetsState;

pub mod nswindow_delegates;

const DEFAULT_TRAFFIC_LIGHTS_INSET: LogicalPosition<f64> = LogicalPosition::new(12.0, 16.0);

pub fn update_traffic_lights_inset<R: Runtime>(window: &Window<R>) {
    let insets_state = &window.state::<TrafficLightsInsetsState>();
    let insets_map = insets_state.0.read();
    let inset = insets_map
        .get(&window.label().to_string())
        .unwrap_or(&DEFAULT_TRAFFIC_LIGHTS_INSET);

    position_traffic_lights(
        UnsafeWindowHandle(window.ns_window().expect("Failed to create window handle")),
        inset.x,
        inset.y,
    );
}

fn position_traffic_lights(ns_window_handle: UnsafeWindowHandle, x: f64, y: f64) {
    use cocoa::appkit::{NSView, NSWindow, NSWindowButton};
    use cocoa::foundation::NSRect;
    let ns_window = ns_window_handle.0 as cocoa::base::id;
    unsafe {
        let close = ns_window.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
        let miniaturize =
            ns_window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
        let zoom = ns_window.standardWindowButton_(NSWindowButton::NSWindowZoomButton);

        let title_bar_container_view = close.superview().superview();

        let close_rect: NSRect = msg_send![close, frame];
        let button_height = close_rect.size.height;

        let title_bar_frame_height = button_height + y;
        let mut title_bar_rect = NSView::frame(title_bar_container_view);
        title_bar_rect.size.height = title_bar_frame_height;
        title_bar_rect.origin.y = NSView::frame(ns_window).size.height - title_bar_frame_height;
        let _: () = msg_send![title_bar_container_view, setFrame: title_bar_rect];

        let window_buttons = vec![close, miniaturize, zoom];
        let space_between = 20.0; // Fixed space between buttons
        let vertical_offset = 4.0; // Adjust this value to push buttons down

        for (i, button) in window_buttons.into_iter().enumerate() {
            let mut rect: NSRect = NSView::frame(button);
            rect.origin.x = x + (i as f64 * space_between);
            // Adjust vertical positioning
            rect.origin.y = ((title_bar_frame_height - button_height) / 2.0) - vertical_offset;
            button.setFrameOrigin(rect.origin);
        }
    }
}
