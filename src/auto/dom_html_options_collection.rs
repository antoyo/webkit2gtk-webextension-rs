// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMHTMLCollection;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLOptionsCollection(Object<ffi::WebKitDOMHTMLOptionsCollection>): DOMHTMLCollection, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_options_collection_get_type(),
    }
}

impl DOMHTMLOptionsCollection {
    pub fn get_length(&self) -> u64 {
        unsafe {
            ffi::webkit_dom_html_options_collection_get_length(self.to_glib_none().0)
        }
    }

    pub fn get_selected_index(&self) -> i64 {
        unsafe {
            ffi::webkit_dom_html_options_collection_get_selected_index(self.to_glib_none().0)
        }
    }

    pub fn named_item(&self, name: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_options_collection_named_item(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn set_selected_index(&self, value: i64) {
        unsafe {
            ffi::webkit_dom_html_options_collection_set_selected_index(self.to_glib_none().0, value);
        }
    }
}
