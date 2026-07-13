//! Regression tests: StableContainer/Profile views against the owned
//! encoding.
//!
//! The owned `ssz_derive` encoding serializes only the *active* fields after
//! the leading bitvector (EIP-7495): a `None` field contributes neither data
//! nor an offset slot. Views used to compute one static layout at codegen
//! time (assuming every field present and every `Optional` variable-size)
//! and read offsets from an end-packed table, which broke StableContainers
//! with inactive or fixed-size fields and Profiles mixing required
//! fixed-size fields after variable-size ones. Views now parse the
//! bitvector and derive the layout from the fields' owned encoding, so they
//! must agree with owned encode/decode and tree hashing.

#![allow(dead_code)]
#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use ssz_derive as _;
use ssz_primitives as _;
use tree_hash_derive as _;

// Include generated code
include!("expected_output/test_2.rs");

use ssz::{
    Decode, Encode,
    view::{DecodeView, SszTypeInfo},
};
use ssz_types::{BitList, BitVector, Optional, VariableList};
use tests::input::test_2::{
    Alpha, InnerBase, InnerBaseRef, InnerProfile1, InnerProfile1Ref, InnerProfile2,
    InnerProfile2Ref,
};
use tree_hash::TreeHash;

fn sample_alpha() -> Alpha {
    let mut bits = BitList::<32>::with_capacity(3).expect("within bound");
    bits.set(1, true).expect("within bound");
    Alpha {
        a: Optional::Some(7),
        b: Optional::Some(bits),
    }
}

/// Asserts the owned encoding round-trips and the view's tree hash agrees
/// with the owned one.
macro_rules! assert_agreement {
    ($owned_ty:ident, $ref_ty:ident, $owned:expr) => {{
        let owned = $owned;
        let bytes = owned.as_ssz_bytes();

        let decoded = $owned_ty::from_ssz_bytes(&bytes).expect("owned decode");
        assert_eq!(&decoded, owned, "owned round-trip");

        let view = $ref_ty::from_ssz_bytes(&bytes).expect("view decode");
        assert_eq!(
            view.tree_hash_root::<tree_hash::Sha256Hasher>(),
            owned.tree_hash_root::<tree_hash::Sha256Hasher>(),
            "view tree hash agrees with owned"
        );
    }};
}

#[test]
fn stable_container_view_all_some() {
    let mut z = BitVector::<16>::new();
    z.set(3, true).expect("within bound");
    let owned = InnerBase {
        x: Optional::Some(1),
        y: Optional::Some(VariableList::new(vec![2, 3]).expect("within bound")),
        z: Optional::Some(z),
        w: Optional::Some(sample_alpha()),
    };
    assert_agreement!(InnerBase, InnerBaseRef, &owned);

    let bytes = owned.as_ssz_bytes();
    let view = InnerBaseRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.x().expect("x"), Optional::Some(1));
    assert_eq!(view.to_owned(), owned);
}

/// Inactive fields shift every later field's position: `x` and `z` are
/// absent, so `y`'s offset slot sits at position 0 of the body and `w`'s
/// right after it.
#[test]
fn stable_container_view_skips_inactive_fields() {
    let owned = InnerBase {
        x: Optional::None,
        y: Optional::Some(VariableList::new(vec![9]).expect("within bound")),
        z: Optional::None,
        w: Optional::Some(sample_alpha()),
    };
    assert_agreement!(InnerBase, InnerBaseRef, &owned);

    let bytes = owned.as_ssz_bytes();
    let view = InnerBaseRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.x().expect("x"), Optional::None);
    assert_eq!(view.z().expect("z"), Optional::None);
    assert_eq!(view.to_owned(), owned);
}

/// An active `Optional[uint8]`/`Optional[Bitvector]` is inlined in the fixed
/// portion by the owned encoding (no offset slot); the old static layout
/// treated every `Optional` as variable-size.
#[test]
fn stable_container_view_inlines_fixed_size_fields() {
    let owned = InnerBase {
        x: Optional::Some(0xAB),
        y: Optional::None,
        z: Optional::Some(BitVector::<16>::new()),
        w: Optional::None,
    };
    assert_agreement!(InnerBase, InnerBaseRef, &owned);

    let bytes = owned.as_ssz_bytes();
    let view = InnerBaseRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.x().expect("x"), Optional::Some(0xAB));
    assert_eq!(view.y().expect("y"), Optional::None);
    assert_eq!(view.to_owned(), owned);
}

#[test]
fn stable_container_view_all_none() {
    let owned = InnerBase {
        x: Optional::None,
        y: Optional::None,
        z: Optional::None,
        w: Optional::None,
    };
    assert_agreement!(InnerBase, InnerBaseRef, &owned);

    let bytes = owned.as_ssz_bytes();
    let view = InnerBaseRef::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.w().expect("w"), Optional::None);
    assert_eq!(view.to_owned(), owned);
}

