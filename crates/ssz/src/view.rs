//! Zero-copy SSZ decoding via reference-backed "view" types.
//!
//! This module provides a borrow-oriented layer on top of the existing [`ssz`](crate) crate that exposes
//! reference-backed "view" types and a decoding trait capable of returning such types from [`&[u8]`](slice)
//! without copying.
//!
//! ## Design
//!
//! - [`DecodeView<'a>`]: trait for zero-copy decoding that constructs borrowed views over input bytes.
//! - View types: [`FixedBytesRef`], [`BytesRef`], [`ListRef`], [`VectorRef`], [`UnionRef`], [`BitVectorRef`], and [`BitListRef`].
//! - All views validate SSZ invariants at construction but avoid copying payload data.
//! - Views provide `to_owned()` methods to materialize owned equivalents when needed.
//!
//! ## Usage
//!
//! ```rust
//! use ssz::view::{DecodeView, BytesRef};
//!
//! let bytes: &[u8] = &[0x01, 0x02, 0x03];
//! let view = BytesRef::from_ssz_bytes(bytes).unwrap();
//! assert_eq!(view.as_bytes(), bytes);
//! ```

use core::marker::PhantomData;

use ssz_primitives::{FixedBytes, U128, U256};

use crate::{
    BYTES_PER_LENGTH_OFFSET, BitList, BitVector, Decode, DecodeError, UnionSelector,
    decode::sanitize_offset, read_offset, split_union_bytes,
};

/// Zero-copy SSZ decoding that constructs borrowed views over input bytes.
///
/// Unlike [`Decode`] which constructs owned values, [`DecodeView`] constructs reference-backed
/// views that avoid allocations and memcpy operations.
///
/// ## Semantics
///
/// Implementations must:
///
/// - Validate all SSZ invariants (offsets, lengths, etc.).
/// - Avoid copying the payload data.
/// - Return views that borrow from the input `bytes`.
///
/// ## Example
///
/// ```rust
/// use ssz::view::{DecodeView, FixedBytesRef};
/// use ssz::DecodeError;
///
/// let bytes = [0x01, 0x02, 0x03, 0x04];
/// let view = FixedBytesRef::<4>::from_ssz_bytes(&bytes)?;
/// assert_eq!(view.as_bytes(), &bytes);
/// # Ok::<(), DecodeError>(())
/// ```
pub trait DecodeView<'a>: Sized {
    /// Construct a borrowed view over `bytes`.
    ///
    /// Must validate SSZ invariants but avoid copying the payload.
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError>;
}

/// A reference to a fixed-length byte array in SSZ encoding.
///
/// This is the zero-copy equivalent of `[u8; N]` or [`FixedBytes<N>`](ssz_primitives::FixedBytes).
///
/// ## Example
///
/// ```rust
/// use ssz::view::{DecodeView, FixedBytesRef};
///
/// let bytes = [0x01, 0x02, 0x03, 0x04];
/// let view = FixedBytesRef::<4>::from_ssz_bytes(&bytes).unwrap();
/// assert_eq!(view.as_bytes(), &bytes);
/// assert_eq!(view.len(), 4);
///
/// // Convert to owned
/// let owned: [u8; 4] = view.to_owned();
/// assert_eq!(owned, bytes);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedBytesRef<'a, const N: usize> {
    /// The underlying byte array reference.
    bytes: &'a [u8; N],
}

impl<'a, const N: usize> FixedBytesRef<'a, N> {
    /// Returns the underlying byte slice.
    pub const fn as_bytes(&self) -> &'a [u8; N] {
        self.bytes
    }

    /// Returns the length of the byte array (always `N`).
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns whether the byte array is empty (always `false` unless `N == 0`).
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Converts this view to an owned array.
    pub const fn to_owned(&self) -> [u8; N] {
        *self.bytes
    }
}

impl<'a, const N: usize> DecodeView<'a> for FixedBytesRef<'a, N> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        if bytes.len() != N {
            Err(DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: N,
            })
        } else {
            Ok(Self { bytes })
        }
    }
}

/// A reference to a variable-length byte sequence in SSZ encoding.
///
/// This is the zero-copy equivalent of [`Vec<u8>`].
///
/// ## Example
///
/// ```rust
/// use ssz::view::{DecodeView, BytesRef};
///
/// let bytes = vec![0x01, 0x02, 0x03];
/// let view = BytesRef::from_ssz_bytes(&bytes).unwrap();
/// assert_eq!(view.as_bytes(), &bytes);
/// assert_eq!(view.len(), 3);
///
/// // Convert to owned
/// let owned: Vec<u8> = view.to_owned();
/// assert_eq!(owned, bytes);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BytesRef<'a> {
    /// The underlying byte slice.
    bytes: &'a [u8],
}

impl<'a> BytesRef<'a> {
    /// Returns the underlying byte slice.
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the length of the byte sequence.
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the byte sequence is empty.
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Converts this view to an owned [`Vec<u8>`].
    pub fn to_owned(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}

impl<'a> DecodeView<'a> for BytesRef<'a> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        // For a raw byte sequence, we just wrap it
        Ok(Self { bytes })
    }
}

