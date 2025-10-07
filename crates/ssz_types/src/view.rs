//! Zero-copy view types for SSZ types.
//!
//! This module provides zero-copy reference-backed views for [`VariableList`] and [`FixedVector`].
//! These views allow decoding SSZ-encoded data without allocating or copying the underlying bytes.
//!
//! ## Example
//!
//! ```rust
//! use ssz::Encode;
//! use ssz::view::DecodeView;
//! use ssz_types::view::VariableListRef;
//!
//! // Encode a list of u64 values
//! let values = vec![1u64, 2, 3, 4];
//! let mut encoded = Vec::new();
//! for v in &values {
//!     v.ssz_append(&mut encoded);
//! }
//!
//! // Create a zero-copy view
//! let view = VariableListRef::<u64, 10>::from_ssz_bytes(&encoded).unwrap();
//! assert_eq!(view.len(), 4);
//!
//! // Convert to owned when needed
//! let owned = view.to_owned().unwrap();
//! assert_eq!(&owned[..], &values[..]);
//! ```

use crate::{Error, FixedVector, VariableList};
use ssz::DecodeError;
use ssz::view::{DecodeView, ListRef, VectorRef};
use ssz_primitives::{FixedBytes, U128, U256};
use tree_hash::{PackedEncoding, TreeHash, TreeHashDigest, TreeHashType};

/// A zero-copy reference to a [`VariableList<T, N>`](VariableList).
///
/// This type provides a borrowed view over an SSZ-encoded variable-length list without
/// allocating or copying the underlying bytes.
///
/// ## Example
///
/// ```rust
/// use ssz::Encode;
/// use ssz::view::DecodeView;
/// use ssz_types::view::VariableListRef;
/// use ssz_types::VariableList;
///
/// // Encode a variable list
/// let list: VariableList<u64, 10> = vec![1, 2, 3].into();
/// let encoded = list.as_ssz_bytes();
///
/// // Create a zero-copy view
/// let view = VariableListRef::<u64, 10>::from_ssz_bytes(&encoded).unwrap();
/// assert_eq!(view.len(), 3);
///
/// // Iterate without allocations
/// for item in view.iter() {
///     let value = item.unwrap();
///     println!("Value: {}", value);
/// }
///
/// // Convert to owned
/// let owned = view.to_owned().unwrap();
/// assert_eq!(list, owned);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct VariableListRef<'a, TRef, const N: usize> {
    /// The underlying list reference.
    inner: ListRef<'a, TRef>,
}

impl<'a, TRef, const N: usize> VariableListRef<'a, TRef, N> {
    /// Creates a new `VariableListRef` from raw SSZ bytes.
    ///
    /// - `bytes`: the SSZ-encoded list bytes.
    /// - `is_fixed_len`: whether items have fixed length.
    /// - `item_size`: for fixed-length items, the size of each item.
    pub fn new(bytes: &'a [u8], is_fixed_len: bool, item_size: usize) -> Result<Self, DecodeError> {
        let inner = ListRef::new(bytes, is_fixed_len, item_size)?;

        // Validate that the list doesn't exceed the maximum length
        if inner.len() > N {
            return Err(DecodeError::BytesInvalid(format!(
                "VariableList length {} exceeds maximum {}",
                inner.len(),
                N
            )));
        }

        Ok(Self { inner })
    }

    /// Returns the number of items in the list.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the maximum length (type-level constant `N`).
    pub const fn max_len() -> usize {
        N
    }

    /// Returns the underlying bytes.
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.inner.as_bytes()
    }

    /// Returns an [`Iterator`] over the list items.
    ///
    /// Each item is decoded lazily as the iterator advances.
    pub fn iter(&self) -> impl Iterator<Item = Result<TRef, DecodeError>> + '_
    where
        TRef: DecodeView<'a>,
    {
        self.inner.iter()
    }
}

impl<'a, TRef, const N: usize> VariableListRef<'a, TRef, N>
where
    TRef: DecodeView<'a>,
{
    /// Converts this view to an owned [`VariableList<T, N>`](VariableList).
    ///
    /// This method collects all items from the iterator and constructs an owned list.
    pub fn to_owned<T>(&self) -> Result<VariableList<T, N>, Error>
    where
        TRef: ToOwnedSsz<T>,
    {
        let items: Result<Vec<T>, _> = self
            .iter()
            .map(|item_result| item_result.map(|item| item.to_owned()))
            .collect();

        let items = items.map_err(|_| Error::OutOfBounds { i: 0, len: N })?;

        VariableList::new(items)
    }
}

