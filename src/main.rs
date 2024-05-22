use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DogOwner {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Dog {
    name: String,
    year_born: i32,
    owner: DogOwner,
}

fn main() {
    let owner_1 = DogOwner {
        first_name: "Trevor".to_string(),
        last_name: "Sullivan".to_string(),
    };
    let dog_1: Dog = Dog {
        name: "Cheyenne".to_string(),
        year_born: 2021,
        owner: owner_1,
    };
    let dog_ser = to_string(&dog_1);

    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    } else {
        println!("{:#?}", dog_ser.err())
    }
}
