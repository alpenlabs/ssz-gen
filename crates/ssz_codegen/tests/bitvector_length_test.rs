#![allow(unused_crate_dependencies)]
#![allow(missing_docs)]

use std::fs;

use ssz_codegen::{ModuleGeneration, build_ssz_files};

#[test]
fn test_view_bitvector_length_uses_max_fields() {
    build_ssz_files(
        &["test_bitvector_len.ssz"],
        "tests/input",
        &[],
        "tests/output/test_bitvector_len.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let actual_output =
        fs::read_to_string("tests/output/test_bitvector_len.rs").expect("Failed to read output");

    assert!(
        actual_output.contains("let bitvector_length = 2usize;"),
        "Expected bitvector_length to use max_fields (9 -> 2 bytes)"
    );
}
