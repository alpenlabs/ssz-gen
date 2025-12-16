// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Bitfield implementation

use core::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::de::{Deserialize, Deserializer};
#[cfg(feature = "serde")]
use serde::ser::{Serialize, Serializer};
use smallvec::{SmallVec, ToSmallVec, smallvec};
use thiserror::Error;

#[cfg(feature = "serde")]
use crate::serde_utils::hex::{PrefixedHexVisitor, encode as hex_encode};
use crate::{Decode, DecodeError, Encode};

pub(crate) mod bitvector_dynamic;

/// Returned when an item encounters an error.
#[derive(PartialEq, Debug, Clone, Error)]
pub enum Error {
    /// The bitfield is out of bounds
    #[error("the bitfield is out of bounds: index {i} is out of bounds for length {len}")]
    OutOfBounds {
        /// The index
        i: usize,
        /// The length
        len: usize,
    },

    /// A `BitList` does not have a set bit, therefore its length is unknowable.
    #[error("the bitfield does not have a set bit, therefore its length is unknowable")]
    MissingLengthInformation,

    /// A `BitList` has excess bits set to true.
    #[error("the bitfield has excess bits set to true")]
    ExcessBits,

    /// A `BitList` has an invalid number of bytes for a given bit length.
    #[error(
        "the bitfield has an invalid number of bytes for a given bit length: given {given} bytes, expected {expected} bytes"
    )]
    InvalidByteCount {
        /// The given length
        given: usize,
        /// The expected length
        expected: usize,
    },
}

/// Maximum number of bytes to store on the stack in a bitfield's `SmallVec`.
///
/// 128 bytes is enough to take us through to ~2M active validators, as the byte
/// length of attestation bitfields is roughly `N // 32 slots // 64 committes //
/// 8 bits`.
pub(crate) const SMALLVEC_LEN: usize = 128;

/// Type alias for the [`SmallVec`] used in bitfields to avoid `const` evaluation cycles.
pub(crate) type BitfieldSmallVec = SmallVec<[u8; 128]>;

/// A marker trait applied to `Variable` and `Fixed` that defines the behaviour of a `Bitfield`.
pub trait BitfieldBehaviour: Clone {}

/// A marker struct used to declare SSZ `Variable` behaviour on a `Bitfield`.
///
/// See the [`Bitfield`](struct.Bitfield.html) docs for usage.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Variable<const N: usize>;

/// A marker struct used to declare SSZ `Fixed` behaviour on a `Bitfield`.
///
/// See the [`Bitfield`](struct.Bitfield.html) docs for usage.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Fixed<const N: usize>;

impl<const N: usize> BitfieldBehaviour for Variable<N> {}
impl<const N: usize> BitfieldBehaviour for Fixed<N> {}

/// A heap-allocated, ordered, variable-length collection of `bool` values, limited to `N` bits.
pub type BitList<const N: usize> = Bitfield<Variable<N>>;

/// A heap-allocated, ordered, fixed-length collection of `bool` values, with `N` bits.
///
/// See [Bitfield](struct.Bitfield.html) documentation.
pub type BitVector<const N: usize> = Bitfield<Fixed<N>>;