/// Convert a view type to its owned equivalent.
///
/// This trait bridges between zero-copy view types (like [`u64`] from [`DecodeView`])
/// and their owned counterparts that can be collected into containers.
pub trait ToOwnedSsz<T> {
    /// Converts this view to an owned value.
    fn to_owned(&self) -> T;
}

/// Implement [`ToOwnedSsz`] for primitive types (they implement [`Copy`]).
macro_rules! impl_to_owned_copy {
    ($($t:ty),*) => {
        $(
            impl ToOwnedSsz<$t> for $t {
                fn to_owned(&self) -> $t {
                    *self
                }
            }
        )*
    };
}

impl_to_owned_copy!(u8, u16, u32, u64, u128, usize, bool);

/// Implement [`ToOwnedSsz`] for [`ssz_primitives`] types.
impl<const N: usize> ToOwnedSsz<FixedBytes<N>> for FixedBytes<N> {
    fn to_owned(&self) -> FixedBytes<N> {
        *self
    }
}

impl ToOwnedSsz<U256> for U256 {
    fn to_owned(&self) -> U256 {
        *self
    }
}

impl ToOwnedSsz<U128> for U128 {
    fn to_owned(&self) -> U128 {
        *self
    }
}

impl<'a, TRef: DecodeView<'a> + SszFixedLen, const N: usize> DecodeView<'a>
    for VariableListRef<'a, TRef, N>
where
    TRef: 'a,
{
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        // Determine if items are fixed-length
        let is_fixed_len = TRef::is_ssz_fixed_len();
        let item_size = if is_fixed_len {
            TRef::ssz_fixed_len()
        } else {
            0
        };

        // For empty lists, ListRef validation requires a non-zero item_size if is_fixed_len is true
        // Since there are no items, we use a placeholder value
        let (effective_is_fixed, effective_size) = if bytes.is_empty() {
            (true, 1) // Use fixed-length with size 1 as placeholder
        } else {
            (is_fixed_len, item_size)
        };

        Self::new(bytes, effective_is_fixed, effective_size)
    }
}

// Helper trait to determine fixed-length status for DecodeView types
trait SszFixedLen {
    fn is_ssz_fixed_len() -> bool;
    fn ssz_fixed_len() -> usize;
}

macro_rules! impl_ssz_fixed_len_primitive {
    ($($t:ty, $size:expr),*) => {
        $(
            impl SszFixedLen for $t {
                fn is_ssz_fixed_len() -> bool {
                    true
                }
                fn ssz_fixed_len() -> usize {
                    $size
                }
            }
        )*
    };
}

impl_ssz_fixed_len_primitive!(u8, 1, u16, 2, u32, 4, u64, 8, u128, 16, usize, 8, bool, 1);

impl<const N: usize> SszFixedLen for FixedBytes<N> {
    fn is_ssz_fixed_len() -> bool {
        true
    }
    fn ssz_fixed_len() -> usize {
        N
    }
}

impl SszFixedLen for U256 {
    fn is_ssz_fixed_len() -> bool {
        true
    }
    fn ssz_fixed_len() -> usize {
        32
    }
}

impl SszFixedLen for U128 {
    fn is_ssz_fixed_len() -> bool {
        true
    }
    fn ssz_fixed_len() -> usize {
        16
    }
}

/// A zero-copy reference to a [`FixedVector<T, N>`](FixedVector).
///
/// This type provides a borrowed view over an SSZ-encoded fixed-length vector without
/// allocating or copying the underlying bytes.
///
/// ## Example
///
/// ```rust
/// use ssz::Encode;
/// use ssz::view::DecodeView;
/// use ssz_types::view::FixedVectorRef;
/// use ssz_types::FixedVector;
///
/// // Encode a fixed vector
/// let vec: FixedVector<u64, 4> = vec![1, 2, 3, 4].into();
/// let encoded = vec.as_ssz_bytes();
///
/// // Create a zero-copy view
/// let view = FixedVectorRef::<u64, 4>::from_ssz_bytes(&encoded).unwrap();
/// assert_eq!(view.len(), 4);
///
/// // Access individual elements
/// assert_eq!(view.get(0).unwrap(), 1);
/// assert_eq!(view.get(3).unwrap(), 4);
///
/// // Convert to owned
/// let owned = view.to_owned().unwrap();
/// assert_eq!(vec, owned);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct FixedVectorRef<'a, TRef, const N: usize> {
    /// The underlying vector reference.
    inner: VectorRef<'a, TRef, N>,
}

