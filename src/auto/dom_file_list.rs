// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMFile;
use DOMObject;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMFileList(Object<ffi::WebKitDOMFileList, ffi::WebKitDOMFileListClass, DOMFileListClass>) @extends DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_file_list_get_type(),
    }
}

pub const NONE_DOM_FILE_LIST: Option<&DOMFileList> = None;

pub trait DOMFileListExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_length(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn item(&self, index: libc::c_ulong) -> Option<DOMFile>;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMFileList>> DOMFileListExt for O {
    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_file_list_get_length(self.as_ref().to_glib_none().0)
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<DOMFile> {
        unsafe {
            from_glib_full(ffi::webkit_dom_file_list_item(self.as_ref().to_glib_none().0, index))
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::length\0".as_ptr() as *const _,
                Some(transmute(notify_length_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMFileList, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMFileList> {
    let f: &F = &*(f as *const F);
    f(&DOMFileList::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMFileList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMFileList")
    }
}
