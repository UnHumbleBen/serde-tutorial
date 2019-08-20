//! # Attributes
//!
//! Used to customize the `Serialize` and `Deserialize` implementations
//! produced by Serde's derive.

use serde::{Deserialize, Serialize};

/// A struct that derives the `Serialize` and `Deserialize` traits.
#[derive(Serialize, Deserialize)]
///
/// Attaches the `#[serde(deny_unknown_fields)]` attribute.
///
/// Container attribute which specifies that deserialization should always
/// error when encoutering unknown fields, rather than simply ignored for
/// self-describing formats like JSON.
///
/// A self-describing format means that the data can be accessed by name and
/// by class, rather than by position. Each data can have its own set of
/// attributes or fields.
#[serde(deny_unknown_fields)]
struct S {
    #[serde(default)]
    /// A field which attaches the field attribute `#[serde(default)]`, which
    /// deserializes the value using `Default::default()` if the value is not
    /// present.
    f: i32,
}

/// A enum which derives the `Serialize` and `Deserialize` traits.
#[derive(Serialize, Deserialize)]
/// It attaches a container attribute `#[serde(rename = "e")]` which serialize
/// and deserialize this enum with the name "e" instead of its Rust name "E".
#[serde(rename = "e")]
enum E {
    #[serde(rename = "a")]
    /// A variant which attaches a variant attribute `#[serde(rename = "a")]`,
    /// which serializes and deserializes this variant with "a" instead of "A".
    A(String),
}

fn main() {
    println!("Hello, Attributes!");
}