#[test]
fn stable_container_view_rejects_bits_beyond_field_count() {
    // InnerBase declares 4 fields in a StableContainer[8]; bit 5 set.
    let bytes = [0b0010_0000u8];
    assert!(InnerBase::from_ssz_bytes(&bytes).is_err());
    assert!(InnerBaseRef::from_ssz_bytes(&bytes).is_err());
}

#[test]
fn stable_container_view_is_variable_size() {
    assert!(!<InnerBaseRef<'_> as SszTypeInfo>::is_ssz_fixed_len());
}

/// Regression test for reading offsets from the field's own slot: `y` is a
/// required variable-size field followed by the required fixed-size `z`, so
/// `y`'s offset entry sits *before* `z` in the fixed portion, not at its
/// end.
#[test]
fn profile_view_fixed_field_after_variable() {
    let owned = InnerProfile2 {
        x: Optional::Some(9),
        y: VariableList::new(vec![4, 5, 6]).expect("within bound"),
        z: BitVector::<16>::new(),
    };
    assert_agreement!(InnerProfile2, InnerProfile2Ref, &owned);

    let bytes = owned.as_ssz_bytes();
    let view = InnerProfile2Ref::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.x().expect("x"), Optional::Some(9));
    assert_eq!(view.y().expect("y").to_owned(), vec![4, 5, 6]);
    assert_eq!(view.to_owned(), owned);
}

#[test]
fn profile_view_with_inactive_optional() {
    let owned = InnerProfile2 {
        x: Optional::None,
        y: VariableList::new(vec![4, 5]).expect("within bound"),
        z: BitVector::<16>::new(),
    };
    assert_agreement!(InnerProfile2, InnerProfile2Ref, &owned);

    let bytes = owned.as_ssz_bytes();
    let view = InnerProfile2Ref::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.x().expect("x"), Optional::None);
    assert_eq!(view.y().expect("y").to_owned(), vec![4, 5]);
    assert_eq!(view.to_owned(), owned);
}

#[test]
fn profile_view_mixed_required_and_optional() {
    let mut z = BitVector::<16>::new();
    z.set(15, true).expect("within bound");
    let owned = InnerProfile1 {
        x: 3,
        y: Optional::None,
        z: Optional::Some(z),
        w: Optional::None,
    };
    assert_agreement!(InnerProfile1, InnerProfile1Ref, &owned);

    let bytes = owned.as_ssz_bytes();
    let view = InnerProfile1Ref::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.x().expect("x"), 3);
    assert_eq!(view.y().expect("y"), Optional::None);
    assert_eq!(view.to_owned(), owned);
}

#[test]
fn profile_view_all_some() {
    let owned = InnerProfile1 {
        x: 3,
        y: Optional::Some(VariableList::new(vec![1]).expect("within bound")),
        z: Optional::Some(BitVector::<16>::new()),
        w: Optional::Some(sample_alpha()),
    };
    assert_agreement!(InnerProfile1, InnerProfile1Ref, &owned);

    let bytes = owned.as_ssz_bytes();
    let view = InnerProfile1Ref::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.to_owned(), owned);
}

/// An active optional whose inner encoding is zero-length (`Some` of an
/// empty list) must stay `Some`: the presence bit, not the slice length,
/// decides activeness.
#[test]
fn profile_active_optional_with_empty_inner() {
    let owned = InnerProfile1 {
        x: 3,
        y: Optional::Some(VariableList::new(vec![]).expect("within bound")),
        z: Optional::None,
        w: Optional::None,
    };
    assert_agreement!(InnerProfile1, InnerProfile1Ref, &owned);

    let bytes = owned.as_ssz_bytes();
    let view = InnerProfile1Ref::from_ssz_bytes(&bytes).expect("view decode");
    match view.y().expect("y") {
        Optional::Some(v) => assert_eq!(v.to_owned(), Vec::<u8>::new()),
        Optional::None => panic!("active empty list decoded as None"),
    }
    assert_eq!(view.to_owned(), owned);
}

#[test]
fn stable_container_active_optional_with_empty_inner() {
    let owned = InnerBase {
        x: Optional::None,
        y: Optional::Some(VariableList::new(vec![]).expect("within bound")),
        z: Optional::None,
        w: Optional::None,
    };
    assert_agreement!(InnerBase, InnerBaseRef, &owned);

    let bytes = owned.as_ssz_bytes();
    let view = InnerBaseRef::from_ssz_bytes(&bytes).expect("view decode");
    match view.y().expect("y") {
        Optional::Some(v) => assert_eq!(v.to_owned(), Vec::<u8>::new()),
        Optional::None => panic!("active empty list decoded as None"),
    }
    assert_eq!(view.to_owned(), owned);
}
