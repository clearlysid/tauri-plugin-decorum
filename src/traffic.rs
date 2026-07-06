use std::cell::Cell;
use std::ffi::c_void;
use objc2::rc::Retained;
use objc2::{define_class, msg_send, ClassType, DefinedClass, MainThreadMarker, MainThreadOnly};
use objc2::runtime::{Bool, NSObject, ProtocolObject};
use objc2_app_kit::{
    NSApplicationPresentationOptions, NSButton, NSWindow,
    NSWindowButton, NSWindowDelegate,
};
use objc2_foundation::{NSNotification, NSObjectProtocol};
use tauri::{Emitter, Runtime, Window};

pub const WINDOW_CONTROL_PAD_X: f64 = 12.0;
pub const WINDOW_CONTROL_PAD_Y: f64 = 16.0;

#[cfg(target_os = "macos")]
pub fn position_traffic_lights(ns_window: &NSWindow, x: f64, y: f64) {
    let Some(close) = ns_window.standardWindowButton(NSWindowButton::CloseButton) else {
        return;
    };
    let miniaturize = ns_window.standardWindowButton(NSWindowButton::MiniaturizeButton);
    let zoom = ns_window.standardWindowButton(NSWindowButton::ZoomButton);

    let Some(close_superview) = (unsafe { close.superview() }) else {
        return;
    };
    let Some(title_bar_container_view) = (unsafe { close_superview.superview() }) else {
        return;
    };

    let close_frame = close.frame();
    let button_height = close_frame.size.height;

    let title_bar_frame_height = button_height + y;
    let mut title_bar_rect = title_bar_container_view.frame();
    title_bar_rect.size.height = title_bar_frame_height;
    title_bar_rect.origin.y = ns_window.frame().size.height - title_bar_frame_height;
    title_bar_container_view.setFrame(title_bar_rect);

    let mut window_buttons: Vec<Retained<NSButton>> = Vec::new();
    window_buttons.push(close);
    if let Some(m) = miniaturize { window_buttons.push(m); }
    if let Some(z) = zoom { window_buttons.push(z); }

    if window_buttons.is_empty() {
        return;
    }

    let space_between = 20.0;
    let vertical_offset = 4.0;

    for (i, button) in window_buttons.iter().enumerate() {
        let mut rect = button.frame();
        rect.origin.x = x + (i as f64 * space_between);
        rect.origin.y = ((title_bar_frame_height - button_height) / 2.0) - vertical_offset;
        button.setFrameOrigin(rect.origin);
    }
}

#[cfg(target_os = "macos")]
struct WindowState<R: Runtime> {
    window: Window<R>,
    traffic_light_x: f64,
    traffic_light_y: f64,
}

#[cfg(target_os = "macos")]
pub struct TrafficLightIvars {
    ns_window: Retained<NSWindow>,
    super_delegate: Option<Retained<ProtocolObject<dyn NSWindowDelegate>>>,
    traffic_light_x: Cell<f64>,
    traffic_light_y: Cell<f64>,
    state_ptr: Cell<*mut c_void>,
}

unsafe impl Send for TrafficLightIvars {}
unsafe impl Sync for TrafficLightIvars {}

