// This file was generated by gir (06c1d5c+) from gir-files (???)
// DO NOT EDIT

use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMXPathExpression(Object<ffi::WebKitDOMXPathExpression>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_xpath_expression_get_type(),
    }
}

impl DOMXPathExpression {
    //pub fn evaluate<T: IsA<DOMNode>>(&self, contextNode: &T, type_: /*Unimplemented*/Fundamental: UShort, inResult: &DOMXPathResult) -> Result<DOMXPathResult, Error> {
    //    unsafe { TODO: call ffi::webkit_dom_xpath_expression_evaluate() }
    //}
}
