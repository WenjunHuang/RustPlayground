extern crate glib_sys;

use glib_sys as glib_ffi;
use std::ffi::*;
use std::os::raw::*;

trait FromGlib<T>: Sized {
    fn from_glib(val: T) -> Self;
}

trait ToGlib {
    type GlibType;
    fn to_glib(&self) -> Self::GlibType;
}

impl FromGlib<glib_ffi::gboolean> for bool {
    fn from_glib(val: glib_ffi::gboolean) -> bool {
        !(val == glib_ffi::GFALSE)
    }
}

impl ToGlib for bool {
    type GlibType = glib_ffi::gboolean;

    fn to_glib(&self) -> glib_ffi::gboolean {
        if *self { glib_ffi::GTRUE } else { glib_ffi::GFALSE }
    }
}

trait FromGlibPtrNone<P>:Sized {
    unsafe fn from_glib_none(ptr:P) ->Self;
}
unsafe fn from_glib_none<P,T: FromGlibPtrNone<P>>(ptr:P) -> T{
    FromGlibPtrNone::from_glib_none(ptr)
}

impl FromGlibPtrNone<*const c_char> for String {
    unsafe fn from_glib_none(ptr: *const c_char) -> Self {
        assert!(!ptr.is_null());
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }
}

fn main() {
    let v = glib_ffi::GTRUE;
    let r: bool = FromGlib::from_glib(v);
    assert_eq!(true,r);

    let gv = ToGlib::to_glib(&r);
    assert_eq!(glib_ffi::GTRUE,gv);

    let to_print = CString::new("Hello!").unwrap();
    let result:String;
    unsafe {
        result = FromGlibPtrNone::from_glib_none(to_print.as_ptr());
    }
    println!("{}",result);
}