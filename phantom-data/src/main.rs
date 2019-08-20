//! # PhantomData
//!
//! When working with unsafe code, we can often end up in a situation where
//! types or lifetimes are logically associated with a struct, but not actually
//! part of a field. This most commonly occurs with lifetimes. For instance,
//! the `Iter` for `&'a [t]` is (approximately) defined as follows
//!
//! ```
//! struct Iter<'a, T: 'a> {
//!     ptr: *const T,
//!     end: *const T<
//! }
//! ```
//!
//! However, because `'a` is unused within the struct's body, its *unbounded*.
//! Because of the troubles this has historically caused, unbounded lifetimes
//! and types are *forbidden* in struct definitions. Therefore, we must somehow
//! refer to these types in the body. Correctly doing this is neccesary to
//! have correct variance and drop checking.
//!
//! We do this using `PhantomData`, which is a special marker type.
//! `PhantomData` consumes no space, but simulates a field of the given type
//! for the purpose of static analysis. This was deemed to be less error-prone
//! than explictly telling the type-system the kind of variance that you want,
//! while also providing other useful things such as the information needed by
//! drop checking.
//!
//! `Iter` logically contains a bunch of `&'a T`s, so this is exactly what we
//! tell the `PhantomData` to simulate:
//!
//! Source: [https://doc.rust-lang.org/nightly/nomicon/phantom-data.html#table-of-phantomdata-patterns](https://doc.rust-lang.org/nightly/nomicon/phantom-data.html#table-of-phantomdata-patterns)

/// Primitive traits and types representing basic properties of types.
use std::marker;

/// `Iter` logically contains a bunch of `&'a T`s, so this is exactly what we
/// tell the `PhantomData` to simulate.
struct Iter<'a, T: 'a> {
    /// An immutable raw pointer. In this context, *immutable* means that the
    /// pointer can't be directly assigned to after being dereferenced.
    ptr: *const T,
    end: *const T,
    /// This bounds the lifetime.
    _marker: marker::PhantomData<&'a T>,
}

/// Approximate definition of `Vec`.
///
/// Unlike the previous example, it appears that everything is exactly what we
/// want. Every generic argument to `Vec` shows up in at least one field.
///
/// Unfortunately, the drop checker generously determines that `Vec<T>` does
/// not own any values of type `T`. This will in turn make it conclude that it
/// doesn't need to worry about `Vec` dropping any `T`s in its destructor for
/// determining drop check soundness. This allows people to create unsoundndess
/// using `Vec`'s destructor.
///
/// In order to tell the drop checker that we *do* own values of type `T`, and
/// therefore may drop some `T`s when *we* drop, we must add an extra
/// `PhantomData` saying exactly that.
struct Vec<T> {
    /// Use `*const T` for variance. Variance is a property that *type
    /// constructors* have with respect to their arguments. A type constructor
    /// is any generic type with unbound arguments. For instance,
    /// `std::vec::Vec` is a type constructor that takes a type `T` and returns
    /// `std::vec::Vec<T>` `&` and `&mut` are type constructors that take two
    /// inputs: a lifetime, and a type to point to.
    ///
    /// Three kinds of variance:
    ///
    /// * *covariant*
    /// * *contravariant*
    /// * *invariant*
    ///
    /// For covariant, we need a `*const T`.
    data: *const T,
    len: usize,
    cap: usize,
}

fn main() {
    println!("Hello, phantom-data!\nTo learn about PhantomData, run:\ncargo doc --open\ninside this directory, or view the source code!");
}
