use std::fmt;
use std::marker::PhantomData;

/// IgnoredAny is an efficient way of discarding data from a deserializer.
///
/// DeserializeSeed is the stateful form of the Deserialize trait.
use serde::de::{self, Deserialize, DeserializeSeed, Deserializer, IgnoredAny, SeqAccess, Visitor};

// Imports the json macro.
use serde_json::json;

pub struct NthElement<T> {
    n: usize,
    marker: PhantomData<fn() -> T>,
}

impl<T> NthElement<T> {
    pub fn new(n: usize) -> Self {
        NthElement {
            n: n,
            marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for NthElement<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a sequence in which we care about element {}",
            self.n
        )
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        for i in 0..self.n {
            if seq.next_element::<IgnoredAny>()?.is_none() {
                return Err(de::Error::invalid_length(i, &self));
            }
        }

        let nth = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(self.n, &self))?;

        while let Some(IgnoredAny) = seq.next_element()? {}

        Ok(nth)
    }
}

impl<'de, T> DeserializeSeed<'de> for NthElement<T>
where
    T: Deserialize<'de>,
{
    type Value = T;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}

fn main() {
    // Constructs a `serde_json::Value` from the JSON literal.
    let array = json!(["a", "b", "c", "d", "e"]);

    let nth: String = NthElement::new(3).deserialize(&array).unwrap();

    println!("array = {:?}", array);
    println!("array[3] = {} and nth = {}", array[3], nth);
}
