// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMNode;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use DOMNodeList;
use DOMObject;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use Error;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use std::ptr;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMDocumentFragment(Object<webkit2_webextension_sys::WebKitDOMDocumentFragment, webkit2_webextension_sys::WebKitDOMDocumentFragmentClass, DOMDocumentFragmentClass>) @extends DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_document_fragment_get_type(),
    }
}

pub const NONE_DOM_DOCUMENT_FRAGMENT: Option<&DOMDocumentFragment> = None;

pub trait DOMDocumentFragmentExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_child_element_count(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_children(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_element_by_id(&self, elementId: &str) -> Option<DOMElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_first_element_child(&self) -> Option<DOMElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_last_element_child(&self) -> Option<DOMElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn query_selector(&self, selectors: &str) -> Result<DOMElement, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn query_selector_all(&self, selectors: &str) -> Result<DOMNodeList, Error>;

    fn get_property_child_element_count(&self) -> libc::c_ulong;

    fn get_property_children(&self) -> Option<DOMHTMLCollection>;

    fn get_property_first_element_child(&self) -> Option<DOMElement>;

    fn get_property_last_element_child(&self) -> Option<DOMElement>;

    fn connect_property_child_element_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_children_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_first_element_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_last_element_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMDocumentFragment>> DOMDocumentFragmentExt for O {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_child_element_count(&self) -> libc::c_ulong {
        unsafe {
            webkit2_webextension_sys::webkit_dom_document_fragment_get_child_element_count(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_children(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_document_fragment_get_children(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_element_by_id(&self, elementId: &str) -> Option<DOMElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_document_fragment_get_element_by_id(self.as_ref().to_glib_none().0, elementId.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_first_element_child(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_document_fragment_get_first_element_child(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_last_element_child(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_document_fragment_get_last_element_child(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn query_selector(&self, selectors: &str) -> Result<DOMElement, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_document_fragment_query_selector(self.as_ref().to_glib_none().0, selectors.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn query_selector_all(&self, selectors: &str) -> Result<DOMNodeList, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_document_fragment_query_selector_all(self.as_ref().to_glib_none().0, selectors.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_child_element_count(&self) -> libc::c_ulong {
        unsafe {
            let mut value = Value::from_type(<libc::c_ulong as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"child-element-count\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_children(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            let mut value = Value::from_type(<DOMHTMLCollection as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"children\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_first_element_child(&self) -> Option<DOMElement> {
        unsafe {
            let mut value = Value::from_type(<DOMElement as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"first-element-child\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_last_element_child(&self) -> Option<DOMElement> {
        unsafe {
            let mut value = Value::from_type(<DOMElement as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"last-element-child\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_child_element_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::child-element-count\0".as_ptr() as *const _,
                Some(transmute(notify_child_element_count_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_children_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::children\0".as_ptr() as *const _,
                Some(transmute(notify_children_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_first_element_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::first-element-child\0".as_ptr() as *const _,
                Some(transmute(notify_first_element_child_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_last_element_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::last-element-child\0".as_ptr() as *const _,
                Some(transmute(notify_last_element_child_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_child_element_count_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMDocumentFragment, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMDocumentFragment> {
    let f: &F = &*(f as *const F);
    f(&DOMDocumentFragment::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_children_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMDocumentFragment, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMDocumentFragment> {
    let f: &F = &*(f as *const F);
    f(&DOMDocumentFragment::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_first_element_child_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMDocumentFragment, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMDocumentFragment> {
    let f: &F = &*(f as *const F);
    f(&DOMDocumentFragment::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_last_element_child_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMDocumentFragment, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMDocumentFragment> {
    let f: &F = &*(f as *const F);
    f(&DOMDocumentFragment::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMDocumentFragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMDocumentFragment")
    }
}
