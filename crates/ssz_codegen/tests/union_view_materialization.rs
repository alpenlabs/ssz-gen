//! Regression tests: union views materialize every variant shape.
//!
//! `test_union_edge_cases` only compares generated text against a golden, so it
//! cannot catch a variant whose emitted conversion does not compile — which is
//! how a `Vector[T, N]` variant went unnoticed: the generated arm materializes
//! through `ToOwnedSsz`, which `FixedVectorRef` did not implement.
//!
//! These tests compile the generated fixture and exercise the variants through
//! `try_to_owned`, which must report a malformed payload rather than panic.

#![allow(dead_code)]
#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use ssz::{Encode, view::DecodeView};
use ssz_derive as _;
use ssz_primitives as _;
use ssz_types::{FixedVector, VariableList, view::ToOwnedSsz};
use tree_hash_derive as _;

include!("expected_output/test_union_edge_cases.rs");

use tests::input::test_union_edge_cases::{ComplexUnion, ComplexUnionRef};

/// A `Vector[uint16, 5]` variant materializes through the trait.
#[test]
fn vector_variant_materializes() {
    let owned = ComplexUnion::Selector1(
        FixedVector::new(vec![1u16, 2, 3, 4, 5]).expect("exactly five elements"),
    );
    let bytes = owned.as_ssz_bytes();

    let view = ComplexUnionRef::from_ssz_bytes(&bytes).expect("view borrows");

    assert_eq!(
        ToOwnedSsz::<ComplexUnion>::try_to_owned(&view).expect("materializes"),
        owned
    );
}

/// A `List[uint8, 10]` variant materializes through the trait.
#[test]
fn list_variant_materializes() {
    let owned =
        ComplexUnion::Selector0(VariableList::new(vec![9u8, 8, 7]).expect("within the bound"));
    let bytes = owned.as_ssz_bytes();

    let view = ComplexUnionRef::from_ssz_bytes(&bytes).expect("view borrows");

    assert_eq!(
        ToOwnedSsz::<ComplexUnion>::try_to_owned(&view).expect("materializes"),
        owned
    );
}

/// An out-of-range selector is reported rather than panicking.
#[test]
fn out_of_range_selector_is_reported() {
    // ComplexUnion has four arms; 9 is not one of them.
    let bytes = [9u8, 0u8];

    let view = ComplexUnionRef::from_ssz_bytes(&bytes).expect("view borrows");

    assert!(ToOwnedSsz::<ComplexUnion>::try_to_owned(&view).is_err());
}
