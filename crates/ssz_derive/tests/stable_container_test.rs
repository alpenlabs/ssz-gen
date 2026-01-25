#![allow(missing_docs)]

use darling as _;
use quote as _;
use ssz::{Decode, Encode};
use ssz_derive::{Decode, Encode};
use ssz_types::{BitVector, Optional};
use syn as _;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "stable_container", max_fields = 16)]
struct TestStableContainer {
    field_a: Optional<u64>,
    field_b: Optional<u64>,
}

#[test]
fn test_stable_container_roundtrip_both_some() {
    let original = TestStableContainer {
        field_a: Optional::Some(100),
        field_b: Optional::Some(200),
    };

    let encoded = original.as_ssz_bytes();
    eprintln!("Both some encoded: {:?}", encoded);
    eprintln!("Both some length: {}", encoded.len());
    let decoded = TestStableContainer::from_ssz_bytes(&encoded).unwrap();

    assert_eq!(original, decoded);
    assert_eq!(decoded.field_a, Optional::Some(100));
    assert_eq!(decoded.field_b, Optional::Some(200));
}

#[test]
fn test_stable_container_roundtrip_both_none() {
    let original = TestStableContainer {
        field_a: Optional::None,
        field_b: Optional::None,
    };

    let encoded = original.as_ssz_bytes();
    let decoded = TestStableContainer::from_ssz_bytes(&encoded).unwrap();

    assert_eq!(original, decoded);
    assert_eq!(decoded.field_a, Optional::None);
    assert_eq!(decoded.field_b, Optional::None);
}

#[test]
fn test_stable_container_roundtrip_mixed() {
    let original = TestStableContainer {
        field_a: Optional::Some(42),
        field_b: Optional::None,
    };

    let encoded = original.as_ssz_bytes();
    let decoded = TestStableContainer::from_ssz_bytes(&encoded).unwrap();

    assert_eq!(original, decoded);
    assert_eq!(decoded.field_a, Optional::Some(42));
    assert_eq!(decoded.field_b, Optional::None);
}

#[test]
fn test_stable_container_ssz_bytes_len_matches_encoding() {
    let value = TestStableContainer {
        field_a: Optional::Some(0x1111_1111_1111_1111),
        field_b: Optional::None,
    };

    let bytes = value.as_ssz_bytes();
    assert_eq!(value.ssz_bytes_len(), bytes.len());
}
