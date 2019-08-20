//! Implementing Serialize
//!
//! Source: [https://serde.rs/impl-serialize.html](https://serde.rs/impl-serialize.html)

use serde::ser::{Serialize, SerializeStruct, Serializer};

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.end()
    }
}

fn main() {
    let color = Color {
        r: 233,
        g: 103,
        b: 134,
    };

    let serialized_color = serde_json::to_string(&color).unwrap();
    println!("Serialized color = {}", serialized_color);
}
