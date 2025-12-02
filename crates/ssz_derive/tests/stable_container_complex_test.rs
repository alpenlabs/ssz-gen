#![allow(missing_docs)]

use darling as _;
use quote as _;
use ssz::{Decode, Encode};
use ssz_derive::{Decode, Encode};
use ssz_types::{BitVector, Optional};
use syn as _;

// Simple container (like GamTxPayload)
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
struct Payload1 {
    value: u64,
}

// Another container variant
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
struct Payload2 {
    data: u32,
}

// Union of two containers (like TransactionPayload)
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[ssz(enum_behaviour = "union")]
enum PayloadUnion {
    Variant1(Payload1),
    Variant2(Payload2),
}

// StableContainer with Optional Union (like OLTransaction)
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[ssz(struct_behaviour = "stable_container", max_fields = 16)]
struct ComplexStableContainer {
    payload: Optional<PayloadUnion>,
    extra: Optional<u64>,
}

#[test]
fn test_complex_stable_both_some() {
    let original = ComplexStableContainer {
        payload: Optional::Some(PayloadUnion::Variant1(Payload1 { value: 100 })),
        extra: Optional::Some(42),
    };

    let encoded = original.as_ssz_bytes();
    println!("Encoded: {:?}", encoded);
    let decoded = ComplexStableContainer::from_ssz_bytes(&encoded).unwrap();

    assert_eq!(original, decoded);
}

#[test]
fn test_complex_stable_payload_none() {
    let original = ComplexStableContainer {
        payload: Optional::None,
        extra: Optional::Some(42),
    };

    let encoded = original.as_ssz_bytes();
    println!("Encoded: {:?}", encoded);
    let decoded = ComplexStableContainer::from_ssz_bytes(&encoded).unwrap();

    assert_eq!(original, decoded);
}

#[test]
fn test_complex_stable_both_none() {
    let original = ComplexStableContainer {
        payload: Optional::None,
        extra: Optional::None,
    };

    let encoded = original.as_ssz_bytes();
    println!("Encoded: {:?}", encoded);
    let decoded = ComplexStableContainer::from_ssz_bytes(&encoded).unwrap();

    assert_eq!(original, decoded);
}
