// This file was generated by gir (e6cb5d0) from gir-files (11e0e6d)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureDrag;
use GestureSingle;
#[cfg(feature = "v3_14")]
use Orientation;
#[cfg(feature = "v3_14")]
use PanDirection;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_14")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_14")]
use glib_ffi;
#[cfg(feature = "v3_14")]
use libc;
#[cfg(feature = "v3_14")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_14")]
use std::mem::transmute;

glib_wrapper! {
    pub struct GesturePan(Object<ffi::GtkGesturePan>): GestureDrag, GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_pan_get_type(),
    }
}

impl GesturePan {
    #[cfg(feature = "v3_14")]
    pub fn new<T: IsA<Widget>>(widget: &T, orientation: Orientation) -> GesturePan {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_pan_new(widget.to_glib_none().0, orientation.to_glib())).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_gesture_pan_get_orientation(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_gesture_pan_set_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_pan<F: Fn(&GesturePan, PanDirection, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&GesturePan, PanDirection, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pan",
                transmute(pan_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn pan_trampoline(this: *mut ffi::GtkGesturePan, direction: ffi::GtkPanDirection, offset: libc::c_double, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&GesturePan, PanDirection, f64) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(direction), offset)
}
