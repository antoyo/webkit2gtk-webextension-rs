// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMFileList;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLInputElement(Object<ffi::WebKitDOMHTMLInputElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_input_element_get_type(),
    }
}

impl DOMHTMLInputElement {
    pub fn get_accept(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_accept(self.to_glib_none().0))
        }
    }

    pub fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_align(self.to_glib_none().0))
        }
    }

    pub fn get_alt(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_alt(self.to_glib_none().0))
        }
    }

    pub fn get_autofocus(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_autofocus(self.to_glib_none().0))
        }
    }

    pub fn get_capture(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_capture(self.to_glib_none().0))
        }
    }

    pub fn get_checked(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_checked(self.to_glib_none().0))
        }
    }

    pub fn get_default_checked(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_default_checked(self.to_glib_none().0))
        }
    }

    pub fn get_default_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_default_value(self.to_glib_none().0))
        }
    }

    pub fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_disabled(self.to_glib_none().0))
        }
    }

    pub fn get_files(&self) -> Option<DOMFileList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_files(self.to_glib_none().0))
        }
    }

    pub fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_input_element_get_form(self.to_glib_none().0))
        }
    }

    pub fn get_height(&self) -> u64 {
        unsafe {
            ffi::webkit_dom_html_input_element_get_height(self.to_glib_none().0)
        }
    }

    pub fn get_indeterminate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_indeterminate(self.to_glib_none().0))
        }
    }

    pub fn get_input_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_input_type(self.to_glib_none().0))
        }
    }

    pub fn get_max_length(&self) -> i64 {
        unsafe {
            ffi::webkit_dom_html_input_element_get_max_length(self.to_glib_none().0)
        }
    }

    pub fn get_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_multiple(self.to_glib_none().0))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_size(&self) -> u64 {
        unsafe {
            ffi::webkit_dom_html_input_element_get_size(self.to_glib_none().0)
        }
    }

    pub fn get_src(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_src(self.to_glib_none().0))
        }
    }

    pub fn get_use_map(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_use_map(self.to_glib_none().0))
        }
    }

    pub fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_value(self.to_glib_none().0))
        }
    }

    pub fn get_width(&self) -> u64 {
        unsafe {
            ffi::webkit_dom_html_input_element_get_width(self.to_glib_none().0)
        }
    }

    pub fn get_will_validate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_will_validate(self.to_glib_none().0))
        }
    }

    pub fn is_edited(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_is_edited(self.to_glib_none().0))
        }
    }

    pub fn select(&self) {
        unsafe {
            ffi::webkit_dom_html_input_element_select(self.to_glib_none().0);
        }
    }

    pub fn set_accept(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_accept(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_alt(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_alt(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_autofocus(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_autofocus(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_checked(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_checked(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_default_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_default_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_files(&self, value: &DOMFileList) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_files(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_height(&self, value: u64) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_height(self.to_glib_none().0, value);
        }
    }

    pub fn set_indeterminate(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_indeterminate(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_input_type(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_input_type(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_max_length(&self, value: i64) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_input_element_set_max_length(self.to_glib_none().0, value, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_multiple(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_multiple(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_read_only(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_read_only(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_size(&self, value: u64) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_input_element_set_size(self.to_glib_none().0, value, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_src(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_use_map(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_use_map(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_width(&self, value: u64) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_width(self.to_glib_none().0, value);
        }
    }
}