/// A heap-allocated, ordered, fixed-length, collection of `bool` values. Use of
/// [`BitList`](type.BitList.html) or [`BitVector`](type.BitVector.html) type aliases is preferred
/// over direct use of this struct.
///
/// The `T` type parameter is used to define length behaviour with the `Variable` or `Fixed` marker
/// structs.
///
/// The length of the Bitfield is set at instantiation (i.e., runtime, not compile time). However,
/// use with a `Variable` sets a type-level (i.e., compile-time) maximum length and `Fixed`
/// provides a type-level fixed length.
///
/// ## Example
///
/// The example uses the following crate-level type aliases:
///
/// - `BitList<N>` is an alias for `Bitfield<Variable<N>>`
/// - `BitVector<N>` is an alias for `Bitfield<Fixed<N>>`
///
/// ```
/// use ssz::{BitVector, BitList};
///
/// // `BitList` has a type-level maximum length. The length of the list is specified at runtime
/// // and it must be less than or equal to `N`. After instantiation, `BitList` cannot grow or
/// // shrink.
/// type BitList8 = BitList<8>;
///
/// // Creating a `BitList` with a larger-than-`N` capacity returns `None`.
/// assert!(BitList8::with_capacity(9).is_err());
///
/// let mut bitlist = BitList8::with_capacity(4).unwrap();  // `BitList` permits a capacity of less than the maximum.
/// assert!(bitlist.set(3, true).is_ok());  // Setting inside the instantiation capacity is permitted.
/// assert!(bitlist.set(5, true).is_err());  // Setting outside that capacity is not.
///
/// // `BitVector` has a type-level fixed length. Unlike `BitList`, it cannot be instantiated with a custom length
/// // or grow/shrink.
/// type BitVector8 = BitVector<8>;
///
/// let mut bitvector = BitVector8::new();
/// assert_eq!(bitvector.len(), 8); // `BitVector` length is fixed at the type-level.
/// assert!(bitvector.set(7, true).is_ok());  // Setting inside the capacity is permitted.
/// assert!(bitvector.set(9, true).is_err());  // Setting outside the capacity is not.
/// ```
///
/// ## Note
///
/// The internal representation of the bitfield is the same as that required by SSZ. The lowest
/// byte (by `Vec` index) stores the lowest bit-indices and the right-most bit stores the lowest
/// bit-index. E.g., `smallvec![0b0000_0001, 0b0000_0010]` has bits `0, 9` set.
#[derive(Clone, Debug)]
pub struct Bitfield<T> {
    bytes: BitfieldSmallVec,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<const N: usize> Bitfield<Variable<N>> {
    /// Instantiate with capacity for `num_bits` boolean values. The length cannot be grown or
    /// shrunk after instantiation.
    ///
    /// All bits are initialized to `false`.
    ///
    /// Returns `Err` if `num_bits > N`.
    pub fn with_capacity(num_bits: usize) -> Result<Self, Error> {
        if num_bits <= N {
            Ok(Self {
                bytes: smallvec![0; bytes_for_bit_len(num_bits)],
                len: num_bits,
                _phantom: PhantomData,
            })
        } else {
            Err(Error::OutOfBounds {
                i: num_bits,
                len: Self::max_len(),
            })
        }
    }

    /// Equal to `N` regardless of the value supplied to `with_capacity`.
    pub fn max_len() -> usize {
        N
    }

    /// Consumes `self`, returning a serialized representation.
    ///
    /// The output is faithful to the SSZ encoding of `self`, such that a leading `true` bit is
    /// used to indicate the length of the bitfield.
    ///
    /// ## Example
    /// ```
    /// use smallvec::SmallVec;
    /// use ssz::BitList;
    ///
    /// type BitList8 = BitList<8>;
    ///
    /// let b = BitList8::with_capacity(4).unwrap();
    ///
    /// assert_eq!(b.into_bytes(), SmallVec::from_buf([0b0001_0000]));
    /// ```
    pub fn into_bytes(self) -> BitfieldSmallVec {
        let len = self.len();
        let mut bytes = self.bytes;

        bytes.resize(bytes_for_bit_len(len + 1), 0);

        let mut bitfield: Bitfield<Variable<N>> = Bitfield::from_raw_bytes(bytes, len + 1)
            .unwrap_or_else(|_| {
                unreachable!(
                    "Bitfield with {} bytes must have enough capacity for {} bits.",
                    bytes_for_bit_len(len + 1),
                    len + 1
                )
            });
        bitfield
            .set(len, true)
            .expect("len must be in bounds for bitfield.");

        bitfield.bytes
    }

    /// Instantiates a new instance from `bytes`. Consumes the same format that `self.into_bytes()`
    /// produces (SSZ).
    ///
    /// Returns `None` if `bytes` are not a valid encoding.
    pub fn from_bytes(bytes: BitfieldSmallVec) -> Result<Self, Error> {
        let bytes_len = bytes.len();
        let mut initial_bitfield: Bitfield<Variable<N>> = {
            let num_bits = bytes.len() * 8;
            Bitfield::from_raw_bytes(bytes, num_bits)?
        };

        let len = initial_bitfield
            .highest_set_bit()
            .ok_or(Error::MissingLengthInformation)?;

        // The length bit should be in the last byte, or else it means we have too many bytes.
        if len / 8 + 1 != bytes_len {
            return Err(Error::InvalidByteCount {
                given: bytes_len,
                expected: len / 8 + 1,
            });
        }

        if len <= Self::max_len() {
            initial_bitfield
                .set(len, false)
                .expect("Bit has been confirmed to exist");

            let mut bytes = initial_bitfield.into_raw_bytes();

            bytes.truncate(bytes_for_bit_len(len));

            Self::from_raw_bytes(bytes, len)
        } else {
            Err(Error::OutOfBounds {
                i: Self::max_len(),
                len: Self::max_len(),
            })
        }
    }

    /// Compute the intersection of two BitLists of potentially different lengths.
    ///
    /// Return a new BitList with length equal to the shorter of the two inputs.
    pub fn intersection(&self, other: &Self) -> Self {
        let min_len = std::cmp::min(self.len(), other.len());
        let mut result = Self::with_capacity(min_len).expect("min len always less than N");
        // Bitwise-and the bytes together, starting from the left of each vector. This takes care
        // of masking out any entries beyond `min_len` as well, assuming the bitfield doesn't
        // contain any set bits beyond its length.
        for i in 0..result.bytes.len() {
            result.bytes[i] = self.bytes[i] & other.bytes[i];
        }
        result
    }

    /// Compute the union of two BitLists of potentially different lengths.
    ///
    /// Return a new BitList with length equal to the longer of the two inputs.
    pub fn union(&self, other: &Self) -> Self {
        let max_len = std::cmp::max(self.len(), other.len());
        let mut result = Self::with_capacity(max_len).expect("max len always less than N");
        for i in 0..result.bytes.len() {
            result.bytes[i] =
                self.bytes.get(i).copied().unwrap_or(0) | other.bytes.get(i).copied().unwrap_or(0);
        }
        result
    }

    /// Returns `true` if `self` is a subset of `other` and `false` otherwise.
    pub fn is_subset(&self, other: &Self) -> bool {
        self.difference(other).is_zero()
    }

    /// Returns a new BitList of length M, with the same bits set as `self`.
    pub fn resize<const M: usize>(&self) -> Result<Bitfield<Variable<M>>, Error> {
        if N > M {
            return Err(Error::InvalidByteCount {
                given: M,
                expected: N + 1,
            });
        }
        let mut resized = Bitfield::<Variable<M>>::with_capacity(M)?;
        for (i, bit) in self.iter().enumerate() {
            resized.set(i, bit)?;
        }
        Ok(resized)
    }
}

impl<const N: usize> Bitfield<Fixed<N>> {
    /// Instantiate a new `Bitfield` with a fixed-length of `N` bits.
    ///
    /// All bits are initialized to `false`.
    pub fn new() -> Self {
        Self {
            bytes: smallvec![0; bytes_for_bit_len(Self::capacity())],
            len: Self::capacity(),
            _phantom: PhantomData,
        }
    }

    /// Returns `N`, the number of bits in `Self`.
    pub fn capacity() -> usize {
        N
    }

    /// Consumes `self`, returning a serialized representation.
    ///
    /// The output is faithful to the SSZ encoding of `self`.
    ///
    /// ## Example
    /// ```
    /// use smallvec::SmallVec;
    /// use ssz::BitVector;
    ///
    /// type BitVector4 = BitVector<4>;
    ///
    /// assert_eq!(
    ///     BitVector4::new().into_bytes(),
    ///     SmallVec::from_buf([0b0000_0000])
    /// );
    /// ```
    pub fn into_bytes(self) -> BitfieldSmallVec {
        self.into_raw_bytes()
    }

    /// Instantiates a new instance from `bytes`. Consumes the same format that `self.into_bytes()`
    /// produces (SSZ).
    ///
    /// Returns `None` if `bytes` are not a valid encoding.
    pub fn from_bytes(bytes: BitfieldSmallVec) -> Result<Self, Error> {
        Self::from_raw_bytes(bytes, Self::capacity())
    }

    /// Compute the intersection of two fixed-length `Bitfield`s.
    ///
    /// Return a new fixed-length `Bitfield`.
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Self::new();
        // Bitwise-and the bytes together, starting from the left of each vector. This takes care
        // of masking out any entries beyond `min_len` as well, assuming the bitfield doesn't
        // contain any set bits beyond its length.
        for i in 0..result.bytes.len() {
            result.bytes[i] = self.bytes[i] & other.bytes[i];
        }
        result
    }

