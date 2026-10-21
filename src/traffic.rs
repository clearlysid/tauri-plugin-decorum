// Most contents of this file are taken from Hoppscotch's tauri app.
// I think there is work to be done to improve it, but I'm happy with it for now.
// Reference source code is linked below.
// https://github.com/hoppscotch/hoppscotch/blob/286fcd2bb08a84f027b10308d1e18da368f95ebf/packages/hoppscotch-selfhost-desktop/src-tauri/src/mac/window.rs

use objc::{msg_send, sel, sel_impl};
use rand::{distributions::Alphanumeric, Rng};
use std::cell::Cell;
use tauri::{Emitter, Runtime, Window};

const WINDOW_CONTROL_PAD_X: f64 = 12.0;
const WINDOW_CONTROL_PAD_Y: f64 = 16.0;

pub struct UnsafeWindowHandle(pub *mut std::ffi::c_void);
unsafe impl Send for UnsafeWindowHandle {}
unsafe impl Sync for UnsafeWindowHandle {}

#[cfg(target_os = "macos")]
fn position_traffic_lights(
    ns_window_handle: UnsafeWindowHandle,
    x: f64,
    y: f64,
    offsets: &[(f64, f64); 3],
) {
    use cocoa::appkit::{NSView, NSWindow, NSWindowButton};
    use cocoa::base::{id, nil};
    use cocoa::foundation::{NSPoint, NSRect};
    let window = ns_window_handle.0 as id;
    unsafe {
        let buttons = [
            window.standardWindowButton_(NSWindowButton::NSWindowCloseButton),
            window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton),
            window.standardWindowButton_(NSWindowButton::NSWindowZoomButton),
        ];
        let close = buttons[0];
        if close.is_null() {
            return;
        }
        let host: id = msg_send![close, window];
        // Fullscreen can reparent the native controls into AppKit's separate toolbar window.
        if host != window {
            return;
        }
        let parent = close.superview();
        if parent.is_null() {
            return;
        }
        let container = parent.superview();
        if container.is_null() {
            return;
        }
        let close_rect = NSView::frame(close);
        let height = close_rect.size.height + 2.0 * y;
        let mut frame = NSView::frame(container);
        frame.size.height = height;
        frame.origin.y = NSView::frame(window).size.height - height;
        let current = NSView::frame(container);
        if current.origin.y != frame.origin.y || current.size.height != frame.size.height {
            let _: () = msg_send![container, setFrame: frame];
        }
        let top = NSView::frame(window).size.height - y - close_rect.size.height;
        for (button, offset) in buttons.into_iter().zip(offsets) {
            if button.is_null() {
                continue;
            }
            let parent = button.superview();
            if parent.is_null() {
                continue;
            }
            let origin = parent.convertPoint_fromView_(NSPoint::new(x + offset.0, top + offset.1), nil);
            let frame: NSRect = NSView::frame(button);
            if frame.origin.x != origin.x || frame.origin.y != origin.y {
                button.setFrameOrigin(origin);
            }
        }
    }
}

#[cfg(target_os = "macos")]
#[derive(Debug)]
struct WindowState<R: Runtime> {
    window: Window<R>,
    traffic_light_x: Cell<f64>,
    traffic_light_y: Cell<f64>,
    native_offsets: [(f64, f64); 3],
    fullscreen: Cell<bool>,
    positioning: Cell<bool>,
    active: Cell<bool>,
}

#[cfg(target_os = "macos")]
impl<R: Runtime> WindowState<R> {
    fn position(&self) {
        if !self.active.get() || self.fullscreen.get() || self.positioning.replace(true) {
            return;
        }
        if let Ok(window) = self.window.ns_window() {
            position_traffic_lights(
                UnsafeWindowHandle(window),
                self.traffic_light_x.get(),
                self.traffic_light_y.get(),
                &self.native_offsets,
            );
        }
        self.positioning.set(false);
    }

