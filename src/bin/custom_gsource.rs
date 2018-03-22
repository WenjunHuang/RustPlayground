extern crate glib_sys as glib_ffi;
extern crate gio_sys as gio_ffi;
extern crate gobject_sys as gobject_ffi;

#[repr(C)]
struct MessageQueueSource{
    parent: glib_ffi::GSource,
}