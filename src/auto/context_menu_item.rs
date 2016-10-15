// This file was generated by gir (06c1d5c+) from gir-files (???)
// DO NOT EDIT

use ContextMenu;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct ContextMenuItem(Object<ffi::WebKitContextMenuItem>);

    match fn {
        get_type => || ffi::webkit_context_menu_item_get_type(),
    }
}

impl ContextMenuItem {
    //pub fn new<T: IsA</*Ignored*/gtk::Action>>(action: &T) -> ContextMenuItem {
    //    unsafe { TODO: call ffi::webkit_context_menu_item_new() }
    //}

    //pub fn new_from_stock_action(action: /*Ignored*/ContextMenuAction) -> ContextMenuItem {
    //    unsafe { TODO: call ffi::webkit_context_menu_item_new_from_stock_action() }
    //}

    //pub fn new_from_stock_action_with_label(action: /*Ignored*/ContextMenuAction, label: &str) -> ContextMenuItem {
    //    unsafe { TODO: call ffi::webkit_context_menu_item_new_from_stock_action_with_label() }
    //}

    pub fn new_separator() -> ContextMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_context_menu_item_new_separator())
        }
    }

    pub fn new_with_submenu(label: &str, submenu: &ContextMenu) -> ContextMenuItem {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::webkit_context_menu_item_new_with_submenu(label.to_glib_none().0, submenu.to_glib_none().0))
        }
    }

    //pub fn get_action(&self) -> /*Ignored*/Option<gtk::Action> {
    //    unsafe { TODO: call ffi::webkit_context_menu_item_get_action() }
    //}

    //pub fn get_stock_action(&self) -> /*Ignored*/ContextMenuAction {
    //    unsafe { TODO: call ffi::webkit_context_menu_item_get_stock_action() }
    //}

    pub fn get_submenu(&self) -> Option<ContextMenu> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_item_get_submenu(self.to_glib_none().0))
        }
    }

    pub fn is_separator(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_context_menu_item_is_separator(self.to_glib_none().0))
        }
    }

    pub fn set_submenu(&self, submenu: Option<&ContextMenu>) {
        unsafe {
            ffi::webkit_context_menu_item_set_submenu(self.to_glib_none().0, submenu.to_glib_none().0);
        }
    }
}