/// A reference to a variable-length list in SSZ encoding.
///
/// This is the zero-copy equivalent of [`Vec<T>`] or `VariableList<T, N>`.
///
/// The list provides [`Iterator`]s that decode items lazily without materializing
/// the entire list in memory.
///
/// ## Example
///
/// ```rust
/// use ssz::view::{DecodeView, ListRef};
/// use ssz::Encode;
///
/// // List of u64 values encoded as SSZ
/// let values = vec![1u64, 2, 3, 4];
/// let mut encoded = Vec::new();
/// for v in &values {
///     v.ssz_append(&mut encoded);
/// }
///
/// // Create a view over the encoded bytes (fixed-length items, 8 bytes each)
/// let view = ListRef::<u64>::new(&encoded, true, 8).unwrap();
/// assert_eq!(view.len(), 4);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct ListRef<'a, TRef> {
    /// The underlying byte slice.
    bytes: &'a [u8],

    /// Whether items have fixed length.
    is_fixed_len: bool,

    /// For fixed-length items, the size of each item.
    item_size: usize,

    /// The type of the items.
    _marker: PhantomData<TRef>,
}

impl<'a, TRef> ListRef<'a, TRef> {
    /// Create a new [`ListRef`] from raw bytes and item metadata.
    ///
    /// - `bytes`: the SSZ-encoded list bytes.
    /// - `is_fixed_len`: whether items have fixed length.
    /// - `item_size`: for fixed-length items, the size of each item.
    pub const fn new(
        bytes: &'a [u8],
        is_fixed_len: bool,
        item_size: usize,
    ) -> Result<Self, DecodeError> {
        if is_fixed_len {
            // For fixed-length items, validate that bytes length is a multiple of item_size
            if item_size == 0 {
                return Err(DecodeError::ZeroLengthItem);
            }
            if !bytes.len().is_multiple_of(item_size) {
                return Err(DecodeError::InvalidByteLength {
                    len: bytes.len(),
                    expected: (bytes.len() / item_size) * item_size,
                });
            }
        } else {
            // For variable-length items, validate offset structure
            if !bytes.is_empty() {
                // Must have at least one offset
                if bytes.len() < BYTES_PER_LENGTH_OFFSET {
                    return Err(DecodeError::InvalidListFixedBytesLen(bytes.len()));
                }
                // Validate first offset - it should point to the first byte after all offsets
                // We don't know the exact fixed portion size yet, so we'll validate during iteration
            }
        }
        Ok(Self {
            bytes,
            is_fixed_len,
            item_size,
            _marker: PhantomData,
        })
    }

    /// Returns the number of items in the list.
    pub fn len(&self) -> usize {
        if self.bytes.is_empty() {
            return 0;
        }
        if self.is_fixed_len {
            self.bytes.len() / self.item_size
        } else {
            // Count offsets in the fixed portion
            let first_offset = read_offset(self.bytes).unwrap_or(0);
            if first_offset == 0 || !first_offset.is_multiple_of(BYTES_PER_LENGTH_OFFSET) {
                return 0;
            }
            first_offset / BYTES_PER_LENGTH_OFFSET
        }
    }

    /// Returns whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the underlying bytes.
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }
}

impl<'a, TRef> ListRef<'a, TRef> {
    /// Returns an [`Iterator`] over the list items.
    ///
    /// Each item is decoded lazily as the iterator advances.
    pub fn iter(&self) -> ListRefIter<'a, TRef>
    where
        TRef: DecodeView<'a>,
    {
        ListRefIter {
            list: ListRef {
                bytes: self.bytes,
                is_fixed_len: self.is_fixed_len,
                item_size: self.item_size,
                _marker: PhantomData,
            },
            index: 0,
        }
    }
}

/// Iterator over items in a [`ListRef`].
#[derive(Debug, Clone)]
pub struct ListRefIter<'a, TRef> {
    /// The underlying list.
    list: ListRef<'a, TRef>,

    /// The current index.
    index: usize,
}

impl<'a, TRef: DecodeView<'a>> Iterator for ListRefIter<'a, TRef> {
    type Item = Result<TRef, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.list.len() {
            return None;
        }

        let result = if self.list.is_fixed_len {
            // Fixed-length items: direct indexing
            let start = self.index * self.list.item_size;
            let end = start + self.list.item_size;
            let item_bytes = &self.list.bytes[start..end];
            TRef::from_ssz_bytes(item_bytes)
        } else {
            // Variable-length items: walk offsets
            let first_offset = match read_offset(self.list.bytes) {
                Ok(offset) => offset,
                Err(e) => return Some(Err(e)),
            };

            let current_offset_pos = self.index * BYTES_PER_LENGTH_OFFSET;
            let current_offset = match read_offset(&self.list.bytes[current_offset_pos..]) {
                Ok(offset) => offset,
                Err(e) => return Some(Err(e)),
            };

            let next_offset = if self.index + 1 < self.list.len() {
                let next_offset_pos = (self.index + 1) * BYTES_PER_LENGTH_OFFSET;
                match read_offset(&self.list.bytes[next_offset_pos..]) {
                    Ok(offset) => offset,
                    Err(e) => return Some(Err(e)),
                }
            } else {
                self.list.bytes.len()
            };

            // Validate offsets
            // For the first item, validate against the fixed portion size
            // For subsequent items, validate against the previous offset
            let prev_offset = if self.index == 0 {
                None
            } else {
                let prev_offset_pos = (self.index - 1) * BYTES_PER_LENGTH_OFFSET;
                match read_offset(&self.list.bytes[prev_offset_pos..]) {
                    Ok(offset) => Some(offset),
                    Err(e) => return Some(Err(e)),
                }
            };

            let num_fixed = if self.index == 0 {
                Some(first_offset)
            } else {
                None
            };

            if let Err(e) = sanitize_offset(
                current_offset,
                prev_offset,
                self.list.bytes.len(),
                num_fixed,
            ) {
                return Some(Err(e));
            }

            if next_offset < current_offset || next_offset > self.list.bytes.len() {
                return Some(Err(DecodeError::OffsetsAreDecreasing(next_offset)));
            }

            let item_bytes = &self.list.bytes[current_offset..next_offset];
            TRef::from_ssz_bytes(item_bytes)
        };

        self.index += 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.list.len().saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<'a, TRef: DecodeView<'a>> ExactSizeIterator for ListRefIter<'a, TRef> {
    fn len(&self) -> usize {
        self.list.len().saturating_sub(self.index)
    }
}

