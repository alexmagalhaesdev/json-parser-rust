use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
struct Dog {
    name: String,
    year_born: i32,
}

fn main() {
    let dog_1: Dog = Dog {
        name: "Cheyenne".to_string(),
        year_born: 2021,
    };
    let dog_ser = to_string(&dog_1);

    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    } else {
        println!("{:#?}", dog_ser.err())
    }
}
