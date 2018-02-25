fn main() {
  let mut x = 10;
  let ptr_x = &mut x as *mut i32;

  let y = Box::new(20);
  let ptr_y = &*y as *const i32;

  unsafe {
    *ptr_x += *ptr_y;
  }
  assert_eq!(x,30);

  assert!(!option_to_raw(Some(&("pea","pod"))).is_null());
  assert_eq!(option_to_raw::<i32>(None),std::ptr::null());

  let trucks = vec!["garbage truck","dump truck","moonstruck"];
  let first = &trucks[0];
  let last = &trucks[2];
  assert_eq!(distance(last,first),2);
  assert_eq!(distance(first,last),-2);

}

fn option_to_raw<T>(opt:Option<&T>) -> *const T {
  match opt{
    None => std::ptr::null(),
    Some(r) => r as *const T
  }
}

fn distance<T>(left: *const T,right: *const T) -> isize {
  (left as isize - right as isize) / std::mem::size_of::<T>() as isize
}