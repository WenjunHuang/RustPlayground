extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize,Deserialize)]
enum APIResult {
    Success {
        name: String,
        age: u32,
    },
    Failure { code: String, msg: String },
}

fn main(){
    let success = APIResult::Success{
        name:"wenjun".to_owned(),
        age: 38,
    };

    let failed = APIResult::Failure{
        code:"300".to_owned(),
        msg:"there is a failure".to_owned()
    };

    let serialized_success = serde_json::to_string(&success).unwrap();
    let serialized_failed = serde_json::to_string(&failed).unwrap();

    println!("{}",serialized_success);
    println!("{}",serialized_failed);

}


