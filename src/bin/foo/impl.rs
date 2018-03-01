extern crate gobject_sys;
extern crate glib_sys;
extern crate glib;

use std;
use std::os::raw::*;
use std::ffi::CString;
use std::sync::{Once, ONCE_INIT};
use std::cell::RefCell;
use gobject_sys as gobject_ffi;
use glib_sys as glib_ffi;
use glib;
use glib::ToValue;
use glib::translate::{from_glib_none, from_glib_borrow, ToGlibPtr};

use Foo as FooWrapper;

#[repr(C)]
pub struct Foo {
    pub parent: gobject_ffi::GObject,
}

#[repr(C)]
pub struct FooClass {
    pub parent_class: gobject_ffi::GObjectClass,
}

struct FooPrivate {
    name: RefCell<Option<String>>,
    counter: RefCell<i32>,
}

struct FooClassPrivate {
    parent_class: *const gobject_ffi::GObjectClass,
    properties: *const Vec<*const gobject_ffi::GParamSpec>,
    signals: *const Vec<u32>,
}

static mut PRIV: FooClassPrivate = FooClassPrivate {
    parent_class: 0 as *const _,
    properties: 0 as *const _,
    signals: 0 as *const _,
};

#[repr(u32)]
struct Properties {
    Name = 1,
}

#[repr(u32)]
enum Signals {
    Incremented = 0,
}

#[no_mangle]
pub unsafe extern "C" fn ex_foo_get_type() -> glib_ffi::GType {
    callback_guard!();

    static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
    static ONCE: Once = ONCE_INIT;

    ONCE.call_once(|| {
        let type_info = gobject_ffi::GTypeInfo {
            class_size: mem::size_of::<FooClass>() as u16,
            base_init: None,
            base_finalize: None,
            class_init: Some(FooClass::init),
            class_finalize: None,
            class_data: ptr::null(),
            instance_size: mem::size_of::<Foo>() as u16,
            n_preallocs: 0,
            instance_init: Some(Foo::init),
            value_table: ptr::null(),
        };

        let type_name = CString::new("ExFoo").unwrap();
        TYPE = gobject_ffi::g_type_register_static(
            gobject_ffi::g_object_get_type(),
            type_name.as_ptr(),
            &type_info,
            gobject_ffi::GTypeFlags::empty());
    });
    TYPE
}

impl Foo {
    fn get_class(&self) -> &FooClass {
        unsafe {
            let klass = (*(self as *const _ as *const gobject_ffi::GTypeInstance)).g_class;
            &*(klass as *const FooClass)
        }
    }

    fn get_priv(&self) -> &Fooprivate {
        unsafe {
            let private = gobject_ffi::g_type_instance_get_private(
                self as *const _ as *mut gobject_ffi::GTypeInstance,
                ex_foo_get_type(),
            ) as *const Option<FooPrivate>;
            (&*private).as_ref().unwrap()
        }
    }
    unsafe extern "C" fn init(obj: *mut gobject_ffi::GTypeInstance, _klass: glib_ffi::gpointer) {
        callback_guard!();

        let private = gobject_ffi::g_type_instance_get_private(obj as *mut gobject_ffi::GTypeInstance, ex_foo_get_type()) as *mut Option<FooPrivate>;

        ptr::write(private, Some(FooPrivate {
            name: RefCell::new(None),
            counter: RefCell::new(0),
        }));
    }

    unsafe extern "C" fn finalize(obj: *mut gobject_ffi::GObject) {
        callback_guard!();

        let private = gobject_ffi::g_type_instance_get_private(
            obj as *mut gobject_ffi::GTypeInstance,
            ex_foo_get_type(),
        ) as *mut Option<FooPrivate>;
        let _ = (*private).take();

        (*PRIV.parent_class).finalize.map(|f| f(obj));
    }

    unsafe extern "C" fn set_property(obj: *mut gobject_ffi::GObject, id: u32, value: *mut gobject_ffi::GValue, _pspec: *mut gobject_ffi::GParamSpec) {
        callback_guard!();

        let this = &*(obj as *mut Foo);
        let private = (*this).get_priv();

        match mem::transmute::<u32, Properties>(id) {
            Properties::Name => {
                let name = gobject_ffi::g_value_get_string(value);
                Foo::set_name(&from_glib_borrow(obj as *mut Foo),
                              private,
                              from_glib_none(name));
            }
            _ => unreachable!(),
        }
    }

