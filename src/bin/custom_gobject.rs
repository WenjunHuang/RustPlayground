extern crate gobject_sys;
extern crate glib_sys;

use std::mem;
use std::ptr;
use std::cell::RefCell;
use std::sync::{Once,ONCE_INIT};
use gobject_sys as gobject_ffi;
use glib_sys as glib_ffi;

#[repr(C)]
pub struct Foo{
  pub parent: gobject_ffi::GObject,
}

#[repr(C)]
pub struct FooClass {
  pub parent_class:gobject_ffi::GObjectClass,
}

struct FooPrivate {
  name: RefCell<Option<String>>,
  counter: RefCell<i32>,
}

#[no_mangle]
pub unsafe extern "C" fn ex_foo_get_type()->glib_ffi::GType {
  callback_guard!();

  static mut TYPE:glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
  static ONCE: Once = ONCE_INIT;

  ONCE.call_once(||{
    let type_info = gobject_ffi::GTypeInfo{
      class_size: mem::size_of::<FooClass>() as u16,
      base_init: None,
      base_finalize: None,
      class_init: Some(FooClass::init),
      class_finalize:None,
      class_data: ptr::null(),
      instance_size: mem::size_of::<Foo>() as u16,
      n_preallocs: 0,
      instance_init: Some(Foo::init),
      value_table: ptr::null(),
    };
  })
}