impl Drop for TrafficLightIvars {
    fn drop(&mut self) {
        let ptr = self.state_ptr.get();
        if !ptr.is_null() {
            unsafe { drop(Box::from_raw(ptr as *mut WindowState<tauri::Wry>)); }
        }
    }
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = TrafficLightIvars]
    pub struct TrafficLightDelegate;

    unsafe impl NSWindowDelegate for TrafficLightDelegate {
        #[unsafe(method(windowDidResize:))]
        fn windowDidResize(&self, notification: &NSNotification) {
            let ivars = self.ivars();
            position_traffic_lights(&ivars.ns_window, ivars.traffic_light_x.get(), ivars.traffic_light_y.get());
            if let Some(ref super_del) = ivars.super_delegate {
                super_del.windowDidResize(notification);
            }
        }

        #[unsafe(method(windowDidMove:))]
        fn windowDidMove(&self, notification: &NSNotification) {
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.windowDidMove(notification);
            }
        }

        #[unsafe(method(windowDidBecomeKey:))]
        fn windowDidBecomeKey(&self, notification: &NSNotification) {
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.windowDidBecomeKey(notification);
            }
        }

        #[unsafe(method(windowDidResignKey:))]
        fn windowDidResignKey(&self, notification: &NSNotification) {
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.windowDidResignKey(notification);
            }
        }

        #[unsafe(method(windowWillClose:))]
        fn windowWillClose(&self, notification: &NSNotification) {
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.windowWillClose(notification);
            }
        }

        #[unsafe(method(windowDidChangeBackingProperties:))]
        fn windowDidChangeBackingProperties(&self, notification: &NSNotification) {
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.windowDidChangeBackingProperties(notification);
            }
        }

        #[unsafe(method(windowDidEnterFullScreen:))]
        fn windowDidEnterFullScreen(&self, notification: &NSNotification) {
            emit_state_event(self, "did-enter-fullscreen");
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.windowDidEnterFullScreen(notification);
            }
        }

        #[unsafe(method(windowWillEnterFullScreen:))]
        fn windowWillEnterFullScreen(&self, notification: &NSNotification) {
            emit_state_event(self, "will-enter-fullscreen");
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.windowWillEnterFullScreen(notification);
            }
        }

        #[unsafe(method(windowDidExitFullScreen:))]
        fn windowDidExitFullScreen(&self, notification: &NSNotification) {
            emit_state_event(self, "did-exit-fullscreen");
            let ivars = self.ivars();
            position_traffic_lights(&ivars.ns_window, ivars.traffic_light_x.get(), ivars.traffic_light_y.get());
            if let Some(ref super_del) = ivars.super_delegate {
                super_del.windowDidExitFullScreen(notification);
            }
        }

        #[unsafe(method(windowWillExitFullScreen:))]
        fn windowWillExitFullScreen(&self, notification: &NSNotification) {
            emit_state_event(self, "will-exit-fullscreen");
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.windowWillExitFullScreen(notification);
            }
        }

        #[unsafe(method(windowDidFailToEnterFullScreen:))]
        fn windowDidFailToEnterFullScreen(&self, window: &NSWindow) {
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.windowDidFailToEnterFullScreen(window);
            }
        }

        #[unsafe(method(windowShouldClose:))]
        fn windowShouldClose(&self, sender: &NSWindow) -> bool {
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.windowShouldClose(sender)
            } else {
                true
            }
        }

        #[unsafe(method(window:willUseFullScreenPresentationOptions:))]
        fn window_willUseFullScreenPresentationOptions(
            &self,
            window: &NSWindow,
            proposed_options: NSApplicationPresentationOptions,
        ) -> NSApplicationPresentationOptions {
            if let Some(ref super_del) = self.ivars().super_delegate {
                super_del.window_willUseFullScreenPresentationOptions(window, proposed_options)
            } else {
                proposed_options
            }
        }

        #[unsafe(method(draggingEntered:))]
        fn draggingEntered(&self, notification: &NSNotification) -> Bool {
            if let Some(ref super_del) = self.ivars().super_delegate {
                unsafe { msg_send![super_del, draggingEntered: notification] }
            } else {
                Bool::new(false)
            }
        }

        #[unsafe(method(prepareForDragOperation:))]
        fn prepareForDragOperation(&self, notification: &NSNotification) -> Bool {
            if let Some(ref super_del) = self.ivars().super_delegate {
                unsafe { msg_send![super_del, prepareForDragOperation: notification] }
            } else {
                Bool::new(false)
            }
        }

        #[unsafe(method(performDragOperation:))]
        fn performDragOperation(&self, sender: &NSWindow) -> Bool {
            if let Some(ref super_del) = self.ivars().super_delegate {
                unsafe { msg_send![super_del, performDragOperation: sender] }
            } else {
                Bool::new(false)
            }
        }

        #[unsafe(method(concludeDragOperation:))]
        fn concludeDragOperation(&self, notification: &NSNotification) {
            if let Some(ref super_del) = self.ivars().super_delegate {
                unsafe { msg_send![super_del, concludeDragOperation: notification] }
            }
        }

        #[unsafe(method(draggingExited:))]
        fn draggingExited(&self, notification: &NSNotification) {
            if let Some(ref super_del) = self.ivars().super_delegate {
                unsafe { msg_send![super_del, draggingExited: notification] }
            }
        }

        #[unsafe(method(effectiveAppearanceDidChange:))]
        fn effectiveAppearanceDidChange(&self, notification: &NSNotification) {
            if let Some(ref super_del) = self.ivars().super_delegate {
                unsafe { msg_send![super_del, effectiveAppearanceDidChange: notification] }
            }
        }
    }
);

