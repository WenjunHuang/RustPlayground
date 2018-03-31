extern crate glib_sys as glib_ffi;
extern crate glib;
extern crate gio_sys as gio_ffi;

use glib_ffi::*;
use gio_ffi::*;
use glib::*;
use glib::translate::*;
use std::os::raw::*;

#[no_mangle]
pub extern "C" fn callback(channel: gpointer) -> gboolean {
    unsafe {
        let mut s: *mut c_char = std::ptr::null_mut();
        let mut len: usize = 0;
        g_io_channel_read_line(channel as *mut GIOChannel, &mut s, &mut len, std::ptr::null_mut(), std::ptr::null_mut());

        let mut my_str: String = from_glib_full(s);
        let my_str = my_str.trim().chars().rev().collect::<String>();
        println!("{}", my_str);
    }
    true.to_glib()
}

fn add_source(context: &MainContext) {
    unsafe {
        let channel = g_io_channel_unix_new(1);
        let source = from_glib_full(g_io_create_watch(channel, G_IO_IN));

        g_io_channel_unref(channel);
        g_source_set_callback(source,Some(callback),channel as gpointer,None);
        g_source_attach(source,context.g_obj());
    }
}

fn main() {
    let main_context = MainContext::new();

    let main_loop = MainLoop::new(&main_context, false);
    main_loop.run();
}