use super::*;
use crate::hash;

/// Merkleizes bytes and returns the root, using a simple algorithm that does not optimize to avoid
/// processing or storing padding bytes.
///
/// **Note**: This function is generally worse than using the `crate::merkle_root` which uses
/// `MerkleHasher`. We only keep this function around for reference testing.
///
/// The input `bytes` will be padded to ensure that the number of leaves is a power-of-two.
///
/// ## CPU Performance
///
/// Will hash all nodes in the tree, even if they are padding and pre-determined.
///
/// ## Memory Performance
///
///  - Duplicates the input `bytes`.
///  - Stores all internal nodes, even if they are padding.
///  - Does not free up unused memory during operation.
pub fn merkleize_standard_with_hasher<H: TreeHashDigest>(bytes: &[u8]) -> H::Output {
    // If the bytes are just one chunk (or less than one chunk) just return them.
    if bytes.len() <= H::HASH_SIZE {
        let mut o = bytes.to_vec();
        o.resize(H::HASH_SIZE, 0);
        return H::from_bytes(&o[0..H::HASH_SIZE]);
    }

    let leaves = num_sanitized_leaves_with_hasher::<H>(bytes.len());
    let nodes = num_nodes(leaves);
    let internal_nodes = nodes - leaves;

    let num_bytes = std::cmp::max(internal_nodes, 1) * H::HASH_SIZE + bytes.len();

    let mut o: Vec<u8> = vec![0; internal_nodes * H::HASH_SIZE];

    o.append(&mut bytes.to_vec());

    assert_eq!(o.len(), num_bytes);

    let empty_chunk_hash = hash::<sha2::Sha256>(&[0; MERKLE_HASH_CHUNK]);

    let mut i = nodes * H::HASH_SIZE;
    let mut j = internal_nodes * H::HASH_SIZE;

    while i >= MERKLE_HASH_CHUNK {
        i -= MERKLE_HASH_CHUNK;

        j -= H::HASH_SIZE;
        let hash = match o.get(i..i + MERKLE_HASH_CHUNK) {
            // All bytes are available, hash as usual.
            Some(slice) => hash::<sha2::Sha256>(slice),
            // Unable to get all the bytes.
            None => {
                match o.get(i..) {
                    // Able to get some of the bytes, pad them out.
                    Some(slice) => {
                        let mut bytes = slice.to_vec();
                        bytes.resize(MERKLE_HASH_CHUNK, 0);
                        hash::<sha2::Sha256>(&bytes)
                    }
                    // Unable to get any bytes, use the empty-chunk hash.
                    None => empty_chunk_hash.clone(),
                }
            }
        };

        o[j..j + H::HASH_SIZE].copy_from_slice(&hash);
    }

    H::from_bytes(&o[0..H::HASH_SIZE])
}

fn num_sanitized_leaves_with_hasher<H: TreeHashDigest>(num_bytes: usize) -> usize {
    let leaves = num_bytes.div_ceil(H::HASH_SIZE);
    leaves.next_power_of_two()
}

fn num_nodes(num_leaves: usize) -> usize {
    2 * num_leaves - 1
}