    /// Compute the union of two fixed-length `Bitfield`s.
    ///
    /// Return a new fixed-length `Bitfield`.
    pub fn union(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..result.bytes.len() {
            result.bytes[i] =
                self.bytes.get(i).copied().unwrap_or(0) | other.bytes.get(i).copied().unwrap_or(0);
        }
        result
    }

    /// Returns `true` if `self` is a subset of `other` and `false` otherwise.
    pub fn is_subset(&self, other: &Self) -> bool {
        self.difference(other).is_zero()
    }
}

impl<const N: usize> std::fmt::Display for Bitfield<Fixed<N>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut field: String = "".to_string();
        for i in self.iter() {
            if i { field.push('1') } else { field.push('0') }
        }
        write!(f, "{field}")
    }
}

impl<const N: usize> Default for Bitfield<Fixed<N>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: BitfieldBehaviour> Bitfield<T> {
    /// Sets the `i`'th bit to `value`.
    ///
    /// Returns `None` if `i` is out-of-bounds of `self`.
    pub fn set(&mut self, i: usize, value: bool) -> Result<(), Error> {
        let len = self.len;

        if i < len {
            let byte = self
                .bytes
                .get_mut(i / 8)
                .ok_or(Error::OutOfBounds { i, len })?;

            if value {
                *byte |= 1 << (i % 8)
            } else {
                *byte &= !(1 << (i % 8))
            }

            Ok(())
        } else {
            Err(Error::OutOfBounds { i, len: self.len })
        }
    }

    /// Returns the value of the `i`'th bit.
    ///
    /// Returns `Error` if `i` is out-of-bounds of `self`.
    pub fn get(&self, i: usize) -> Result<bool, Error> {
        if i < self.len {
            let byte = self
                .bytes
                .get(i / 8)
                .ok_or(Error::OutOfBounds { i, len: self.len })?;

            Ok(*byte & (1 << (i % 8)) > 0)
        } else {
            Err(Error::OutOfBounds { i, len: self.len })
        }
    }

    /// Returns the number of bits stored in `self`.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if `self.len() == 0`.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the underlying bytes representation of the bitfield.
    pub fn into_raw_bytes(self) -> BitfieldSmallVec {
        self.bytes
    }

    /// Returns a view into the underlying bytes representation of the bitfield.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Instantiates from the given `bytes`, which are the same format as output from
    /// `self.into_raw_bytes()`.
    ///
    /// Returns `None` if:
    ///
    /// - `bytes` is not the minimal required bytes to represent a bitfield of `bit_len` bits.
    /// - `bit_len` is not a multiple of 8 and `bytes` contains set bits that are higher than, or
    ///   equal to `bit_len`.
    fn from_raw_bytes(bytes: BitfieldSmallVec, bit_len: usize) -> Result<Self, Error> {
        if bit_len == 0 {
            if bytes.len() == 1 && bytes[0] == 0 {
                // A bitfield with `bit_len` 0 can only be represented by a single zero byte.
                Ok(Self {
                    bytes,
                    len: 0,
                    _phantom: PhantomData,
                })
            } else {
                Err(Error::ExcessBits)
            }
        } else if bytes.len() != bytes_for_bit_len(bit_len) {
            // The number of bytes must be the minimum required to represent `bit_len`.
            Err(Error::InvalidByteCount {
                given: bytes.len(),
                expected: bytes_for_bit_len(bit_len),
            })
        } else {
            // Ensure there are no bits higher than `bit_len` that are set to true.
            let (mask, _) = u8::MAX.overflowing_shr(8 - (bit_len as u32 % 8));

            if (bytes.last().expect("Guarded against empty bytes") & !mask) == 0 {
                Ok(Self {
                    bytes,
                    len: bit_len,
                    _phantom: PhantomData,
                })
            } else {
                Err(Error::ExcessBits)
            }
        }
    }

    /// Returns the `Some(i)` where `i` is the highest index with a set bit. Returns `None` if
    /// there are no set bits.
    pub fn highest_set_bit(&self) -> Option<usize> {
        self.bytes
            .iter()
            .enumerate()
            .rev()
            .find(|(_, byte)| **byte > 0)
            .map(|(i, byte)| i * 8 + 7 - byte.leading_zeros() as usize)
    }

    /// Returns an iterator across bitfield `bool` values, starting at the lowest index.
    pub fn iter(&self) -> BitIter<'_, T> {
        BitIter {
            bitfield: self,
            i: 0,
        }
    }

    /// Returns true if no bits are set.
    pub fn is_zero(&self) -> bool {
        self.bytes.iter().all(|byte| *byte == 0)
    }

    /// Returns the number of bits that are set to `true`.
    pub fn num_set_bits(&self) -> usize {
        self.bytes
            .iter()
            .map(|byte| byte.count_ones() as usize)
            .sum()
    }

    /// Compute the difference of this Bitfield and another of potentially different length.
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.difference_inplace(other);
        result
    }

    /// Compute the difference of this Bitfield and another of potentially different length.
    pub fn difference_inplace(&mut self, other: &Self) {
        let min_byte_len = std::cmp::min(self.bytes.len(), other.bytes.len());

        for i in 0..min_byte_len {
            self.bytes[i] &= !other.bytes[i];
        }
    }

    /// Shift the bits to higher indices, filling the lower indices with zeroes.
    ///
    /// The amount to shift by, `n`, must be less than or equal to `self.len()`.
    pub fn shift_up(&mut self, n: usize) -> Result<(), Error> {
        if n <= self.len() {
            // Shift the bits up (starting from the high indices to avoid overwriting)
            for i in (n..self.len()).rev() {
                self.set(i, self.get(i - n)?)?;
            }
            // Zero the low bits
            for i in 0..n {
                self.set(i, false).unwrap();
            }
            Ok(())
        } else {
            Err(Error::OutOfBounds {
                i: n,
                len: self.len(),
            })
        }
    }
}

impl<T> Eq for Bitfield<T> {}
impl<T> PartialEq for Bitfield<T> {
    #[inline]
    fn eq(&self, other: &Bitfield<T>) -> bool {
        self.len == other.len && self.bytes == other.bytes
    }
}

impl<T> core::hash::Hash for Bitfield<T> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.bytes, state);
        core::hash::Hash::hash(&self.len, state);
    }
}

/// Returns the minimum required bytes to represent a given number of bits.
///
/// `bit_len == 0` requires a single byte.
fn bytes_for_bit_len(bit_len: usize) -> usize {
    std::cmp::max(1, bit_len.div_ceil(8))
}