/// A reference to a fixed-length vector in SSZ encoding.
///
/// This is the zero-copy equivalent of `FixedVector<T, N>` or `[T; N]`.
///
/// ## Example
///
/// ```rust
/// use ssz::view::{DecodeView, VectorRef};
/// use ssz::Encode;
///
/// // Fixed vector of 4 u64 values encoded as SSZ
/// let values = [1u64, 2, 3, 4];
/// let mut encoded = Vec::new();
/// for v in &values {
///     v.ssz_append(&mut encoded);
/// }
///
/// // Create a view over the encoded bytes (fixed-length items, 8 bytes each)
/// let view = VectorRef::<u64, 4>::new(&encoded, true, 8).unwrap();
/// assert_eq!(view.len(), 4);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct VectorRef<'a, TRef, const N: usize> {
    /// The underlying byte slice.
    bytes: &'a [u8],

    /// Whether items have fixed length.
    is_fixed_len: bool,

    /// For fixed-length items, the size of each item.
    item_size: usize,

    /// The type of the items.
    _marker: PhantomData<TRef>,
}

impl<'a, TRef, const N: usize> VectorRef<'a, TRef, N> {
    /// Create a new [`VectorRef`] from raw bytes and item metadata.
    pub fn new(bytes: &'a [u8], is_fixed_len: bool, item_size: usize) -> Result<Self, DecodeError> {
        if is_fixed_len {
            if item_size == 0 {
                return Err(DecodeError::ZeroLengthItem);
            }
            let expected_len = N * item_size;
            if bytes.len() != expected_len {
                return Err(DecodeError::InvalidByteLength {
                    len: bytes.len(),
                    expected: expected_len,
                });
            }
        } else {
            // For variable-length items, validate offset structure
            if N > 0 {
                if bytes.len() < N * BYTES_PER_LENGTH_OFFSET {
                    return Err(DecodeError::InvalidByteLength {
                        len: bytes.len(),
                        expected: N * BYTES_PER_LENGTH_OFFSET,
                    });
                }
                // Validate first offset
                let first_offset = read_offset(bytes)?;
                let expected_first = N * BYTES_PER_LENGTH_OFFSET;
                sanitize_offset(first_offset, None, bytes.len(), Some(expected_first))?;
            }
        }
        Ok(Self {
            bytes,
            is_fixed_len,
            item_size,
            _marker: PhantomData,
        })
    }

    /// Returns the length of the vector (always `N`).
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns whether the vector is empty (always `false` unless `N == 0`).
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Returns the underlying bytes.
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }
}

impl<'a, TRef, const N: usize> VectorRef<'a, TRef, N> {
    /// Returns an [`Iterator`] over the vector items.
    pub fn iter(&self) -> VectorRefIter<'a, TRef, N>
    where
        TRef: DecodeView<'a>,
    {
        VectorRefIter {
            vector: VectorRef {
                bytes: self.bytes,
                is_fixed_len: self.is_fixed_len,
                item_size: self.item_size,
                _marker: PhantomData,
            },
            index: 0,
        }
    }

    /// Gets the item at the specified index.
    pub fn get(&self, index: usize) -> Result<TRef, DecodeError>
    where
        TRef: DecodeView<'a>,
    {
        if index >= N {
            return Err(DecodeError::OutOfBoundsByte { i: index });
        }

        if self.is_fixed_len {
            let start = index * self.item_size;
            let end = start + self.item_size;
            let item_bytes = &self.bytes[start..end];
            TRef::from_ssz_bytes(item_bytes)
        } else {
            let first_offset = read_offset(self.bytes)?;
            let current_offset_pos = index * BYTES_PER_LENGTH_OFFSET;
            let current_offset = read_offset(&self.bytes[current_offset_pos..])?;

            let next_offset = if index + 1 < N {
                let next_offset_pos = (index + 1) * BYTES_PER_LENGTH_OFFSET;
                read_offset(&self.bytes[next_offset_pos..])?
            } else {
                self.bytes.len()
            };

            sanitize_offset(current_offset, None, self.bytes.len(), Some(first_offset))?;

            if next_offset < current_offset || next_offset > self.bytes.len() {
                return Err(DecodeError::OffsetsAreDecreasing(next_offset));
            }

            let item_bytes = &self.bytes[current_offset..next_offset];
            TRef::from_ssz_bytes(item_bytes)
        }
    }
}