unsafe impl NSObjectProtocol for TrafficLightDelegate {}

fn emit_state_event(delegate: &TrafficLightDelegate, event: &str) {
    let ptr = delegate.ivars().state_ptr.get();
    if !ptr.is_null() {
        let state = unsafe { &mut *(ptr as *mut WindowState<tauri::Wry>) };
        let _ = state.window.emit(event, ());
    }
}

#[cfg(target_os = "macos")]
pub fn setup_traffic_light_positioner<R: Runtime>(window: Window<R>) {
    let ns_window: Retained<NSWindow> = match window.ns_window() {
        Ok(ptr) => unsafe { Retained::retain(ptr as *mut NSWindow).unwrap() },
        Err(_) => return,
    };

    if ns_window.standardWindowButton(NSWindowButton::CloseButton).is_none() {
        return;
    }

    position_traffic_lights(&ns_window, WINDOW_CONTROL_PAD_X, WINDOW_CONTROL_PAD_Y);

    let current_delegate = ns_window.delegate();

    let state = WindowState {
        window,
        traffic_light_x: WINDOW_CONTROL_PAD_X,
        traffic_light_y: WINDOW_CONTROL_PAD_Y,
    };
    let state_ptr = Box::into_raw(Box::new(state)) as *mut c_void;

    let mtm = unsafe { MainThreadMarker::new_unchecked() };
    let delegate = TrafficLightDelegate::alloc(mtm).set_ivars(TrafficLightIvars {
        ns_window: ns_window.clone(),
        super_delegate: current_delegate,
        traffic_light_x: Cell::new(WINDOW_CONTROL_PAD_X),
        traffic_light_y: Cell::new(WINDOW_CONTROL_PAD_Y),
        state_ptr: Cell::new(state_ptr),
    });
    let delegate: Retained<TrafficLightDelegate> = unsafe { msg_send![super(delegate), init] };
    ns_window.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));

    std::mem::forget(delegate);
}

#[cfg(target_os = "macos")]
pub fn update_traffic_light_positions(window: &tauri::WebviewWindow, x: f64, y: f64) {
    let Ok(ns_win) = window.ns_window() else { return };
    let ns_window: &NSWindow = unsafe { &*(ns_win as *const NSWindow) };
    let Some(delegate) = ns_window.delegate() else { return };

    let delegate_any: &objc2::runtime::AnyObject = delegate.as_ref();
    let delegate_class = delegate_any.class();
    if delegate_class.name() != TrafficLightDelegate::class().name() {
        return;
    }

    let ptr: *const ProtocolObject<dyn NSWindowDelegate> = &*delegate;
    let our_delegate: &TrafficLightDelegate = unsafe { &*(ptr as *const TrafficLightDelegate) };
    our_delegate.ivars().traffic_light_x.set(x);
    our_delegate.ivars().traffic_light_y.set(y);
}
