struct Foo {
  name:String,
}

impl Foo {
  fn get_name(&self) -> &String {
    &self.name
  }
}

fn foo_new(name:&str)->Foo {
  Foo{
    name:name.to_owned(),
  }
}

fn main(){
  // error: temporary value dropped
//  let name = foo_new("wenjun").get_name();

  let foo = foo_new("wenjun");
  let name = foo.get_name();
  println!("{}",name);

}