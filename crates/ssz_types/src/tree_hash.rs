// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

use tree_hash::{MerkleHasher, TreeHash, TreeHashDigest, TreeHashType};

/// A helper function providing common functionality between the `TreeHash` implementations for
/// `FixedVector` and `VariableList`.
pub(crate) fn vec_tree_hash_root<T, const N: usize, H: TreeHashDigest>(vec: &[T]) -> H::Output
where
    T: TreeHash<H>,
{
    match T::tree_hash_type() {
        TreeHashType::Basic => {
            let mut hasher =
                MerkleHasher::<H>::with_leaves(N.div_ceil(T::tree_hash_packing_factor()));

            for item in vec {
                hasher
                    .write(&item.tree_hash_packed_encoding())
                    .expect("ssz_types variable vec should not contain more elements than max");
            }

            hasher
                .finish()
                .expect("ssz_types variable vec should not have a remaining buffer")
        }
        TreeHashType::Container
        | TreeHashType::StableContainer
        | TreeHashType::List
        | TreeHashType::Vector => {
            let mut hasher = MerkleHasher::<H>::with_leaves(N);

            for item in vec {
                hasher
                    .write(item.tree_hash_root().as_ref())
                    .expect("ssz_types vec should not contain more elements than max");
            }

            hasher
                .finish()
                .expect("ssz_types vec should not have a remaining buffer")
        }
    }
}
