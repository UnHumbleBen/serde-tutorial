//! # The Vistor trait
//!
//! A `Visitor` is instantiated by a `Deserialize` impl and passed to a
//! `Deserializer`. The `Deserializer` then calls a method on the `Visitor` in
//! order to construct the desired type.

use std::fmt;

use serde::de::{self, Visitor};

/// A `Visitor` that is able to deserialize a primitive `i32` from a variety
/// of types.
#[allow(dead_code)]
struct I32Vistor;

impl<'de> Visitor<'de> for I32Vistor {
    type Value = i32;

    /// Formats a message stating that this `I32Visitor` expects to recieve
    /// an `i32`.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // Writes the string to the formatter.
        formatter.write_str("an integer between -2^32 and 2^31")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(i32::from(value))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(i32::from(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use std::i32;
        if value >= i64::from(i32::MIN) && value <= i64::from(i32::MAX) {
            Ok(value as i32)
        } else {
            Err(E::custom(format!("i32 out of range: {}", value)))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
