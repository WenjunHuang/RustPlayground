extern crate glib_sys;
extern crate libc;

use glib_sys as glib_ffi;
use std::ffi::*;
use std::os::raw::*;
use std::borrow::Cow;

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

trait FromGlibPtrNone<P>: Sized {
    unsafe fn from_glib_none(ptr: P) -> Self;
}

unsafe fn from_glib_none<P, T: FromGlibPtrNone<P>>(ptr: P) -> T {
    FromGlibPtrNone::from_glib_none(ptr)
}

impl FromGlibPtrNone<*const c_char> for String {
    unsafe fn from_glib_none(ptr: *const c_char) -> String {
        assert!(!ptr.is_null());
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }
}

trait FromGlibPtrFull<P>: Sized {
    unsafe fn from_glib_full(ptr: P) -> Self;
}

impl FromGlibPtrFull<*const c_char> for String {
    unsafe fn from_glib_full(ptr: *const c_char) -> Self {
        let res = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        res
    }
}

struct Stash<'a, P: Copy, T: ? Sized + ToGlibPtr<'a, P>> (
    P,
    <T as ToGlibPtr<'a, P>>::Storage,
);

trait ToGlibPtr<'a, P: Copy> {
    type Storage;
    fn to_glib_none(&'a self) -> Stash<'a, P, Self>;
    fn to_glib_full(&self) -> P;
}

impl<'a> ToGlibPtr<'a, *const c_char> for String {
    type Storage = CString;

    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, String> {
        let tmp = CString::new(&self[..]).unwrap();
        Stash(tmp.as_ptr(), tmp)
    }

    fn to_glib_full(&self) -> *const c_char {
        unsafe {
            glib_ffi::g_strndup(self.as_ptr() as *const c_char,
                                self.len() as size_t) as *const c_char
        }
    }
}

trait FromGlibPtrContainer<P>

fn main() {
    let v = glib_ffi::GTRUE;
    let r: bool = FromGlib::from_glib(v);
    assert_eq!(true, r);

    let gv = ToGlib::to_glib(&r);
    assert_eq!(glib_ffi::GTRUE, gv);

    let to_print = CString::new("Hello!").unwrap();
    let result: String;
    unsafe {
        result = FromGlibPtrNone::from_glib_none(to_print.as_ptr());
    }
    println!("{}", result);

    let my_string = "wenjun".to_owned();
    let ptr = my_string.to_glib_none();
    unsafe {
        println!("{}", libc::strlen(ptr.0));
    }
}