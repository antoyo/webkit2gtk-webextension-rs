// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMCSSStyleSheet;
use DOMObject;
use Error;
use ffi;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMCSSRule(Object<ffi::WebKitDOMCSSRule, ffi::WebKitDOMCSSRuleClass, DOMCSSRuleClass>) @extends DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_css_rule_get_type(),
    }
}

pub const NONE_DOMCSS_RULE: Option<&DOMCSSRule> = None;

pub trait DOMCSSRuleExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_css_text(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_parent_rule(&self) -> Option<DOMCSSRule>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_parent_style_sheet(&self) -> Option<DOMCSSStyleSheet>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_rule_type(&self) -> libc::c_ushort;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_css_text(&self, value: &str) -> Result<(), Error>;

    fn get_property_type(&self) -> u32;

    fn connect_property_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_style_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMCSSRule>> DOMCSSRuleExt for O {
    fn get_css_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_rule_get_css_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_parent_rule(&self) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_rule_get_parent_rule(self.as_ref().to_glib_none().0))
        }
    }

    fn get_parent_style_sheet(&self) -> Option<DOMCSSStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_rule_get_parent_style_sheet(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rule_type(&self) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_css_rule_get_rule_type(self.as_ref().to_glib_none().0)
        }
    }

    fn set_css_text(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_rule_set_css_text(self.as_ref().to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_type(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::css-text\0".as_ptr() as *const _,
                Some(transmute(notify_css_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_parent_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::parent-rule\0".as_ptr() as *const _,
                Some(transmute(notify_parent_rule_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_parent_style_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::parent-style-sheet\0".as_ptr() as *const _,
                Some(transmute(notify_parent_style_sheet_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                Some(transmute(notify_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_css_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMCSSRule, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSRule> {
    let f: &F = &*(f as *const F);
    f(&DOMCSSRule::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_parent_rule_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMCSSRule, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSRule> {
    let f: &F = &*(f as *const F);
    f(&DOMCSSRule::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_parent_style_sheet_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMCSSRule, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSRule> {
    let f: &F = &*(f as *const F);
    f(&DOMCSSRule::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMCSSRule, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCSSRule> {
    let f: &F = &*(f as *const F);
    f(&DOMCSSRule::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMCSSRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMCSSRule")
    }
}
