use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }

  fn set_value(&mut self,n:T) {
    self.0 = n;
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

fn main() {
  let x = 5;
  let mut z = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *z);

  let m = MyBox::new(String::from("Rust"));
  hello(&m);

  let mb = &mut z;
  let ir = return_what_you_give(&mb);
  assert_eq!(5,*ir);
  mb.set_value(10);
  assert_eq!(10,*ir);
}

fn return_what_you_give(input:&i32) -> &i32 {
  input
}
fn hello(name: &str) {
  println!("Hello, {}!",name);
}