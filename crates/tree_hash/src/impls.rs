// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

//! Tree hash implementations for different types

use super::*;
use ssz::{Bitfield, Fixed, Variable};
use ssz_primitives::{FixedBytes, U128, U256};
use std::sync::Arc;
use typenum::Unsigned;

fn int_to_hasher_output<H: TreeHashDigest>(int: u64) -> H::Output {
    let mut bytes = [0; HASHSIZE];
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
                HASHSIZE / ($bit_size / 8)
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
pub fn bitfield_bytes_tree_hash_root<N: Unsigned, H: TreeHashDigest>(bytes: &[u8]) -> H::Output {
    let byte_size = N::to_usize().div_ceil(8);
    let leaf_count = byte_size.div_ceil(BYTES_PER_CHUNK);

    let mut hasher = MerkleHasher::<H>::with_leaves(leaf_count);
    hasher
        .write(bytes)
        .expect("bitfield should not exceed tree hash leaf limit");
    hasher
        .finish()
        .expect("bitfield tree hash buffer should not exceed leaf limit")
}

impl<N: Unsigned + Clone, H: TreeHashDigest> TreeHash<H> for Bitfield<Variable<N>> {
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

impl<N: Unsigned + Clone, H: TreeHashDigest> TreeHash<H> for Bitfield<Fixed<N>> {
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
                let root = Hash256::ZERO;
                let selector = 0u8;
                mix_in_selector(&root, selector)
                    .expect("derive macro should prevent out-of-bounds selectors")
            }
            Some(inner) => {
                let root = inner.tree_hash_root();
                let selector = 1u8;
                mix_in_selector(&root, selector)
                    .expect("derive macro should prevent out-of-bounds selectors")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ssz::{BitList, BitVector};
    use typenum::{U8, U32};

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
        let empty_bitvector = BitVector::<U8>::new();
        assert_eq!(
            <BitVector<U8> as TreeHash<Sha256Hasher>>::tree_hash_root(&empty_bitvector),
            Hash256::ZERO
        );

        let small_bitvector_bytes = vec![0xff_u8, 0xee, 0xdd, 0xcc];
        let small_bitvector =
            BitVector::<U32>::from_bytes(small_bitvector_bytes.clone().into()).unwrap();
        assert_eq!(
            <BitVector<U32> as TreeHash<Sha256Hasher>>::tree_hash_root(&small_bitvector).as_slice()
                [..4],
            small_bitvector_bytes
        );
    }

    #[test]
    fn bitlist() {
        let empty_bitlist = BitList::<U8>::with_capacity(8).unwrap();
        assert_eq!(
            <BitList<U8> as TreeHash<Sha256Hasher>>::tree_hash_root(&empty_bitlist),
            "0x5ac78d953211aa822c3ae6e9b0058e42394dd32e5992f29f9c12da3681985130"
                .parse()
                .unwrap()
        );

        let mut small_bitlist = BitList::<U32>::with_capacity(4).unwrap();
        small_bitlist.set(1, true).unwrap();
        assert_eq!(
            <BitList<U32> as TreeHash<Sha256Hasher>>::tree_hash_root(&small_bitlist),
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
