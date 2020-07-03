// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_x11_sys;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use std::fmt;

glib_wrapper! {
    pub struct X11DeviceXI2(Object<gdk_x11_sys::GdkX11DeviceXI2, gdk_x11_sys::GdkX11DeviceXI2Class, X11DeviceXI2Class>) @extends gdk::Device;

    match fn {
        get_type => || gdk_x11_sys::gdk_x11_device_xi2_get_type(),
    }
}

impl X11DeviceXI2 {
    pub fn get_property_device_id(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"device-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `device-id` getter")
                .unwrap()
        }
    }
}

impl fmt::Display for X11DeviceXI2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11DeviceXI2")
    }
}
