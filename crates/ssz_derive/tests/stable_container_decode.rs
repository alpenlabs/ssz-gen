#![allow(missing_docs)]
#![allow(unused_crate_dependencies)]

use ssz::{Decode, DecodeError};
use ssz_derive::Decode as DeriveDecode;
use ssz_types::{BitVector, Optional};

#[derive(Debug, PartialEq, DeriveDecode)]
#[ssz(struct_behaviour = "stable_container", max_fields = 8)]
struct TestStableContainer {
    a: Optional<u64>,
    b: Optional<u64>,
}

#[test]
fn stable_container_decode_rejects_short_bitvector() {
    let bytes = [];
    let err = TestStableContainer::from_ssz_bytes(&bytes).unwrap_err();
    assert!(matches!(err, DecodeError::InvalidByteLength { .. }));
}

#[test]
fn stable_container_decode_rejects_extra_active_bits() {
    let bytes = [0b1000_0000];
    let err = TestStableContainer::from_ssz_bytes(&bytes).unwrap_err();
    assert!(matches!(err, DecodeError::BytesInvalid(_)));
}
