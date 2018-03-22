#![feature(nll)]
use std::collections::HashMap;

fn get_default(map:&mut HashMap<usize,String>,key:usize) -> &mut String {
    match map.get_mut(&key) {
        Some(value) => value,
        None => {
            map.insert(key,"".to_string());
            map.get_mut(&key).unwrap()
        }
    }
}

fn main(){
    let map = &mut HashMap::new();
    map.insert(22,format!("Hello,World"));
    map.insert(44,format!("Goodbye, world"));
    assert_eq!(&*get_default(map,22),"Hello,World");
    assert_eq!(&*get_default(map,66),"");

}
