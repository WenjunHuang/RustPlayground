use std::marker::PhantomData;
use std::mem::align_of;

pub struct RefWithFlag<'a,T:'a> {
    ptr_and_bit: usize,
    behaves_like: PhantomData<&'a T>
}

impl<'a,T:'a> RefWithFlag<'a,T> {
    pub fn new(ptr:&'a T,flag:bool) -> RefWithFlag<T> {
        assert!(align_of::<T>() % 2 ==0);
        RefWithFlag{
            ptr_and_bit: ptr as *const T as usize | flag as usize,
            behaves_like: PhantomData
        }
    }

    /// attach &T lifetime to the passin lifetime 'a when construct this struct
    /// not the self's lifetime
    pub fn get_ref(&self) -> &'a T {
        unsafe {
            let ptr = (self.ptr_and_bit & !1) as *const T;
            &*ptr
        }
    }

    pub fn get_flag(&self) -> bool {
        self.ptr_and_bit & 1 != 0
    }
}

fn main(){
    let vec = vec![10,20,30];
    let some_thing;
    {
        let flagged = RefWithFlag::new(&vec, true);
        assert_eq!(flagged.get_ref()[1], 20);
        assert_eq!(flagged.get_flag(), true);

        // lifetime magic
        some_thing = flagged.get_ref();
    }

    assert_eq!(some_thing[1],20);
}