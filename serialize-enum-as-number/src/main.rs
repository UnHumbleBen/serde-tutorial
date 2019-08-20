/// Derive `Serialize` and `Deserialize` that delegate to the underlying
/// repr of a C-like enum.
use serde_repr::*;

/// A C-like enum which discriminants are the smallest prime numbers.
///
/// It derives the `Serialize_repr` and `Deserialize_repr` traits.
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
/// Instead of the default representation which interprets each discriminant as
/// `isize`, the enum attaches the `#[repr(u8)]` attribute, which specifies
/// that it should set the size and alignment to be the same as `u8`.
#[repr(u8)]
enum SmallPrime {
    Two = 2,
    Three = 3,
    Five = 5,
    Seven = 7,
}
fn main() {
    /// For readibility, bring each variant in scope.
    use SmallPrime::*;
    let nums = vec![Two, Three, Five, Seven];

    println!("{}", serde_json::to_string(&nums).unwrap());

    assert_eq!(Two, serde_json::from_str("2").unwrap());
}
