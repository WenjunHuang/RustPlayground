use std::mem::ManuallyDrop;
struct Foo {
    name: ManuallyDrop<String>,
    age: u32,
}

fn main() {
    let f = Foo { name: "rust".to_owned(), age: 5 };
    drop(f);

    println!("{}",f.name);
}