impl<'a, TRef, const N: usize> FixedVectorRef<'a, TRef, N> {
    /// Creates a new [`FixedVectorRef`] from raw SSZ bytes.
    ///
    /// - `bytes`: the SSZ-encoded vector bytes.
    /// - `is_fixed_len`: whether items have fixed length.
    /// - `item_size`: for fixed-length items, the size of each item.
    pub fn new(bytes: &'a [u8], is_fixed_len: bool, item_size: usize) -> Result<Self, DecodeError> {
        Ok(Self {
            inner: VectorRef::new(bytes, is_fixed_len, item_size)?,
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
    pub fn as_bytes(&self) -> &'a [u8] {
        self.inner.as_bytes()
    }

    /// Returns an [`Iterator`] over the vector items.
    pub fn iter(&self) -> impl Iterator<Item = Result<TRef, DecodeError>> + '_
    where
        TRef: DecodeView<'a>,
    {
        self.inner.iter()
    }

    /// Gets the item at the specified index.
    pub fn get(&self, index: usize) -> Result<TRef, DecodeError>
    where
        TRef: DecodeView<'a>,
    {
        self.inner.get(index)
    }
}

impl<'a, TRef, const N: usize> FixedVectorRef<'a, TRef, N>
where
    TRef: DecodeView<'a>,
{
    /// Converts this view to an owned [`FixedVector<T, N>`](FixedVector).
    ///
    /// This method collects all items from the iterator and constructs an owned vector.
    pub fn to_owned<T>(&self) -> Result<FixedVector<T, N>, Error>
    where
        TRef: ToOwnedSsz<T>,
    {
        let items: Result<Vec<T>, _> = self
            .iter()
            .map(|item_result| item_result.map(|item| item.to_owned()))
            .collect();

        let items = items.map_err(|_| Error::OutOfBounds { i: 0, len: N })?;

        FixedVector::new(items)
    }
}

impl<'a, TRef: DecodeView<'a> + SszFixedLen, const N: usize> DecodeView<'a>
    for FixedVectorRef<'a, TRef, N>
where
    TRef: 'a,
{
    fn from_ssz_bytes(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        // Determine if items are fixed-length
        let is_fixed_len = TRef::is_ssz_fixed_len();
        let item_size = if is_fixed_len {
            TRef::ssz_fixed_len()
        } else {
            0
        };

        Self::new(bytes, is_fixed_len, item_size)
    }
}

impl<'a, TRef, const N: usize, H> TreeHash<H> for VariableListRef<'a, TRef, N>
where
    TRef: DecodeView<'a> + ToOwnedSsz<TRef>,
    TRef: TreeHash<H>,
    H: TreeHashDigest,
{
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::List
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        unreachable!("List should never be packed.")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("List should never be packed.")
    }

    fn tree_hash_root(&self) -> H::Output {
        let owned = self.to_owned().expect("TreeHash conversion failed");
        owned.tree_hash_root()
    }
}

impl<'a, TRef, const N: usize, H> TreeHash<H> for FixedVectorRef<'a, TRef, N>
where
    TRef: DecodeView<'a> + ToOwnedSsz<TRef>,
    TRef: TreeHash<H>,
    H: TreeHashDigest,
{
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::Vector
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        unreachable!("Vector should never be packed.")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("Vector should never be packed.")
    }