/// An iterator over the bits in a `Bitfield`.
#[derive(Debug)]
pub struct BitIter<'a, T> {
    bitfield: &'a Bitfield<T>,
    i: usize,
}

impl<T: BitfieldBehaviour> Iterator for BitIter<'_, T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.bitfield.get(self.i).ok()?;
        self.i += 1;
        Some(res)
    }
}

impl<const N: usize> Encode for Bitfield<Variable<N>> {
    fn is_ssz_fixed_len() -> bool {
        false
    }

    fn ssz_bytes_len(&self) -> usize {
        // We could likely do better than turning this into bytes and reading the length, however
        // it is kept this way for simplicity.
        self.clone().into_bytes().len()
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.clone().into_bytes())
    }
}

impl<const N: usize> Decode for Bitfield<Variable<N>> {
    fn is_ssz_fixed_len() -> bool {
        false
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_bytes(bytes.to_smallvec())
            .map_err(|e| DecodeError::BytesInvalid(format!("BitList failed to decode: {e:?}")))
    }
}

impl<const N: usize> Encode for Bitfield<Fixed<N>> {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_bytes_len(&self) -> usize {
        self.as_slice().len()
    }

    fn ssz_fixed_len() -> usize {
        bytes_for_bit_len(N)
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.clone().into_bytes())
    }
}

impl<const N: usize> Decode for Bitfield<Fixed<N>> {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        bytes_for_bit_len(N)
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_bytes(bytes.to_smallvec())
            .map_err(|e| DecodeError::BytesInvalid(format!("BitVector failed to decode: {e:?}")))
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> Serialize for Bitfield<Variable<N>> {
    /// Serde serialization is compliant with the Ethereum YAML test format.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex_encode(self.as_ssz_bytes()))
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> Deserialize<'de> for Bitfield<Variable<N>> {
    /// Serde serialization is compliant with the Ethereum YAML test format.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = deserializer.deserialize_str(PrefixedHexVisitor)?;
        Self::from_ssz_bytes(&bytes)
            .map_err(|e| serde::de::Error::custom(format!("Bitfield {e:?}")))
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> Serialize for Bitfield<Fixed<N>> {
    /// Serde serialization is compliant with the Ethereum YAML test format.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex_encode(self.as_ssz_bytes()))
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> Deserialize<'de> for Bitfield<Fixed<N>> {
    /// Serde serialization is compliant with the Ethereum YAML test format.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = deserializer.deserialize_str(PrefixedHexVisitor)?;
        Self::from_ssz_bytes(&bytes)
            .map_err(|e| serde::de::Error::custom(format!("Bitfield {e:?}")))
    }
}

#[cfg(feature = "arbitrary")]
impl<const N: usize> arbitrary::Arbitrary<'_> for Bitfield<Fixed<N>> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let size = N;
        let mut vec = smallvec![0u8; size];
        u.fill_buffer(&mut vec)?;
        Self::from_bytes(vec).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

#[cfg(feature = "arbitrary")]
impl<const N: usize> arbitrary::Arbitrary<'_> for Bitfield<Variable<N>> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let max_size = N;
        let rand = usize::arbitrary(u)?;
        let size = std::cmp::min(rand, max_size);
        let mut vec = smallvec![0u8; size];
        u.fill_buffer(&mut vec)?;
        Self::from_bytes(vec).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

#[cfg(test)]
mod bitvector {
    use super::*;
    use crate::BitVector;

    pub(crate) type BitVector0 = BitVector<0>;
    pub(crate) type BitVector1 = BitVector<1>;
    pub(crate) type BitVector4 = BitVector<4>;
    pub(crate) type BitVector8 = BitVector<8>;
    pub(crate) type BitVector16 = BitVector<16>;
    pub(crate) type BitVector64 = BitVector<64>;

    #[test]
    fn ssz_encode() {
        assert_eq!(BitVector0::new().as_ssz_bytes(), vec![0b0000_0000]);
        assert_eq!(BitVector1::new().as_ssz_bytes(), vec![0b0000_0000]);
        assert_eq!(BitVector4::new().as_ssz_bytes(), vec![0b0000_0000]);
        assert_eq!(BitVector8::new().as_ssz_bytes(), vec![0b0000_0000]);
        assert_eq!(
            BitVector16::new().as_ssz_bytes(),
            vec![0b0000_0000, 0b0000_0000]
        );

        let mut b = BitVector8::new();
        for i in 0..8 {
            b.set(i, true).unwrap();
        }
        assert_eq!(b.as_ssz_bytes(), vec![255]);

        let mut b = BitVector4::new();
        for i in 0..4 {
            b.set(i, true).unwrap();
        }
        assert_eq!(b.as_ssz_bytes(), vec![0b0000_1111]);
    }

    #[test]
    fn ssz_decode() {
        assert!(BitVector0::from_ssz_bytes(&[0b0000_0000]).is_ok());
        assert!(BitVector0::from_ssz_bytes(&[0b0000_0001]).is_err());
        assert!(BitVector0::from_ssz_bytes(&[0b0000_0010]).is_err());

        assert!(BitVector1::from_ssz_bytes(&[0b0000_0001]).is_ok());
        assert!(BitVector1::from_ssz_bytes(&[0b0000_0010]).is_err());
        assert!(BitVector1::from_ssz_bytes(&[0b0000_0100]).is_err());
        assert!(BitVector1::from_ssz_bytes(&[0b0000_0000, 0b0000_0000]).is_err());

        assert!(BitVector8::from_ssz_bytes(&[0b0000_0000]).is_ok());
        assert!(BitVector8::from_ssz_bytes(&[1, 0b0000_0000]).is_err());
        assert!(BitVector8::from_ssz_bytes(&[0b0000_0000, 1]).is_err());
        assert!(BitVector8::from_ssz_bytes(&[0b0000_0001]).is_ok());
        assert!(BitVector8::from_ssz_bytes(&[0b0000_0010]).is_ok());
        assert!(BitVector8::from_ssz_bytes(&[0b0000_0100, 0b0000_0001]).is_err());
        assert!(BitVector8::from_ssz_bytes(&[0b0000_0100, 0b0000_0010]).is_err());
        assert!(BitVector8::from_ssz_bytes(&[0b0000_0100, 0b0000_0100]).is_err());

        assert!(BitVector16::from_ssz_bytes(&[0b0000_0000]).is_err());
        assert!(BitVector16::from_ssz_bytes(&[0b0000_0000, 0b0000_0000]).is_ok());
        assert!(BitVector16::from_ssz_bytes(&[1, 0b0000_0000, 0b0000_0000]).is_err());
    }