/// Iterator over items in a [`VectorRef`].
#[derive(Debug, Clone)]
pub struct VectorRefIter<'a, TRef, const N: usize> {
    /// The underlying vector.
    vector: VectorRef<'a, TRef, N>,

    /// The current index.
    index: usize,
}

impl<'a, TRef: DecodeView<'a>, const N: usize> Iterator for VectorRefIter<'a, TRef, N> {
    type Item = Result<TRef, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= N {
            return None;
        }
        let result = self.vector.get(self.index);
        self.index += 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = N.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<'a, TRef: DecodeView<'a>, const N: usize> ExactSizeIterator for VectorRefIter<'a, TRef, N> {
    fn len(&self) -> usize {
        N.saturating_sub(self.index)
    }
}

/// A reference to a union (tagged enum) in SSZ encoding.
///
/// This is the zero-copy equivalent of a Rust enum with SSZ union encoding.
///
/// ## Example
///
/// ```rust
/// use ssz::view::{DecodeView, UnionRef, BytesRef};
///
/// // A union with selector 0 and a 3-byte payload
/// let bytes = vec![0u8, 0x01, 0x02, 0x03];
/// let view = UnionRef::<BytesRef<'_>>::from_ssz_bytes(&bytes).unwrap();
/// assert_eq!(u8::from(view.selector()), 0);
/// assert_eq!(view.body().unwrap().as_bytes(), &[0x01, 0x02, 0x03]);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct UnionRef<'a, VRef> {
    /// The union selector.
    selector: UnionSelector,

    /// The body bytes.
    body: &'a [u8],

    /// The type of the body.
    _marker: PhantomData<VRef>,
}

impl<'a, VRef> UnionRef<'a, VRef> {
    /// Returns the union selector (discriminant).
    pub const fn selector(&self) -> UnionSelector {
        self.selector
    }

    /// Returns the raw body bytes (without the selector byte).
    pub const fn body_bytes(&self) -> &'a [u8] {
        self.body
    }
}

impl<'a, VRef: DecodeView<'a>> UnionRef<'a, VRef> {
    /// Decodes and returns the body as the specified view type.
    pub fn body(&self) -> Result<VRef, DecodeError> {
        VRef::from_ssz_bytes(self.body)
    }
}

impl<'a, VRef> DecodeView<'a> for UnionRef<'a, VRef> {
    /// Decodes the union from the given bytes.
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        let (selector, body) = split_union_bytes(bytes)?;
        Ok(Self {
            selector,
            body,
            _marker: PhantomData,
        })
    }
}

/// Helper const function to compute the number of bytes needed for N bits.
///
/// This function is public because it's used in type signatures with `generic_const_exprs`.
pub const fn bytes_for_bits(bits: usize) -> usize {
    if bits == 0 {
        1
    } else {
        bits.div_ceil(8)
    }
}

/// A reference to a fixed-length bitvector in SSZ encoding.
///
/// This is the zero-copy equivalent of [`BitVector<N>`](crate::BitVector).
///
/// Note: Uses the unstable `generic_const_exprs` feature to compute the array size
/// from the bit count at compile time: `&[u8; bytes_for_bits(N)]`
///
/// ## Example
///
/// ```rust
/// use ssz::view::{DecodeView, BitVectorRef};
///
/// // A bitvector with 8 bits: [true, false, true, false, true, false, true, false]
/// let bytes = [0b01010101u8];
/// let view = BitVectorRef::<8>::from_ssz_bytes(&bytes).unwrap();
/// assert_eq!(view.len(), 8);
/// assert_eq!(view.get(0).unwrap(), true);
/// assert_eq!(view.get(1).unwrap(), false);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitVectorRef<'a, const N: usize>
where
    [(); bytes_for_bits(N)]:,
{
    /// The underlying byte array reference.
    /// Size is computed from N bits: `bytes_for_bits(N)` = `(N + 7) / 8` (or 1 if N == 0).
    /// Public to allow external crates (like tree_hash) to access when using generic_const_exprs.
    pub bytes: &'a [u8; bytes_for_bits(N)],
}

impl<'a, const N: usize> BitVectorRef<'a, N>
where
    [(); bytes_for_bits(N)]:,
{
    /// Returns the underlying byte array.
    pub const fn as_bytes(&self) -> &'a [u8; bytes_for_bits(N)] {
        self.bytes
    }

    /// Returns the number of bits in the bitvector (always `N`).
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns whether the bitvector is empty (always `false` unless `N == 0`).
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Gets the value of the bit at the specified index.
    pub fn get(&self, index: usize) -> Result<bool, DecodeError> {
        if index >= N {
            return Err(DecodeError::OutOfBoundsByte { i: index });
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        let byte = self.bytes[byte_index];
        Ok((byte & (1 << bit_index)) != 0)
    }

    /// Returns an iterator over the bits.
    pub fn iter(&self) -> BitVectorRefIter<'a, N> {
        BitVectorRefIter {
            bitvector: *self,
            index: 0,
        }
    }

    /// Converts this view to an owned [`BitVector<N>`].
    pub fn to_owned(&self) -> BitVector<N> {
        BitVector::<N>::from_ssz_bytes(self.bytes.as_slice()).expect("BitVectorRef is always valid")
    }
}

