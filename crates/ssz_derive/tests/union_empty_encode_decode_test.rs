//! Integration test for union enums with empty first variant using derive macros.
//!
//! Per SSZ spec, the None (empty) variant is only legal as the first option (index 0).
//! This test verifies that Union[None, T, ...] works correctly.

use darling as _;
use quote as _;
use ssz::{Decode, Encode};
use ssz_derive::{Decode, Encode};
use ssz_types as _;
use syn as _;

fn assert_encode_decode<T: Encode + Decode + PartialEq + std::fmt::Debug>(item: &T, bytes: &[u8]) {
    assert_eq!(item.as_ssz_bytes(), bytes, "encoding mismatch");
    let decoded = T::from_ssz_bytes(bytes).expect("decoding should succeed");
    assert_eq!(decoded, *item, "roundtrip mismatch");
}

/// Test union enum with empty first variant (the fix target)
#[derive(PartialEq, Debug, Encode, Decode)]
#[ssz(enum_behaviour = "union")]
enum UnionEmptyFirst {
    Empty,
    Value(u64),
}

#[test]
fn test_union_empty_first_variant() {
    // Empty variant should encode as selector 0 with no body
    assert_encode_decode(&UnionEmptyFirst::Empty, &[0]);

    // Value variant should encode as selector 1 with u64 body
    // THIS IS THE KEY TEST - before the fix, decoding would fail with UnionSelectorInvalid(1)
    assert_encode_decode(&UnionEmptyFirst::Value(42), &[1, 42, 0, 0, 0, 0, 0, 0, 0]);
    assert_encode_decode(&UnionEmptyFirst::Value(0), &[1, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_encode_decode(
        &UnionEmptyFirst::Value(u64::MAX),
        &[1, 255, 255, 255, 255, 255, 255, 255, 255],
    );
}

/// Test union with multiple variants after empty
#[derive(PartialEq, Debug, Encode, Decode)]
#[ssz(enum_behaviour = "union")]
enum UnionEmptyFirstMultiple {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
    Bytes(Vec<u8>),
}

#[test]
fn test_union_empty_first_multiple_variants() {
    // None variant (selector 0)
    assert_encode_decode(&UnionEmptyFirstMultiple::None, &[0]);

    // U8 variant (selector 1)
    assert_encode_decode(&UnionEmptyFirstMultiple::U8(42), &[1, 42]);
    assert_encode_decode(&UnionEmptyFirstMultiple::U8(0), &[1, 0]);
    assert_encode_decode(&UnionEmptyFirstMultiple::U8(255), &[1, 255]);

    // U16 variant (selector 2)
    assert_encode_decode(&UnionEmptyFirstMultiple::U16(1000), &[2, 232, 3]);
    assert_encode_decode(&UnionEmptyFirstMultiple::U16(0), &[2, 0, 0]);
    assert_encode_decode(&UnionEmptyFirstMultiple::U16(u16::MAX), &[2, 255, 255]);

    // U32 variant (selector 3)
    assert_encode_decode(&UnionEmptyFirstMultiple::U32(70000), &[3, 112, 17, 1, 0]);

    // Bytes variant (selector 4)
    assert_encode_decode(&UnionEmptyFirstMultiple::Bytes(vec![]), &[4]);
    assert_encode_decode(
        &UnionEmptyFirstMultiple::Bytes(vec![1, 2, 3]),
        &[4, 1, 2, 3],
    );
}

/// Test union with empty variant in a container
#[derive(PartialEq, Debug, Encode, Decode)]
#[ssz(struct_behaviour = "container")]
struct ContainerWithUnion {
    value: u32,
    state: UnionEmptyFirst,
}

#[test]
fn test_union_empty_in_container() {
    // Container with empty variant
    let container_empty = ContainerWithUnion {
        value: 100,
        state: UnionEmptyFirst::Empty,
    };
    let encoded = container_empty.as_ssz_bytes();
    let decoded = ContainerWithUnion::from_ssz_bytes(&encoded).expect("decoding should succeed");
    assert_eq!(
        decoded, container_empty,
        "roundtrip mismatch for empty variant"
    );

    // Container with non-empty variant
    let container_value = ContainerWithUnion {
        value: 200,
        state: UnionEmptyFirst::Value(42),
    };
    let encoded = container_value.as_ssz_bytes();
    let decoded = ContainerWithUnion::from_ssz_bytes(&encoded).expect("decoding should succeed");
    assert_eq!(
        decoded, container_value,
        "roundtrip mismatch for value variant"
    );
}

/// Test that invalid selectors are properly rejected
#[test]
fn test_invalid_selectors() {
    // For a 2-variant union (Empty + Value), only selectors 0 and 1 are valid
    let invalid_cases = vec![
        (vec![2], "selector 2"),
        (vec![3, 0, 0], "selector 3"),
        (vec![127], "selector 127"),
        (vec![255], "selector 255"),
    ];

    for (bytes, desc) in invalid_cases {
        let result = UnionEmptyFirst::from_ssz_bytes(&bytes);
        assert!(
            result.is_err(),
            "Should reject {} but decoded successfully",
            desc
        );

        if let Err(e) = result {
            let error_str = format!("{:?}", e);
            assert!(
                error_str.contains("UnionSelectorInvalid"),
                "Expected UnionSelectorInvalid for {}, got: {}",
                desc,
                error_str
            );
        }
    }
}
