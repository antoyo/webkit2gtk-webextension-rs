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
    pub struct DOMHTMLHtmlElement(Object<ffi::WebKitDOMHTMLHtmlElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_html_element_get_type(),
    }
}

impl DOMHTMLHtmlElement {
    pub fn get_version(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_html_element_get_version(self.to_glib_none().0))
        }
    }

    pub fn set_version(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_html_element_set_version(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
