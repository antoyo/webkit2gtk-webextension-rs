// This file was generated by gir (06c1d5c+) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMObject(Object<ffi::WebKitDOMObject>);

    match fn {
        get_type => || ffi::webkit_dom_object_get_type(),
    }
}

pub trait DOMObjectExt {}

impl<O: IsA<DOMObject>> DOMObjectExt for O {}
