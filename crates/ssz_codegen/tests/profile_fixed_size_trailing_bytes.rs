//! Owned and view decoding of fixed-size Profiles.
//!
//! A Profile with only required fixed-size fields has no presence bitvector
//! and an exact encoded length. Both the owned `ssz_derive` decoder and the
//! generated view reject input of any other length with `InvalidByteLength`,
//! and they agree on the error.

#![allow(dead_code)]
#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]

use ssz_derive as _;
use ssz_primitives as _;
use tree_hash_derive as _;

// Include generated code
include!("expected_output/test_profile_fixed_size.rs");

use ssz::{Decode, DecodeError, Encode, view::DecodeView};
use tests::input::test_profile_fixed_size::{StateV1, StateV1Ref, StateWrapper, StateWrapperRef};

fn sample_state() -> StateV1 {
    StateV1 {
        balance: 7,
        nonce: 0,
    }
}

#[test]
fn fixed_profile_is_fixed_size() {
    assert!(<StateV1 as Decode>::is_ssz_fixed_len());
    assert_eq!(<StateV1 as Decode>::ssz_fixed_len(), 16);
    assert_eq!(sample_state().as_ssz_bytes().len(), 16);
}

#[test]
fn fixed_profile_round_trips() {
    let owned = sample_state();
    let bytes = owned.as_ssz_bytes();

    assert_eq!(
        StateV1::from_ssz_bytes(&bytes).expect("owned decode"),
        owned
    );
    let view = StateV1Ref::from_ssz_bytes(&bytes).expect("view decode");
    assert_eq!(view.to_owned(), owned);
}

#[test]
fn fixed_profile_owned_decode_rejects_trailing_bytes() {
    let mut bytes = sample_state().as_ssz_bytes();
    bytes.push(0xff);

    assert_eq!(
        StateV1::from_ssz_bytes(&bytes),
        Err(DecodeError::InvalidByteLength {
            len: 17,
            expected: 16,
        })
    );
}

#[test]
fn fixed_profile_view_decode_rejects_trailing_bytes() {
    let mut bytes = sample_state().as_ssz_bytes();
    bytes.push(0xff);

    assert_eq!(
        StateV1Ref::from_ssz_bytes(&bytes).err(),
        Some(DecodeError::InvalidByteLength {
            len: 17,
            expected: 16,
        })
    );
}

#[test]
fn fixed_profile_owned_and_view_agree_on_trailing_bytes() {
    let mut bytes = sample_state().as_ssz_bytes();
    bytes.push(0xff);

    let owned_err =
        StateV1::from_ssz_bytes(&bytes).expect_err("owned decode must reject trailing bytes");
    let view_err =
        StateV1Ref::from_ssz_bytes(&bytes).expect_err("view decode must reject trailing bytes");
    assert_eq!(owned_err, view_err, "owned and view decoders must agree");
}

#[test]
fn fixed_profile_rejects_short_input() {
    let mut bytes = sample_state().as_ssz_bytes();
    bytes.pop();

    let expected = DecodeError::InvalidByteLength {
        len: 15,
        expected: 16,
    };
    assert_eq!(StateV1::from_ssz_bytes(&bytes), Err(expected.clone()));
    assert_eq!(StateV1Ref::from_ssz_bytes(&bytes).err(), Some(expected));
}

// The outer container checks its total length before decoding the embedded
// Profile, so it rejects trailing bytes on its own.
#[test]
fn container_with_fixed_profile_rejects_trailing_bytes() {
    let owned = StateWrapper {
        state: sample_state(),
        tag: 3,
    };
    let mut bytes = owned.as_ssz_bytes();
    assert_eq!(bytes.len(), 17);
    assert_eq!(
        StateWrapper::from_ssz_bytes(&bytes).expect("owned decode"),
        owned
    );

    bytes.push(0xff);
    assert!(StateWrapper::from_ssz_bytes(&bytes).is_err());
    assert!(StateWrapperRef::from_ssz_bytes(&bytes).is_err());
}
