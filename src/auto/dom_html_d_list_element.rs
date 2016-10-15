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
    pub struct DOMHTMLDListElement(Object<ffi::WebKitDOMHTMLDListElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_d_list_element_get_type(),
    }
}

impl DOMHTMLDListElement {
    pub fn get_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_d_list_element_get_compact(self.to_glib_none().0))
        }
    }

    pub fn set_compact(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_d_list_element_set_compact(self.to_glib_none().0, value.to_glib());
        }
    }
}
