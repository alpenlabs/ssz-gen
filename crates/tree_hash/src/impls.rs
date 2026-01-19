// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Tree hash implementations for different types

use std::sync::Arc;

use ssz::{
    Bitfield, Fixed, Variable,
    view::{
        BitListRef, BitVectorRef, BytesRef, DecodeView, FixedBytesRef, ListRef, UnionRef, VectorRef,
    },
};
use ssz_primitives::{FixedBytes, U128, U256};

use super::*;

fn int_to_hasher_output<H: TreeHashDigest>(int: u64) -> H::Output {
    let mut bytes = vec![0; H::HASH_SIZE];
    bytes[0..8].copy_from_slice(&int.to_le_bytes());
    H::from_bytes(&bytes)
}

macro_rules! impl_for_bitsize {
    ($type: ident, $bit_size: expr) => {
        impl<H: TreeHashDigest> TreeHash<H> for $type {
            fn tree_hash_type() -> TreeHashType {
                TreeHashType::Basic
            }

            fn tree_hash_packed_encoding(&self) -> PackedEncoding {
                PackedEncoding::from_slice(&self.to_le_bytes())
            }

            fn tree_hash_packing_factor() -> usize {
                H::HASH_SIZE / ($bit_size / 8)
            }

            #[allow(clippy::cast_lossless)] // Lint does not apply to all uses of this macro.
            fn tree_hash_root(&self) -> H::Output {
                int_to_hasher_output::<H>(*self as u64)
            }
        }
    };
}

impl_for_bitsize!(u8, 8);
impl_for_bitsize!(u16, 16);
impl_for_bitsize!(u32, 32);
impl_for_bitsize!(u64, 64);
impl_for_bitsize!(usize, 64);

impl<H: TreeHashDigest> TreeHash<H> for bool {
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::Basic
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        <u8 as TreeHash<H>>::tree_hash_packed_encoding(&(*self as u8))
    }

    fn tree_hash_packing_factor() -> usize {
        <u8 as TreeHash<H>>::tree_hash_packing_factor()
    }

    fn tree_hash_root(&self) -> H::Output {
        int_to_hasher_output::<H>(*self as u64)
    }
}

impl<const N: usize, H: TreeHashDigest> TreeHash<H> for [u8; N] {
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
        let values_per_chunk = BYTES_PER_CHUNK;
        let minimum_chunk_count = N.div_ceil(values_per_chunk);
        merkle_root_with_hasher::<H>(self, minimum_chunk_count)
    }
}

impl<H: TreeHashDigest> TreeHash<H> for U128 {
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::Basic
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        PackedEncoding::from_slice(&self.to_le_bytes::<{ Self::BYTES }>())
    }

    fn tree_hash_packing_factor() -> usize {
        2
    }

    fn tree_hash_root(&self) -> H::Output {
        H::from_bytes(&self.to_le_bytes::<{ Self::BYTES }>())
    }
}

impl<H: TreeHashDigest> TreeHash<H> for U256 {
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::Basic
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        PackedEncoding::from(self.to_le_bytes::<{ Self::BYTES }>())
    }

    fn tree_hash_packing_factor() -> usize {
        1
    }

    fn tree_hash_root(&self) -> H::Output {
        H::from_bytes(&self.to_le_bytes::<{ Self::BYTES }>())
    }
}

// This implementation covers `Hash256`/`B256` as well.
impl<const N: usize, H: TreeHashDigest> TreeHash<H> for FixedBytes<N> {
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
        if N <= 32 {
            H::from_bytes(&self.0)
        } else {
            let values_per_chunk = BYTES_PER_CHUNK;
            let minimum_chunk_count = N.div_ceil(values_per_chunk);
            merkle_root_with_hasher::<H>(&self.0, minimum_chunk_count)
        }
    }
}

impl<T: TreeHash<H>, H: TreeHashDigest> TreeHash<H> for Arc<T> {
    fn tree_hash_type() -> TreeHashType {
        T::tree_hash_type()
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        self.as_ref().tree_hash_packed_encoding()
    }

    fn tree_hash_packing_factor() -> usize {
        T::tree_hash_packing_factor()
    }

    fn tree_hash_root(&self) -> H::Output {
        self.as_ref().tree_hash_root()
    }
}

