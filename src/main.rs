use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
struct DogOwner {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Dog {
    name: String,
    year_born: i32,
    owner: DogOwner,
}

fn main() {
    serialize();
    deserialize();
}

fn serialize() {
    let owner_1 = DogOwner {
        first_name: "Trevor".to_string(),
        last_name: "Sullivan".to_string(),
    };
    let dog_1: Dog = Dog {
        name: "Cheyenne".to_string(),
        year_born: 2021,
        owner: owner_1,
    };
    let dog_ser: Result<String, serde_json::Error> = to_string_pretty(&dog_1);

    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    } else {
        println!("{:#?}", dog_ser.err())
    }
}

fn deserialize() {
    let json_string: &str = r#"
    {
        "name": "Cheyenne",
        "year_born": 2021,
        "owner": {
          "first_name": "Trevor",
          "last_name": "Sullivan"
        }
      }
    "#;

    let dog_deser: Result<Dog, serde_json::Error> = from_str::<Dog>(json_string);

    if dog_deser.is_ok() {
        println!("{:#?}", dog_deser.ok().unwrap());
    } else {
        println!("{:#?}", dog_deser.err())
    }
}
