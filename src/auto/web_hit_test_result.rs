// This file was generated by gir (06c1d5c+) from gir-files (???)
// DO NOT EDIT

#[cfg(feature = "v2_8")]
use DOMNode;
use HitTestResult;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct WebHitTestResult(Object<ffi::WebKitWebHitTestResult>): HitTestResult;

    match fn {
        get_type => || ffi::webkit_web_hit_test_result_get_type(),
    }
}

impl WebHitTestResult {
    #[cfg(feature = "v2_8")]
    pub fn get_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_web_hit_test_result_get_node(self.to_glib_none().0))
        }
    }
}