/// A helper function providing common functionality for finding the Merkle root of some bytes that
/// represent a bitfield.
pub fn bitfield_bytes_tree_hash_root<const N: usize, H: TreeHashDigest>(bytes: &[u8]) -> H::Output {
    let byte_size = N.div_ceil(8);
    let leaf_count = byte_size.div_ceil(BYTES_PER_CHUNK);

    let mut hasher = MerkleHasher::<H>::with_leaves(leaf_count);
    hasher
        .write(bytes)
        .expect("bitfield should not exceed tree hash leaf limit");
    hasher
        .finish()
        .expect("bitfield tree hash buffer should not exceed leaf limit")
}

impl<const N: usize, H: TreeHashDigest> TreeHash<H> for Bitfield<Variable<N>> {
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
        // Note: we use `as_slice` because it does _not_ have the length-delimiting bit set (or
        // present).
        let root = bitfield_bytes_tree_hash_root::<N, H>(self.as_slice());
        mix_in_length_with_hasher::<H>(&root, self.len())
    }
}

impl<const N: usize, H: TreeHashDigest> TreeHash<H> for Bitfield<Fixed<N>> {
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
        bitfield_bytes_tree_hash_root::<N, H>(self.as_slice())
    }
}

// `Option<T>` represents `Union[None, T]`
impl<T: TreeHash<H>, H: TreeHashDigest> TreeHash<H> for Option<T> {
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::Container
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        unreachable!("Enum should never be packed")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("Enum should never be packed")
    }

    fn tree_hash_root(&self) -> H::Output {
        match self {
            None => {
                let root = H::get_zero_hash(0);
                let selector = 0u8;
                mix_in_selector_with_hasher::<H>(&root, selector)
                    .expect("derive macro should prevent out-of-bounds selectors")
            }
            Some(inner) => {
                let root = inner.tree_hash_root();
                let selector = 1u8;
                mix_in_selector_with_hasher::<H>(&root, selector)
                    .expect("derive macro should prevent out-of-bounds selectors")
            }
        }
    }
}

impl<'a, const N: usize, H: TreeHashDigest> TreeHash<H> for FixedBytesRef<'a, N> {
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
        // Directly hash the bytes without copying
        merkle_root_with_hasher::<H>(self.as_bytes(), 0)
    }
}

impl<'a, H: TreeHashDigest> TreeHash<H> for BytesRef<'a> {
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
        // Directly hash the bytes without copying to owned Vec
        let chunks_root = merkle_root_with_hasher::<H>(self.as_bytes(), 0);
        mix_in_length_with_hasher::<H>(&chunks_root, self.len())
    }
}

// TreeHash implementation for ListRef
impl<'a, TRef, H> TreeHash<H> for ListRef<'a, TRef>
where
    TRef: DecodeView<'a> + TreeHash<H> + ssz::view::SszTypeInfo,
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
        let item_type = TRef::tree_hash_type();

        if self.is_empty() {
            let chunks_root = H::get_zero_hash(0);
            return mix_in_length_with_hasher::<H>(&chunks_root, 0);
        }

        match item_type {
            TreeHashType::Basic => {
                // For basic types with fixed length, bytes are already properly laid out
                if !self.is_empty() {
                    let chunks_root = merkle_root_with_hasher::<H>(self.as_bytes(), 0);
                    mix_in_length_with_hasher::<H>(&chunks_root, self.len())
                } else {
                    let chunks_root = H::get_zero_hash(0);
                    mix_in_length_with_hasher::<H>(&chunks_root, 0)
                }
            }
            _ => {
                // For composite types (Container, Vector, List), hash each item
                tree_hash_composite_list_items::<TRef, H>(self)
            }
        }
    }
}

/// Helper function to hash composite items in a [`ListRef`].
fn tree_hash_composite_list_items<'a, TRef, H>(list: &ListRef<'a, TRef>) -> H::Output
where
    TRef: DecodeView<'a> + TreeHash<H> + ssz::view::SszTypeInfo,
    H: TreeHashDigest,
{
    let num_items = list.len();
    if num_items == 0 {
        let chunks_root = H::get_zero_hash(0);
        return mix_in_length_with_hasher::<H>(&chunks_root, 0);
    }

    // Create a hasher with enough leaves for all items
    let mut hasher = MerkleHasher::<H>::with_leaves(num_items);

    // Hash each item and write to the hasher
    for item_result in list.iter() {
        let item = item_result.expect("ListRef iteration should not fail during tree hashing");
        let item_root = item.tree_hash_root();
        hasher
            .write(item_root.as_ref())
            .expect("hasher has sufficient leaves");
    }

    let chunks_root = hasher.finish().expect("hasher has sufficient leaves");
    mix_in_length_with_hasher::<H>(&chunks_root, num_items)
}

