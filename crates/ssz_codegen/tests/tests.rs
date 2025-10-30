//! Tests for the ssz_codegen crate.

use std::fs;

use prettyplease as _;
use proc_macro2 as _;
use quote as _;
use serde as _;
use sizzle_parser as _;
use ssz as _;
use ssz_codegen::{ModuleGeneration, build_ssz_files, build_ssz_files_with_derives};
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

#[test]
fn test_allow_attribute_and_default_derives() {
    let out_path = "tests/output/test_derives_default.rs";
    build_ssz_files(
        &["test_1.ssz"],
        "tests/input",
        &[],
        out_path,
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let output = fs::read_to_string(out_path).expect("read output");
    assert!(
        output.contains("#![allow(unused_imports, reason = \"generated code using ssz-gen\")]")
    );
    // Owned derives should include Clone and SSZ required derives somewhere
    assert!(output.contains("Clone"));
    assert!(output.contains("Encode") && output.contains("Decode") && output.contains("TreeHash"));
    // View derives should include Copy and Clone on a Ref struct (scan near AlphaRef)
    let lines = output.lines().collect::<Vec<_>>();
    let mut ref_idx = None;
    for (i, line) in lines.iter().enumerate() {
        if line.contains("struct AlphaRef") {
            ref_idx = Some(i);
            break;
        }
    }
    let ref_idx = ref_idx.expect("AlphaRef not found");
    let mut found_attr = false;
    for line in lines.iter().take(ref_idx).skip(ref_idx.saturating_sub(8)) {
        if line.contains("#[derive(") {
            assert!(line.contains("Copy") && line.contains("Clone"));
            found_attr = true;
            break;
        }
    }
    assert!(found_attr, "derive attribute not found above AlphaRef")
}

#[test]
fn test_derives_toml_override() {
    let out_path = "tests/output/test_derives_toml.rs";
    build_ssz_files_with_derives(
        &["test_1.ssz"],
        "tests/input",
        &[],
        out_path,
        ModuleGeneration::NestedModules,
        None,
        Some("tests/derives.toml"),
    )
    .expect("Failed to generate SSZ types with derives config");

    let output = fs::read_to_string(out_path).expect("read output");
    // Find the derive attribute above Alpha, allowing for multi-line derives
    let lines = output.lines().collect::<Vec<_>>();
    let mut alpha_idx = None;
    for (i, line) in lines.iter().enumerate() {
        if line.contains("pub struct Alpha ") || line.contains("pub struct Alpha{") {
            alpha_idx = Some(i);
            break;
        }
    }
    let idx = alpha_idx.expect("Alpha struct not found");
    let mut derive_found = false;
    let mut derive_text = String::new();
    for line in lines.iter().take(idx).skip(idx.saturating_sub(8)) {
        if line.contains("#[derive(") {
            derive_text = line.to_string();
            derive_found = true;
            break;
        }
    }
    assert!(derive_found, "derive attribute not found above Alpha");
    // Per-type override for Alpha is ["Eq"], owned must still include Clone + SSZ derives
    assert!(derive_text.contains("Eq"));
    assert!(derive_text.contains("Clone"));
    assert!(
        derive_text.contains("Encode")
            && derive_text.contains("Decode")
            && derive_text.contains("TreeHash")
    );
    // And should not contain Debug (since per-type replaces default)
    assert!(!derive_text.contains("Debug"));
}
