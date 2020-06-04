// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_x11_sys;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct X11Monitor(Object<gdk_x11_sys::GdkX11Monitor, gdk_x11_sys::GdkX11MonitorClass, X11MonitorClass>) @extends gdk::Monitor;

    match fn {
        get_type => || gdk_x11_sys::gdk_x11_monitor_get_type(),
    }
}

impl X11Monitor {
    //pub fn get_output<P: IsA<gdk::Monitor>>(monitor: &P) -> /*Ignored*/xlib::XID {
    //    unsafe { TODO: call gdk_x11_sys:gdk_x11_monitor_get_output() }
    //}
}

impl fmt::Display for X11Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11Monitor")
    }
}