/// TreeHash implementation for [`VectorRef`].
impl<'a, TRef, const N: usize, H> TreeHash<H> for VectorRef<'a, TRef, N>
where
    TRef: DecodeView<'a> + TreeHash<H> + ssz::view::SszTypeInfo,
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
        let item_type = TRef::tree_hash_type();

        if N == 0 {
            return H::get_zero_hash(0);
        }

        match item_type {
            TreeHashType::Basic => {
                // For basic types with fixed length, bytes are already properly laid out
                merkle_root_with_hasher::<H>(self.as_bytes(), 0)
            }
            _ => {
                // For composite types, hash each item
                tree_hash_composite_vector_items::<TRef, H, N>(self)
            }
        }
    }
}

/// Helper function to hash composite items in a [`VectorRef`].
fn tree_hash_composite_vector_items<'a, TRef, H, const N: usize>(
    vector: &VectorRef<'a, TRef, N>,
) -> H::Output
where
    TRef: DecodeView<'a> + TreeHash<H> + ssz::view::SszTypeInfo,
    H: TreeHashDigest,
{
    if N == 0 {
        return H::get_zero_hash(0);
    }

    // Create a hasher with enough leaves for all items
    let mut hasher = MerkleHasher::<H>::with_leaves(N);

    // Hash each item and write to the hasher
    for item_result in vector.iter() {
        let item = item_result.expect("VectorRef iteration should not fail during tree hashing");
        let item_root = item.tree_hash_root();
        hasher
            .write(item_root.as_ref())
            .expect("hasher has sufficient leaves");
    }

    hasher.finish().expect("hasher has sufficient leaves")
}

impl<'a, VRef, H> TreeHash<H> for UnionRef<'a, VRef>
where
    VRef: DecodeView<'a> + TreeHash<H>,
    H: TreeHashDigest,
{
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::Container // Unions are hashed like containers
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        unreachable!("Union should never be packed.")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("Union should never be packed.")
    }

    fn tree_hash_root(&self) -> H::Output {
        // Decode the body and get its tree hash root
        let body = VRef::from_ssz_bytes(self.body_bytes())
            .expect("UnionRef body should be valid during tree hashing");
        let body_root = body.tree_hash_root();

        // Mix in the selector
        mix_in_selector_with_hasher::<H>(&body_root, u8::from(self.selector()))
            .expect("selector should be valid")
    }
}

impl<'a, const N: usize, H: TreeHashDigest> TreeHash<H> for BitVectorRef<'a, N>
where
    [(); ssz::view::bytes_for_bits(N)]:,
{
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::Vector
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding
    where
        [(); ssz::view::bytes_for_bits(N)]:,
    {
        unreachable!("BitVector should never be packed.")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("BitVector should never be packed.")
    }

    fn tree_hash_root(&self) -> H::Output
    where
        [(); ssz::view::bytes_for_bits(N)]:,
    {
        // Convert the fixed-size array to a slice
        merkle_root_with_hasher::<H>(self.bytes.as_slice(), 0)
    }
}

impl<'a, const N: usize, H: TreeHashDigest> TreeHash<H> for BitListRef<'a, N> {
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::List
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        unreachable!("BitList should never be packed.")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("BitList should never be packed.")
    }

    fn tree_hash_root(&self) -> H::Output {
        let bit_len = self.len();
        let data_len = if bit_len == 0 { 0 } else { bit_len.div_ceil(8) };
        let mut bytes = self.as_bytes()[..data_len].to_vec();

        if let Some(last) = bytes.last_mut() {
            let bits_in_last = bit_len % 8;
            if bits_in_last != 0 {
                let mask = (1u8 << bits_in_last) - 1;
                *last &= mask;
            }
        }

        let chunks_root = bitfield_bytes_tree_hash_root::<N, H>(&bytes);
        mix_in_length_with_hasher::<H>(&chunks_root, bit_len)
    }
}
#[cfg(test)]
mod test {
    use ssz::{BitList, BitVector};

    use super::*;

    #[test]
    fn bool() {
        let mut true_bytes: Vec<u8> = vec![1];
        true_bytes.append(&mut vec![0; 31]);

        let false_bytes: Vec<u8> = vec![0; 32];

        assert_eq!(
            <bool as TreeHash<Sha256Hasher>>::tree_hash_root(&true).as_slice(),
            true_bytes.as_slice()
        );
        assert_eq!(
            <bool as TreeHash<Sha256Hasher>>::tree_hash_root(&false).as_slice(),
            false_bytes.as_slice()
        );
    }