impl<'a, const N: usize> DecodeView<'a> for BitVectorRef<'a, N>
where
    [(); bytes_for_bits(N)]:,
{
    /// Decodes the bitvector from the given bytes.
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        let expected_bytes = bytes_for_bits(N);
        
        // Convert to fixed-size array reference
        let bytes: &'a [u8; bytes_for_bits(N)] = bytes.try_into().map_err(|_| DecodeError::InvalidByteLength {
            len: bytes.len(),
            expected: expected_bytes,
        })?;

        // Validate that excess bits are zero
        if N > 0 {
            let last_byte = bytes[expected_bytes - 1];
            let used_bits = N % 8;
            if used_bits != 0 {
                let mask = (1u8 << used_bits) - 1;
                if (last_byte & !mask) != 0 {
                    return Err(DecodeError::BytesInvalid(
                        "BitVector has excess bits set".to_string(),
                    ));
                }
            }
        }

        Ok(Self { bytes })
    }
}

/// Iterator over bits in a [`BitVectorRef`].
#[derive(Debug, Clone)]
pub struct BitVectorRefIter<'a, const N: usize>
where
    [(); bytes_for_bits(N)]:,
{
    /// The underlying bitvector.
    bitvector: BitVectorRef<'a, N>,

    /// The current index.
    index: usize,
}

impl<'a, const N: usize> Iterator for BitVectorRefIter<'a, N>
where
    [(); bytes_for_bits(N)]:,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= N {
            return None;
        }
        let result = self.bitvector.get(self.index).ok()?;
        self.index += 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = N.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<'a, const N: usize> ExactSizeIterator for BitVectorRefIter<'a, N>
where
    [(); bytes_for_bits(N)]:,
{
    fn len(&self) -> usize {
        N.saturating_sub(self.index)
    }
}

/// A reference to a variable-length bitlist in SSZ encoding.
///
/// This is the zero-copy equivalent of [`BitList<N>`].
///
/// ## Example
///
/// ```rust
/// use ssz::view::{DecodeView, BitListRef};
/// use ssz::{Encode, BitList};
///
/// // Create a bitlist with 5 bits
/// let mut bitlist = BitList::<8>::with_capacity(5).unwrap();
/// bitlist.set(0, true).unwrap();
/// bitlist.set(2, true).unwrap();
/// bitlist.set(4, true).unwrap();
/// let encoded = bitlist.as_ssz_bytes();
///
/// let view = BitListRef::<8>::from_ssz_bytes(&encoded).unwrap();
/// assert_eq!(view.len(), 5);
/// assert_eq!(view.get(0).unwrap(), true);
/// assert_eq!(view.get(1).unwrap(), false);
/// assert_eq!(view.get(2).unwrap(), true);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitListRef<'a, const N: usize> {
    bytes: &'a [u8],
    bit_len: usize,
}

impl<'a, const N: usize> BitListRef<'a, N> {
    /// Returns the underlying byte slice (including the length bit).
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the number of bits in the bitlist (excluding the length bit).
    pub const fn len(&self) -> usize {
        self.bit_len
    }

    /// Returns whether the bitlist is empty.
    pub const fn is_empty(&self) -> bool {
        self.bit_len == 0
    }

    /// Gets the value of the bit at the specified index.
    pub const fn get(&self, index: usize) -> Result<bool, DecodeError> {
        if index >= self.bit_len {
            return Err(DecodeError::OutOfBoundsByte { i: index });
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        let byte = self.bytes[byte_index];
        Ok((byte & (1 << bit_index)) != 0)
    }

    /// Returns an [`Iterator`] over the bits.
    pub const fn iter(&self) -> BitListRefIter<'a, N> {
        BitListRefIter {
            bitlist: *self,
            index: 0,
        }
    }

    /// Converts this view to an owned [`BitList<N>`].
    pub fn to_owned(&self) -> BitList<N> {
        BitList::<N>::from_ssz_bytes(self.bytes).expect("BitListRef is always valid")
    }
}

impl<'a, const N: usize> DecodeView<'a> for BitListRef<'a, N> {
    /// Decodes the bitlist from the given bytes.
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        if bytes.is_empty() {
            return Err(DecodeError::InvalidByteLength {
                len: 0,
                expected: 1,
            });
        }

        // Find the length bit (highest set bit)
        let mut bit_len = None;
        for (byte_idx, &byte) in bytes.iter().enumerate().rev() {
            if byte != 0 {
                let highest_bit = 7 - byte.leading_zeros() as usize;
                bit_len = Some(byte_idx * 8 + highest_bit);
                break;
            }
        }

        let bit_len = bit_len.ok_or(DecodeError::BytesInvalid(
            "BitList missing length information".to_string(),
        ))?;

        // The length bit should be in the last byte
        let expected_byte_len = bit_len / 8 + 1;
        if bytes.len() != expected_byte_len {
            return Err(DecodeError::BytesInvalid(format!(
                "BitList has incorrect byte count: {} expected {}",
                bytes.len(),
                expected_byte_len
            )));
        }

        // The actual data length is bit_len - 1 (excluding the length bit)
        let data_len = bit_len;
        if data_len > N {
            return Err(DecodeError::BytesInvalid(format!(
                "BitList length {} exceeds maximum {}",
                data_len, N
            )));
        }

        Ok(Self {
            bytes,
            bit_len: data_len,
        })
    }
}

