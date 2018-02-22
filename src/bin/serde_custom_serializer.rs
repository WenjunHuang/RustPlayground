#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde::ser::SerializeStruct;
use serde::de::{self,Visitor};
use std::fmt;


// An ordinary struct.Use three-step process:
// 1. serialize_struct
// 2. serialize_field
// 3. end
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl serde::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
        let mut state = serializer.serialize_struct("Color",4)?;
        state.serialize_field("g",&self.g)?;
        state.serialize_field("r",&self.r)?;
        state.serialize_field("b",&self.b)?;
        state.serialize_field("a",&0)?;
        state.end()
    }
}

struct I32Visitor;
impl<'de> Visitor<'de> for I32Visitor {
    type Value = i32;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an interger between ")
    }
}

fn main(){
    let color = Color{r:120,g:0,b:0};
    let serialized = serde_json::to_string_pretty(&color).unwrap();
    println!("{}",serialized);
}