    #[test]
    fn arc() {
        let one = U128::from(1);
        let one_arc = Arc::new(one);
        assert_eq!(
            <Arc<U128> as TreeHash<Sha256Hasher>>::tree_hash_root(&one_arc),
            <U128 as TreeHash<Sha256Hasher>>::tree_hash_root(&one)
        );
    }

    #[test]
    fn int_to_bytes() {
        assert_eq!(int_to_hasher_output::<Sha256Hasher>(0).as_slice(), &[0; 32]);
        assert_eq!(
            int_to_hasher_output::<Sha256Hasher>(1).as_slice(),
            &[
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
        assert_eq!(
            int_to_hasher_output::<Sha256Hasher>(u64::MAX).as_slice(),
            &[
                255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn bitvector() {
        let empty_bitvector = BitVector::<8>::new();
        assert_eq!(
            <BitVector<8> as TreeHash<Sha256Hasher>>::tree_hash_root(&empty_bitvector),
            Hash256::ZERO
        );

        let small_bitvector_bytes = vec![0xff_u8, 0xee, 0xdd, 0xcc];
        let small_bitvector =
            BitVector::<32>::from_bytes(small_bitvector_bytes.clone().into()).unwrap();
        assert_eq!(
            <BitVector<32> as TreeHash<Sha256Hasher>>::tree_hash_root(&small_bitvector).as_slice()
                [..4],
            small_bitvector_bytes
        );
    }

    #[test]
    fn bitlist() {
        let empty_bitlist = BitList::<8>::with_capacity(8).unwrap();
        assert_eq!(
            <BitList<8> as TreeHash<Sha256Hasher>>::tree_hash_root(&empty_bitlist),
            "0x5ac78d953211aa822c3ae6e9b0058e42394dd32e5992f29f9c12da3681985130"
                .parse()
                .unwrap()
        );

        let mut small_bitlist = BitList::<32>::with_capacity(4).unwrap();
        small_bitlist.set(1, true).unwrap();
        assert_eq!(
            <BitList<32> as TreeHash<Sha256Hasher>>::tree_hash_root(&small_bitlist),
            "0x7eb03d394d83a389980b79897207be3a6512d964cb08978bb7f3cfc0db8cfb8a"
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn fixed_bytes_7() {
        let data = [
            [0, 1, 2, 3, 4, 5, 6],
            [6, 5, 4, 3, 2, 1, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        ];
        for bytes in data {
            assert_eq!(
                <[u8; 7] as TreeHash<Sha256Hasher>>::tree_hash_root(&bytes),
                Hash256::right_padding_from(&bytes)
            );
        }
    }

    #[test]
    fn fixed_bytes_32() {
        let data = [
            Hash256::ZERO,
            Hash256::repeat_byte(0xff),
            Hash256::right_padding_from(&[0, 1, 2, 3, 4, 5]),
            Hash256::left_padding_from(&[10, 9, 8, 7, 6]),
        ];
        for bytes in data {
            assert_eq!(
                <FixedBytes<32> as TreeHash<Sha256Hasher>>::tree_hash_root(&bytes),
                bytes
            );
        }
    }

    #[test]
    fn fixed_bytes_48() {
        let data = [
            (
                FixedBytes::<48>::zero(),
                "0xf5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b",
            ),
            (
                FixedBytes::<48>::repeat_byte(0xff),
                "0x1e3915ef9ca4ed8619d472b72fb1833448756054b4de9acb439da54dff7166aa",
            ),
        ];
        for (bytes, expected) in data {
            assert_eq!(
                <FixedBytes<48> as TreeHash<Sha256Hasher>>::tree_hash_root(&bytes),
                expected.parse().unwrap()
            );
        }
    }

    // Only basic types should be packed.
    #[test]
    #[should_panic]
    fn fixed_bytes_no_packed_encoding() {
        <FixedBytes<32> as TreeHash<Sha256Hasher>>::tree_hash_packed_encoding(&Hash256::ZERO);
    }

    #[test]
    #[should_panic]
    fn fixed_bytes_no_packing_factor() {
        <FixedBytes<32> as TreeHash<Sha256Hasher>>::tree_hash_packing_factor();
    }
}
