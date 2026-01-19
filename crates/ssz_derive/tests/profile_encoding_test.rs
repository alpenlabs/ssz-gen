#![allow(missing_docs)]
#![allow(unused_crate_dependencies)]

use ssz::Encode;
use ssz_derive::{Decode, Encode};

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
