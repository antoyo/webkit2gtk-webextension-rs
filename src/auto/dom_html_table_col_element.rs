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
    pub struct DOMHTMLTableColElement(Object<ffi::WebKitDOMHTMLTableColElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_table_col_element_get_type(),
    }
}

impl DOMHTMLTableColElement {
    pub fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_col_element_get_align(self.to_glib_none().0))
        }
    }

    pub fn get_ch(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_col_element_get_ch(self.to_glib_none().0))
        }
    }

    pub fn get_ch_off(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_col_element_get_ch_off(self.to_glib_none().0))
        }
    }

    pub fn get_span(&self) -> i64 {
        unsafe {
            ffi::webkit_dom_html_table_col_element_get_span(self.to_glib_none().0)
        }
    }

    pub fn get_v_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_col_element_get_v_align(self.to_glib_none().0))
        }
    }

    pub fn get_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_col_element_get_width(self.to_glib_none().0))
        }
    }

    pub fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_col_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_ch(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_col_element_set_ch(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_ch_off(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_col_element_set_ch_off(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_span(&self, value: i64) {
        unsafe {
            ffi::webkit_dom_html_table_col_element_set_span(self.to_glib_none().0, value);
        }
    }

    pub fn set_v_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_col_element_set_v_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_col_element_set_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