    fn tree_hash_root(&self) -> H::Output {
        let owned = self.to_owned().expect("TreeHash conversion failed");
        owned.tree_hash_root()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ssz::{Decode, Encode};

    #[test]
    fn variable_list_ref_basic() {
        // Create a list of u64 values
        let values = vec![1u64, 2, 3, 4];
        let mut encoded = Vec::new();
        for v in &values {
            v.ssz_append(&mut encoded);
        }

        // Create a view
        let view = VariableListRef::<u64, 10>::from_ssz_bytes(&encoded).unwrap();
        assert_eq!(view.len(), 4);
        assert!(!view.is_empty());
        assert_eq!(VariableListRef::<u64, 10>::max_len(), 10);

        // Check values
        let decoded: Vec<u64> = view.iter().map(|r| r.unwrap()).collect();
        assert_eq!(decoded, values);
    }

    #[test]
    fn variable_list_ref_empty() {
        let bytes: &[u8] = &[];
        let view = VariableListRef::<u64, 10>::from_ssz_bytes(bytes).unwrap();
        assert_eq!(view.len(), 0);
        assert!(view.is_empty());
    }

    #[test]
    fn variable_list_ref_exceeds_max() {
        // Create a list with 5 items
        let values = vec![1u64, 2, 3, 4, 5];
        let mut encoded = Vec::new();
        for v in &values {
            v.ssz_append(&mut encoded);
        }

        // Try to decode as VariableListRef with max 4
        let result = VariableListRef::<u64, 4>::from_ssz_bytes(&encoded);
        assert!(result.is_err());
    }

    #[test]
    fn variable_list_ref_to_owned() {
        let values = vec![1u64, 2, 3];
        let list: VariableList<u64, 10> = values.clone().into();
        let encoded = list.as_ssz_bytes();

        let view = VariableListRef::<u64, 10>::from_ssz_bytes(&encoded).unwrap();
        let owned = view.to_owned().unwrap();

        assert_eq!(list, owned);
    }

    #[test]
    fn fixed_vector_ref_basic() {
        // Create a fixed vector of 4 u64 values
        let values = vec![1u64, 2, 3, 4];
        let vec: FixedVector<u64, 4> = values.clone().into();
        let encoded = vec.as_ssz_bytes();

        // Create a view
        let view = FixedVectorRef::<u64, 4>::from_ssz_bytes(&encoded).unwrap();
        assert_eq!(view.len(), 4);
        assert!(!view.is_empty());

        // Check values
        for (i, result) in view.iter().enumerate() {
            assert_eq!(result.unwrap(), values[i]);
        }

        // Check get
        assert_eq!(view.get(0).unwrap(), 1);
        assert_eq!(view.get(3).unwrap(), 4);
        assert!(view.get(4).is_err());
    }

    #[test]
    fn fixed_vector_ref_wrong_length() {
        // Create a vector with 3 items
        let values = vec![1u64, 2, 3];
        let mut encoded = Vec::new();
        for v in &values {
            v.ssz_append(&mut encoded);
        }

        // Try to decode as FixedVectorRef<_, 4>
        let result = FixedVectorRef::<u64, 4>::from_ssz_bytes(&encoded);
        assert!(result.is_err());
    }

    #[test]
    fn fixed_vector_ref_to_owned() {
        let values = vec![1u64, 2, 3, 4];
        let vec: FixedVector<u64, 4> = values.into();
        let encoded = vec.as_ssz_bytes();

        let view = FixedVectorRef::<u64, 4>::from_ssz_bytes(&encoded).unwrap();
        let owned = view.to_owned().unwrap();

        assert_eq!(vec, owned);
    }

    #[test]
    fn round_trip_variable_list() {
        // Test that view decoding matches owned decoding
        let original: VariableList<u64, 10> = vec![1, 2, 3, 4, 5].into();
        let encoded = original.as_ssz_bytes();

        let view = VariableListRef::<u64, 10>::from_ssz_bytes(&encoded).unwrap();
        let owned = view.to_owned().unwrap();

        assert_eq!(original, owned);
    }

    #[test]
    fn round_trip_fixed_vector() {
        // Test that view decoding matches owned decoding
        let original: FixedVector<u64, 5> = vec![1, 2, 3, 4, 5].into();
        let encoded = original.as_ssz_bytes();

        let view = FixedVectorRef::<u64, 5>::from_ssz_bytes(&encoded).unwrap();
        let owned = view.to_owned().unwrap();

        assert_eq!(original, owned);
    }

    #[test]
    fn variable_list_ref_u8() {
        // Test with u8 (byte list)
        let values = vec![0x01u8, 0x02, 0x03, 0x04];
        let list: VariableList<u8, 10> = values.clone().into();
        let encoded = list.as_ssz_bytes();

        let view = VariableListRef::<u8, 10>::from_ssz_bytes(&encoded).unwrap();
        let decoded: Vec<u8> = view.iter().map(|r| r.unwrap()).collect();
        assert_eq!(decoded, values);
    }

    #[test]
    fn fixed_vector_ref_u16() {
        // Test with u16
        let values = vec![100u16, 200, 300, 400];
        let vec: FixedVector<u16, 4> = values.clone().into();
        let encoded = vec.as_ssz_bytes();

        let view = FixedVectorRef::<u16, 4>::from_ssz_bytes(&encoded).unwrap();
        let decoded: Vec<u16> = view.iter().map(|r| r.unwrap()).collect();
        assert_eq!(decoded, values);
    }

    #[test]
    fn tree_hash_variable_list_ref() {
        use tree_hash::{Sha256Hasher, TreeHash};

        // Test that tree hash of view matches tree hash of owned
        let values = vec![1u64, 2, 3, 4, 5];
        let list: VariableList<u64, 10> = values.into();
        let encoded = list.as_ssz_bytes();

        let view = VariableListRef::<u64, 10>::from_ssz_bytes(&encoded).unwrap();

        let owned_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&list);
        let view_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);

