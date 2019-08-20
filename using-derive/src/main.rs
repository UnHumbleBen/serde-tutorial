//! # Using derive
//!
//! Serde provides a derive macro to generate implementations of the
//! `Serialize` and `Deserialize` traits for user-defined data structures.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    #[serde(rename = "renamed_x")]
    x: i32,
    y: i32,
    #[serde(default)]
    z: i32,
}

fn main() {
    let point = Point { x: 1, y: 2, z: 0 };

    // Serializes `point` as a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();
    println!("Serialized   point = {}", serialized);

    let serialized = "{\"renamed_x\":1,\"y\":2}";

    // Deserialize a `Point` from the JSON string.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized point = {:?}", deserialized);
}
