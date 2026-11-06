#![allow(missing_docs)]
#![allow(unused_crate_dependencies)]

use ssz::{Decode, DecodeError, Encode};
use ssz_derive::{Decode, Encode};
use ssz_types::Optional;

#[derive(Debug, PartialEq, Encode, Decode)]
#[ssz(struct_behaviour = "profile")]
struct ProfileInner {
    a: u8,
    b: Option<u16>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ProfileOuter {
    inner: ProfileInner,
}

#[test]
fn profile_nested_encoding_matches_profile_bytes() {
    let inner = ProfileInner { a: 1, b: Some(2) };
    let outer = ProfileOuter { inner };

    let mut expected = 4u32.to_le_bytes().to_vec();
    expected.extend_from_slice(&outer.inner.as_ssz_bytes());

    assert_eq!(outer.as_ssz_bytes(), expected);
}

// Profile with only required fixed-size fields: fixed-size, exactly 16 bytes.
#[derive(Debug, PartialEq, Encode, Decode)]
#[ssz(struct_behaviour = "profile")]
struct FixedProfile {
    balance: u64,
    nonce: u64,
}

// Fixed-size container embedding a fixed-size Profile.
#[derive(Debug, PartialEq, Encode, Decode)]
struct FixedProfileWrapper {
    state: FixedProfile,
    tag: u8,
}

// Profile with an optional field: variable-size because of the bitvector.
#[derive(Debug, PartialEq, Encode, Decode)]
#[ssz(struct_behaviour = "profile")]
struct ProfileWithOptionalField {
    balance: u64,
    extra: Optional<u64>,
}

fn fixed_profile() -> FixedProfile {
    FixedProfile {
        balance: 7,
        nonce: 0,
    }
}

#[test]
fn fixed_profile_round_trips() {
    let owned = fixed_profile();
    let bytes = owned.as_ssz_bytes();

    assert!(<FixedProfile as Decode>::is_ssz_fixed_len());
    assert_eq!(bytes.len(), 16);
    assert_eq!(FixedProfile::from_ssz_bytes(&bytes), Ok(owned));
}

#[test]
fn fixed_profile_rejects_trailing_bytes() {
    let mut bytes = fixed_profile().as_ssz_bytes();
    bytes.push(0xff);

    assert_eq!(
        FixedProfile::from_ssz_bytes(&bytes),
        Err(DecodeError::InvalidByteLength {
            len: 17,
            expected: 16,
        })
    );
}

#[test]
fn fixed_profile_rejects_short_input() {
    let mut bytes = fixed_profile().as_ssz_bytes();
    bytes.pop();

    assert_eq!(
        FixedProfile::from_ssz_bytes(&bytes),
        Err(DecodeError::InvalidByteLength {
            len: 15,
            expected: 16,
        })
    );
}

// The outer container checks its total length before decoding the embedded
// Profile, so it rejects trailing bytes on its own.
#[test]
fn container_with_fixed_profile_rejects_trailing_bytes() {
    let owned = FixedProfileWrapper {
        state: fixed_profile(),
        tag: 3,
    };
    let mut bytes = owned.as_ssz_bytes();
    assert_eq!(bytes.len(), 17);
    assert_eq!(FixedProfileWrapper::from_ssz_bytes(&bytes), Ok(owned));

    bytes.push(0xff);
    assert!(FixedProfileWrapper::from_ssz_bytes(&bytes).is_err());
}

// The two tests below exercise the variable-size decoder path. The optional
// field makes the Profile variable-size, so decoding goes through
// `SszDecoderBuilder` rather than the fixed-size branch. In both fixtures
// every active field is fixed-size, so there are no offsets to absorb extra
// bytes, and the decoder builder itself must reject the trailing byte.

#[test]
fn profile_with_inactive_optional_rejects_trailing_bytes() {
    let owned = ProfileWithOptionalField {
        balance: 7,
        extra: Optional::None,
    };
    let mut bytes = owned.as_ssz_bytes();
    assert_eq!(ProfileWithOptionalField::from_ssz_bytes(&bytes), Ok(owned));

    bytes.push(0xff);
    assert!(ProfileWithOptionalField::from_ssz_bytes(&bytes).is_err());
}

#[test]
fn profile_with_active_optional_rejects_trailing_bytes() {
    let owned = ProfileWithOptionalField {
        balance: 7,
        extra: Optional::Some(9),
    };
    let mut bytes = owned.as_ssz_bytes();
    assert_eq!(ProfileWithOptionalField::from_ssz_bytes(&bytes), Ok(owned));

    bytes.push(0xff);
    assert!(ProfileWithOptionalField::from_ssz_bytes(&bytes).is_err());
}
