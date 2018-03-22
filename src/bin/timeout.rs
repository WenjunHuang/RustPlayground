extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate glib;

use glib::prelude::*;

#[no_mangle]
unsafe extern "C" fn call_back() {
    static mut COUNT: i32 = 0;

    println!("call back by main loop");
    COUNT += 1;
//    if count >= 10 {
//        glib_ffi::g_main_loop_quit();
//    }
}

fn main() {
//    let main_context = glib_ffi::g_main_context_default();
//    let main_loop = glib_ffi::g_main_loop_new(main_context, false);
    let main_context = glib::MainContext::default().unwrap();
    let main_loop = glib::MainLoop::new(&main_context,true);
    glib::source::timeout_add(1000,||{
        println!("Hello");
        glib::source::Continue(true)
    });
    main_loop.run();
//    glib_ffi::g_timeout_add(1000,
//                            Some(call_back),
//                            None);
}

