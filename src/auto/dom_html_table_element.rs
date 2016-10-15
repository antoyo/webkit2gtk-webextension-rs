// This file was generated by gir (06c1d5c+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMHTMLElement;
use DOMHTMLTableCaptionElement;
use DOMHTMLTableSectionElement;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLTableElement(Object<ffi::WebKitDOMHTMLTableElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_table_element_get_type(),
    }
}

impl DOMHTMLTableElement {
    pub fn create_caption(&self) -> Option<DOMHTMLElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_create_caption(self.to_glib_none().0))
        }
    }

    pub fn create_t_foot(&self) -> Option<DOMHTMLElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_create_t_foot(self.to_glib_none().0))
        }
    }

    pub fn create_t_head(&self) -> Option<DOMHTMLElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_create_t_head(self.to_glib_none().0))
        }
    }

    pub fn delete_caption(&self) {
        unsafe {
            ffi::webkit_dom_html_table_element_delete_caption(self.to_glib_none().0);
        }
    }

    pub fn delete_row(&self, index: i64) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_table_element_delete_row(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn delete_t_foot(&self) {
        unsafe {
            ffi::webkit_dom_html_table_element_delete_t_foot(self.to_glib_none().0);
        }
    }

    pub fn delete_t_head(&self) {
        unsafe {
            ffi::webkit_dom_html_table_element_delete_t_head(self.to_glib_none().0);
        }
    }

    pub fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_align(self.to_glib_none().0))
        }
    }

    pub fn get_bg_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_bg_color(self.to_glib_none().0))
        }
    }

    pub fn get_border(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_border(self.to_glib_none().0))
        }
    }

    pub fn get_caption(&self) -> Option<DOMHTMLTableCaptionElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_get_caption(self.to_glib_none().0))
        }
    }

    pub fn get_cell_padding(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_cell_padding(self.to_glib_none().0))
        }
    }

    pub fn get_cell_spacing(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_cell_spacing(self.to_glib_none().0))
        }
    }

    pub fn get_rows(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_rows(self.to_glib_none().0))
        }
    }

    pub fn get_rules(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_rules(self.to_glib_none().0))
        }
    }

    pub fn get_summary(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_summary(self.to_glib_none().0))
        }
    }

    pub fn get_t_bodies(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_t_bodies(self.to_glib_none().0))
        }
    }

    pub fn get_t_foot(&self) -> Option<DOMHTMLTableSectionElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_get_t_foot(self.to_glib_none().0))
        }
    }

    pub fn get_t_head(&self) -> Option<DOMHTMLTableSectionElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_get_t_head(self.to_glib_none().0))
        }
    }

    pub fn get_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_width(self.to_glib_none().0))
        }
    }

    pub fn insert_row(&self, index: i64) -> Result<DOMHTMLElement, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_html_table_element_insert_row(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_bg_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_bg_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_border(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_caption(&self, value: &DOMHTMLTableCaptionElement) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_table_element_set_caption(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_cell_padding(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_cell_padding(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_cell_spacing(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_cell_spacing(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_rules(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_rules(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_summary(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_summary(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_t_foot(&self, value: &DOMHTMLTableSectionElement) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_table_element_set_t_foot(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_t_head(&self, value: &DOMHTMLTableSectionElement) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_table_element_set_t_head(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