    #[test]
    fn intersection() {
        let a = BitVector16::from_raw_bytes(smallvec![0b1100, 0b0001], 16).unwrap();
        let b = BitVector16::from_raw_bytes(smallvec![0b1011, 0b1001], 16).unwrap();
        let c = BitVector16::from_raw_bytes(smallvec![0b1000, 0b0001], 16).unwrap();

        assert_eq!(a.intersection(&b), c);
        assert_eq!(b.intersection(&a), c);
        assert_eq!(a.intersection(&c), c);
        assert_eq!(b.intersection(&c), c);
        assert_eq!(a.intersection(&a), a);
        assert_eq!(b.intersection(&b), b);
        assert_eq!(c.intersection(&c), c);
    }

    #[test]
    fn intersection_diff_length() {
        let a = BitVector16::from_bytes(smallvec![0b0010_1110, 0b0010_1011]).unwrap();
        let b = BitVector16::from_bytes(smallvec![0b0010_1101, 0b0000_0001]).unwrap();
        let c = BitVector16::from_bytes(smallvec![0b0010_1100, 0b0000_0001]).unwrap();

        assert_eq!(a.len(), 16);
        assert_eq!(b.len(), 16);
        assert_eq!(c.len(), 16);
        assert_eq!(a.intersection(&b), c);
        assert_eq!(b.intersection(&a), c);
    }

    #[test]
    fn subset() {
        let a = BitVector16::from_raw_bytes(smallvec![0b1000, 0b0001], 16).unwrap();
        let b = BitVector16::from_raw_bytes(smallvec![0b1100, 0b0001], 16).unwrap();
        let c = BitVector16::from_raw_bytes(smallvec![0b1100, 0b1001], 16).unwrap();

        assert_eq!(a.len(), 16);
        assert_eq!(b.len(), 16);
        assert_eq!(c.len(), 16);

        // a vector is always a subset of itself
        assert!(a.is_subset(&a));
        assert!(b.is_subset(&b));
        assert!(c.is_subset(&c));

        assert!(a.is_subset(&b));
        assert!(a.is_subset(&c));
        assert!(b.is_subset(&c));

        assert!(!b.is_subset(&a));
        assert!(!c.is_subset(&a));
        assert!(!c.is_subset(&b));
    }

    #[test]
    fn union() {
        let a = BitVector16::from_raw_bytes(smallvec![0b1100, 0b0001], 16).unwrap();
        let b = BitVector16::from_raw_bytes(smallvec![0b1011, 0b1001], 16).unwrap();
        let c = BitVector16::from_raw_bytes(smallvec![0b1111, 0b1001], 16).unwrap();

        assert_eq!(a.union(&b), c);
        assert_eq!(b.union(&a), c);
        assert_eq!(a.union(&a), a);
        assert_eq!(b.union(&b), b);
        assert_eq!(c.union(&c), c);
    }

    #[test]
    fn union_diff_length() {
        let a = BitVector16::from_bytes(smallvec![0b0010_1011, 0b0010_1110]).unwrap();
        let b = BitVector16::from_bytes(smallvec![0b0000_0001, 0b0010_1101]).unwrap();
        let c = BitVector16::from_bytes(smallvec![0b0010_1011, 0b0010_1111]).unwrap();

        assert_eq!(a.len(), c.len());
        assert_eq!(a.union(&b), c);
        assert_eq!(b.union(&a), c);
    }

    #[test]
    fn ssz_round_trip() {
        assert_round_trip(BitVector0::new());

        let mut b = BitVector1::new();
        b.set(0, true).unwrap();
        assert_round_trip(b);

        let mut b = BitVector8::new();
        for j in 0usize..8 {
            if j.is_multiple_of(2) {
                b.set(j, true).unwrap();
            }
        }
        assert_round_trip(b);

        let mut b = BitVector8::new();
        for j in 0..8 {
            b.set(j, true).unwrap();
        }
        assert_round_trip(b);

        let mut b = BitVector16::new();
        for j in 0usize..16 {
            if j.is_multiple_of(2) {
                b.set(j, true).unwrap();
            }
        }
        assert_round_trip(b);

        let mut b = BitVector16::new();
        for j in 0..16 {
            b.set(j, true).unwrap();
        }
        assert_round_trip(b);
    }

    fn assert_round_trip<T: Encode + Decode + PartialEq + std::fmt::Debug>(t: T) {
        assert_eq!(T::from_ssz_bytes(&t.as_ssz_bytes()).unwrap(), t);
    }

    #[test]
    fn ssz_bytes_len() {
        for i in 0..64 {
            let mut bitfield = BitVector64::new();
            for j in 0..i {
                bitfield.set(j, true).expect("should set bit in bounds");
            }
            let bytes = bitfield.as_ssz_bytes();
            assert_eq!(bitfield.ssz_bytes_len(), bytes.len(), "i = {i}");
        }
    }

    #[test]
    fn excess_bits_nimbus() {
        let bad = vec![0b0001_1111];

        assert!(BitVector4::from_ssz_bytes(&bad).is_err());
    }

    // Ensure that stack size of a BitVector is manageable.
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<BitVector64>(), SMALLVEC_LEN + 24);
    }
}

#[cfg(test)]
#[allow(clippy::cognitive_complexity)]
mod bitlist {
    use super::*;
    use crate::BitList;

    pub(crate) type BitList0 = BitList<0>;
    pub(crate) type BitList1 = BitList<1>;
    pub(crate) type BitList8 = BitList<8>;
    pub(crate) type BitList16 = BitList<16>;
    pub(crate) type BitList1024 = BitList<1024>;

    #[test]
    fn ssz_encode() {
        assert_eq!(
            BitList0::with_capacity(0).unwrap().as_ssz_bytes(),
            vec![0b0000_0001],
        );

        assert_eq!(
            BitList1::with_capacity(0).unwrap().as_ssz_bytes(),
            vec![0b0000_0001],
        );

        assert_eq!(
            BitList1::with_capacity(1).unwrap().as_ssz_bytes(),
            vec![0b0000_0010],
        );

        assert_eq!(
            BitList8::with_capacity(8).unwrap().as_ssz_bytes(),
            vec![0b0000_0000, 0b0000_0001],
        );

        assert_eq!(
            BitList8::with_capacity(7).unwrap().as_ssz_bytes(),
            vec![0b1000_0000]
        );

        let mut b = BitList8::with_capacity(8).unwrap();
        for i in 0..8 {
            b.set(i, true).unwrap();
        }
        assert_eq!(b.as_ssz_bytes(), vec![255, 0b0000_0001]);

        let mut b = BitList8::with_capacity(8).unwrap();
        for i in 0..4 {
            b.set(i, true).unwrap();
        }
        assert_eq!(b.as_ssz_bytes(), vec![0b0000_1111, 0b0000_0001]);

        assert_eq!(
            BitList16::with_capacity(16).unwrap().as_ssz_bytes(),
            vec![0b0000_0000, 0b0000_0000, 0b0000_0001]
        );
    }

