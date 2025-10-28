// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

//! Tree hash implementation

#[cfg(test)]
use ssz_types as _;

pub mod impls;
mod merkle_hasher;
mod merkleize_padded;
mod merkleize_standard;

use std::sync::LazyLock;

pub use merkle_hasher::{Error, MerkleHasher};
pub use merkleize_padded::merkleize_padded_with_hasher;
pub use merkleize_standard::merkleize_standard_with_hasher;

use digest::Digest;
use sha2 as _;

use smallvec::SmallVec;

/// Number of bytes in a chunk
pub const BYTES_PER_CHUNK: usize = 32;
/// Size of a merkle hash chunk
pub const MERKLE_HASH_CHUNK: usize = 2 * BYTES_PER_CHUNK;
/// Maximum union selector
pub const MAX_UNION_SELECTOR: u8 = 127;
/// Size of a smallvec
pub const SMALLVEC_SIZE: usize = 32;
/// Maximum index for zero hashes
pub const ZERO_HASHES_MAX_INDEX: usize = 48;

/// 256-bit hash
pub type Hash256 = ssz_primitives::Hash256;

/// Packed encoding
pub type PackedEncoding = SmallVec<[u8; SMALLVEC_SIZE]>;

/// Default MerkleHasher using SHA256
pub type Sha256MerkleHasher = MerkleHasher<Sha256Hasher>;

/// Zero hashes for SHA256
pub static ZERO_HASHES_SHA256: LazyLock<Vec<Hash256>> = LazyLock::new(|| {
    let mut hashes = vec![Hash256::zero(); ZERO_HASHES_MAX_INDEX + 1];
    for i in 0..ZERO_HASHES_MAX_INDEX {
        hashes[i + 1] = Hash256::from_slice(&hash32_concat::<sha2::Sha256>(
            hashes[i].as_ref(),
            hashes[i].as_ref(),
        ));
    }
    hashes
});

/// Trait for tree hash digests with incremental hashing support
pub trait TreeHashDigest {
    /// Output type
    type Output: AsRef<[u8]> + Clone;

    /// Size of the hash
    const HASH_SIZE: usize;
    /// Length of the hash
    const HASH_LEN: usize;

    /// Associated zero hashes function
    fn zero_hashes() -> &'static [Self::Output];

    /// Hash function
    fn hash(data: &[u8]) -> Self::Output;
    /// Fixed hash function
    fn hash_fixed(data: &[u8]) -> Self::Output;
    /// Hash32 concat function
    fn hash32_concat(left: &[u8], right: &[u8]) -> Self::Output;
    /// Get zero hash at a specific depth
    fn get_zero_hash(depth: usize) -> Self::Output;
    /// Get zero hash at a specific depth as a slice
    fn get_zero_hash_slice(depth: usize) -> &'static [u8];
    /// Create output from raw bytes (without hashing)
    fn from_bytes(bytes: &[u8]) -> Self::Output;

    /// Create a new incremental hasher context
    fn new_context() -> Self;
    /// Update the hasher context with new data
    fn update(&mut self, data: &[u8]);
    /// Finalize the hash and return the result
    fn finalize(self) -> Self::Output;
}

/// SHA256 hasher with incremental support
#[derive(Debug)]
pub struct Sha256Hasher {
    hasher: sha2::Sha256,
}

/// SHA256 hasher implementation
impl TreeHashDigest for Sha256Hasher {
    type Output = Hash256;
    const HASH_SIZE: usize = 32;
    const HASH_LEN: usize = 32;

    fn zero_hashes() -> &'static [Self::Output] {
        &ZERO_HASHES_SHA256
    }

    fn hash(data: &[u8]) -> Self::Output {
        Hash256::from_slice(&hash::<sha2::Sha256>(data))
    }

    fn hash_fixed(data: &[u8]) -> Self::Output {
        Hash256::from_slice(&hash_fixed_with_digest::<sha2::Sha256>(data))
    }

    fn hash32_concat(left: &[u8], right: &[u8]) -> Self::Output {
        Hash256::from_slice(&hash32_concat::<sha2::Sha256>(left, right))
    }

    fn get_zero_hash(depth: usize) -> Self::Output {
        Self::zero_hashes()[depth]
    }

    fn get_zero_hash_slice(depth: usize) -> &'static [u8] {
        Self::zero_hashes()[depth].as_ref()
    }

    fn from_bytes(bytes: &[u8]) -> Self::Output {
        let mut padded = [0u8; Self::HASH_SIZE];
        let len = std::cmp::min(bytes.len(), Self::HASH_SIZE);
        padded[..len].copy_from_slice(&bytes[..len]);
        Hash256::from_slice(&padded)
    }

    fn new_context() -> Self {
        use digest::Digest;
        Self {
            hasher: sha2::Sha256::new(),
        }
    }

    fn update(&mut self, data: &[u8]) {
        use digest::Digest;
        self.hasher.update(data);
    }

    fn finalize(self) -> Self::Output {
        use digest::Digest;
        Hash256::from_slice(&self.hasher.finalize())
    }
}

