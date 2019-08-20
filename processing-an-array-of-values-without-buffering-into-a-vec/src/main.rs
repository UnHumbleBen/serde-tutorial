//! # Processing an array of values without buffering into a Vec
//!
//! Suppose we have a JSON string and we want to figure out the maximum value without holding the
//! whole array in memory all at once. This approach can be adapted to handle a variety of other
//! situations in which data needs to be processed while being deserialized instead of after.

use serde::{Deserialize, Deserializer};

/// `SeqAccess` provides a `Visitor` access to each element of a sequence in the input.
use serde::de::{self, SeqAccess, Visitor};

/// Funtionality for ordering and comparing.
use std::cmp;

use std::fmt;

use std::marker::PhantomData;

/// A struct which processes data during serialization.
#[derive(Deserialize)]
struct Outer {
    /// The id of the processed array.
    id: String,

    /// Deserialize this field by computing the maximum value of a sequence (JSON array) of values.
    #[serde(deserialize_with = "deserialize_max")]
    /// Despite the struct field being named `max_value`, it is going to come from a JSON field
    /// called `values`.
    #[serde(rename(deserialize = "values"))]
    max_value: u64,
}

/// Deserialize the maximum of a sequence of values. The entire sequence is not buffered into
/// memory as it would be if we deserialize to `Vec<T>` and then compute the maximum later.
///
/// This function is generic over `T` which can be any thpe that implements `Ord`. Above, it is
/// used `T = u64`.
///
/// # Params
///
/// ## Generic Parameters
///
/// ### Generic Lifetime Parameters
///
/// * `'de` is the lifetime bounded by the lifetime of the borrowed data.
///
/// ### Generic Type Parameters
///
/// * `T` is any type that implements `Deserialize<'de>` and `Ord`, meaning that it can
/// be serialized from Serde-supported data formats and forms a [total order](https://en.wikipedia.org/wiki/Total_order)
/// respectively.
///
/// * `D` is any type that implements `Deserializer<'de>`, meaning that it can deserialize any
/// data structure supported by Serde.
///
/// ## Parameters
///
/// * `deserializer` is the data format that deserialize a data structure of type `T`.
fn deserialize_max<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Ord,
    D: Deserializer<'de>,
{
    /// A struct that represents a visitor that walks through the deserializer `deserializer`.
    struct MaxVisitor<T>(PhantomData<fn() -> T>);

    /// # Parameters
    ///
    /// ## Generic Parameters
    ///
    /// ### Generic Lifetime Parameters
    ///
    /// * `'de` is the lifetime bounded by the lifetime of the data borrowed by the deserializer.
    ///
    /// ### Generic Type Parameters
    ///
    /// * `T` is any type that implements `Deserialize<'de>` and `Ord`, meaning that it can
    /// be serialized from Serde-supported data formats and forms a [total order](https://en.wikipedia.org/wiki/Total_order)
    /// respectively.
    impl<'de, T> Visitor<'de> for MaxVisitor<T>
    where
        T: Deserialize<'de> + Ord,
    {
        /// Return type of this visitor. This vistor computes the max of a sequence of values of
        /// type `T`, so the type of the maximum is `T`.
        type Value = T;

        /// Formats a message stating that this Visitor expects a nonempty sequence of numbers.
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nonempty sequence of numbers")
        }

        /// The input contains a sequence of elements. Overrides the default implementation which
        /// fails with a type error.
        ///
        /// # Params
        ///
        /// ## Generic Parameters
        ///
        /// ### Generic Type Parameters
        ///
        /// * `S` is any type that implements `SeqAccess<'de>` meaning it provides methods to
        /// access each element of a sequence in the input.
        ///
        /// ## Parameters
        ///
        /// * `self`: This method consumes the `Visitor`.
        ///
        /// * `seq` provides access to the elements in the sequence in the input.
        fn visit_seq<S>(self, mut seq: S) -> Result<T, S::Error>
        where
            S: SeqAccess<'de>,
        {
            // Start with max equal to the first value in the seq.
            let mut max = seq.next_element()?.ok_or_else(||
                // Cannot take the maximum of an empty seq.
                de::Error::custom("No values in seq when looking for maximum"))?;

            while let Some(value) = seq.next_element()? {
                max = cmp::max(max, value);
            }

            Ok(max)
        }
    }

    let visitor = MaxVisitor(PhantomData);
    // Hints that the `T` is expecting a sequence of values.
    deserializer.deserialize_seq(visitor)
}

fn main() {
    let j = r#"
        {
            "id": "demo-deserialize-max",
            "values": [
                256,
                100,
                384,
                314,
                271
            ]
        }
    "#;

    let out: Outer = serde_json::from_str(j).unwrap();

    // Should print "Max value: 384".
    println!("Max value: {}", out.max_value);
}
