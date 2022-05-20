// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Monitor;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct ToplevelLayout(Shared<ffi::GdkToplevelLayout>);

    match fn {
        ref => |ptr| ffi::gdk_toplevel_layout_ref(ptr),
        unref => |ptr| ffi::gdk_toplevel_layout_unref(ptr),
        type_ => || ffi::gdk_toplevel_layout_get_type(),
    }
}

impl ToplevelLayout {
    #[doc(alias = "gdk_toplevel_layout_new")]
    pub fn new() -> ToplevelLayout {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gdk_toplevel_layout_new()) }
    }

    #[doc(alias = "gdk_toplevel_layout_copy")]
    #[must_use]
    pub fn copy(&self) -> ToplevelLayout {
        unsafe { from_glib_full(ffi::gdk_toplevel_layout_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_toplevel_layout_equal")]
    fn equal(&self, other: &ToplevelLayout) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_layout_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_toplevel_layout_get_fullscreen")]
    #[doc(alias = "get_fullscreen")]
    pub fn fullscreen(&self) -> Option<bool> {
        unsafe {
            let mut fullscreen = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_toplevel_layout_get_fullscreen(
                self.to_glib_none().0,
                fullscreen.as_mut_ptr(),
            ));
            if ret {
                Some(from_glib(fullscreen.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_toplevel_layout_get_fullscreen_monitor")]
    #[doc(alias = "get_fullscreen_monitor")]
    pub fn fullscreen_monitor(&self) -> Option<Monitor> {
        unsafe {
            from_glib_none(ffi::gdk_toplevel_layout_get_fullscreen_monitor(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_toplevel_layout_get_maximized")]
    #[doc(alias = "get_maximized")]
    pub fn maximized(&self) -> Option<bool> {
        unsafe {
            let mut maximized = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_toplevel_layout_get_maximized(
                self.to_glib_none().0,
                maximized.as_mut_ptr(),
            ));
            if ret {
                Some(from_glib(maximized.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_toplevel_layout_get_resizable")]
    #[doc(alias = "get_resizable")]
    pub fn is_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_layout_get_resizable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_toplevel_layout_set_fullscreen")]
    pub fn set_fullscreen(&self, fullscreen: bool, monitor: Option<&impl IsA<Monitor>>) {
        unsafe {
            ffi::gdk_toplevel_layout_set_fullscreen(
                self.to_glib_none().0,
                fullscreen.into_glib(),
                monitor.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_toplevel_layout_set_maximized")]
    pub fn set_maximized(&self, maximized: bool) {
        unsafe {
            ffi::gdk_toplevel_layout_set_maximized(self.to_glib_none().0, maximized.into_glib());
        }
    }

    #[doc(alias = "gdk_toplevel_layout_set_resizable")]
    pub fn set_resizable(&self, resizable: bool) {
        unsafe {
            ffi::gdk_toplevel_layout_set_resizable(self.to_glib_none().0, resizable.into_glib());
        }
    }
}

impl Default for ToplevelLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for ToplevelLayout {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for ToplevelLayout {}