/// Generate zero hashes for a specific hasher up to the maximum depth
pub fn get_zero_hashes<H: TreeHashDigest>(hash_len: usize) -> Vec<H::Output> {
    let mut hashes = Vec::with_capacity(ZERO_HASHES_MAX_INDEX + 1);
    let zero_bytes = vec![0u8; hash_len];

    // First hash is all zeros
    hashes.push(H::from_bytes(&zero_bytes));

    // Each subsequent hash is hash(previous || previous)
    for i in 0..ZERO_HASHES_MAX_INDEX {
        let current = &hashes[i];
        let next = H::hash32_concat(current.as_ref(), current.as_ref());
        hashes.push(next);
    }

    hashes
}

/// Generic hash32_concat function using Digest trait
pub fn hash32_concat<D: Digest + Default>(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut hasher = D::default();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().to_vec()
}

/// Generic hash function using Digest trait
pub fn hash<D: Digest + Default>(data: &[u8]) -> Vec<u8> {
    let mut hasher = D::default();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Generic hash_fixed function using Digest trait - returns fixed-size array (no allocation!)
pub fn hash_fixed_with_digest<D: Digest + Default>(data: &[u8]) -> [u8; 32] {
    let mut hasher = D::default();
    hasher.update(data);
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    output
}

/// Convenience method for `MerkleHasher` which also provides some fast-paths for small trees.
///
/// `minimum_leaf_count` will only be used if it is greater than or equal to the minimum number of leaves that can be created from `bytes`.
pub fn merkle_root_with_hasher<H: TreeHashDigest>(
    bytes: &[u8],
    minimum_leaf_count: usize,
) -> H::Output {
    let leaves = std::cmp::max(bytes.len().div_ceil(H::HASH_SIZE), minimum_leaf_count);

    if leaves == 0 {
        H::get_zero_hash(0)
    } else if leaves == 1 {
        H::from_bytes(bytes)
    } else if leaves == 2 {
        let mut leaves_data = vec![0; H::HASH_SIZE * 2];
        leaves_data[0..bytes.len()].copy_from_slice(bytes);
        H::hash_fixed(&leaves_data)
    } else {
        // Use the generic MerkleHasher for complex trees
        let mut hasher = MerkleHasher::<H>::with_leaves(leaves);
        hasher
            .write(bytes)
            .expect("the number of leaves is adequate for the number of bytes");
        hasher
            .finish()
            .expect("the number of leaves is adequate for the number of bytes")
    }
}

/// Returns the node created by hashing `root` and `length`.
///
/// Used in `TreeHash` for inserting the length of a list above it's root.
pub fn mix_in_length_with_hasher<H: TreeHashDigest>(root: &H::Output, length: usize) -> H::Output {
    let usize_len = std::mem::size_of::<usize>();

    let mut length_bytes = [0; BYTES_PER_CHUNK];
    length_bytes[0..usize_len].copy_from_slice(&length.to_le_bytes());

    H::hash32_concat(root.as_ref(), &length_bytes)
}

/// Returns `Some(root)` created by hashing `root` and `selector`, if `selector <=
/// MAX_UNION_SELECTOR`. Otherwise, returns `None`.
///
/// Used in `TreeHash` for the "union" type.
///
/// ## Specification
///
/// ```ignore,text
/// mix_in_selector_with_hasher: Given a Merkle root root and a type selector selector ("uint256" little-endian
/// serialization) return hash(root + selector).
/// ```
///
/// <https://github.com/ethereum/consensus-specs/blob/v1.1.0-beta.3/ssz/simple-serialize.md#union>
pub fn mix_in_selector_with_hasher<H: TreeHashDigest>(
    root: &H::Output,
    selector: u8,
) -> Option<H::Output> {
    if selector > MAX_UNION_SELECTOR {
        return None;
    }

    let mut chunk = [0; BYTES_PER_CHUNK];
    chunk[0] = selector;

    Some(H::hash32_concat(root.as_ref(), &chunk))
}

/// Returns `root` created by hashing `root` and `aux`.
pub fn mix_in_aux_with_hasher<H: TreeHashDigest>(root: &H::Output, aux: &H::Output) -> H::Output {
    H::hash32_concat(root.as_ref(), aux.as_ref())
}

/// Type of the tree hash.
#[derive(Debug, PartialEq, Clone)]
pub enum TreeHashType {
    /// Basic tree hash.
    Basic,
    /// Vector tree hash.
    Vector,
    /// List tree hash.
    List,
    /// Container tree hash.
    Container,
    /// Stable container tree hash.
    StableContainer,
}

/// Trait for types that can be hashed into a merkle tree.
pub trait TreeHash<H: TreeHashDigest = Sha256Hasher> {
    /// Returns the type of the tree hash.
    fn tree_hash_type() -> TreeHashType;

    /// Returns the packed encoding of the tree hash.
    fn tree_hash_packed_encoding(&self) -> PackedEncoding;

    /// Returns the packing factor of the tree hash.
    fn tree_hash_packing_factor() -> usize;

    /// Returns the root of the tree hash.
    fn tree_hash_root(&self) -> H::Output;
}

/// Punch through references.
impl<T, H: TreeHashDigest> TreeHash<H> for &T
where
    T: TreeHash<H>,
{
    fn tree_hash_type() -> TreeHashType {
        T::tree_hash_type()
    }

    fn tree_hash_packed_encoding(&self) -> PackedEncoding {
        T::tree_hash_packed_encoding(*self)
    }

    fn tree_hash_packing_factor() -> usize {
        T::tree_hash_packing_factor()
    }

    fn tree_hash_root(&self) -> H::Output {
        T::tree_hash_root(*self)
    }
}

/// Macro for implementing `TreeHash` for a type that is encoded as a vector.
#[macro_export]
macro_rules! tree_hash_ssz_encoding_as_vector {
    ($type: ident) => {
        impl<H: TreeHashDigest> tree_hash::TreeHash<H> for $type {
            fn tree_hash_type() -> tree_hash::TreeHashType {
                tree_hash::TreeHashType::Vector
            }

            fn tree_hash_packed_encoding(&self) -> PackedEncoding {
                unreachable!("Vector should never be packed.")
            }

            fn tree_hash_packing_factor() -> usize {
                unreachable!("Vector should never be packed.")
            }

            fn tree_hash_root(&self) -> H::Output {
                tree_hash::merkle_root_with_hasher::<H>(&ssz::ssz_encode(self), 0)
            }
        }
    };
}

/// Macro for implementing `TreeHash` for a type that is encoded as a list.
#[macro_export]
macro_rules! tree_hash_ssz_encoding_as_list {
    ($type: ident) => {
        impl<H: TreeHashDigest> tree_hash::TreeHash<H> for $type {
            fn tree_hash_type() -> tree_hash::TreeHashType {
                tree_hash::TreeHashType::List
            }

            fn tree_hash_packed_encoding(&self) -> PackedEncoding {
                unreachable!("List should never be packed.")
            }

            fn tree_hash_packing_factor() -> usize {
                unreachable!("List should never be packed.")
            }

            fn tree_hash_root(&self) -> H::Output {
                tree_hash::merkle_root_with_hasher::<H>(&ssz::ssz_encode(self), 0)
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use ssz_derive as _;
    use ssz_primitives as _;
    use tree_hash_derive as _;

    #[test]
    fn mix_length() {
        let hash = {
            let mut preimage = vec![42; BYTES_PER_CHUNK];
            preimage.append(&mut vec![42]);
            preimage.append(&mut vec![0; BYTES_PER_CHUNK - 1]);
            hash::<sha2::Sha256>(&preimage)
        };

        let result = mix_in_length_with_hasher::<Sha256Hasher>(
            &Hash256::from_slice(&[42; BYTES_PER_CHUNK]),
            42,
        );
        assert_eq!(result.as_ref(), &hash[..]);
    }

    #[test]
    fn zero_hashes() {
        let zero_hashes = Sha256Hasher::zero_hashes();
        assert_eq!(zero_hashes.len(), ZERO_HASHES_MAX_INDEX + 1);
        assert_eq!(zero_hashes[0], Hash256::zero());
        assert_eq!(
            zero_hashes[1],
            Sha256Hasher::hash32_concat(&[0; 32], &[0; 32])
        );
    }
}
