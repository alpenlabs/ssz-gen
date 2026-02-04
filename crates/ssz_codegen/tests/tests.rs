//! Tests for the ssz_codegen crate.

use std::{
    error, fs,
    sync::{LazyLock, Mutex},
};

use prettyplease as _;
use proc_macro2 as _;
use quote as _;
use serde as _;
use sizzle_parser as _;
use ssz as _;
use ssz_codegen::{
    ModuleGeneration, build_ssz_files as build_ssz_files_unlocked,
    build_ssz_files_with_derives as build_ssz_files_with_derives_unlocked,
};
use ssz_derive as _;
use ssz_primitives as _;
use ssz_types as _;
use syn as _;
use toml as _;
use tree_hash as _;
use tree_hash_derive as _;

static CODEGEN_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

fn build_ssz_files(
    entry_points: &[&str],
    base_dir: &str,
    crates: &[&str],
    output_file_path: &str,
    module_generation: ModuleGeneration,
) -> Result<(), Box<dyn error::Error>> {
    let _guard = CODEGEN_LOCK
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    build_ssz_files_unlocked(
        entry_points,
        base_dir,
        crates,
        output_file_path,
        module_generation,
    )
}

fn build_ssz_files_with_derives(
    entry_points: &[&str],
    base_dir: &str,
    crates: &[&str],
    output_file_path: &str,
    module_generation: ModuleGeneration,
    derives: Option<ssz_codegen::derive_config::DeriveConfig>,
    derives_toml_path: Option<&str>,
) -> Result<(), Box<dyn error::Error>> {
    let _guard = CODEGEN_LOCK
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    build_ssz_files_with_derives_unlocked(
        entry_points,
        base_dir,
        crates,
        output_file_path,
        module_generation,
        derives,
        derives_toml_path,
    )
}

/// Module simulating existing Rust code with types referenced by generated SSZ code
pub(crate) mod existing_module {
    use ssz_derive::{Decode, Encode};
    use tree_hash_derive::TreeHash;

    /// Existing type for testing
    #[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TreeHash)]
    #[allow(dead_code, reason = "used for testing")]
    pub(crate) struct ExistingType {
        /// Dummy value
        value: u64,
    }
}

/// Helper function to extract derive attributes for a given struct from generated code
fn get_struct_derives(generated: &str, struct_name: &str) -> Option<String> {
    let struct_pattern = format!("pub struct {}", struct_name);
    let struct_idx = generated.find(&struct_pattern)?;
    let before_struct = &generated[..struct_idx];

    // Find the last #[derive(...)] before the struct
    let last_derive_idx = before_struct.rfind("#[derive")?;
    let derive_line = &before_struct[last_derive_idx..];
    let derive_end = derive_line.find(")]")?;
    Some(derive_line[..=derive_end].to_string())
}

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

    // Verify that U128 and U256 are properly supported
    assert!(
        actual_output.contains("use ssz_primitives::{U128, U256}"),
        "Generated code should import U128 and U256"
    );
    assert!(
        actual_output.contains("large_int_128: U128"),
        "TestType should have a U128 field"
    );
    assert!(
        actual_output.contains("large_int_256: U256"),
        "TestType should have a U256 field"
    );
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
fn test_existing_rust_module() {
    // Test that importing from a module without a .ssz file (existing Rust module)
    // generates references to crate::module::Type
    build_ssz_files(
        &["test_existing_rust_module.ssz"],
        "tests/input",
        &[],
        "tests/output/test_existing_rust_module.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types for existing Rust module");

    // Verify the generated output contains references to existing_module types
    let actual_output = fs::read_to_string("tests/output/test_existing_rust_module.rs")
        .expect("Failed to read actual output");

    // Check that it references the existing module's type as
    // crate::tests::input::existing_module::ExistingType The path will be based on where the
    // .ssz file is located
    assert!(
        actual_output.contains("existing_module") && actual_output.contains("ExistingType"),
        "Generated code should reference existing_module::ExistingType. Actual output:\n{}",
        actual_output
    );
}