        assert_eq!(owned_hash, view_hash);
    }

    #[test]
    fn tree_hash_fixed_vector_ref() {
        use tree_hash::{Sha256Hasher, TreeHash};

        // Test that tree hash of view matches tree hash of owned
        let values = vec![1u64, 2, 3, 4, 5];
        let vec: FixedVector<u64, 5> = values.into();
        let encoded = vec.as_ssz_bytes();

        let view = FixedVectorRef::<u64, 5>::from_ssz_bytes(&encoded).unwrap();

        let owned_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&vec);
        let view_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);

        assert_eq!(owned_hash, view_hash);
    }

    #[test]
    fn tree_hash_empty_variable_list() {
        use tree_hash::{Sha256Hasher, TreeHash};

        let list: VariableList<u64, 10> = vec![].into();
        let encoded = list.as_ssz_bytes();

        let view = VariableListRef::<u64, 10>::from_ssz_bytes(&encoded).unwrap();

        let owned_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&list);
        let view_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);
        assert_eq!(owned_hash, view_hash);
    }

    // Property tests: verify that view decoding produces equivalent results to owned decoding

    #[test]
    fn property_variable_list_decode_equivalence() {
        // Test various list sizes and verify view produces same results as owned
        for size in [0, 1, 2, 5, 10, 15, 20] {
            let values: Vec<u64> = (0..size).collect();
            let list: VariableList<u64, 32> = values.clone().into();
            let encoded = list.as_ssz_bytes();

            // Decode as owned
            let owned = VariableList::<u64, 32>::from_ssz_bytes(&encoded).unwrap();

            // Decode as view
            let view = VariableListRef::<u64, 32>::from_ssz_bytes(&encoded).unwrap();
            let view_owned = view.to_owned().unwrap();

            // They should be equal
            assert_eq!(owned, view_owned, "Size {}", size);

            // Element-wise comparison
            let view_values: Vec<u64> = view.iter().map(|r| r.unwrap()).collect();
            assert_eq!(values, view_values, "Size {}", size);
        }
    }

    #[test]
    fn property_fixed_vector_decode_equivalence() {
        // Test that fixed vector view produces same results as owned
        for (i, value) in [0u64, 1, 100, 1000, u64::MAX].iter().enumerate() {
            let values = vec![*value; 8];
            let vec: FixedVector<u64, 8> = values.clone().into();
            let encoded = vec.as_ssz_bytes();

            // Decode as owned
            let owned = FixedVector::<u64, 8>::from_ssz_bytes(&encoded).unwrap();

            // Decode as view
            let view = FixedVectorRef::<u64, 8>::from_ssz_bytes(&encoded).unwrap();
            let view_owned = view.to_owned().unwrap();

            // They should be equal
            assert_eq!(owned, view_owned, "Iteration {}", i);

            // Element-wise comparison
            let view_values: Vec<u64> = view.iter().map(|r| r.unwrap()).collect();
            assert_eq!(values, view_values, "Iteration {}", i);
        }
    }

    #[test]
    fn property_variable_list_tree_hash_equivalence() {
        use tree_hash::{Sha256Hasher, TreeHash};

        // Property: tree hash of view must equal tree hash of owned for all inputs
        for size in [0, 1, 5, 10, 20] {
            let values: Vec<u32> = (0..size).map(|i| i * 7).collect();
            let list: VariableList<u32, 32> = values.into();
            let encoded = list.as_ssz_bytes();

            let view = VariableListRef::<u32, 32>::from_ssz_bytes(&encoded).unwrap();

            let owned_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&list);
            let view_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);

            assert_eq!(
                owned_hash, view_hash,
                "Tree hash mismatch for size {}",
                size
            );
        }
    }

    #[test]
    fn property_fixed_vector_tree_hash_equivalence() {
        use tree_hash::{Sha256Hasher, TreeHash};

        // Property: tree hash of view must equal tree hash of owned for all inputs
        for seed in 0..10 {
            let values: Vec<u16> = (0..16).map(|i| (i + seed) * 13).collect();
            let vec: FixedVector<u16, 16> = values.into();
            let encoded = vec.as_ssz_bytes();

            let view = FixedVectorRef::<u16, 16>::from_ssz_bytes(&encoded).unwrap();

            let owned_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&vec);
            let view_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);

            assert_eq!(
                owned_hash, view_hash,
                "Tree hash mismatch for seed {}",
                seed
            );
        }
    }

    #[test]
    fn property_variable_list_length_validation() {
        // Property: decoding should fail when list exceeds maximum length
        let values: Vec<u64> = (0..10).collect();
        let list: VariableList<u64, 20> = values.into();
        let encoded = list.as_ssz_bytes();

        // Should succeed with sufficient max
        assert!(VariableListRef::<u64, 10>::from_ssz_bytes(&encoded).is_ok());
        assert!(VariableListRef::<u64, 20>::from_ssz_bytes(&encoded).is_ok());

        // Should fail when max is too small
        assert!(VariableListRef::<u64, 9>::from_ssz_bytes(&encoded).is_err());
        assert!(VariableListRef::<u64, 5>::from_ssz_bytes(&encoded).is_err());
    }

    #[test]
    fn property_fixed_vector_length_validation() {
        // Property: decoding should fail when vector has wrong length
        let values: Vec<u64> = vec![1, 2, 3, 4, 5];
        let vec: FixedVector<u64, 5> = values.into();
        let encoded = vec.as_ssz_bytes();

        // Should succeed with correct length
        assert!(FixedVectorRef::<u64, 5>::from_ssz_bytes(&encoded).is_ok());

        // Should fail with wrong lengths
        assert!(FixedVectorRef::<u64, 4>::from_ssz_bytes(&encoded).is_err());
        assert!(FixedVectorRef::<u64, 6>::from_ssz_bytes(&encoded).is_err());
    }

    #[test]
    fn property_variable_list_multiple_types() {
        use tree_hash::{Sha256Hasher, TreeHash};

        // Test with u8
        let list_u8: VariableList<u8, 10> = vec![1u8, 2, 3].into();
        let encoded_u8 = list_u8.as_ssz_bytes();
        let view_u8 = VariableListRef::<u8, 10>::from_ssz_bytes(&encoded_u8).unwrap();
        assert_eq!(
            TreeHash::<Sha256Hasher>::tree_hash_root(&list_u8),
            TreeHash::<Sha256Hasher>::tree_hash_root(&view_u8)
        );

        // Test with u16
        let list_u16: VariableList<u16, 10> = vec![100u16, 200, 300].into();
        let encoded_u16 = list_u16.as_ssz_bytes();
        let view_u16 = VariableListRef::<u16, 10>::from_ssz_bytes(&encoded_u16).unwrap();
        assert_eq!(
            TreeHash::<Sha256Hasher>::tree_hash_root(&list_u16),
            TreeHash::<Sha256Hasher>::tree_hash_root(&view_u16)
        );

        // Test with u32
        let list_u32: VariableList<u32, 10> = vec![1000u32, 2000, 3000].into();
        let encoded_u32 = list_u32.as_ssz_bytes();
        let view_u32 = VariableListRef::<u32, 10>::from_ssz_bytes(&encoded_u32).unwrap();
        assert_eq!(
            TreeHash::<Sha256Hasher>::tree_hash_root(&list_u32),
            TreeHash::<Sha256Hasher>::tree_hash_root(&view_u32)
        );

        // Test with u64
        let list_u64: VariableList<u64, 10> = vec![10000u64, 20000, 30000].into();
        let encoded_u64 = list_u64.as_ssz_bytes();
        let view_u64 = VariableListRef::<u64, 10>::from_ssz_bytes(&encoded_u64).unwrap();
        assert_eq!(
            TreeHash::<Sha256Hasher>::tree_hash_root(&list_u64),
            TreeHash::<Sha256Hasher>::tree_hash_root(&view_u64)
        );
    }
}
