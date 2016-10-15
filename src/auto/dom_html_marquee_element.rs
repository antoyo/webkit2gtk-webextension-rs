// This file was generated by gir (06c1d5c+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLMarqueeElement(Object<ffi::WebKitDOMHTMLMarqueeElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_marquee_element_get_type(),
    }
}

impl DOMHTMLMarqueeElement {
    pub fn start(&self) {
        unsafe {
            ffi::webkit_dom_html_marquee_element_start(self.to_glib_none().0);
        }
    }

    pub fn stop(&self) {
        unsafe {
            ffi::webkit_dom_html_marquee_element_stop(self.to_glib_none().0);
        }
    }
}
