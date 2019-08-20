//! Handwritten generic type bounds
//!
//! When deriving `Serialize` and `Deserialize` implmentations for structs and
//! generic type parameters, most of the time Serde is able to infer the
//! correct trait bounds. It uses several heuristics to guess the right bound,
//! to guess the right bound, but most importantly, it puts a bound of
//! `T: Serialize` on every type parameter `T` that is part of a serailized
//! field and a bound of `T: Deserialize` on every type parameter `T` that is
//! part of a deserialized field. This is not always right, so Serde provides
//! an escape hatch to replace the automatically generated bound by one written
//! by the programmer.

use serde::{de, Deserialize, Deserializer};

use std::fmt::{Debug, Display};
use std::str::FromStr;

/// A struct that owns a element of type `S` and points to an element of type
/// `T`.
#[derive(Deserialize, Debug)]
struct Outer<'a, S, T: 'a + ?Sized> {
    /// When deriving the `Deserialize`, Serde would want to generate a
    /// bound `S: Deserialize` on the type of this field, but we are going to
    /// use type's `FromStr` impl instead of its `Deserialize` impl by going
    /// `deserialize_from_str`, so we override the automatically generated
    /// bound by the one required for `deserialize_from_str`.
    #[serde(deserialize_with = "deserialize_from_str")]
    #[serde(bound(deserialize = "S: FromStr, S::Err: Display"))]
    s: S,

    /// Here, Serde uses a stricter condition than necessary: `T: Deserialize`.
    /// This prevents `T=str` since `str` does not implement `Deserialize`. We
    /// override the automically generated bound with a looser one. Now, only
    /// the pointer to the `str` need to implement `Deserialize`, rather than
    /// the `str` itself.
    #[serde(bound(deserialize = "Ptr<'a, T>: Deserialize<'de>"))]
    ptr: Ptr<'a, T>,
}

/// Deserialize a type `S` by deserializing a string, then using the `FromStr`
/// impl of `S` to create the result. The generic type `S` is not required to
/// implement `Deserialize`.
fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    // Deserializes the string from `deserializer`.
    let s: String = Deserialize::deserialize(deserializer)?;
    // Attempts to convert the deserialized string to type `S`.
    // Calls the deserializer's `custom` method if the parsing fails.
    S::from_str(&s).map_err(de::Error::custom)
}

/// A pointer to `T` which may or may not own the data. When deserializing, we
/// always want to produce owned data.
#[derive(Debug)]
enum Ptr<'a, T: 'a + ?Sized> {
    Ref(&'a T),
    Owned(Box<T>),
}

impl<'de, 'a, T: 'a + ?Sized> Deserialize<'de> for Ptr<'a, T>
where
    Box<T>: Deserialize<'de> + Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|e| {
            println!("Deserialized first to Boxed({:?})", e);
            Ptr::Owned(e)
        })
    }
}

fn main() {
    let json = r#"
        {
            "s": "1234567890",
            "ptr": "owned"
        }
    "#;

    let result: Outer<u64, str> = serde_json::from_str(json).unwrap();
    println!("result = {:?}", result);
}
