// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ScrollDirection;
#[cfg(any(feature = "v4_8", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
use crate::ScrollUnit;
use glib::translate::*;
use std::{fmt, mem};

glib::wrapper! {
    #[doc(alias = "GdkScrollEvent")]
    pub struct ScrollEvent(Shared<ffi::GdkScrollEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

impl glib::StaticType for ScrollEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_scroll_event_get_type()) }
    }
}

impl ScrollEvent {
    #[doc(alias = "gdk_scroll_event_get_deltas")]
    #[doc(alias = "get_deltas")]
    pub fn deltas(&self) -> (f64, f64) {
        unsafe {
            let mut delta_x = mem::MaybeUninit::uninit();
            let mut delta_y = mem::MaybeUninit::uninit();
            ffi::gdk_scroll_event_get_deltas(
                self.to_glib_none().0,
                delta_x.as_mut_ptr(),
                delta_y.as_mut_ptr(),
            );
            (delta_x.assume_init(), delta_y.assume_init())
        }
    }

    #[doc(alias = "gdk_scroll_event_get_direction")]
    #[doc(alias = "get_direction")]
    pub fn direction(&self) -> ScrollDirection {
        unsafe { from_glib(ffi::gdk_scroll_event_get_direction(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v4_8", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gdk_scroll_event_get_unit")]
    #[doc(alias = "get_unit")]
    pub fn unit(&self) -> ScrollUnit {
        unsafe { from_glib(ffi::gdk_scroll_event_get_unit(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_scroll_event_is_stop")]
    pub fn is_stop(&self) -> bool {
        unsafe { from_glib(ffi::gdk_scroll_event_is_stop(self.to_glib_none().0)) }
    }
}

impl fmt::Display for ScrollEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ScrollEvent")
    }
}
