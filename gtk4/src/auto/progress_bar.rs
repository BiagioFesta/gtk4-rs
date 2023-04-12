// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v4_10", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
use crate::AccessibleRange;
use crate::{
    Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Orientable,
    Orientation, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

#[cfg(any(feature = "v4_10", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
glib::wrapper! {
    #[doc(alias = "GtkProgressBar")]
    pub struct ProgressBar(Object<ffi::GtkProgressBar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, AccessibleRange, Orientable;

    match fn {
        type_ => || ffi::gtk_progress_bar_get_type(),
    }
}

#[cfg(not(any(feature = "v4_10", docsrs)))]
glib::wrapper! {
    #[doc(alias = "GtkProgressBar")]
    pub struct ProgressBar(Object<ffi::GtkProgressBar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_progress_bar_get_type(),
    }
}

impl ProgressBar {
    #[doc(alias = "gtk_progress_bar_new")]
    pub fn new() -> ProgressBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_progress_bar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ProgressBar`] objects.
    ///
    /// This method returns an instance of [`ProgressBarBuilder`](crate::builders::ProgressBarBuilder) which can be used to create [`ProgressBar`] objects.
    pub fn builder() -> ProgressBarBuilder {
        ProgressBarBuilder::new()
    }

    #[doc(alias = "gtk_progress_bar_get_ellipsize")]
    #[doc(alias = "get_ellipsize")]
    pub fn ellipsize(&self) -> pango::EllipsizeMode {
        unsafe { from_glib(ffi::gtk_progress_bar_get_ellipsize(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_progress_bar_get_fraction")]
    #[doc(alias = "get_fraction")]
    pub fn fraction(&self) -> f64 {
        unsafe { ffi::gtk_progress_bar_get_fraction(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_progress_bar_get_inverted")]
    #[doc(alias = "get_inverted")]
    pub fn is_inverted(&self) -> bool {
        unsafe { from_glib(ffi::gtk_progress_bar_get_inverted(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_progress_bar_get_pulse_step")]
    #[doc(alias = "get_pulse_step")]
    pub fn pulse_step(&self) -> f64 {
        unsafe { ffi::gtk_progress_bar_get_pulse_step(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_progress_bar_get_show_text")]
    #[doc(alias = "get_show_text")]
    pub fn shows_text(&self) -> bool {
        unsafe { from_glib(ffi::gtk_progress_bar_get_show_text(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_progress_bar_get_text")]
    #[doc(alias = "get_text")]
    pub fn text(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_progress_bar_get_text(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_progress_bar_pulse")]
    pub fn pulse(&self) {
        unsafe {
            ffi::gtk_progress_bar_pulse(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_progress_bar_set_ellipsize")]
    pub fn set_ellipsize(&self, mode: pango::EllipsizeMode) {
        unsafe {
            ffi::gtk_progress_bar_set_ellipsize(self.to_glib_none().0, mode.into_glib());
        }
    }

    #[doc(alias = "gtk_progress_bar_set_fraction")]
    pub fn set_fraction(&self, fraction: f64) {
        unsafe {
            ffi::gtk_progress_bar_set_fraction(self.to_glib_none().0, fraction);
        }
    }

    #[doc(alias = "gtk_progress_bar_set_inverted")]
    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_progress_bar_set_inverted(self.to_glib_none().0, inverted.into_glib());
        }
    }

    #[doc(alias = "gtk_progress_bar_set_pulse_step")]
    pub fn set_pulse_step(&self, fraction: f64) {
        unsafe {
            ffi::gtk_progress_bar_set_pulse_step(self.to_glib_none().0, fraction);
        }
    }

    #[doc(alias = "gtk_progress_bar_set_show_text")]
    pub fn set_show_text(&self, show_text: bool) {
        unsafe {
            ffi::gtk_progress_bar_set_show_text(self.to_glib_none().0, show_text.into_glib());
        }
    }

    #[doc(alias = "gtk_progress_bar_set_text")]
    pub fn set_text(&self, text: Option<&str>) {
        unsafe {
            ffi::gtk_progress_bar_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[doc(alias = "ellipsize")]
    pub fn connect_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ellipsize_trampoline<F: Fn(&ProgressBar) + 'static>(
            this: *mut ffi::GtkProgressBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ellipsize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ellipsize_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fraction")]
    pub fn connect_fraction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fraction_trampoline<F: Fn(&ProgressBar) + 'static>(
            this: *mut ffi::GtkProgressBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::fraction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fraction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "inverted")]
    pub fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<F: Fn(&ProgressBar) + 'static>(
            this: *mut ffi::GtkProgressBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pulse-step")]
    pub fn connect_pulse_step_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pulse_step_trampoline<F: Fn(&ProgressBar) + 'static>(
            this: *mut ffi::GtkProgressBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pulse-step\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pulse_step_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-text")]
    pub fn connect_show_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_text_trampoline<F: Fn(&ProgressBar) + 'static>(
            this: *mut ffi::GtkProgressBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text")]
    pub fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<F: Fn(&ProgressBar) + 'static>(
            this: *mut ffi::GtkProgressBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ProgressBar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ProgressBarBuilder {
    builder: glib::object::ObjectBuilder<'static, ProgressBar>,
}

impl ProgressBarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn ellipsize(self, ellipsize: pango::EllipsizeMode) -> Self {
        Self {
            builder: self.builder.property("ellipsize", ellipsize),
        }
    }

    pub fn fraction(self, fraction: f64) -> Self {
        Self {
            builder: self.builder.property("fraction", fraction),
        }
    }

    pub fn inverted(self, inverted: bool) -> Self {
        Self {
            builder: self.builder.property("inverted", inverted),
        }
    }

    pub fn pulse_step(self, pulse_step: f64) -> Self {
        Self {
            builder: self.builder.property("pulse-step", pulse_step),
        }
    }

    pub fn show_text(self, show_text: bool) -> Self {
        Self {
            builder: self.builder.property("show-text", show_text),
        }
    }

    pub fn text(self, text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("text", text.into()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ProgressBar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ProgressBar {
        self.builder.build()
    }
}

impl fmt::Display for ProgressBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ProgressBar")
    }
}