    fn observe_layout(&self, observe: bool) {
        use cocoa::appkit::{NSView, NSWindow, NSWindowButton};
        use cocoa::base::{id, nil, YES};
        use cocoa::foundation::NSString;
        unsafe {
            let window = match self.window.ns_window() {
                Ok(window) => window as id,
                Err(_) => return,
            };
            let delegate: id = msg_send![window, delegate];
            let center: id = msg_send![objc::class!(NSNotificationCenter), defaultCenter];
            for notification in ["NSViewFrameDidChangeNotification", "NSViewBoundsDidChangeNotification"] {
                let name = NSString::alloc(nil).init_str(notification);
                let _: () = msg_send![center, removeObserver: delegate name: name object: nil];
                if observe {
                    let close = window.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
                    if !close.is_null() {
                        let parent = close.superview();
                        let container = if parent.is_null() { nil } else { parent.superview() };
                        for view in [
                            close,
                            window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton),
                            window.standardWindowButton_(NSWindowButton::NSWindowZoomButton),
                            parent,
                            container,
                        ] {
                            if !view.is_null() {
                                let _: () = msg_send![view, setPostsFrameChangedNotifications: YES];
                                let _: () = msg_send![view, setPostsBoundsChangedNotifications: YES];
                                let _: () = msg_send![center,
                                    addObserver: delegate
                                    selector: sel!(trafficLightsLayoutChanged:)
                                    name: name
                                    object: view
                                ];
                            }
                        }
                    }
                }
                let _: () = msg_send![name, release];
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub fn setup_traffic_light_positioner<R: Runtime>(window: Window<R>) {
    use cocoa::appkit::{NSWindow, NSWindowButton};
    use cocoa::base::{id, BOOL};
    use cocoa::foundation::NSUInteger;
    use objc::runtime::{Object, Sel};
    use std::ffi::c_void;

    let native_offsets = unsafe {
        use cocoa::appkit::NSView;
        let window = match window.ns_window() {
            Ok(window) => window as id,
            Err(_) => return,
        };
        let close = window.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
        if close.is_null() {
            return;
        }
        let origin = NSView::frame(close).origin;
        [
            close,
            window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton),
            window.standardWindowButton_(NSWindowButton::NSWindowZoomButton),
        ].map(|button| {
            if button.is_null() {
                (0.0, 0.0)
            } else {
                let frame = NSView::frame(button);
                (frame.origin.x - origin.x, frame.origin.y - origin.y)
            }
        })
    };

    // Ensure they stay in place while resizing the window.
    fn with_window_state<R: Runtime, F: FnOnce(&WindowState<R>) -> T, T>(
        this: &Object,
        func: F,
    ) {
        let ptr = unsafe {
            let x: *mut c_void = *this.get_ivar("app_box");
            &*(x as *const WindowState<R>)
        };
        func(ptr);
    }

    unsafe {
        let ns_win = window
            .ns_window()
            .expect("NS Window should exist to mount traffic light delegate.")
            as id;

        let current_delegate: id = ns_win.delegate();

        extern "C" fn on_window_should_close(this: &Object, _cmd: Sel, sender: id) -> BOOL {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                msg_send![super_del, windowShouldClose: sender]
            }
        }
        extern "C" fn on_window_will_close<R: Runtime>(this: &Object, _cmd: Sel, notification: id) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                with_window_state(this, |state: &WindowState<R>| {
                    state.active.set(false);
                    state.observe_layout(false);
                });
                let _: () = msg_send![super_del, windowWillClose: notification];
            }
        }
        extern "C" fn on_window_did_resize<R: Runtime>(this: &Object, _cmd: Sel, notification: id) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowDidResize: notification];
                with_window_state(this, |state: &WindowState<R>| state.position());
            }
        }
        extern "C" fn on_layout_changed<R: Runtime>(this: &Object, _cmd: Sel, _notification: id) {
            with_window_state(this, |state: &WindowState<R>| state.position());
        }
        extern "C" fn on_window_did_move(this: &Object, _cmd: Sel, notification: id) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowDidMove: notification];
            }
        }
        extern "C" fn on_window_did_change_backing_properties(
            this: &Object,
            _cmd: Sel,
            notification: id,
        ) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowDidChangeBackingProperties: notification];
            }
        }
        extern "C" fn on_window_did_become_key<R: Runtime>(this: &Object, _cmd: Sel, notification: id) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowDidBecomeKey: notification];
                with_window_state(this, |state: &WindowState<R>| state.position());
            }
        }
        extern "C" fn on_window_did_resign_key(this: &Object, _cmd: Sel, notification: id) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowDidResignKey: notification];
            }
        }
        extern "C" fn on_dragging_entered(this: &Object, _cmd: Sel, notification: id) -> BOOL {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                msg_send![super_del, draggingEntered: notification]
            }
        }
        extern "C" fn on_prepare_for_drag_operation(
            this: &Object,
            _cmd: Sel,
            notification: id,
        ) -> BOOL {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                msg_send![super_del, prepareForDragOperation: notification]
            }
        }
        extern "C" fn on_perform_drag_operation(this: &Object, _cmd: Sel, sender: id) -> BOOL {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                msg_send![super_del, performDragOperation: sender]
            }
        }
        extern "C" fn on_conclude_drag_operation(this: &Object, _cmd: Sel, notification: id) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, concludeDragOperation: notification];
            }
        }
        extern "C" fn on_dragging_exited(this: &Object, _cmd: Sel, notification: id) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, draggingExited: notification];
            }
        }
        extern "C" fn on_window_will_use_full_screen_presentation_options(
            this: &Object,
            _cmd: Sel,
            window: id,
            proposed_options: NSUInteger,
        ) -> NSUInteger {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                msg_send![super_del, window: window willUseFullScreenPresentationOptions: proposed_options]
            }
        }
        extern "C" fn on_window_did_enter_full_screen<R: Runtime>(
            this: &Object,
            _cmd: Sel,
            notification: id,
        ) {
            unsafe {
                with_window_state(&*this, |state: &WindowState<R>| {
                    state.fullscreen.set(true);
                    state.observe_layout(true);
                    state
                        .window
                        .emit("did-enter-fullscreen", ())
                        .expect("Failed to emit event");
                });

                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowDidEnterFullScreen: notification];
            }
        }
        extern "C" fn on_window_will_enter_full_screen<R: Runtime>(
            this: &Object,
            _cmd: Sel,
            notification: id,
        ) {
            unsafe {
                with_window_state(&*this, |state: &WindowState<R>| {
                    state.observe_layout(true);
                    state.position();
                    state
                        .window
                        .emit("will-enter-fullscreen", ())
                        .expect("Failed to emit event");
                });

                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowWillEnterFullScreen: notification];
            }
        }
        extern "C" fn on_window_did_exit_full_screen<R: Runtime>(
            this: &Object,
            _cmd: Sel,
            notification: id,
        ) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowDidExitFullScreen: notification];
                with_window_state(&*this, |state: &WindowState<R>| {
                    state
                        .window
                        .emit("did-exit-fullscreen", ())
                        .expect("Failed to emit event");

                    state.fullscreen.set(false);
                    state.observe_layout(true);
                    state.position();
                });
            }
        }
        extern "C" fn on_window_will_exit_full_screen<R: Runtime>(
            this: &Object,
            _cmd: Sel,
            notification: id,
        ) {
            unsafe {
                with_window_state(&*this, |state: &WindowState<R>| {
                    state.fullscreen.set(false);
                    state.observe_layout(true);
                    state.position();
                    state
                        .window
                        .emit("will-exit-fullscreen", ())
                        .expect("Failed to emit event");
                });

                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowWillExitFullScreen: notification];
            }
        }
        extern "C" fn on_window_did_fail_to_enter_full_screen<R: Runtime>(
            this: &Object,
            _cmd: Sel,
            window: id,
        ) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowDidFailToEnterFullScreen: window];
                with_window_state(this, |state: &WindowState<R>| {
                    state.fullscreen.set(false);
                    state.observe_layout(true);
                    state.position();
                });
            }
        }
        extern "C" fn on_window_did_fail_to_exit_full_screen<R: Runtime>(this: &Object, _cmd: Sel, window: id) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, windowDidFailToExitFullScreen: window];
                with_window_state(this, |state: &WindowState<R>| state.fullscreen.set(true));
            }
        }
        extern "C" fn on_effective_appearance_did_change(
            this: &Object,
            _cmd: Sel,
            notification: id,
        ) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![super_del, effectiveAppearanceDidChange: notification];
            }
        }
        extern "C" fn on_effective_appearance_did_changed_on_main_thread(
            this: &Object,
            _cmd: Sel,
            notification: id,
        ) {
            unsafe {
                let super_del: id = *this.get_ivar("super_delegate");
                let _: () = msg_send![
                    super_del,
                    effectiveAppearanceDidChangedOnMainThread: notification
                ];
            }
        }

        // Are we deallocing this properly ? (I miss safe Rust :(  )
        let window_label = window.label().to_string();

        let app_state = WindowState { 
            window,
            traffic_light_x: Cell::new(WINDOW_CONTROL_PAD_X),
            traffic_light_y: Cell::new(WINDOW_CONTROL_PAD_Y),
            native_offsets,
            fullscreen: Cell::new(false),
            positioning: Cell::new(false),
            active: Cell::new(true),
        };
        let app_box = Box::into_raw(Box::new(app_state)) as *mut c_void;
        let random_str: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();

        // We need to ensure we have a unique delegate name, otherwise we will panic while trying to create a duplicate
        // delegate with the same name.
        let delegate_name = format!("windowDelegate_{}_{}", window_label, random_str);

        ns_win.setDelegate_(cocoa::delegate!(&delegate_name, {
            window: id = ns_win,
            app_box: *mut c_void = app_box,
            toolbar: id = cocoa::base::nil,
            super_delegate: id = current_delegate,
            (windowShouldClose:) => on_window_should_close as extern fn(&Object, Sel, id) -> BOOL,
            (windowWillClose:) => on_window_will_close::<R> as extern fn(&Object, Sel, id),
            (windowDidResize:) => on_window_did_resize::<R> as extern fn(&Object, Sel, id),
            (trafficLightsLayoutChanged:) => on_layout_changed::<R> as extern fn(&Object, Sel, id),
            (windowDidMove:) => on_window_did_move as extern fn(&Object, Sel, id),
            (windowDidChangeBackingProperties:) => on_window_did_change_backing_properties as extern fn(&Object, Sel, id),
            (windowDidBecomeKey:) => on_window_did_become_key::<R> as extern fn(&Object, Sel, id),
            (windowDidResignKey:) => on_window_did_resign_key as extern fn(&Object, Sel, id),
            (draggingEntered:) => on_dragging_entered as extern fn(&Object, Sel, id) -> BOOL,
            (prepareForDragOperation:) => on_prepare_for_drag_operation as extern fn(&Object, Sel, id) -> BOOL,
            (performDragOperation:) => on_perform_drag_operation as extern fn(&Object, Sel, id) -> BOOL,
            (concludeDragOperation:) => on_conclude_drag_operation as extern fn(&Object, Sel, id),
            (draggingExited:) => on_dragging_exited as extern fn(&Object, Sel, id),
            (window:willUseFullScreenPresentationOptions:) => on_window_will_use_full_screen_presentation_options as extern fn(&Object, Sel, id, NSUInteger) -> NSUInteger,
            (windowDidEnterFullScreen:) => on_window_did_enter_full_screen::<R> as extern fn(&Object, Sel, id),
            (windowWillEnterFullScreen:) => on_window_will_enter_full_screen::<R> as extern fn(&Object, Sel, id),
            (windowDidExitFullScreen:) => on_window_did_exit_full_screen::<R> as extern fn(&Object, Sel, id),
            (windowWillExitFullScreen:) => on_window_will_exit_full_screen::<R> as extern fn(&Object, Sel, id),
            (windowDidFailToEnterFullScreen:) => on_window_did_fail_to_enter_full_screen::<R> as extern fn(&Object, Sel, id),
            (windowDidFailToExitFullScreen:) => on_window_did_fail_to_exit_full_screen::<R> as extern fn(&Object, Sel, id),
            (effectiveAppearanceDidChange:) => on_effective_appearance_did_change as extern fn(&Object, Sel, id),
            (effectiveAppearanceDidChangedOnMainThread:) => on_effective_appearance_did_changed_on_main_thread as extern fn(&Object, Sel, id)
        }));
        let delegate: id = msg_send![ns_win, delegate];
        with_window_state(&*delegate, |state: &WindowState<R>| {
            state.observe_layout(true);
            state.position();
        });
    }
}

#[cfg(target_os = "macos")]
pub fn update_traffic_light_positions(window: &tauri::WebviewWindow, x: f64, y: f64) {
    use objc::runtime::Object;
    use std::ffi::c_void;
    use tauri::Wry;
    
    unsafe {
        let ns_win = match window.ns_window() {
            Ok(win) => win as cocoa::base::id,
            Err(_) => return,
        };
        
        let delegate: *mut Object = msg_send![ns_win, delegate];
        if delegate.is_null() {
            return;
        }
        
        // Try to access the ivar directly with proper type annotation
        let app_box: *mut c_void = match std::panic::catch_unwind(|| {
            *(*delegate).get_ivar::<*mut c_void>("app_box")
        }) {
            Ok(ptr) if !ptr.is_null() => ptr,
            _ => return, // Either the ivar doesn't exist or it's null
        };
        
        // Specify Wry as the concrete runtime type
        let state: &WindowState<Wry> = &*(app_box as *const WindowState<Wry>);
        state.traffic_light_x.set(x);
        state.traffic_light_y.set(y);
        state.position();
    }
}
