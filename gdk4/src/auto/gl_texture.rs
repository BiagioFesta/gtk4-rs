// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{Paintable, Texture};
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkGLTexture")]
    pub struct GLTexture(Object<ffi::GdkGLTexture, ffi::GdkGLTextureClass>) @extends Texture, @implements Paintable, gio::Icon, gio::LoadableIcon;

    match fn {
        type_ => || ffi::gdk_gl_texture_get_type(),
    }
}

impl GLTexture {
    #[doc(alias = "gdk_gl_texture_release")]
    pub fn release(&self) {
        unsafe {
            ffi::gdk_gl_texture_release(self.to_glib_none().0);
        }
    }
}

unsafe impl Send for GLTexture {}
unsafe impl Sync for GLTexture {}

impl fmt::Display for GLTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLTexture")
    }
}
