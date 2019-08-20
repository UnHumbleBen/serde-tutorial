/// Provides utilties for formatting and printing `String`s.
use std::fmt;
/// A zero-sized type used to mark things that "act like" they own a `T`.
use std::marker::PhantomData;

/// Generic data structures deserialization framework.
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};

/// A tuple struct containing a item of type `K` and another item of type `V`.
struct MyMap<K, V>(PhantomData<K>, PhantomData<V>);

impl<K, V> MyMap<K, V> {
    /// Creates a new `MyMap` with the provided capacity.
    fn with_capacity(_: usize) -> Self {
        unimplemented!()
    }

    /// Inserts a key value pair to this map.
    fn insert(&mut self, _: K, _: V) {
        unimplemented!()
    }
}

/// A `Visitor` is a type that holds methods that a `Deserializer` can drive
/// depending on what is contained in the input data.
///
/// In the case of a map, we need generic type parameters `K` and `V` to be
/// able to set the output type correclty, but don't require any state.
/// This is an example of a "zero sized type" in Rust. The `PhantomData` keeps
/// the compiler from complaining about unused generic type parameters.
struct MyMapVisitor<K, V> {
    marker: PhantomData<fn() -> MyMap<K, V>>,
}

impl<K, V> MyMapVisitor<K, V> {
    /// Creates a new `MyMapVisitor`.
    fn new() -> Self {
        MyMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for MyMapVisitor<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    /// The `Visitor` is going to produce a `MyMap<K, V>`.
    type Value = MyMap<K, V>;

    /// Formats a message stating what data this `Visitor` expects to recieve.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    /// Deserialize `MyMap` from an abstract "map" provided by the
    /// `Deserializer`. The `MapAccess` input is a callback provided by the
    /// `Deserializer` to let us see every entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        // Initializes a `MyMap` with a capacity equal to the number of entries
        // in the abstract "map" provided by the `Deserializer`, unless the
        // number of entries is not known, in which case, the capacity defaults
        // to 0.
        let mut map = MyMap::with_capacity(access.size_hint().unwrap_or(0));

        // While there are entries remaining in the input, add them into our
        // map.
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'de, K, V> Deserialize<'de> for MyMap<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our `Visitor` and ask `Deserializer` to drive it over
        // the input data, resulting in an instance of `MyMap`.
        deserializer.deserialize_map(MyMapVisitor::new())
    }
}

fn main() {
    println!("Hello, Implement Deserialize for a custom map type!");
}
