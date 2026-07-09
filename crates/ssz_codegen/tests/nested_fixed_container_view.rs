//! Regression test: views over containers with fixed-size container fields.
//!
//! The owned `ssz_derive` encoding inlines fixed-size container fields in the
//! fixed portion. Views used to assume every container-typed field was
//! variable-size (an offset-table entry) and failed to decode owned-encoded
//! bytes with `OffsetIntoFixedPortion`; the view TreeHash also wrote raw
//! basic-field bytes instead of one padded leaf per field. Views now derive
//! their layout from the field types' `Encode` impls and must agree with the
//! owned encoding.

#![allow(dead_code)]
#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]

use ssz_derive as _;
use ssz_primitives as _;
use tree_hash_derive as _;

// Include generated code
include!("expected_output/test_nested_fixed_container.rs");

use ssz::{
    Encode,
    view::{DecodeView, SszTypeInfo},
};
use ssz_types::VariableList;
use tests::input::test_nested_fixed_container::{
    BasicPair, BasicPairRef, FixedInner, FixedOuter, FixedOuterRef, FixedPair, Interleaved,
    InterleavedRef, MixedOuter, MixedOuterRef, VarThenFixed, VarThenFixedRef,
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
fn mixed_view_agrees_with_owned_encoding() {
    let owned = sample_mixed();
    let bytes = owned.as_ssz_bytes();

    let view = MixedOuterRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.inner().expect("inner").tag().expect("tag"), 7);
    assert_eq!(view.count().expect("count"), 0xAABBCCDD);
    let pair = view.pair().expect("pair");
    assert_eq!(pair.x().expect("x"), 1);
    assert_eq!(pair.y().expect("y"), 2);
    assert_eq!(view.tail().expect("tail").len(), 3);
    assert_eq!(view.to_owned(), owned);
}

#[test]
fn fully_fixed_view_agrees_with_owned_encoding() {
    let owned = FixedOuter {
        inner: FixedInner { tag: 3 },
        pair: FixedPair { x: 9, y: 10 },
    };
    let bytes = owned.as_ssz_bytes();

    assert!(<FixedOuterRef<'_> as SszTypeInfo>::is_ssz_fixed_len());
    assert_eq!(
        <FixedOuterRef<'_> as SszTypeInfo>::ssz_fixed_len(),
        bytes.len()
    );

    let view = FixedOuterRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.to_owned(), owned);
}

#[test]
fn view_tree_hash_agrees_with_owned() {
    let owned = sample_mixed();
    let bytes = owned.as_ssz_bytes();
    let view = MixedOuterRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(
        view.tree_hash_root::<tree_hash::Sha256Hasher>(),
        owned.tree_hash_root::<tree_hash::Sha256Hasher>()
    );
}

/// Regression test for reading variable offsets from the field's own slot:
/// the offset entry of `entries` sits at position 0 of the fixed portion,
/// followed by `name`. An end-packed offset table reading would interpret
/// `name`'s bytes as the offset and reject valid owned encodings.
#[test]
fn variable_before_fixed_view_agrees_with_owned_encoding() {
    let owned = VarThenFixed {
        entries: VariableList::new(vec![1, 2, 3]).expect("within bound"),
        name: 0xDEADBEEF,
    };
    let bytes = owned.as_ssz_bytes();

    let view = VarThenFixedRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.entries().expect("entries").len(), 3);
    assert_eq!(view.name().expect("name"), 0xDEADBEEF);
    assert_eq!(view.to_owned(), owned);
    assert_eq!(
        view.tree_hash_root::<tree_hash::Sha256Hasher>(),
        owned.tree_hash_root::<tree_hash::Sha256Hasher>()
    );
}

#[test]
fn interleaved_view_agrees_with_owned_encoding() {
    let owned = Interleaved {
        head: VariableList::new(vec![0xAA, 0xBB]).expect("within bound"),
        mid: 7,
        tail: VariableList::new(vec![0xCC]).expect("within bound"),
    };
    let bytes = owned.as_ssz_bytes();

    let view = InterleavedRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.head().expect("head").to_owned(), vec![0xAA, 0xBB]);
    assert_eq!(view.mid().expect("mid"), 7);
    assert_eq!(view.tail().expect("tail").to_owned(), vec![0xCC]);
    assert_eq!(view.to_owned(), owned);
    assert_eq!(
        view.tree_hash_root::<tree_hash::Sha256Hasher>(),
        owned.tree_hash_root::<tree_hash::Sha256Hasher>()
    );
}

#[test]
fn basic_view_tree_hash_agrees_with_owned() {
    let owned = BasicPair {
        tag: 0x11,
        b: 0x22334455,
    };
    let bytes = owned.as_ssz_bytes();

    let view = BasicPairRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(
        view.tree_hash_root::<tree_hash::Sha256Hasher>(),
        owned.tree_hash_root::<tree_hash::Sha256Hasher>()
    );
}
