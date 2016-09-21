// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMDOMWindow;
use DOMEvent;
use DOMMouseEvent;
use DOMObject;
use DOMUIEvent;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMWheelEvent(Object<ffi::WebKitDOMWheelEvent>): DOMMouseEvent, DOMUIEvent, DOMEvent, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_wheel_event_get_type(),
    }
}

impl DOMWheelEvent {
    pub fn get_wheel_delta(&self) -> i64 {
        unsafe {
            ffi::webkit_dom_wheel_event_get_wheel_delta(self.to_glib_none().0)
        }
    }

    pub fn get_wheel_delta_x(&self) -> i64 {
        unsafe {
            ffi::webkit_dom_wheel_event_get_wheel_delta_x(self.to_glib_none().0)
        }
    }

    pub fn get_wheel_delta_y(&self) -> i64 {
        unsafe {
            ffi::webkit_dom_wheel_event_get_wheel_delta_y(self.to_glib_none().0)
        }
    }

    pub fn init_wheel_event(&self, wheelDeltaX: i64, wheelDeltaY: i64, view: &DOMDOMWindow, screenX: i64, screenY: i64, clientX: i64, clientY: i64, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool) {
        unsafe {
            ffi::webkit_dom_wheel_event_init_wheel_event(self.to_glib_none().0, wheelDeltaX, wheelDeltaY, view.to_glib_none().0, screenX, screenY, clientX, clientY, ctrlKey.to_glib(), altKey.to_glib(), shiftKey.to_glib(), metaKey.to_glib());
        }
    }
}
