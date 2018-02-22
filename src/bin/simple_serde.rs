#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
use serde_json::{Map,Value};

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all="camelCase")]
struct Point{
    x_point:i32,
    y_point:i32,
}

fn main(){
    let point = Point{x_point:1,y_point:2};


    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}",serialized);

    // Convert the JSON string back to a Point.
//    let deserialized:Point = serde_json::from_str(&serialized).unwrap();
    let deserialized:Map<String,Value> = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point{x:1,y:2}
    println!("deserialized = {:?}", deserialized);

}