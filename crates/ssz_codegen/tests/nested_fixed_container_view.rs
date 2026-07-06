//! Regression test: views over containers with fixed-size container fields.
//!
//! The owned `ssz_derive` encoding inlines fixed-size container fields in the
//! fixed portion, but generated views assume every container-typed field is
//! variable-size (an offset-table entry), so they fail to decode
//! owned-encoded bytes with `OffsetIntoFixedPortion`. Similarly, the view
//! TreeHash impl writes raw basic-field bytes into the `MerkleHasher` (which
//! only forms a leaf per `HASH_SIZE` bytes) while the owned impl gives every
//! field a padded leaf, so their roots disagree.
//!
//! These tests currently PIN THE BROKEN BEHAVIOR to document the bug; the
//! fix flips them to assert agreement with the owned encoding.

#![allow(dead_code)]
#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]

use ssz_derive as _;
use ssz_primitives as _;
use tree_hash_derive as _;

// Include generated code
include!("expected_output/test_nested_fixed_container.rs");

use ssz::{DecodeError, Encode, view::DecodeView};
use ssz_types::VariableList;
use tests::input::test_nested_fixed_container::{
    BasicPair, BasicPairRef, FixedInner, FixedOuter, FixedOuterRef, FixedPair, MixedOuter,
    MixedOuterRef,
};
use tree_hash::TreeHash;

fn sample_mixed() -> MixedOuter {
    MixedOuter {
        inner: FixedInner { tag: 7 },
        count: 0xAABBCCDD,
        pair: FixedPair { x: 1, y: 2 },
        tail: VariableList::new(vec![1, 2, 3]).expect("within bound"),
    }
}

#[test]
fn mixed_view_rejects_owned_encoding() {
    let owned = sample_mixed();
    let bytes = owned.as_ssz_bytes();

    // BUG: the view assumes `inner` and `pair` sit behind offset-table
    // entries, but the owned encoding inlines them (they are fixed-size), so
    // the view cannot decode bytes produced by the owned Encode impl.
    let err = MixedOuterRef::from_ssz_bytes(&bytes).expect_err("view layout disagrees");
    assert!(matches!(err, DecodeError::OffsetIntoFixedPortion(_)));
}

#[test]
fn fully_fixed_view_rejects_owned_encoding() {
    let owned = FixedOuter {
        inner: FixedInner { tag: 3 },
        pair: FixedPair { x: 9, y: 10 },
    };
    let bytes = owned.as_ssz_bytes();

    // BUG: same disagreement for a container whose owned encoding is fully
    // fixed-size (9 bytes inline; the view expects an 8-byte offset table).
    assert!(FixedOuterRef::from_ssz_bytes(&bytes).is_err());
}

#[test]
fn basic_view_tree_hash_disagrees_with_owned() {
    let owned = BasicPair {
        tag: 0x11,
        b: 0x22334455,
    };
    let bytes = owned.as_ssz_bytes();

    // Basic-only containers decode fine (their layout has no container
    // fields), but the view TreeHash packs the raw field bytes into a single
    // leaf while the owned impl hashes one padded leaf per field.
    let view = BasicPairRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_ne!(
        view.tree_hash_root::<tree_hash::Sha256Hasher>(),
        owned.tree_hash_root::<tree_hash::Sha256Hasher>(),
        "BUG: view and owned tree hashes should agree but do not yet"
    );
}