/// Iterator over bits in a [`BitListRef`].
#[derive(Debug, Clone)]
pub struct BitListRefIter<'a, const N: usize> {
    /// The underlying bitlist.
    bitlist: BitListRef<'a, N>,

    /// The current index.
    index: usize,
}

impl<'a, const N: usize> Iterator for BitListRefIter<'a, N> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.bitlist.bit_len {
            return None;
        }
        let result = self.bitlist.get(self.index).ok()?;
        self.index += 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.bitlist.bit_len.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<'a, const N: usize> ExactSizeIterator for BitListRefIter<'a, N> {
    fn len(&self) -> usize {
        self.bitlist.bit_len.saturating_sub(self.index)
    }
}

// DecodeView implementations for primitive types
// These types are cheap to copy, so we just decode them into values

macro_rules! impl_decode_view_for_primitive {
    ($type:ty) => {
        impl<'a> DecodeView<'a> for $type {
            fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
                crate::Decode::from_ssz_bytes(bytes)
            }
        }
    };
}

impl_decode_view_for_primitive!(u8);
impl_decode_view_for_primitive!(u16);
impl_decode_view_for_primitive!(u32);
impl_decode_view_for_primitive!(u64);
impl_decode_view_for_primitive!(u128);
impl_decode_view_for_primitive!(usize);
impl_decode_view_for_primitive!(bool);

// Implement DecodeView for ssz_primitives types
impl<'a, const N: usize> DecodeView<'a> for FixedBytes<N> {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        Decode::from_ssz_bytes(bytes)
    }
}

impl<'a> DecodeView<'a> for U256 {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        Decode::from_ssz_bytes(bytes)
    }
}

impl<'a> DecodeView<'a> for U128 {
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        Decode::from_ssz_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Encode;

    /// Helper function to encode a list for testing.
    pub(crate) fn encode_list<T: Encode>(items: &[T]) -> Vec<u8> {
        let mut buf = Vec::new();
        for item in items {
            item.ssz_append(&mut buf);
        }
        buf
    }

    #[test]
    fn fixed_bytes_ref_basic() {
        let bytes = [0x01, 0x02, 0x03, 0x04];
        let view = FixedBytesRef::<4>::from_ssz_bytes(&bytes).unwrap();
        assert_eq!(view.as_bytes(), &bytes);
        assert_eq!(view.len(), 4);
        assert!(!view.is_empty());
        assert_eq!(view.to_owned(), bytes);
    }

