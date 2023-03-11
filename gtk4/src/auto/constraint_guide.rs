// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ConstraintStrength, ConstraintTarget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkConstraintGuide")]
    pub struct ConstraintGuide(Object<ffi::GtkConstraintGuide, ffi::GtkConstraintGuideClass>) @implements ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_constraint_guide_get_type(),
    }
}

impl ConstraintGuide {
    #[doc(alias = "gtk_constraint_guide_new")]
    pub fn new() -> ConstraintGuide {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_constraint_guide_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ConstraintGuide`] objects.
    ///
    /// This method returns an instance of [`ConstraintGuideBuilder`](crate::builders::ConstraintGuideBuilder) which can be used to create [`ConstraintGuide`] objects.
    pub fn builder() -> ConstraintGuideBuilder {
        ConstraintGuideBuilder::new()
    }

    #[doc(alias = "gtk_constraint_guide_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_constraint_guide_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_guide_get_strength")]
    #[doc(alias = "get_strength")]
    pub fn strength(&self) -> ConstraintStrength {
        unsafe {
            from_glib(ffi::gtk_constraint_guide_get_strength(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_guide_set_max_size")]
    pub fn set_max_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_constraint_guide_set_max_size(self.to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "gtk_constraint_guide_set_min_size")]
    pub fn set_min_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_constraint_guide_set_min_size(self.to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "gtk_constraint_guide_set_name")]
    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_constraint_guide_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_constraint_guide_set_nat_size")]
    pub fn set_nat_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_constraint_guide_set_nat_size(self.to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "gtk_constraint_guide_set_strength")]
    pub fn set_strength(&self, strength: ConstraintStrength) {
        unsafe {
            ffi::gtk_constraint_guide_set_strength(self.to_glib_none().0, strength.into_glib());
        }
    }

    #[doc(alias = "max-height")]
    pub fn max_height(&self) -> i32 {
        glib::ObjectExt::property(self, "max-height")
    }

    #[doc(alias = "max-height")]
    pub fn set_max_height(&self, max_height: i32) {
        glib::ObjectExt::set_property(self, "max-height", max_height)
    }

    #[doc(alias = "max-width")]
    pub fn max_width(&self) -> i32 {
        glib::ObjectExt::property(self, "max-width")
    }

    #[doc(alias = "max-width")]
    pub fn set_max_width(&self, max_width: i32) {
        glib::ObjectExt::set_property(self, "max-width", max_width)
    }

    #[doc(alias = "min-height")]
    pub fn min_height(&self) -> i32 {
        glib::ObjectExt::property(self, "min-height")
    }

    #[doc(alias = "min-height")]
    pub fn set_min_height(&self, min_height: i32) {
        glib::ObjectExt::set_property(self, "min-height", min_height)
    }

    #[doc(alias = "min-width")]
    pub fn min_width(&self) -> i32 {
        glib::ObjectExt::property(self, "min-width")
    }

    #[doc(alias = "min-width")]
    pub fn set_min_width(&self, min_width: i32) {
        glib::ObjectExt::set_property(self, "min-width", min_width)
    }

    #[doc(alias = "nat-height")]
    pub fn nat_height(&self) -> i32 {
        glib::ObjectExt::property(self, "nat-height")
    }

    #[doc(alias = "nat-height")]
    pub fn set_nat_height(&self, nat_height: i32) {
        glib::ObjectExt::set_property(self, "nat-height", nat_height)
    }

    #[doc(alias = "nat-width")]
    pub fn nat_width(&self) -> i32 {
        glib::ObjectExt::property(self, "nat-width")
    }

    #[doc(alias = "nat-width")]
    pub fn set_nat_width(&self, nat_width: i32) {
        glib::ObjectExt::set_property(self, "nat-width", nat_width)
    }

    #[doc(alias = "max-height")]
    pub fn connect_max_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_height_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::max-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-width")]
    pub fn connect_max_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_width_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::max-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-height")]
    pub fn connect_min_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_height_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::min-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-width")]
    pub fn connect_min_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_width_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::min-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "nat-height")]
    pub fn connect_nat_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_nat_height_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::nat-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_nat_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "nat-width")]
    pub fn connect_nat_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_nat_width_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::nat-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_nat_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "strength")]
    pub fn connect_strength_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_strength_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::strength\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_strength_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ConstraintGuide {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ConstraintGuide`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ConstraintGuideBuilder {
    builder: glib::object::ObjectBuilder<'static, ConstraintGuide>,
}

impl ConstraintGuideBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn max_height(self, max_height: i32) -> Self {
        Self {
            builder: self.builder.property("max-height", max_height),
        }
    }

    pub fn max_width(self, max_width: i32) -> Self {
        Self {
            builder: self.builder.property("max-width", max_width),
        }
    }

    pub fn min_height(self, min_height: i32) -> Self {
        Self {
            builder: self.builder.property("min-height", min_height),
        }
    }

    pub fn min_width(self, min_width: i32) -> Self {
        Self {
            builder: self.builder.property("min-width", min_width),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn nat_height(self, nat_height: i32) -> Self {
        Self {
            builder: self.builder.property("nat-height", nat_height),
        }
    }

    pub fn nat_width(self, nat_width: i32) -> Self {
        Self {
            builder: self.builder.property("nat-width", nat_width),
        }
    }

    pub fn strength(self, strength: ConstraintStrength) -> Self {
        Self {
            builder: self.builder.property("strength", strength),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ConstraintGuide`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ConstraintGuide {
        self.builder.build()
    }
}

impl fmt::Display for ConstraintGuide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConstraintGuide")
    }
}
