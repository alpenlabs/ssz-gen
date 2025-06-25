//! Tests for the ssz_codegen crate.

use prettyplease as _;
use proc_macro2 as _;
use quote as _;
use sizzle_parser as _;
use ssz as _;
use ssz_derive as _;
use ssz_types as _;
use syn as _;
use tree_hash as _;
use tree_hash_derive as _;
use typenum as _;

use ssz_codegen::build_ssz_files;
use std::fs;

#[test]
fn test_basic_codegen() {
    build_ssz_files(&["test_1.ssz"], "tests/input", "tests/output/test_1.rs")
        .expect("Failed to generate SSZ types");

    let expected_output = fs::read_to_string("tests/expected_output/test_1.rs")
        .expect("Failed to read expected output");
    let actual_output =
        fs::read_to_string("tests/output/test_1.rs").expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_profile() {
    build_ssz_files(&["test_2.ssz"], "tests/input", "tests/output/test_2.rs")
        .expect("Failed to generate SSZ types");

    let expected_output = fs::read_to_string("tests/expected_output/test_2.rs")
        .expect("Failed to read expected output");
    let actual_output =
        fs::read_to_string("tests/output/test_2.rs").expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_imports() {
    build_ssz_files(
        &["test_import_1.ssz", "test_import_2.ssz"],
        "tests/input",
        "tests/output/test_import.rs",
    )
    .expect("Failed to generate SSZ types");

    let expected_output = fs::read_to_string("tests/expected_output/test_import.rs")
        .expect("Failed to read expected output");
    let actual_output =
        fs::read_to_string("tests/output/test_import.rs").expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}