    #[test]
    fn ssz_decode() {
        assert!(BitList0::from_ssz_bytes(&[]).is_err());
        assert!(BitList1::from_ssz_bytes(&[]).is_err());
        assert!(BitList8::from_ssz_bytes(&[]).is_err());
        assert!(BitList16::from_ssz_bytes(&[]).is_err());

        assert!(BitList0::from_ssz_bytes(&[0b0000_0000]).is_err());
        assert!(BitList1::from_ssz_bytes(&[0b0000_0000, 0b0000_0000]).is_err());
        assert!(BitList8::from_ssz_bytes(&[0b0000_0000]).is_err());
        assert!(BitList16::from_ssz_bytes(&[0b0000_0000]).is_err());

        assert!(BitList0::from_ssz_bytes(&[0b0000_0001]).is_ok());
        assert!(BitList0::from_ssz_bytes(&[0b0000_0010]).is_err());

        assert!(BitList1::from_ssz_bytes(&[0b0000_0001]).is_ok());
        assert!(BitList1::from_ssz_bytes(&[0b0000_0010]).is_ok());
        assert!(BitList1::from_ssz_bytes(&[0b0000_0100]).is_err());

        assert!(BitList8::from_ssz_bytes(&[0b0000_0001]).is_ok());
        assert!(BitList8::from_ssz_bytes(&[0b0000_0010]).is_ok());
        assert!(BitList8::from_ssz_bytes(&[0b0000_0001, 0b0000_0001]).is_ok());
        assert!(BitList8::from_ssz_bytes(&[0b0000_0001, 0b0000_0010]).is_err());
        assert!(BitList8::from_ssz_bytes(&[0b0000_0001, 0b0000_0100]).is_err());
    }

    #[test]
    fn ssz_decode_extra_bytes() {
        assert!(BitList0::from_ssz_bytes(&[0b0000_0001, 0b0000_0000]).is_err());
        assert!(BitList1::from_ssz_bytes(&[0b0000_0001, 0b0000_0000]).is_err());
        assert!(BitList8::from_ssz_bytes(&[0b0000_0001, 0b0000_0000]).is_err());
        assert!(BitList16::from_ssz_bytes(&[0b0000_0001, 0b0000_0000]).is_err());
        assert!(BitList1024::from_ssz_bytes(&[0b1000_0000, 0]).is_err());
        assert!(BitList1024::from_ssz_bytes(&[0b1000_0000, 0, 0]).is_err());
        assert!(BitList1024::from_ssz_bytes(&[0b1000_0000, 0, 0, 0, 0]).is_err());
    }

    #[test]
    fn ssz_round_trip() {
        assert_round_trip(BitList0::with_capacity(0).unwrap());

        for i in 0..2 {
            assert_round_trip(BitList1::with_capacity(i).unwrap());
        }
        for i in 0..9 {
            assert_round_trip(BitList8::with_capacity(i).unwrap());
        }
        for i in 0..17 {
            assert_round_trip(BitList16::with_capacity(i).unwrap());
        }

        let mut b = BitList1::with_capacity(1).unwrap();
        b.set(0, true).unwrap();
        assert_round_trip(b);

        for i in 0..8 {
            let mut b = BitList8::with_capacity(i).unwrap();
            for j in 0usize..i {
                if j.is_multiple_of(2) {
                    b.set(j, true).unwrap();
                }
            }
            assert_round_trip(b);

            let mut b = BitList8::with_capacity(i).unwrap();
            for j in 0..i {
                b.set(j, true).unwrap();
            }
            assert_round_trip(b);
        }

        for i in 0..16 {
            let mut b = BitList16::with_capacity(i).unwrap();
            for j in 0usize..i {
                if j.is_multiple_of(2) {
                    b.set(j, true).unwrap();
                }
            }
            assert_round_trip(b);

            let mut b = BitList16::with_capacity(i).unwrap();
            for j in 0..i {
                b.set(j, true).unwrap();
            }
            assert_round_trip(b);
        }
    }

    fn assert_round_trip<T: Encode + Decode + PartialEq + std::fmt::Debug>(t: T) {
        assert_eq!(T::from_ssz_bytes(&t.as_ssz_bytes()).unwrap(), t);
    }

