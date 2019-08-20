//! # Enum representations
//!
//! Source: [https://serde.rs/enum-representations.html](https://serde.rs/enum-representations.html)

use serde::{Deserialize, Serialize};

/// Default representation for an enum in Serde is the externally tagged enum
/// representation. The variant is explicit. Not ideal for JSON.
#[derive(Serialize, Deserialize, Debug)]
enum MessageDefault {
    Request {
        id: String,
        method: String,
        params: Params,
    },
    Response {
        id: String,
        result: Value,
    },
}

/// The tag identifying the variant is not inside of the content, next to other
/// fields of the variant. Works for enums that contains struct variants,
/// newtype variants containing structs or map, and unit vairants, but does not
/// work for tuple variants.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum MessageInternallyTagged {
    Request {
        id: String,
        method: String,
        params: Params,
    },
    Response {
        id: String,
        result: Value,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
enum MessageAdjacentlyTagged {
    Request {
        id: String,
        method: String,
        params: Params,
    },
    Response {
        id: String,
        result: Value,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum MessageUntagged {
    Request {
        id: String,
        method: String,
        tuple: (u32, u32),
        params: Params,
    },
    Response {
        id: String,
        result: Value,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Params {
    field_1: u32,
    field_2: bool,
    field_3: Vec<i32>,
}

impl Params {
    fn new() -> Params {
        Params {
            field_1: 30,
            field_2: false,
            field_3: vec![-4, -4, 0, 3],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Value {
    field_1: bool,
    field_2: String,
}

fn print_serialized<T: Serialize>(serialize: &T) {
    let serialized = serde_json::to_string(serialize).unwrap();
    println!("Serialized request = {}\n", serialized);
}

fn main() {
    let request = MessageDefault::Request {
        id: String::from("a37cacc3-71d5-40f0-a329-a051a3949ced"),
        method: String::from("GET"),
        params: Params::new(),
    };
    println!("Default:");
    print_serialized(&request);

    let request = MessageInternallyTagged::Request {
        id: String::from("a37cacc3-71d5-40f0-a329-a051a3949ced"),
        method: String::from("GET"),
        params: Params::new(),
    };
    println!("Internally Tagged:");
    print_serialized(&request);

    let request = MessageAdjacentlyTagged::Request {
        id: String::from("a37cacc3-71d5-40f0-a329-a051a3949ced"),
        method: String::from("GET"),
        params: Params::new(),
    };
    println!("Adjacently Tagged:");
    print_serialized(&request);

    let request = MessageUntagged::Request {
        id: String::from("a37cacc3-71d5-40f0-a329-a051a3949ced"),
        method: String::from("GET"),
        tuple: (4, 3),
        params: Params::new(),
    };
    println!("Untagged:");
    print_serialized(&request);
}
