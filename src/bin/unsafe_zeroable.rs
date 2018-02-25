#![feature(nonzero)]

extern crate core;
use core::nonzero::Zeroable;

fn zeroed_vector<T>(len:usize) -> Vec<T>
where T:Zeroable {
    let mut vec = Vec::with_capacity(len);
    unsafe {
        std::ptr::write_bytes(vec.as_mut_ptr(),0,len);
        vec.set_len(len);
    }
    vec
}

/// sample not satisfied with contract of Zeroable
/// will crash
struct HoldsRef<'a>(&'a mut i32);
unsafe impl<'a> Zeroable for HoldsRef<'a>{
    fn is_zero(&self) -> bool {
        true
    }
}

fn main(){
    let v:Vec<usize> = zeroed_vector(100_000);
    assert!(v.iter().all(|&u| u == 0));

    // will crash
    let mut v:Vec<HoldsRef> = zeroed_vector(1);
    *v[0].0 = 1;
}