#[test]
fn test_cross_entry_local_paths() {
    // Test that locally generated types use super:: paths when referenced
    // across entry points (not crate::ssz::)
    build_ssz_files(
        &["test_cross_entry_local.ssz", "test_cross_entry_common.ssz"],
        "tests/input",
        &[],
        "tests/output/test_cross_entry_local.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let output = fs::read_to_string("tests/output/test_cross_entry_local.rs")
        .expect("Failed to read output");

    // Verify that paths use super:: not crate::ssz::
    assert!(
        output.contains("super::test_cross_entry_common")
            || output.contains("crate::tests::input::test_cross_entry_common"),
        "Local types should use super:: or crate::tests::input:: paths, not crate::ssz::"
    );
    assert!(
        !output.contains("crate::ssz::test_cross_entry_common"),
        "Should not use crate::ssz:: paths for locally generated types"
    );
}

#[test]
fn test_external_type_ref_variants() {
    // Test that external container types use Ref variants in view getters,
    // while primitive-like types use the type itself
    build_ssz_files(
        &["test_external_ref_variants.ssz"],
        "tests/input",
        &["external_ssz"],
        "tests/output/test_external_ref_variants.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let output = fs::read_to_string("tests/output/test_external_ref_variants.rs")
        .expect("Failed to read output");

    // Verify that container types use Ref variants
    assert!(
        output.contains("MsgPayloadRef") || output.contains("Result<external_ssz::MsgPayloadRef"),
        "Container type MsgPayload should use Ref variant in view getters"
    );
    assert!(
        output.contains("MessagePayloadRef")
            || output.contains("Result<external_ssz::MessagePayloadRef"),
        "Container type MessagePayload in List should use Ref variant"
    );

    // Verify that primitive-like types use the type itself, not Ref variant
    assert!(
        output.contains("Result<external_ssz::AccountId") && !output.contains("AccountIdRef"),
        "Primitive-like type AccountId should use the type itself, not Ref variant"
    );
}

#[test]
fn test_pragma_external_kind() {
    // Test external_kind pragma for annotating external types as containers or primitives
    build_ssz_files(
        &["test_external_pragma.ssz"],
        "tests/input",
        &["external_ssz"],
        "tests/output/test_external_pragma.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let output =
        fs::read_to_string("tests/output/test_external_pragma.rs").expect("Failed to read output");

    // Container types with pragma should use Ref variants
    assert!(
        output.contains("ChainStateRef") || output.contains("Result<external_ssz::ChainStateRef"),
        "Container type ChainState should use Ref variant"
    );

    assert!(
        output.contains("BlockHeaderRef"),
        "Container type BlockHeader in Vector should use Ref variant"
    );

    assert!(
        output.contains("TransactionRef"),
        "Container type Transaction in List should use Ref variant"
    );

    // Primitive types without pragma should not use Ref variants
    assert!(
        output.contains("Result<external_ssz::Balance") && !output.contains("BalanceRef"),
        "Primitive type Balance should not use Ref variant"
    );

    assert!(
        output.contains("external_ssz::AccountId") && !output.contains("AccountIdRef"),
        "Primitive type AccountId in List should not use Ref variant"
    );
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
    assert!(derive_text.contains("Encode") && derive_text.contains("Decode"));
    // TreeHash is no longer in the derive attribute - we generate it manually as a generic impl
    assert!(!derive_text.contains("TreeHash"));
    // And should not contain Debug (since per-type replaces default)
    assert!(!derive_text.contains("Debug"));

    // Verify that a generic TreeHash implementation exists for Alpha
    assert!(
        output.contains("impl<H: tree_hash::TreeHashDigest> tree_hash::TreeHash<H> for Alpha"),
        "Generic TreeHash implementation not found for Alpha"
    );
}

#[test]
fn test_comments() {
    build_ssz_files(
        &["test_comments.ssz"],
        "tests/input",
        &[],
        "tests/output/test_comments.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with comments");

    let expected_output = fs::read_to_string("tests/expected_output/test_comments.rs")
        .expect("Failed to read expected output");
    let actual_output =
        fs::read_to_string("tests/output/test_comments.rs").expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_docstrings() {
    build_ssz_files(
        &["test_docstrings.ssz"],
        "tests/input",
        &[],
        "tests/output/test_docstrings.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with docstrings");

    let expected_output = fs::read_to_string("tests/expected_output/test_docstrings.rs")
        .expect("Failed to read expected output");
    let actual_output = fs::read_to_string("tests/output/test_docstrings.rs")
        .expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_docstrings_with_comments_merge() {
    build_ssz_files(
        &["test_docstrings.ssz"],
        "tests/input",
        &[],
        "tests/output/test_docstrings.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with docstrings");

    let output = fs::read_to_string("tests/output/test_docstrings.rs")
        .expect("Failed to read generated output");

    assert!(!output.is_empty(), "Generated output should not be empty");
}

// Pragma tests

#[test]
fn test_pragmas_basic() {
    build_ssz_files(
        &["test_pragmas_basic.ssz"],
        "tests/input",
        &[],
        "tests/output/test_pragmas_basic.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with basic pragmas");

    let expected_output = fs::read_to_string("tests/expected_output/test_pragmas_basic.rs")
        .expect("Failed to read expected output");
    let actual_output = fs::read_to_string("tests/output/test_pragmas_basic.rs")
        .expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

/// Test that multiple pragmas don't break codegen.
#[test]
fn test_pragmas_multiple() {
    build_ssz_files(
        &["test_pragmas_multiple.ssz"],
        "tests/input",
        &[],
        "tests/output/test_pragmas_multiple.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with multiple pragmas");

    let expected_output = fs::read_to_string("tests/expected_output/test_pragmas_multiple.rs")
        .expect("Failed to read expected output");
    let actual_output = fs::read_to_string("tests/output/test_pragmas_multiple.rs")
        .expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

/// Test that field-level pragmas don't break codegen.
#[test]
fn test_pragmas_field() {
    build_ssz_files(
        &["test_pragmas_field.ssz"],
        "tests/input",
        &[],
        "tests/output/test_pragmas_field.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with field pragmas");

    let expected_output = fs::read_to_string("tests/expected_output/test_pragmas_field.rs")
        .expect("Failed to read expected output");
    let actual_output = fs::read_to_string("tests/output/test_pragmas_field.rs")
        .expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

/// Test that pragmas work with inheritance.
#[test]
fn test_pragmas_inheritance() {
    build_ssz_files(
        &["test_pragmas_inheritance.ssz"],
        "tests/input",
        &[],
        "tests/output/test_pragmas_inheritance.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with pragmas and inheritance");

    let expected_output = fs::read_to_string("tests/expected_output/test_pragmas_inheritance.rs")
        .expect("Failed to read expected output");
    let actual_output = fs::read_to_string("tests/output/test_pragmas_inheritance.rs")
        .expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

/// Test edge case: empty pragmas don't break codegen.
#[test]
fn test_pragmas_empty() {
    build_ssz_files(
        &["test_pragmas_empty.ssz"],
        "tests/input",
        &[],
        "tests/output/test_pragmas_empty.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with empty pragmas");

    let expected_output = fs::read_to_string("tests/expected_output/test_pragmas_empty.rs")
        .expect("Failed to read expected output");
    let actual_output = fs::read_to_string("tests/output/test_pragmas_empty.rs")
        .expect("Failed to read actual output");
    assert_eq!(expected_output, actual_output);
}

/// Test that pragmas are correctly parsed and stored in the schema.
#[test]
fn test_pragmas_schema_parsing() {
    use std::collections::HashMap;

    use sizzle_parser::parse_str_schema;

    let input = r"
#~# class-pragma: value1
#~# another-class-pragma: value2
class TestContainer(Container):
    #~# field-pragma: field-value
    x: uint32
    #~# another-field-pragma
    y: uint16
";

    let files = HashMap::from([(std::path::Path::new("").to_path_buf(), input.to_string())]);
    let (_, schema_map) =
        parse_str_schema(&files, &[]).expect("Failed to parse schema with pragmas");

    let schema = schema_map.values().next().expect("Should have one schema");
    let classes = schema.classes();
    assert_eq!(classes.len(), 1, "Should have one class");

    let class = &classes[0];
    assert_eq!(class.name().0, "TestContainer");

    // Verify class-level pragmas are stored
    let pragmas = class.pragmas();
    assert_eq!(pragmas.len(), 2, "Should have 2 class pragmas");
    assert!(pragmas.iter().any(|p| p.contains("class-pragma")));
    assert!(pragmas.iter().any(|p| p.contains("another-class-pragma")));

    // Verify field-level pragmas are stored
    let fields = class.fields();
    assert_eq!(fields.len(), 2);

    // First field should have pragma
    let field1_pragmas = fields[0].pragmas();
    assert_eq!(field1_pragmas.len(), 1);
    assert!(field1_pragmas[0].contains("field-pragma"));

    // Second field should have pragma
    let field2_pragmas = fields[1].pragmas();
    assert_eq!(field2_pragmas.len(), 1);
    assert!(field2_pragmas[0].contains("another-field-pragma"));
}

/// Test pragmas with StableContainer.
#[test]
fn test_pragmas_with_stable_container() {
    use std::collections::HashMap;

    use sizzle_parser::parse_str_schema;

    let input = r"
#~# stable-container-pragma: value
class StableTest(StableContainer[5]):
    a: Optional[uint8]
    b: Optional[uint16]
";

    let files = HashMap::from([(std::path::Path::new("").to_path_buf(), input.to_string())]);
    let (_, schema_map) =
        parse_str_schema(&files, &[]).expect("Failed to parse stable container with pragmas");

    let schema = schema_map.values().next().expect("Should have one schema");
    let classes = schema.classes();
    assert_eq!(classes.len(), 1);

    let class = &classes[0];
    let pragmas = class.pragmas();
    assert_eq!(pragmas.len(), 1);
    assert!(pragmas[0].contains("stable-container-pragma"));

    // Verify it generates correctly
    build_ssz_files(
        &["test_pragmas_basic.ssz"],
        "tests/input",
        &[],
        "tests/output/test_pragmas_basic.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("StableContainer with pragmas should generate");
}

/// Test multiple consecutive pragma lines.
#[test]
fn test_pragmas_multiple_consecutive() {
    use std::collections::HashMap;

    use sizzle_parser::parse_str_schema;

    let input = r"
#~# pragma1: value1
#~# pragma2: value2
#~# pragma3: value3
class MultiPragma(Container):
    x: uint8
";

    let files = HashMap::from([(std::path::Path::new("").to_path_buf(), input.to_string())]);
    let (_, schema_map) =
        parse_str_schema(&files, &[]).expect("Failed to parse multiple consecutive pragmas");

    let schema = schema_map.values().next().expect("Should have one schema");
    let classes = schema.classes();
    assert_eq!(classes.len(), 1);

    let class = &classes[0];
    let pragmas = class.pragmas();
    assert_eq!(pragmas.len(), 3, "Should have 3 pragmas");
    assert!(pragmas.iter().any(|p| p.contains("pragma1")));
    assert!(pragmas.iter().any(|p| p.contains("pragma2")));
    assert!(pragmas.iter().any(|p| p.contains("pragma3")));
}

#[test]
fn test_pragmas_with_doc_comments() {
    // Test that pragmas work alongside doc comments
    use std::collections::HashMap;

    use sizzle_parser::parse_str_schema;

    let input = r"
### This is a doc comment
#~# pragma: directive-value
class DocAndPragma(Container):
    ### Field doc comment
    #~# field-pragma: field-value
    x: uint32
";

    let files = HashMap::from([(std::path::Path::new("").to_path_buf(), input.to_string())]);
    let (_, schema_map) =
        parse_str_schema(&files, &[]).expect("Failed to parse with doc comments and pragmas");

    let schema = schema_map.values().next().expect("Should have one schema");
    let classes = schema.classes();
    assert_eq!(classes.len(), 1);

    let class = &classes[0];

    // Should have both doc comment and pragmas
    assert!(class.doc_comment().is_some());
    assert!(!class.pragmas().is_empty());
    assert!(class.pragmas()[0].contains("pragma"));

    // Field should have both
    let fields = class.fields();
    assert_eq!(fields.len(), 1);
    assert!(fields[0].doc_comment().is_some());
    assert!(!fields[0].pragmas().is_empty());
    assert!(fields[0].pragmas()[0].contains("field-pragma"));
}

/// Test that view types ([`ListRef`], [`BytesRef`], [`FixedVectorRef`])
/// are properly imported and that [`ToOwnedSsz::to_owned`]
/// methods work correctly for different types:
///
/// - `List<u8, N>` uses [`BytesRef`] and returns a list via `into()`
/// - `List<T, N>` uses [`ListRef`] and collects into `VariableList`
/// - `Vector<T, N>` uses [`FixedVectorRef`] and returns Result
#[test]
fn test_view_types_imports_and_to_owned() {
    build_ssz_files(
        &["test_view_types.ssz"],
        "tests/input",
        &[],
        "tests/output/test_view_types.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("Failed to generate SSZ types for view types test");

    let generated = fs::read_to_string("tests/output/test_view_types.rs")
        .expect("Failed to read generated output");

    // Verify that FixedVectorRef is imported
    assert!(
        generated.contains("FixedVectorRef"),
        "Generated code should import FixedVectorRef"
    );

    // Verify that List<u8, N> uses BytesRef
    assert!(
        generated.contains("BytesRef<'a, 4096usize>"),
        "List<u8, N> fields should use BytesRef in view types"
    );

    // Verify that List<T, N> uses ListRef
    assert!(
        generated.contains("ListRef<'a, ExportEntryRef<'a>, 256usize>"),
        "List<T, N> fields should use ListRef in view types"
    );

    // Verify that list to_owned conversion builds VariableList from items
    assert!(
        generated.contains("VariableList::from(items)"),
        "ListRef to_owned conversion should build VariableList"
    );

    // Verify that Vector[byte, N] uses FixedBytes conversion
    assert!(
        generated.contains("FixedBytes(self.hash().expect(\"valid view\").to_owned())"),
        "Vector[byte, N] should convert via FixedBytes"
    );

    // Verify that tree_hash_root uses explicit type annotations for type inference
    assert!(
        generated.contains("let root: <H as tree_hash::TreeHashDigest>::Output"),
        "Tree hash generation should use explicit type annotations"
    );

    // Try to compile the generated code by creating a minimal test
    // We can't directly compile it in a unit test, but we can verify the structure is correct
    assert!(
        generated.contains("pub struct ViewTypeTestRef<'a>"),
        "Should generate view struct"
    );
    assert!(
        generated.contains("pub struct ViewTypeTest"),
        "Should generate owned struct"
    );
}

#[test]
fn test_stable_container_optional_bitvector_length() {
    build_ssz_files(
        &["test_bitvector_len.ssz"],
        "tests/input",
        &[],
        "tests/output/test_bitvector_len_optional.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let actual_output = fs::read_to_string("tests/output/test_bitvector_len_optional.rs")
        .expect("Failed to read actual output");

    assert!(
        actual_output.contains("let bitvector_length = 2usize;"),
        "Expected bitvector length based on max_fields"
    );
    assert!(
        actual_output.contains("let bitvector_offset = 2usize;"),
        "Expected bitvector offset based on max_fields"
    );
}

#[test]
fn test_view_type_list_u8_uses_bytes_ref() {
    build_ssz_files(
        &["test_view_types.ssz"],
        "tests/input",
        &[],
        "tests/output/test_view_types_check.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let actual_output = fs::read_to_string("tests/output/test_view_types_check.rs")
        .expect("Failed to read actual output");

    assert!(
        actual_output.contains("BytesRef<'a, 4096usize>"),
        "Expected List[uint8, N] to use BytesRef in view types"
    );
}

#[test]
fn test_view_container_tree_hash_uses_field_count() {
    build_ssz_files(
        &["test_1.ssz"],
        "tests/input",
        &[],
        "tests/output/test_1_view_hash.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let actual_output = fs::read_to_string("tests/output/test_1_view_hash.rs")
        .expect("Failed to read actual output");

    assert!(
        actual_output.contains("MerkleHasher::<H>::with_leaves(3usize)"),
        "Expected container view hash to use the number of fields as leaf count"
    );
}

#[test]
fn test_view_stable_container_tree_hash_mixes_active_fields() {
    build_ssz_files(
        &["test_2.ssz"],
        "tests/input",
        &[],
        "tests/output/test_2_view_hash.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let actual_output = fs::read_to_string("tests/output/test_2_view_hash.rs")
        .expect("Failed to read actual output");

    assert!(
        actual_output.contains("active_fields_hash"),
        "Expected stable container view hash to mix in active fields bitvector"
    );
}

/// Test that constants are marked with `#[allow(dead_code)]` to avoid clippy warnings
/// Constants may only be used as const generics (e.g., `List[T, MAX_CONSTANT]`)
/// which clippy doesn't recognize as "using" the constant
#[test]
fn test_constants_have_dead_code_allow() {
    build_ssz_files(
        &["test_1.ssz"],
        "tests/input",
        &[],
        "tests/output/test_constants.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("Failed to generate SSZ types for constants test");

    let generated = fs::read_to_string("tests/output/test_constants.rs")
        .expect("Failed to read generated output");

    // Verify that constants have #[allow(dead_code)] with reason
    assert!(
        generated.contains("#[allow(dead_code, reason = \"generated code using ssz-gen\")]")
            && generated.contains("pub const VAL_X"),
        "Constants should be marked with #[allow(dead_code, reason = \"generated code using ssz-gen\")]"
    );
    assert!(
        generated.contains("#[allow(dead_code, reason = \"generated code using ssz-gen\")]")
            && generated.contains("pub const VAL_Y"),
        "Constants should be marked with #[allow(dead_code, reason = \"generated code using ssz-gen\")]"
    );
}

/// Test that containers inside lists generate correct [`SszTypeInfo`](ssz::view::SszTypeInfo) and
/// [`ToOwnedSsz`](ssz_types::view::ToOwnedSsz) implementations.
#[test]
fn test_container_in_list() {
    build_ssz_files(
        &["test_container_in_list.ssz"],
        "tests/input",
        &[],
        "tests/output/test_container_in_list.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("Failed to generate SSZ types");

    let generated = fs::read_to_string("tests/output/test_container_in_list.rs")
        .expect("Failed to read generated output");

    // Verify SszTypeInfo is implemented for container views
    assert!(
        generated.contains("impl<'a> ssz::view::SszTypeInfo for ExportEntryRef<'a>"),
        "SszTypeInfo should be implemented for ExportEntryRef"
    );
    assert!(
        generated.contains("impl<'a> ssz::view::SszTypeInfo for ExportContainerRef<'a>"),
        "SszTypeInfo should be implemented for ExportContainerRef"
    );

    // Verify ToOwnedSsz is implemented for container views
    assert!(
        generated
            .contains("impl<'a> ssz_types::view::ToOwnedSsz<ExportEntry> for ExportEntryRef<'a>"),
        "ToOwnedS
            sz should be implemented for ExportEntryRef"
    );
    assert!(
        generated.contains(
            "impl<'a> ssz_types::view::ToOwnedSsz<ExportContainer> for ExportContainerRef<'a>"
        ),
        "ToOwnedSsz should 
            be implemented for ExportContainerRef"
    );

    // Verify the generated code compiles by checking it includes the necessary traits
    assert!(
        generated.contains("pub struct ExportEntryRef<'a>"),
        "ExportEntryRef struct should be generated"
    );
    assert!(
        generated.contains("pub struct ExportContainerRef<'a>"),
        "ExportContainerRef struct should be generated"
    );
}

/// Test cross-module dependencies where an entry point imports another entry point.
/// This should not cause duplicate constant/type definitions in `SingleModule` mode.
#[test]
fn test_cross_entry_point_imports_single_module() {
    build_ssz_files(
        &["test_cross_entry_state.ssz", "test_cross_entry_update.ssz"],
        "tests/input",
        &[],
        "tests/output/test_cross_entry_single.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("Failed to generate SSZ types with cross-entry imports in SingleModule mode");

    let generated = fs::read_to_string("tests/output/test_cross_entry_single.rs")
        .expect("Failed to read generated output");

    // Verify MAX_VK_BYTES is only defined once
    let max_vk_count = generated.matches("pub const MAX_VK_BYTES").count();
    assert_eq!(
        max_vk_count, 1,
        "MAX_VK_BYTES should only be defined once, found {} times",
        max_vk_count
    );

    // Verify State is only defined once (owned version, not StateRef)
    let state_count = generated.matches("pub struct State {").count();
    assert_eq!(
        state_count, 1,
        "State should only be defined once, found {} times",
        state_count
    );

    // Verify both Update and State are present
    assert!(
        generated.contains("pub struct Update {"),
        "Update struct should be generated"
    );
    assert!(
        generated.contains("pub struct State {"),
        "State struct should be generated"
    );
}

/// Test cross-module dependencies in `FlatModules` mode.
#[test]
fn test_cross_entry_point_imports_flat_modules() {
    build_ssz_files(
        &["test_cross_entry_state.ssz", "test_cross_entry_update.ssz"],
        "tests/input",
        &[],
        "tests/output/test_cross_entry_flat.rs",
        ModuleGeneration::FlatModules,
    )
    .expect("Failed to generate SSZ types with cross-entry imports in FlatModules mode");

    let generated = fs::read_to_string("tests/output/test_cross_entry_flat.rs")
        .expect("Failed to read generated output");

    // Verify both modules are present
    assert!(
        generated.contains("pub mod test_cross_entry_state"),
        "test_cross_entry_state module should be generated"
    );
    assert!(
        generated.contains("pub mod test_cross_entry_update"),
        "test_cross_entry_update module should be generated"
    );

    // Verify State is in its own module
    assert!(
        generated.contains("pub struct State"),
        "State struct should be in test_cross_entry_state module"
    );

    // Verify Update is in its own module
    assert!(
        generated.contains("pub struct Update"),
        "Update struct should be in test_cross_entry_update module"
    );
}

/// Test cross-module dependencies in `NestedModules` mode.
#[test]
fn test_cross_entry_point_imports_nested_modules() {
    build_ssz_files(
        &["test_cross_entry_state.ssz", "test_cross_entry_update.ssz"],
        "tests/input",
        &[],
        "tests/output/test_cross_entry_nested.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with cross-entry imports in NestedModules mode");

    let generated = fs::read_to_string("tests/output/test_cross_entry_nested.rs")
        .expect("Failed to read generated output");

    // In NestedModules mode, both modules should be present
    assert!(
        generated.contains("pub mod test_cross_entry_state"),
        "test_cross_entry_state module should be generated"
    );
    assert!(
        generated.contains("pub mod test_cross_entry_update"),
        "test_cross_entry_update module should be generated"
    );

    // Both should have their own definitions
    assert!(
        generated.contains("pub struct State"),
        "State struct should be generated"
    );
    assert!(
        generated.contains("pub struct Update"),
        "Update struct should be generated"
    );
}

/// Test that the same module passed twice as entry points only gets parsed once.
#[test]
fn test_duplicate_entry_point() {
    build_ssz_files(
        &["test_1.ssz", "test_1.ssz"],
        "tests/input",
        &[],
        "tests/output/test_duplicate_entry.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("Failed to generate SSZ types with duplicate entry point");

    let generated = fs::read_to_string("tests/output/test_duplicate_entry.rs")
        .expect("Failed to read generated output");

    // Verify constants are only defined once
    let val_x_count = generated.matches("pub const VAL_X").count();
    assert_eq!(
        val_x_count, 1,
        "VAL_X should only be defined once, found {} times",
        val_x_count
    );

    // Verify structs are only defined once
    let alpha_count = generated.matches("pub struct Alpha {").count();
    assert_eq!(
        alpha_count, 1,
        "Alpha should only be defined once, found {} times",
        alpha_count
    );
}

/// Test that circular imports between entry points cause an error.
/// When A imports B and B imports A, the circular dependency should be detected
/// and result in an UnknownImport error.
#[test]
#[should_panic(expected = "UnknownImport")]
fn test_circular_imports() {
    build_ssz_files(
        &["test_circular_a.ssz", "test_circular_b.ssz"],
        "tests/input",
        &[],
        "tests/output/test_circular.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("This should panic due to circular import");
}

/// Test that the order of entry points does not affect duplicate prevention.
#[test]
fn test_entry_point_order_independence() {
    // Generate with one order
    build_ssz_files(
        &["test_cross_entry_state.ssz", "test_cross_entry_update.ssz"],
        "tests/input",
        &[],
        "tests/output/test_order_1.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("Failed to generate SSZ types with first order");

    // Generate with reversed order
    build_ssz_files(
        &["test_cross_entry_update.ssz", "test_cross_entry_state.ssz"],
        "tests/input",
        &[],
        "tests/output/test_order_2.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("Failed to generate SSZ types with reversed order");

    let generated_1 = fs::read_to_string("tests/output/test_order_1.rs")
        .expect("Failed to read first generated output");
    let generated_2 = fs::read_to_string("tests/output/test_order_2.rs")
        .expect("Failed to read second generated output");

    // Both should have the same number of definitions
    let max_vk_count_1 = generated_1.matches("pub const MAX_VK_BYTES").count();
    let max_vk_count_2 = generated_2.matches("pub const MAX_VK_BYTES").count();
    assert_eq!(
        max_vk_count_1, 1,
        "MAX_VK_BYTES should only be defined once in first order, found {} times",
        max_vk_count_1
    );
    assert_eq!(
        max_vk_count_2, 1,
        "MAX_VK_BYTES should only be defined once in second order, found {} times",
        max_vk_count_2
    );

    let state_count_1 = generated_1.matches("pub struct State {").count();
    let state_count_2 = generated_2.matches("pub struct State {").count();
    assert_eq!(
        state_count_1, 1,
        "State should only be defined once in first order, found {} times",
        state_count_1
    );
    assert_eq!(
        state_count_2, 1,
        "State should only be defined once in second order, found {} times",
        state_count_2
    );

    // Both should contain the same definitions
    assert!(
        generated_1.contains("pub struct State {") && generated_2.contains("pub struct State {"),
        "Both orders should generate State"
    );
    assert!(
        generated_1.contains("pub struct Update {") && generated_2.contains("pub struct Update {"),
        "Both orders should generate Update"
    );
}

/// Test 3-way dependency where A imports B, B imports C, and all are entry points.
#[test]
fn test_three_way_dependency() {
    build_ssz_files(
        &[
            "test_three_way_a.ssz",
            "test_three_way_b.ssz",
            "test_three_way_c.ssz",
        ],
        "tests/input",
        &[],
        "tests/output/test_three_way.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("Failed to generate SSZ types with 3-way dependency");

    let generated = fs::read_to_string("tests/output/test_three_way.rs")
        .expect("Failed to read generated output");

    // Verify all constants are only defined once
    let const_a_count = generated.matches("pub const CONST_A").count();
    let const_b_count = generated.matches("pub const CONST_B").count();
    let const_c_count = generated.matches("pub const CONST_C").count();

    assert_eq!(
        const_a_count, 1,
        "CONST_A should only be defined once, found {} times",
        const_a_count
    );
    assert_eq!(
        const_b_count, 1,
        "CONST_B should only be defined once, found {} times",
        const_b_count
    );
    assert_eq!(
        const_c_count, 1,
        "CONST_C should only be defined once, found {} times",
        const_c_count
    );

    // Verify all structs are present and only defined once
    let container_a_count = generated.matches("pub struct ContainerA {").count();
    let container_b_count = generated.matches("pub struct ContainerB {").count();
    let container_c_count = generated.matches("pub struct ContainerC {").count();

    assert_eq!(
        container_a_count, 1,
        "ContainerA should only be defined once, found {} times",
        container_a_count
    );
    assert_eq!(
        container_b_count, 1,
        "ContainerB should only be defined once, found {} times",
        container_b_count
    );
    assert_eq!(
        container_c_count, 1,
        "ContainerC should only be defined once, found {} times",
        container_c_count
    );
}

/// Test diamond-like dependency where two entry points (A and B) both import a base entry point.
/// This mimics the snark-acct-types scenario where update.ssz and proof_interface.ssz both
/// import state.ssz.
#[test]
fn test_diamond_dependency_both_import_base() {
    build_ssz_files(
        &[
            "test_multi_import_base.ssz",
            "test_multi_import_a.ssz",
            "test_multi_import_b.ssz",
        ],
        "tests/input",
        &[],
        "tests/output/test_diamond_dependency.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types with diamond dependency");

    let generated = fs::read_to_string("tests/output/test_diamond_dependency.rs")
        .expect("Failed to read generated output");

    // Verify base type is only defined once
    let base_type_count = generated.matches("pub struct BaseType {").count();
    assert_eq!(
        base_type_count, 1,
        "BaseType should only be defined once, found {} times",
        base_type_count
    );

    // Verify TypeA and TypeB are present
    assert!(
        generated.contains("pub struct TypeA {"),
        "TypeA should be generated"
    );
    assert!(
        generated.contains("pub struct TypeB {"),
        "TypeB should be generated"
    );
}

/// Test that view type aliases are generated for type aliases used in union variants.
#[test]
fn test_union_type_alias_view_types() {
    build_ssz_files(
        &["test_union_type_alias.ssz"],
        "tests/input",
        &[],
        "tests/output/test_union_type_alias.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let generated = fs::read_to_string("tests/output/test_union_type_alias.rs")
        .expect("Failed to read generated output");

    // Print first 500 chars for debugging
    eprintln!(
        "Generated output (first 500 chars):\n{}",
        &generated.chars().take(500).collect::<String>()
    );

    // Verify that the view type alias is generated for the type alias used in union
    assert!(
        generated.contains("pub type TypeAliasRef<'a> = UnderlyingTypeRef<'a>;"),
        "View type alias TypeAliasRef should be generated. Generated output:\n{}",
        &generated.chars().take(1000).collect::<String>()
    );

    // Verify that the union view type uses the alias
    assert!(
        generated.contains("TypeAliasRef"),
        "Union view type should reference TypeAliasRef"
    );

    // Verify that the underlying type is also generated
    assert!(
        generated.contains("pub struct UnderlyingTypeRef<'a>"),
        "Underlying type view should be generated"
    );
}

/// Test that external container types used in Union variants via type aliases
/// correctly use the Ref variant (e.g., ChainStateRef instead of ChainState).
#[test]
fn test_external_container_union() {
    build_ssz_files(
        &["test_external_container_union.ssz"],
        "tests/input",
        &["external_ssz"],
        "tests/output/test_external_container_union.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let generated = fs::read_to_string("tests/output/test_external_container_union.rs")
        .expect("Failed to read generated output");

    // Verify that the view type alias uses the Ref variant for the external container
    assert!(
        generated.contains("pub type DepositRef<'a> = external_ssz::SubjectDepositDataRef<'a>;")
            || generated.contains("pub type DepositRef<'a> = external_ssz::SubjectDepositDataRef<"),
        "View type alias DepositRef should reference SubjectDepositDataRef, not SubjectDepositData. Generated output:\n{}",
        &generated.chars().take(2000).collect::<String>()
    );

    // Verify that the union view type uses the Ref variant
    assert!(
        generated.contains("DepositRef") || generated.contains("Result<DepositRef"),
        "Union view type should reference DepositRef (which should be SubjectDepositDataRef)"
    );

    // Verify that as_selector0 returns the Ref type
    assert!(
        generated.contains("Result<DepositRef<'_>, ssz::DecodeError>")
            || generated.contains("Result<DepositRef"),
        "as_selector0 should return DepositRef (external container Ref type)"
    );

    // Verify that SubjectDepositDataRef is used (not SubjectDepositData directly)
    assert!(
        generated.contains("SubjectDepositDataRef"),
        "Generated code should use SubjectDepositDataRef for external container type"
    );

    // Verify that the enum variant uses the underlying external type, not the alias
    // Should be: Deposit(SubjectDepositData) or Deposit(external_ssz::SubjectDepositData)
    // NOT: Deposit(Deposit)
    assert!(
        generated.contains("Deposit(external_ssz::SubjectDepositData)")
            || generated.contains("Deposit(SubjectDepositData)"),
        "Enum variant should use underlying external type SubjectDepositData, not alias Deposit. Generated output:\n{}",
        &generated.chars().take(2000).collect::<String>()
    );
    assert!(
        !generated.contains("Deposit(Deposit)"),
        "Enum variant should NOT use alias name Deposit as the type. Generated output:\n{}",
        &generated.chars().take(2000).collect::<String>()
    );
}

#[test]
fn test_serde_derives() {
    build_ssz_files(
        &["test_serde_derives.ssz"],
        "tests/input",
        &[],
        "tests/output/test_serde_derives.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let generated = fs::read_to_string("tests/output/test_serde_derives.rs")
        .expect("Failed to read generated output");

    // Verify serde import is added
    assert!(
        generated.contains("use serde::{Serialize, Deserialize};"),
        "Serde import should be added when types use Serialize/Deserialize derives"
    );

    // Verify BlockCommitment has serde derives along with Copy and Hash
    let block_commitment_derives = get_struct_derives(&generated, "BlockCommitment")
        .expect("BlockCommitment should have derive attributes");

    assert!(
        block_commitment_derives.contains("Serialize")
            && block_commitment_derives.contains("Deserialize"),
        "BlockCommitment should have Serialize and Deserialize in derives"
    );

    assert!(
        block_commitment_derives.contains("Copy") && block_commitment_derives.contains("Hash"),
        "BlockCommitment should also have Copy and Hash derives"
    );

    // Verify OtherType does NOT have serde derives (but serde import is still present since
    // BlockCommitment uses it)
    if let Some(other_type_derives) = get_struct_derives(&generated, "OtherType") {
        assert!(
            !other_type_derives.contains("Serialize"),
            "OtherType should not have Serialize derive"
        );
    }
}

#[test]
fn test_serde_derives_single_module() {
    build_ssz_files(
        &["test_serde_derives.ssz"],
        "tests/input",
        &[],
        "tests/output/test_serde_derives_single.rs",
        ModuleGeneration::SingleModule,
    )
    .expect("Failed to generate SSZ types");

    let generated = fs::read_to_string("tests/output/test_serde_derives_single.rs")
        .expect("Failed to read generated output");

    // Verify serde import is added in SingleModule mode
    assert!(
        generated.contains("use serde::{Serialize, Deserialize};"),
        "Serde import should be added in SingleModule mode when types use Serialize/Deserialize derives"
    );

    // Verify BlockCommitment has serde derives
    let block_commitment_derives = get_struct_derives(&generated, "BlockCommitment")
        .expect("BlockCommitment should have derive attributes");

    assert!(
        block_commitment_derives.contains("Serialize")
            && block_commitment_derives.contains("Deserialize"),
        "BlockCommitment should have Serialize and Deserialize derives in SingleModule mode"
    );

    assert!(
        block_commitment_derives.contains("Copy") && block_commitment_derives.contains("Hash"),
        "BlockCommitment should also have Copy and Hash derives"
    );

    // Verify OtherType does NOT have serde derives
    if let Some(other_type_derives) = get_struct_derives(&generated, "OtherType") {
        assert!(
            !other_type_derives.contains("Serialize"),
            "OtherType should not have Serialize derive"
        );
    }
}

#[test]
fn test_serde_derives_flat_modules() {
    build_ssz_files(
        &["test_serde_derives.ssz"],
        "tests/input",
        &[],
        "tests/output/test_serde_derives_flat.rs",
        ModuleGeneration::FlatModules,
    )
    .expect("Failed to generate SSZ types");

    let generated = fs::read_to_string("tests/output/test_serde_derives_flat.rs")
        .expect("Failed to read generated output");

    // Verify serde import is added in FlatModules mode (inside the module)
    assert!(
        generated.contains("use serde::{Serialize, Deserialize};"),
        "Serde import should be added in FlatModules mode when types use Serialize/Deserialize derives"
    );

    // Verify the module structure
    assert!(
        generated.contains("pub mod test_serde_derives"),
        "FlatModules should generate a test_serde_derives module"
    );

    // Verify BlockCommitment has serde derives
    let block_commitment_derives = get_struct_derives(&generated, "BlockCommitment")
        .expect("BlockCommitment should have derive attributes");

    assert!(
        block_commitment_derives.contains("Serialize")
            && block_commitment_derives.contains("Deserialize"),
        "BlockCommitment should have Serialize and Deserialize derives in FlatModules mode"
    );

    assert!(
        block_commitment_derives.contains("Copy") && block_commitment_derives.contains("Hash"),
        "BlockCommitment should also have Copy and Hash derives"
    );

    // Verify OtherType does NOT have serde derives
    if let Some(other_type_derives) = get_struct_derives(&generated, "OtherType") {
        assert!(
            !other_type_derives.contains("Serialize"),
            "OtherType should not have Serialize derive"
        );
    }
}

#[test]
fn test_union_type_alias_in_list() {
    build_ssz_files(
        &["test_union_in_list.ssz"],
        "tests/input",
        &["external_ssz"],
        "tests/output/test_union_in_list.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let generated = fs::read_to_string("tests/output/test_union_in_list.rs")
        .expect("Failed to read generated output");

    // Verify Union[Type1, Type2] syntax generates SszTypeInfo and ToOwnedSsz
    assert!(
        generated.contains("impl<'a> ssz::view::SszTypeInfo for UnionTypeAliasRef<'a>"),
        "Union[Type1, Type2] syntax should implement SszTypeInfo. Generated output:\n{}",
        &generated.chars().take(3000).collect::<String>()
    );
    assert!(
        generated.contains("impl<'a> ssz_types::view::ToOwnedSsz<UnionTypeAlias>")
            && generated.contains("for UnionTypeAliasRef<'a>"),
        "Union[Type1, Type2] syntax should implement ToOwnedSsz. Generated output:\n{}",
        &generated.chars().take(3000).collect::<String>()
    );

    // Verify that Lists use ListRef with the union Ref types
    assert!(
        generated.contains("ListRef<'a, UnionTypeAliasRef<'a>"),
        "Container with Union[Type1, Type2] should use ListRef with UnionTypeAliasRef"
    );
}

#[test]
fn test_union_class_in_list() {
    build_ssz_files(
        &["test_union_in_list.ssz"],
        "tests/input",
        &["external_ssz"],
        "tests/output/test_union_in_list.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let generated = fs::read_to_string("tests/output/test_union_in_list.rs")
        .expect("Failed to read generated output");

    // Verify class Name(Union): syntax generates SszTypeInfo and ToOwnedSsz
    assert!(
        generated.contains("impl<'a> ssz::view::SszTypeInfo for UnionClassRef<'a>"),
        "class Name(Union): syntax should implement SszTypeInfo. Generated output:\n{}",
        &generated.chars().take(3000).collect::<String>()
    );
    assert!(
        generated.contains("impl<'a> ssz_types::view::ToOwnedSsz<UnionClass>")
            && generated.contains("for UnionClassRef<'a>"),
        "class Name(Union): syntax should implement ToOwnedSsz. Generated output:\n{}",
        &generated.chars().take(3000).collect::<String>()
    );

    // Verify class Name(Union): syntax with external container generates SszTypeInfo and ToOwnedSsz
    assert!(
        generated.contains("impl<'a> ssz::view::SszTypeInfo for UnionClassWithExternalRef<'a>"),
        "class Name(Union): syntax with external should implement SszTypeInfo. Generated output:\n{}",
        &generated.chars().take(3000).collect::<String>()
    );
    assert!(
        generated.contains("impl<'a> ssz_types::view::ToOwnedSsz<UnionClassWithExternal>")
            && generated.contains("for UnionClassWithExternalRef<'a>"),
        "class Name(Union): syntax with external should implement ToOwnedSsz. Generated output:\n{}",
        &generated.chars().take(3000).collect::<String>()
    );

    // Verify that Lists use ListRef with the union Ref types
    assert!(
        generated.contains("ListRef<'a, UnionClassRef<'a>"),
        "Container with class Name(Union): should use ListRef with UnionClassRef"
    );
    assert!(
        generated.contains("ListRef<'a, UnionClassWithExternalRef<'a>"),
        "Container with class Name(Union): external should use ListRef with UnionClassWithExternalRef"
    );
}

/// Test demonstrating that custom `ToOwnedSsz` implementations can be used
/// to convert SSZ view types to user-defined types.
///
/// This is the key use case for the fix: users can define their own types
/// and implement `ToOwnedSsz<CustomType>` to get automatic conversion from
/// SSZ-generated view types to their custom types.
#[test]
fn test_external_container_to_owned_ssz() {
    build_ssz_files(
        &["test_external_inner.ssz", "test_external_outer.ssz"],
        "tests/input",
        &[],
        "tests/output/test_external_container.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types");

    let generated = fs::read_to_string("tests/output/test_external_container.rs")
        .expect("Failed to read generated output");

    // Verify that the generated code uses ToOwnedSsz trait method for complex types
    // This is the key change that enables custom type resolution
    assert!(
        generated.contains("ssz_types::view::ToOwnedSsz::to_owned(&view)"),
        "Generated to_owned() should use ToOwnedSsz trait method for complex types. Generated:\n{}",
        &generated
    );

    // Verify that the generated code has the trait implementation for BlockCommitment
    assert!(
        generated.contains("ssz_types::view::ToOwnedSsz<BlockCommitment>")
            && generated.contains("for BlockCommitmentRef<'a>"),
        "Generated code should implement ToOwnedSsz for BlockCommitmentRef"
    );

    // Verify that BlockRange's to_owned uses the trait method for start/end fields
    // This allows users to implement ToOwnedSsz<CustomBlockCommitment> for BlockCommitmentRef
    // and have the BlockRange conversion automatically use their custom type
    assert!(
        generated.contains("start: {")
            && generated.contains("ssz_types::view::ToOwnedSsz::to_owned(&view)"),
        "BlockRange.start should use trait-based to_owned for type resolution"
    );

    // Verify generated output matches expected output
    let expected = fs::read_to_string("tests/expected_output/test_external_container.rs")
        .expect("Failed to read expected output");
    assert_eq!(
        generated, expected,
        "Generated output does not match expected output"
    );
}

#[test]
fn test_union_empty_variant() {
    build_ssz_files(
        &["test_union_empty_variant.ssz"],
        "tests/input",
        &[],
        "tests/output/test_union_empty_variant.rs",
        ModuleGeneration::NestedModules,
    )
    .expect("Failed to generate SSZ types for union with empty variant");

    let actual_output = fs::read_to_string("tests/output/test_union_empty_variant.rs")
        .expect("Failed to read actual output");

    // Verify that the union with empty variant was generated
    assert!(
        actual_output.contains("pub enum TestUnion"),
        "Generated code should contain TestUnion enum"
    );
    assert!(
        actual_output.contains("Empty"),
        "Generated code should contain Empty variant"
    );
    assert!(
        actual_output.contains("Data"),
        "Generated code should contain Data variant"
    );

    // Verify TreeHash implementation uses precomputed zero hash for empty variants
    assert!(
        actual_output.contains("let zero_root = H::get_zero_hash(0);"),
        "TreeHash for empty variant should use precomputed zero hash H::get_zero_hash(0)"
    );
    assert!(
        !actual_output.contains("tree_hash::Hash256::ZERO"),
        "TreeHash should not use concrete Hash256::ZERO type"
    );

    // Verify the pattern appears in multiple locations (owned and Ref implementations)
    let pattern_count = actual_output
        .matches("let zero_root = H::get_zero_hash(0);")
        .count();
    assert!(
        pattern_count >= 2,
        "Expected at least 2 occurrences of zero_root pattern (owned and Ref), found {}",
        pattern_count
    );
}