    unsafe extern "C" fn get_property(obj: *mut gobject_ffi::GObject,
                                      id: u32,
                                      value: *mut gobject_ffi::GValue,
                                      _pspec: *mut gobject_ffi::GParamSpec) {
        callback_guard!();
        let private = (*(obj as *mut Foo)).get_priv();

        match mem::transmute::<u32, Properties>(id) {
            Properties::Name => {
                let name = Foo::get_name(&from_glib_borrow(obj as *mut Foo), private);
                gobject_ffi::g_value_set_string(value, name.to_glib_none().0);
            }
            _ => unreachable!(),
        }
    }

    unsafe extern "C" fn increment_trampoline(this: *mut Foo, inc: i32) -> i32 {
        callback_guard!();

        let private = (*this).get_priv();

        Foo::increment(&from_glib_borrow(this), private, inc)
    }

    unsafe extern "C" fn incremented_trampoline(this: *mut Foo, val: i32, inc: i32) {
        callback_guard!();

        let private = (*this).get_priv();

        Foo::incremented(&from_glib_borrow(this), private, val, inc);
    }

    fn increment(this: &FooWrapper, private: &FooPrivate, inc: i32) -> i32 {
        let mut val = private.counter.borrow_mut();

        *val += inc;

        unsafe {
            let params = [this.to_value(), (*val).to_value(), inc.to_value()];
            gobject_ffi::g_signal_emitv(
                params.as_ptr() as *mut _,
                (*PRIV.signals)[Signals::Incremented as usize],
                0,
                ptr::null_mut(),
            );
        }

        *val
    }

    fn incremented(_this: &FooWrapper, _private: &FooPrivate, _val: i32, _inc: i32) {}

    fn get_counter(_this: &FooWrapper, private: &FooPrivate) -> i32 {
        *private.counter.borrow()
    }

    fn get_name(_this: &FooWrapper, private: &FooPrivate) -> Option<String> {
        private.name.borrow().clone()
    }

    fn set_name(_this: &FooWrapper, private: &FooPrivate, name: Option<String>) {
        *private.name.borrow_mut() = name;
    }

    unsafe extern "C" fn nameable_get_name_trampoline(this: *mut ::nameable::imp::Nameable) -> *mut c_char {
        callback_guard!();

        let this = this as *mut Foo;
        let private = (*this).get_priv();

        Foo::get_name(&from_glib_borrow(this), private).to_glib_full()
    }
}

impl FooClass {
    unsafe extern "C" fn init(klass: glib_ffi::gpointer, _klass_data: glib_ffi::gpointer) {
        callback_guard!();
        gobject_ffi::g_type_class_add_private(klass, mem::size_of::<Option<FooPrivate>>() as usize);

        {
            let gobject_klass = &mut *(klass as *mut gobject_ffi::GObjectClass);
            gobject_klass.finalize = Some(Foo::finalize);
            gobject_klass.set_property = Some(Foo::set_property);
            gobject_klass.get_property = Some(Foo::get_property);

            let mut properties = Vec::new();

            let name_cstr = CString::new("name").unwrap();
            let nick_cstr = CString::new("Name").unwrap();
            let blurb_cstr = CString::new("Name of the object").unwrap();

            properties.push(ptr::null());
            properties.push(gobject_ffi::g_param_spec_string(
                name_cstr.as_ptr(),
                nick_cstr.as_ptr(),
                blurb_cstr.as_ptr(),
                ptr::null_mut(),
                gobject_ffi::G_PARAM_READWRITE | gobject_ffi::G_PARAM_CONSTRUCT_ONLY,
            ));
            gobject_ffi::g_object_class_install_properties(
                gobject_klass,
                properties.len() as u32,
                properties.as_mut_ptr() as *mut *mut _,
            );

            PRIV.properties = Box::into_raw(Box::new(properties));
        }
        {
            let foo_klass = &mut *(klass as *mut FooClass);
            foo_klass.increment = Some(Foo::increment_trampoline);
            foo_klass.incremented = Some(Foo::incremented_trampoline);
        }

        let mut signals = Vec::new();

        let name_cstr = CString::new("incremented").unwrap();
        let param_types = [gobject_ffi::G_TYPE_INT, gobject_ffi::G_TYPE_INT];

        let class_offset = {
            let dummy: FooClass = mem::uninitialized();
            ((&dummy.incremented as *const _ as usize) - (&dummy as *const _ as usize)) as u32
        };

        signals.push(gobject_ffi::g_signal_newv(
            name_cstr.as_ptr(),
            ex_foo_get_type(),
            gobject_ffi::G_SIGNAL_RUN_LAST,
            gobject_ffi::g_signal_type
        ))
    }
}

#[no_mangle]
pub unsafe extern "C" fn ex_foo_new() -> *mut Foo {
    callback_guard!();

    let this = gobject_ffi::g_object_newv(
        ex_foo_get_type(),
        0,
        ptr::null_mut(),
    );

    this as *mut Foo
}