// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

#![feature(generic_const_exprs)]
#![allow(incomplete_features, reason = "we need generic const exprs for BitVectorRef")]

//! Provides encoding (serialization) and decoding (deserialization) in the SimpleSerialize (SSZ)
//! format designed for use in Ethereum 2.0.
//!
//! Adheres to the Ethereum 2.0 [SSZ
//! specification](https://github.com/ethereum/eth2.0-specs/blob/v0.12.1/ssz/simple-serialize.md)
//! at v0.12.1.
//!
//! ## Decoding SSZ Data
//!
//! **For reading/parsing SSZ data, prefer zero-copy views:**
//!
//! ```rust
//! use ssz::view::DecodeView;
//!
//! # #[derive(Debug, Copy, Clone)]
//! # struct MyContainerRef<'a> { bytes: &'a [u8] }
//! # impl<'a> DecodeView<'a> for MyContainerRef<'a> {
//! #     fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, ssz::DecodeError> {
//! #         Ok(Self { bytes })
//! #     }
//! # }
//! # impl<'a> MyContainerRef<'a> {
//! #     fn field_a(&self) -> Result<u64, ssz::DecodeError> { Ok(42) }
//! # }
//! # fn example() -> Result<(), ssz::DecodeError> {
//! # let ssz_bytes = vec![0u8; 8];
//! // Zero-copy view - validates structure but doesn't allocate
//! let view = MyContainerRef::from_ssz_bytes(&ssz_bytes)?;
//! let field = view.field_a()?;  // Lazy field access
//! # Ok(())
//! # }
//! ```
//!
//! **Use owned types only when you need to modify data:**
//!
//! ```rust
//! use ssz_derive::{Encode, Decode};
//! use ssz::{Decode, Encode};
//!
//! #[derive(PartialEq, Debug, Encode, Decode)]
//! struct Foo {
//!     a: u64,
//!     b: Vec<u16>,
//! }
//!
//! fn ssz_encode_decode_example() {
//!     let foo = Foo {
//!         a: 42,
//!         b: vec![1, 3, 3, 7]
//!     };
//!
//!     let ssz_bytes: Vec<u8> = foo.as_ssz_bytes();
//!     let decoded_foo = Foo::from_ssz_bytes(&ssz_bytes).unwrap();
//!     assert_eq!(foo, decoded_foo);
//! }
//! ```
//!
//! See `examples/` for manual implementations and [`view`] module for zero-copy types.

pub mod bitfield;
pub mod decode;
pub mod encode;
pub mod layout;
pub mod legacy;
/// Serde utilities for SSZ types.
#[cfg(feature = "serde")]
pub mod serde_utils;
mod union_selector;
pub mod view;

pub use bitfield::bitvector_dynamic::{BitVectorDynamic, Dynamic};
#[doc(hidden)]
pub use bitfield::{BitList, BitVector, Bitfield, Error as BitfieldError, Fixed, Variable};
pub use decode::{
    Decode, DecodeError, SszDecoder, SszDecoderBuilder,
    impls::decode_list_of_variable_length_items, read_offset, split_union_bytes,
    try_from_iter::TryFromIter,
};
pub use encode::{Encode, SszEncoder, encode_length};
pub use union_selector::UnionSelector;

/// The number of bytes used to represent an offset.
pub const BYTES_PER_LENGTH_OFFSET: usize = 4;
/// The maximum value that can be represented using `BYTES_PER_LENGTH_OFFSET`.
#[cfg(target_pointer_width = "32")]
pub const MAX_LENGTH_VALUE: usize = (u32::MAX >> (8 * (4 - BYTES_PER_LENGTH_OFFSET))) as usize;
/// The maximum value that can be represented using `BYTES_PER_LENGTH_OFFSET`.
#[cfg(target_pointer_width = "64")]
pub const MAX_LENGTH_VALUE: usize = (u64::MAX >> (8 * (8 - BYTES_PER_LENGTH_OFFSET))) as usize;

/// The number of bytes used to indicate the variant of a union.
pub const BYTES_PER_UNION_SELECTOR: usize = 1;
/// The highest possible union selector value (higher values are reserved for backwards compatible
/// extensions).
pub const MAX_UNION_SELECTOR: u8 = 127;

/// Convenience function to SSZ encode an object supporting ssz::Encode.
///
/// Equivalent to `val.as_ssz_bytes()`.
pub fn ssz_encode<T>(val: &T) -> Vec<u8>
where
    T: Encode,
{
    val.as_ssz_bytes()
}