    #[test]
    fn fixed_bytes_ref_wrong_length() {
        let bytes = [0x01, 0x02, 0x03];
        let result = FixedBytesRef::<4>::from_ssz_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn bytes_ref_basic() {
        let bytes = vec![0x01, 0x02, 0x03];
        let view = BytesRef::from_ssz_bytes(&bytes).unwrap();
        assert_eq!(view.as_bytes(), &bytes);
        assert_eq!(view.len(), 3);
        assert!(!view.is_empty());
        assert_eq!(view.to_owned(), bytes);
    }

    #[test]
    fn bytes_ref_empty() {
        let bytes: &[u8] = &[];
        let view = BytesRef::from_ssz_bytes(bytes).unwrap();
        assert_eq!(view.as_bytes(), bytes);
        assert_eq!(view.len(), 0);
        assert!(view.is_empty());
    }

    #[test]
    fn list_ref_fixed_u64() {
        // List of u64 values: [1, 2, 3, 4]
        let values = vec![1u64, 2, 3, 4];
        let encoded = encode_list(&values);

        let view = ListRef::<u64>::new(&encoded, true, 8).unwrap();
        assert_eq!(view.len(), 4);
        assert!(!view.is_empty());

        let decoded: Vec<u64> = view.iter().map(|r| r.unwrap()).collect();
        assert_eq!(decoded, values);
    }

    #[test]
    fn list_ref_empty() {
        let bytes: &[u8] = &[];
        let view = ListRef::<u64>::new(bytes, true, 8).unwrap();
        assert_eq!(view.len(), 0);
        assert!(view.is_empty());
    }

    #[test]
    fn vector_ref_fixed_u64() {
        // Vector of 4 u64 values: [1, 2, 3, 4]
        let values = [1u64, 2, 3, 4];
        let encoded = encode_list(&values);

        let view = VectorRef::<u64, 4>::new(&encoded, true, 8).unwrap();
        assert_eq!(view.len(), 4);
        assert!(!view.is_empty());

        for (i, result) in view.iter().enumerate() {
            assert_eq!(result.unwrap(), values[i]);
        }

        assert_eq!(view.get(0).unwrap(), 1);
        assert_eq!(view.get(3).unwrap(), 4);
        assert!(view.get(4).is_err());
    }

    #[test]
    fn union_ref_basic() {
        // Union with selector 0 and a 3-byte payload
        let bytes = vec![0u8, 0x01, 0x02, 0x03];
        let view = UnionRef::<BytesRef<'_>>::from_ssz_bytes(&bytes).unwrap();
        assert_eq!(u8::from(view.selector()), 0);
        assert_eq!(view.body_bytes(), &[0x01, 0x02, 0x03]);
        assert_eq!(view.body().unwrap().as_bytes(), &[0x01, 0x02, 0x03]);
    }

    #[test]
    fn bitvector_ref_basic() {
        use crate::BitVector;

        // Create a bitvector: [true, false, true, false, true, false, true, false]
        let mut bv = BitVector::<8>::new();
        bv.set(0, true).unwrap();
        bv.set(2, true).unwrap();
        bv.set(4, true).unwrap();
        bv.set(6, true).unwrap();
        let encoded = bv.as_ssz_bytes();

        let view = BitVectorRef::<8>::from_ssz_bytes(&encoded).unwrap();
        assert_eq!(view.len(), 8);
        assert!(!view.is_empty());
        assert!(view.get(0).unwrap());
        assert!(!view.get(1).unwrap());
        assert!(view.get(2).unwrap());
        assert!(!view.get(3).unwrap());

        let bits: Vec<bool> = view.iter().collect();
        assert_eq!(
            bits,
            vec![true, false, true, false, true, false, true, false]
        );

        // Test to_owned
        let owned = view.to_owned();
        assert_eq!(owned, bv);
    }

    #[test]
    fn bitlist_ref_basic() {
        use crate::BitList;

        // Create a bitlist with 5 bits
        let mut bl = BitList::<8>::with_capacity(5).unwrap();
        bl.set(0, true).unwrap();
        bl.set(2, true).unwrap();
        bl.set(4, true).unwrap();
        let encoded = bl.as_ssz_bytes();

        let view = BitListRef::<8>::from_ssz_bytes(&encoded).unwrap();
        assert_eq!(view.len(), 5);
        assert!(!view.is_empty());
        assert!(view.get(0).unwrap());
        assert!(!view.get(1).unwrap());
        assert!(view.get(2).unwrap());
        assert!(!view.get(3).unwrap());
        assert!(view.get(4).unwrap());

        let bits: Vec<bool> = view.iter().collect();
        assert_eq!(bits, vec![true, false, true, false, true]);

        // Test to_owned
        let owned = view.to_owned();
        assert_eq!(owned, bl);
    }

    #[test]
    fn bitlist_ref_empty() {
        use crate::BitList;

        let bl = BitList::<8>::with_capacity(0).unwrap();
        let encoded = bl.as_ssz_bytes();

        let view = BitListRef::<8>::from_ssz_bytes(&encoded).unwrap();
        assert_eq!(view.len(), 0);
        assert!(view.is_empty());
    }

    #[test]
    fn decode_view_primitives() {
        // Test u8
        let val = 42u8;
        let encoded = val.as_ssz_bytes();
        assert_eq!(<u8 as DecodeView>::from_ssz_bytes(&encoded).unwrap(), val);

        // Test u16
        let val = 1234u16;
        let encoded = val.as_ssz_bytes();
        assert_eq!(<u16 as DecodeView>::from_ssz_bytes(&encoded).unwrap(), val);

        // Test u32
        let val = 123456u32;
        let encoded = val.as_ssz_bytes();
        assert_eq!(<u32 as DecodeView>::from_ssz_bytes(&encoded).unwrap(), val);

        // Test u64
        let val = 123456789u64;
        let encoded = val.as_ssz_bytes();
        assert_eq!(<u64 as DecodeView>::from_ssz_bytes(&encoded).unwrap(), val);

        // Test u128
        let val = 123456789123456789u128;
        let encoded = val.as_ssz_bytes();
        assert_eq!(<u128 as DecodeView>::from_ssz_bytes(&encoded).unwrap(), val);
    }

    #[test]
    fn ssz_decoder_decode_next_view() {
        use crate::SszDecoderBuilder;

        // Test decoding a BytesRef
        let bytes = vec![4u8, 0, 0, 0, 0x01, 0x02, 0x03, 0x04];
        let mut builder = SszDecoderBuilder::new(&bytes);
        builder.register_anonymous_variable_length_item().unwrap();
        let mut decoder = builder.build().unwrap();

        let view: BytesRef<'_> = decoder.decode_next_view().unwrap();
        assert_eq!(view.as_bytes(), &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn list_ref_variable_length_items() {
        // Test list of variable-length byte sequences
        // Each inner vec is [len, ...bytes]
        let items = vec![vec![0x01, 0x02], vec![0x03, 0x04, 0x05], vec![0x06]];

        // Encode as SSZ list
        // Fixed portion: 3 offsets (12 bytes)
        // Variable portion: 2 + 3 + 1 = 6 bytes
        let first_offset = 12u32;
        let second_offset = 14u32;
        let third_offset = 17u32;

        let mut encoded = Vec::new();
        encoded.extend_from_slice(&first_offset.to_le_bytes());
        encoded.extend_from_slice(&second_offset.to_le_bytes());
        encoded.extend_from_slice(&third_offset.to_le_bytes());
        encoded.extend_from_slice(&items[0]);
        encoded.extend_from_slice(&items[1]);
        encoded.extend_from_slice(&items[2]);

        let view = ListRef::<BytesRef<'_>>::new(&encoded, false, 0).unwrap();
        assert_eq!(view.len(), 3);

        let decoded: Vec<Vec<u8>> = view.iter().map(|r| r.unwrap().to_owned()).collect();
        assert_eq!(decoded, items);
    }

    #[test]
    fn vector_ref_wrong_length() {
        // Vector with wrong number of items
        let values = [1u64, 2, 3]; // Only 3 items
        let encoded = encode_list(&values);

        let result = VectorRef::<u64, 4>::new(&encoded, true, 8);
        assert!(result.is_err());
    }

    #[test]
    fn bitvector_ref_excess_bits() {
        // Bitvector with excess bits set should fail
        let bytes = [0b1111_1111u8]; // All bits set, but only 4 should be valid
        let result = BitVectorRef::<4>::from_ssz_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn bitlist_ref_wrong_byte_count() {
        // BitList with too many bytes
        let bytes = vec![0b0000_0001, 0b0000_0000]; // Length bit at position 0, but 2 bytes
        let result = BitListRef::<16>::from_ssz_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn bitlist_ref_exceeds_max() {
        use crate::BitList;

        // Create a bitlist that's too long for the max
        let mut bl = BitList::<16>::with_capacity(9).unwrap();
        for i in 0..9 {
            bl.set(i, true).unwrap();
        }
        let encoded = bl.as_ssz_bytes();

        // Try to decode as BitList<8> (max 8 bits)
        let result = BitListRef::<8>::from_ssz_bytes(&encoded);
        assert!(result.is_err());
    }

    #[test]
    fn round_trip_fixed_bytes() {
        // Test that view decoding matches owned decoding
        let original = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let view = FixedBytesRef::<8>::from_ssz_bytes(&original).unwrap();
        let owned: [u8; 8] = crate::Decode::from_ssz_bytes(&original).unwrap();
        assert_eq!(view.to_owned(), owned);
    }

    #[test]
    fn round_trip_bitvector() {
        use crate::BitVector;

        for n in 0..32 {
            let mut bv = BitVector::<32>::new();
            for i in 0..n {
                if i % 2 == 0 {
                    bv.set(i, true).unwrap();
                }
            }
            let encoded = bv.as_ssz_bytes();

            let view = BitVectorRef::<32>::from_ssz_bytes(&encoded).unwrap();
            let owned: BitVector<32> = crate::Decode::from_ssz_bytes(&encoded).unwrap();

            for i in 0..32 {
                assert_eq!(view.get(i).unwrap(), owned.get(i).unwrap());
            }
        }
    }

    #[test]
    fn round_trip_bitlist() {
        use crate::BitList;

        for n in 0..16 {
            let mut bl = BitList::<32>::with_capacity(n).unwrap();
            for i in 0..n {
                if i % 3 == 0 {
                    bl.set(i, true).unwrap();
                }
            }
            let encoded = bl.as_ssz_bytes();

            let view = BitListRef::<32>::from_ssz_bytes(&encoded).unwrap();
            let owned: BitList<32> = crate::Decode::from_ssz_bytes(&encoded).unwrap();

            assert_eq!(view.len(), owned.len());
            for i in 0..n {
                assert_eq!(view.get(i).unwrap(), owned.get(i).unwrap());
            }
        }
    }

    #[test]
    fn list_ref_iterator_exact_size() {
        let values = vec![1u64, 2, 3, 4, 5];
        let encoded = encode_list(&values);
        let view = ListRef::<u64>::new(&encoded, true, 8).unwrap();

        let mut iter = view.iter();
        assert_eq!(iter.len(), 5);

        iter.next();
        assert_eq!(iter.len(), 4);

        iter.next();
        iter.next();
        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn vector_ref_iterator_exact_size() {
        let values = [1u64, 2, 3];
        let encoded = encode_list(&values);
        let view = VectorRef::<u64, 3>::new(&encoded, true, 8).unwrap();

        let mut iter = view.iter();
        assert_eq!(iter.len(), 3);

        iter.next();
        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn bitvector_ref_iterator_exact_size() {
        use crate::BitVector;

        let bv = BitVector::<8>::new();
        let encoded = bv.as_ssz_bytes();
        let view = BitVectorRef::<8>::from_ssz_bytes(&encoded).unwrap();

        let mut iter = view.iter();
        assert_eq!(iter.len(), 8);

        iter.next();
        iter.next();
        assert_eq!(iter.len(), 6);
    }

    #[test]
    fn bitlist_ref_iterator_exact_size() {
        use crate::BitList;

        let bl = BitList::<16>::with_capacity(5).unwrap();
        let encoded = bl.as_ssz_bytes();
        let view = BitListRef::<16>::from_ssz_bytes(&encoded).unwrap();

        let mut iter = view.iter();
        assert_eq!(iter.len(), 5);

        iter.next();
        iter.next();
        iter.next();
        assert_eq!(iter.len(), 2);
    }
}
