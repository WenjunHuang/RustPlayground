#[macro_use]
extern crate hello_world_derive;

macro_rules! failed {
($x:ident) => (format!("failed to parse '{}'",$x).as_str())
}

macro_rules! check_example {
($x:ident,$y:path) => {{
    let _parse_result: $y = syn::parse_str($x).expect(failed!($x));
}}
}

macro_rules! check_examples {
($x:ident,$y:path) => {{
    for ex in $x {
        check_example!(ex,$y);
    }
}}
}
#[derive(HelloWorld)]
struct Pancakes;

fn main(){
    Pancakes::hello_world();
}