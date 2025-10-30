//! Tests for the ssz_codegen crate.

use std::fs;

use prettyplease as _;
use proc_macro2 as _;
use quote as _;
use serde as _;
use sizzle_parser as _;
use ssz as _;
use ssz_codegen::{ModuleGeneration, build_ssz_files};
use ssz_derive as _;
use ssz_types as _;
use syn as _;
use toml as _;
use tree_hash as _;
use tree_hash_derive as _;

#[test]
fn test_basic_codegen() {
    build_ssz_files(
        &["test_1.ssz"],
        "tests/input",
        &[],
        "tests/output/test_1.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let expected_output = fs::read_to_string("tests/expected_output/test_1.rs")
        .expect("Failed to read expected output");
    let actual_output =
        fs::read_to_string("tests/output/test_1.rs").expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_profile() {
    build_ssz_files(
        &["test_2.ssz"],
        "tests/input",
        &[],
        "tests/output/test_2.rs",
        ModuleGeneration::NestedModules,
    )
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
        &[],
        "tests/output/test_import.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let expected_output = fs::read_to_string("tests/expected_output/test_import.rs")
        .expect("Failed to read expected output");
    let actual_output =
        fs::read_to_string("tests/output/test_import.rs").expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_large_unions() {
    build_ssz_files(
        &["test_large_unions.ssz"],
        "tests/input",
        &[],
        "tests/output/test_large_unions.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let expected_output = fs::read_to_string("tests/expected_output/test_large_unions.rs")
        .expect("Failed to read expected output");
    let actual_output = fs::read_to_string("tests/output/test_large_unions.rs")
        .expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_nested_aliases() {
    build_ssz_files(
        &["test_nested_aliases.ssz"],
        "tests/input",
        &[],
        "tests/output/test_nested_aliases.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let expected_output = fs::read_to_string("tests/expected_output/test_nested_aliases.rs")
        .expect("Failed to read expected output");
    let actual_output = fs::read_to_string("tests/output/test_nested_aliases.rs")
        .expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_bitfields() {
    build_ssz_files(
        &["test_bitfields.ssz"],
        "tests/input",
        &[],
        "tests/output/test_bitfields.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let expected_output = fs::read_to_string("tests/expected_output/test_bitfields.rs")
        .expect("Failed to read expected output");
    let actual_output =
        fs::read_to_string("tests/output/test_bitfields.rs").expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_union_edge_cases() {
    build_ssz_files(
        &["test_union_edge_cases.ssz"],
        "tests/input",
        &[],
        "tests/output/test_union_edge_cases.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let expected_output = fs::read_to_string("tests/expected_output/test_union_edge_cases.rs")
        .expect("Failed to read expected output");
    let actual_output = fs::read_to_string("tests/output/test_union_edge_cases.rs")
        .expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_external_import() {
    build_ssz_files(
        &["test_external.ssz"],
        "tests/input",
        &["external_ssz"],
        "tests/output/test_external.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let expected_output = fs::read_to_string("tests/expected_output/test_external.rs")
        .expect("Failed to read expected output");
    let actual_output =
        fs::read_to_string("tests/output/test_external.rs").expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
#[should_panic(expected = "CyclicTypedefs")]
fn test_circular_dep() {
    build_ssz_files(
        &["test_circular_dep.ssz"],
        "tests/input",
        &[],
        "tests/output/test_circular_dep.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("This should panic due to circular dependency");
}

#[test]
#[should_panic(expected = "UnknownImportItem")]
fn test_unknown_import_item() {
    build_ssz_files(
        &["test_unknown_import_item.ssz"],
        "tests/input",
        &[],
        "tests/output/test_unknown_import_item.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("This should panic due to unknown import item");
}

#[test]
#[should_panic(expected = "DuplicateFieldName")]
fn test_duplicate_field_name() {
    build_ssz_files(
        &["test_duplicate_field_name.ssz"],
        "tests/input",
        &[],
        "tests/output/test_duplicate_field_name.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("This should panic due to duplicate field name");
}

#[test]
#[should_panic(expected = "DuplicateItemName")]
fn test_duplicate_item_name() {
    build_ssz_files(
        &["test_duplicate_item_name.ssz"],
        "tests/input",
        &[],
        "tests/output/test_duplicate_item_name.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("This should panic due to duplicate item name");
}

#[test]
#[should_panic(expected = "Optional fields are not allowed in Container classes")]
fn test_optional_field_container() {
    build_ssz_files(
        &["test_optional_field_container.ssz"],
        "tests/input",
        &[],
        "tests/output/test_optional_field_container.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("This should panic due to optional field in container");
}

#[test]
#[should_panic(expected = "All fields in StableContainer classes must be optional")]
fn test_stable_container_without_optional() {
    build_ssz_files(
        &["test_stable_container_without_optional.ssz"],
        "tests/input",
        &[],
        "tests/output/test_stable_container_without_optional.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("This should panic due to stable container without optional");
}

#[test]
#[should_panic(expected = "None is only allowed as the first variant in a Union")]
fn test_union_null_position() {
    build_ssz_files(
        &["test_union_null_position.ssz"],
        "tests/input",
        &[],
        "tests/output/test_union_null_position.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("This should panic due to none not being first in union");
}

#[test]
#[should_panic(expected = "Unions cannot be used anonymously unless they are Union[None, T]")]
fn test_anon_union() {
    build_ssz_files(
        &["test_anon_union.ssz"],
        "tests/input",
        &[],
        "tests/output/test_anon_union.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("This should panic due to anonymous union");
}

#[test]
#[should_panic(expected = "Profile classes cannot add new fields to their parent classes")]
fn test_profile_new_fields() {
    build_ssz_files(
        &["test_profile_new_fields.ssz"],
        "tests/input",
        &[],
        "tests/output/test_profile_new_fields.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("This should panic due to new fields in profile");
}

#[test]
#[should_panic(expected = "Inheritance field order violation")]
fn test_profile_field_order() {
    build_ssz_files(
        &["test_profile_field_order.ssz"],
        "tests/input",
        &[],
        "tests/output/test_profile_field_order.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("This should panic due to field order in profile");
}

#[test]
fn test_single_module_generation() {
    build_ssz_files(
        &["test_1.ssz"],
        "tests/input",
        &[],
        "tests/output/test_single_module.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("Failed to generate SSZ types with SingleModule");

    let expected_output = fs::read_to_string("tests/expected_output/test_single_module.rs")
        .expect("Failed to read expected single module output");
    let actual_output = fs::read_to_string("tests/output/test_single_module.rs")
        .expect("Failed to read actual single module output");

    assert_eq!(expected_output.trim(), actual_output.trim());
}

#[test]
fn test_flat_modules_generation() {
    build_ssz_files(
        &["test_1.ssz"],
        "tests/input",
        &[],
        "tests/output/test_flat_modules.rs",
        ModuleGeneration::FlatModules,
    )
    .expect("Failed to generate SSZ types with FlatModules");

    let expected_output = fs::read_to_string("tests/expected_output/test_flat_modules.rs")
        .expect("Failed to read expected flat modules output");
    let actual_output = fs::read_to_string("tests/output/test_flat_modules.rs")
        .expect("Failed to read actual flat modules output");

    assert_eq!(expected_output.trim(), actual_output.trim());
}

#[test]
fn test_nested_modules_is_default() {
    // Test that default is NestedModules
    build_ssz_files(
        &["test_1.ssz"],
        "tests/input",
        &[],
        "tests/output/test_default_nested.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with default");

    build_ssz_files(
        &["test_1.ssz"],
        "tests/input",
        &[],
        "tests/output/test_explicit_nested.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with explicit NestedModules");

    let default_output = fs::read_to_string("tests/output/test_default_nested.rs")
        .expect("Failed to read default output");
    let explicit_output = fs::read_to_string("tests/output/test_explicit_nested.rs")
        .expect("Failed to read explicit nested output");

    // They should be identical
    assert_eq!(default_output.trim(), explicit_output.trim());
}
