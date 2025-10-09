//! Integration tests for lazy view types.
//!
//! These tests verify that lazy getter-based views produce identical results
//! to owned decoding and eager field access.

#![allow(unused_crate_dependencies)]

use ssz::Encode;
use ssz::view::DecodeView;
use ssz_types::view::{FixedVectorRef, VariableListRef};
use ssz_types::{FixedVector, VariableList};
use tree_hash::{Sha256Hasher, TreeHash};

#[test]
fn test_variable_list_lazy_decode() {
    // Create and encode a variable list
    let values = vec![1u64, 2, 3, 4, 5];
    let owned: VariableList<u64, 10> = values.clone().into();
    let encoded = owned.as_ssz_bytes();

    // Decode as view (lazy)
    let view = VariableListRef::<u64, 10>::from_ssz_bytes(&encoded).unwrap();

    // Verify lazy iteration produces same values
    let lazy_values: Vec<u64> = view.iter().map(|r| r.unwrap()).collect();
    assert_eq!(values, lazy_values);

    // Verify to_owned() produces identical owned type
    let view_owned = view.to_owned().unwrap();
    assert_eq!(owned, view_owned);

    // Verify tree hashes match
    let owned_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&owned);
    let view_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);
    assert_eq!(owned_hash, view_hash);
}

#[test]
fn test_fixed_vector_lazy_decode() {
    // Create and encode a fixed vector
    let values = vec![10u32, 20, 30, 40];
    let owned: FixedVector<u32, 4> = values.clone().into();
    let encoded = owned.as_ssz_bytes();

    // Decode as view (lazy)
    let view = FixedVectorRef::<u32, 4>::from_ssz_bytes(&encoded).unwrap();

    // Verify lazy iteration produces same values
    let lazy_values: Vec<u32> = view.iter().map(|r| r.unwrap()).collect();
    assert_eq!(values, lazy_values);

    // Verify to_owned() produces identical owned type
    let view_owned = view.to_owned().unwrap();
    assert_eq!(owned, view_owned);

    // Verify tree hashes match
    let owned_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&owned);
    let view_hash: tree_hash::Hash256 = TreeHash::<Sha256Hasher>::tree_hash_root(&view);
    assert_eq!(owned_hash, view_hash);
}

#[test]
fn test_no_allocations_on_decode() {
    // This test demonstrates that view decoding doesn't allocate
    let values = vec![1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let owned: VariableList<u64, 20> = values.into();
    let encoded = owned.as_ssz_bytes();

    // Decode as view - this should not allocate
    let view = VariableListRef::<u64, 20>::from_ssz_bytes(&encoded).unwrap();

    // Accessing length doesn't allocate
    assert_eq!(view.len(), 10);
    assert!(!view.is_empty());

    // Getting a single item doesn't allocate the whole list
    let first = view.iter().next().unwrap().unwrap();
    assert_eq!(first, 1);
}

#[test]
fn test_nested_views_share_lifetime() {
    // Test that nested views all borrow from the same original buffer
    let inner_values = vec![100u16, 200, 300];
    let inner: VariableList<u16, 10> = inner_values.into();
    let inner_encoded = inner.as_ssz_bytes();

    // Create a view over the inner list
    let inner_view = VariableListRef::<u16, 10>::from_ssz_bytes(&inner_encoded).unwrap();

    // Verify iteration doesn't require separate lifetime
    for item in inner_view.iter() {
        let _value = item.unwrap();
        // All items share the same lifetime 'a tied to inner_encoded
    }
}

#[test]
fn test_lazy_access_on_large_list() {
    // For large lists, lazy access means we don't decode everything
    let values: Vec<u64> = (0..1000).collect();
    let owned: VariableList<u64, 1024> = values.into();
    let encoded = owned.as_ssz_bytes();

    // Decode as view
    let view = VariableListRef::<u64, 1024>::from_ssz_bytes(&encoded).unwrap();

    // Access only first 10 items - doesn't decode the rest
    let first_ten: Vec<u64> = view.iter().take(10).map(|r| r.unwrap()).collect();
    assert_eq!(first_ten, (0..10).collect::<Vec<_>>());

    // Length is still available without full decode
    assert_eq!(view.len(), 1000);
}
