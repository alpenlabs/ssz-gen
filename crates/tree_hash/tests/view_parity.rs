#![allow(missing_docs)]
#![allow(unused_crate_dependencies)]

use ssz::{
    BitList, Encode,
    view::{BitListRef, DecodeView},
};
use tree_hash::{Sha256Hasher, TreeHash};

#[test]
fn bitlistref_tree_hash_matches_owned_small() {
    let mut owned: BitList<32> = BitList::with_capacity(5).unwrap();
    owned.set(0, true).unwrap();
    owned.set(2, true).unwrap();
    owned.set(4, true).unwrap();

    let bytes = owned.as_ssz_bytes();
    let view = BitListRef::<32>::from_ssz_bytes(&bytes).unwrap();

    let owned_hash = TreeHash::<Sha256Hasher>::tree_hash_root(&owned);
    let view_hash = TreeHash::<Sha256Hasher>::tree_hash_root(&view);

    assert_eq!(owned_hash, view_hash);
}

#[test]
fn bitlistref_tree_hash_matches_owned_large_capacity() {
    let mut owned: BitList<512> = BitList::with_capacity(32).unwrap();
    owned.set(0, true).unwrap();
    owned.set(7, true).unwrap();
    owned.set(15, true).unwrap();
    owned.set(31, true).unwrap();

    let bytes = owned.as_ssz_bytes();
    let view = BitListRef::<512>::from_ssz_bytes(&bytes).unwrap();

    let owned_hash = TreeHash::<Sha256Hasher>::tree_hash_root(&owned);
    let view_hash = TreeHash::<Sha256Hasher>::tree_hash_root(&view);

    assert_eq!(owned_hash, view_hash);
}