    #[test]
    fn from_raw_bytes() {
        assert!(BitList1024::from_raw_bytes(smallvec![0b0000_0000], 0).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0000_0001], 1).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0000_0011], 2).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0000_0111], 3).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0000_1111], 4).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0001_1111], 5).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0011_1111], 6).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0111_1111], 7).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111], 8).is_ok());

        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0000_0001], 9).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0000_0011], 10).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0000_0111], 11).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0000_1111], 12).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0001_1111], 13).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0011_1111], 14).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0111_1111], 15).is_ok());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b1111_1111], 16).is_ok());

        for i in 0..8 {
            assert!(BitList1024::from_raw_bytes(smallvec![], i).is_err());
            assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111], i).is_err());
            assert!(BitList1024::from_raw_bytes(smallvec![0b0000_0000, 0b1111_1110], i).is_err());
        }

        assert!(BitList1024::from_raw_bytes(smallvec![0b0000_0001], 0).is_err());

        assert!(BitList1024::from_raw_bytes(smallvec![0b0000_0001], 0).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0000_0011], 1).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0000_0111], 2).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0000_1111], 3).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0001_1111], 4).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0011_1111], 5).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b0111_1111], 6).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111], 7).is_err());

        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0000_0001], 8).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0000_0011], 9).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0000_0111], 10).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0000_1111], 11).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0001_1111], 12).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0011_1111], 13).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b0111_1111], 14).is_err());
        assert!(BitList1024::from_raw_bytes(smallvec![0b1111_1111, 0b1111_1111], 15).is_err());
    }

    fn test_set_unset(num_bits: usize) {
        let mut bitfield = BitList1024::with_capacity(num_bits).unwrap();

        for i in 0..=num_bits {
            if i < num_bits {
                // Starts as false
                assert_eq!(bitfield.get(i), Ok(false));
                // Can be set true.
                assert!(bitfield.set(i, true).is_ok());
                assert_eq!(bitfield.get(i), Ok(true));
                // Can be set false
                assert!(bitfield.set(i, false).is_ok());
                assert_eq!(bitfield.get(i), Ok(false));
            } else {
                assert!(bitfield.get(i).is_err());
                assert!(bitfield.set(i, true).is_err());
                assert!(bitfield.get(i).is_err());
            }
        }
    }

    fn test_bytes_round_trip(num_bits: usize) {
        for i in 0..num_bits {
            let mut bitfield = BitList1024::with_capacity(num_bits).unwrap();
            bitfield.set(i, true).unwrap();

            let bytes = bitfield.clone().into_raw_bytes();
            assert_eq!(bitfield, Bitfield::from_raw_bytes(bytes, num_bits).unwrap());
        }
    }

    #[test]
    fn set_unset() {
        for i in 0..8 * 5 {
            test_set_unset(i)
        }
    }

    #[test]
    fn bytes_round_trip() {
        for i in 0..8 * 5 {
            test_bytes_round_trip(i)
        }
    }

    /// Type-specialised `smallvec` macro for testing.
    macro_rules! bytevec {
        ($($x : expr),* $(,)*) => {
            {
                let __smallvec: BitfieldSmallVec = smallvec!($($x),*);
                __smallvec
            }
        };
    }

    #[test]
    fn into_raw_bytes() {
        let mut bitfield = BitList1024::with_capacity(9).unwrap();
        bitfield.set(0, true).unwrap();
        assert_eq!(
            bitfield.clone().into_raw_bytes(),
            bytevec![0b0000_0001, 0b0000_0000]
        );
        bitfield.set(1, true).unwrap();
        assert_eq!(
            bitfield.clone().into_raw_bytes(),
            bytevec![0b0000_0011, 0b0000_0000]
        );
        bitfield.set(2, true).unwrap();
        assert_eq!(
            bitfield.clone().into_raw_bytes(),
            bytevec![0b0000_0111, 0b0000_0000]
        );
        bitfield.set(3, true).unwrap();
        assert_eq!(
            bitfield.clone().into_raw_bytes(),
            bytevec![0b0000_1111, 0b0000_0000]
        );
        bitfield.set(4, true).unwrap();
        assert_eq!(
            bitfield.clone().into_raw_bytes(),
            bytevec![0b0001_1111, 0b0000_0000]
        );
        bitfield.set(5, true).unwrap();
        assert_eq!(
            bitfield.clone().into_raw_bytes(),
            bytevec![0b0011_1111, 0b0000_0000]
        );
        bitfield.set(6, true).unwrap();
        assert_eq!(
            bitfield.clone().into_raw_bytes(),
            bytevec![0b0111_1111, 0b0000_0000]
        );
        bitfield.set(7, true).unwrap();
        assert_eq!(
            bitfield.clone().into_raw_bytes(),
            bytevec![0b1111_1111, 0b0000_0000]
        );
        bitfield.set(8, true).unwrap();
        assert_eq!(
            bitfield.into_raw_bytes(),
            bytevec![0b1111_1111, 0b0000_0001]
        );
    }

    #[test]
    fn highest_set_bit() {
        assert_eq!(
            BitList1024::with_capacity(16).unwrap().highest_set_bit(),
            None
        );

        assert_eq!(
            BitList1024::from_raw_bytes(smallvec![0b0000_0001, 0b0000_0000], 16)
                .unwrap()
                .highest_set_bit(),
            Some(0)
        );

        assert_eq!(
            BitList1024::from_raw_bytes(smallvec![0b0000_0010, 0b0000_0000], 16)
                .unwrap()
                .highest_set_bit(),
            Some(1)
        );

        assert_eq!(
            BitList1024::from_raw_bytes(smallvec![0b0000_1000], 8)
                .unwrap()
                .highest_set_bit(),
            Some(3)
        );

        assert_eq!(
            BitList1024::from_raw_bytes(smallvec![0b0000_0000, 0b1000_0000], 16)
                .unwrap()
                .highest_set_bit(),
            Some(15)
        );
    }

    #[test]
    fn intersection() {
        let a = BitList1024::from_raw_bytes(smallvec![0b1100, 0b0001], 16).unwrap();
        let b = BitList1024::from_raw_bytes(smallvec![0b1011, 0b1001], 16).unwrap();
        let c = BitList1024::from_raw_bytes(smallvec![0b1000, 0b0001], 16).unwrap();

        assert_eq!(a.intersection(&b), c);
        assert_eq!(b.intersection(&a), c);
        assert_eq!(a.intersection(&c), c);
        assert_eq!(b.intersection(&c), c);
        assert_eq!(a.intersection(&a), a);
        assert_eq!(b.intersection(&b), b);
        assert_eq!(c.intersection(&c), c);
    }

    #[test]
    fn subset() {
        let a = BitList1024::from_raw_bytes(smallvec![0b1000, 0b0001], 16).unwrap();
        let b = BitList1024::from_raw_bytes(smallvec![0b1100, 0b0001], 16).unwrap();
        let c = BitList1024::from_raw_bytes(smallvec![0b1100, 0b1001], 16).unwrap();

        assert_eq!(a.len(), 16);
        assert_eq!(b.len(), 16);
        assert_eq!(c.len(), 16);

        // a vector is always a subset of itself
        assert!(a.is_subset(&a));
        assert!(b.is_subset(&b));
        assert!(c.is_subset(&c));

        assert!(a.is_subset(&b));
        assert!(a.is_subset(&c));
        assert!(b.is_subset(&c));

        assert!(!b.is_subset(&a));
        assert!(!c.is_subset(&a));
        assert!(!c.is_subset(&b));

        let d = BitList1024::from_raw_bytes(smallvec![0b1100, 0b1001, 0b1010], 24).unwrap();
        assert!(d.is_subset(&d));

        assert!(a.is_subset(&d));
        assert!(b.is_subset(&d));
        assert!(c.is_subset(&d));

        // A bigger length bitlist cannot be a subset of a smaller length bitlist
        assert!(!d.is_subset(&a));
        assert!(!d.is_subset(&b));
        assert!(!d.is_subset(&c));

        let e = BitList1024::from_raw_bytes(smallvec![0b1100, 0b1001, 0b0000], 24).unwrap();
        assert!(e.is_subset(&c));
        assert!(c.is_subset(&e));
    }

    #[test]
    fn intersection_diff_length() {
        let a = BitList1024::from_bytes(smallvec![0b0010_1110, 0b0010_1011]).unwrap();
        let b = BitList1024::from_bytes(smallvec![0b0010_1101, 0b0000_0001]).unwrap();
        let c = BitList1024::from_bytes(smallvec![0b0010_1100, 0b0000_0001]).unwrap();
        let d = BitList1024::from_bytes(smallvec![0b0010_1110, 0b1111_1111, 0b1111_1111]).unwrap();

        assert_eq!(a.len(), 13);
        assert_eq!(b.len(), 8);
        assert_eq!(c.len(), 8);
        assert_eq!(d.len(), 23);
        assert_eq!(a.intersection(&b), c);
        assert_eq!(b.intersection(&a), c);
        assert_eq!(a.intersection(&d), a);
        assert_eq!(d.intersection(&a), a);
    }

    #[test]
    fn union() {
        let a = BitList1024::from_raw_bytes(smallvec![0b1100, 0b0001], 16).unwrap();
        let b = BitList1024::from_raw_bytes(smallvec![0b1011, 0b1001], 16).unwrap();
        let c = BitList1024::from_raw_bytes(smallvec![0b1111, 0b1001], 16).unwrap();

        assert_eq!(a.union(&b), c);
        assert_eq!(b.union(&a), c);
        assert_eq!(a.union(&a), a);
        assert_eq!(b.union(&b), b);
        assert_eq!(c.union(&c), c);
    }

    #[test]
    fn union_diff_length() {
        let a = BitList1024::from_bytes(smallvec![0b0010_1011, 0b0010_1110]).unwrap();
        let b = BitList1024::from_bytes(smallvec![0b0000_0001, 0b0010_1101]).unwrap();
        let c = BitList1024::from_bytes(smallvec![0b0010_1011, 0b0010_1111]).unwrap();
        let d = BitList1024::from_bytes(smallvec![0b0010_1011, 0b1011_1110, 0b1000_1101]).unwrap();

        assert_eq!(a.len(), c.len());
        assert_eq!(a.union(&b), c);
        assert_eq!(b.union(&a), c);
        assert_eq!(a.union(&d), d);
        assert_eq!(d.union(&a), d);
    }

    #[test]
    fn difference() {
        let a = BitList1024::from_raw_bytes(smallvec![0b1100, 0b0001], 16).unwrap();
        let b = BitList1024::from_raw_bytes(smallvec![0b1011, 0b1001], 16).unwrap();
        let a_b = BitList1024::from_raw_bytes(smallvec![0b0100, 0b0000], 16).unwrap();
        let b_a = BitList1024::from_raw_bytes(smallvec![0b0011, 0b1000], 16).unwrap();

        assert_eq!(a.difference(&b), a_b);
        assert_eq!(b.difference(&a), b_a);
        assert!(a.difference(&a).is_zero());
    }

    #[test]
    fn difference_diff_length() {
        let a = BitList1024::from_raw_bytes(smallvec![0b0110, 0b1100, 0b0011], 24).unwrap();
        let b = BitList1024::from_raw_bytes(smallvec![0b1011, 0b1001], 16).unwrap();
        let a_b = BitList1024::from_raw_bytes(smallvec![0b0100, 0b0100, 0b0011], 24).unwrap();
        let b_a = BitList1024::from_raw_bytes(smallvec![0b1001, 0b0001], 16).unwrap();

        assert_eq!(a.difference(&b), a_b);
        assert_eq!(b.difference(&a), b_a);
    }

    #[test]
    fn shift_up() {
        let mut a = BitList1024::from_raw_bytes(smallvec![0b1100_1111, 0b1101_0110], 16).unwrap();
        let mut b = BitList1024::from_raw_bytes(smallvec![0b1001_1110, 0b1010_1101], 16).unwrap();

        a.shift_up(1).unwrap();
        assert_eq!(a, b);
        a.shift_up(15).unwrap();
        assert!(a.is_zero());

        b.shift_up(16).unwrap();
        assert!(b.is_zero());
        assert!(b.shift_up(17).is_err());
    }

    #[test]
    fn num_set_bits() {
        let a = BitList1024::from_raw_bytes(smallvec![0b1100, 0b0001], 16).unwrap();
        let b = BitList1024::from_raw_bytes(smallvec![0b1011, 0b1001], 16).unwrap();

        assert_eq!(a.num_set_bits(), 3);
        assert_eq!(b.num_set_bits(), 5);
    }

    #[test]
    fn iter() {
        let mut bitfield = BitList1024::with_capacity(9).unwrap();
        bitfield.set(2, true).unwrap();
        bitfield.set(8, true).unwrap();

        assert_eq!(
            bitfield.iter().collect::<Vec<bool>>(),
            vec![false, false, true, false, false, false, false, false, true]
        );
    }

    #[test]
    fn ssz_bytes_len() {
        for i in 1..64 {
            let mut bitfield = BitList1024::with_capacity(i).unwrap();
            for j in 0..i {
                bitfield.set(j, true).expect("should set bit in bounds");
            }
            let bytes = bitfield.as_ssz_bytes();
            assert_eq!(bitfield.ssz_bytes_len(), bytes.len(), "i = {i}");
        }
    }

    // Ensure that the stack size of a BitList is manageable.
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<BitList1024>(), SMALLVEC_LEN + 24);
    }

    #[test]
    fn resize() {
        let mut bit_list = BitList1::with_capacity(1).unwrap();
        bit_list.set(0, true).unwrap();
        assert_eq!(bit_list.len(), 1);
        assert_eq!(bit_list.num_set_bits(), 1);
        assert_eq!(bit_list.highest_set_bit().unwrap(), 0);

        let resized_bit_list = bit_list.resize::<1024>().unwrap();
        assert_eq!(resized_bit_list.len(), 1024);
        assert_eq!(resized_bit_list.num_set_bits(), 1);
        assert_eq!(resized_bit_list.highest_set_bit().unwrap(), 0);

        // Can't extend a BitList to a smaller BitList
        resized_bit_list.resize::<16>().unwrap_err();
    }

    #[test]
    fn over_capacity_err() {
        let e = BitList8::with_capacity(9).expect_err("over-sized bit list");
        assert_eq!(e, Error::OutOfBounds { i: 9, len: 8 });
    }
}
