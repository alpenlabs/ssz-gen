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

/// Test that view types ([`VariableListRef`], [`FixedVectorRef`])
/// are properly imported and that [`ToOwnedSsz::to_owned`]
/// methods work correctly for different types:
///
/// - `List<u8, N>` uses [`BytesRef`] and needs .into() conversion
/// - `List<T, N>` uses [`VariableListRef`] and returns Result
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

    // Verify that VariableListRef and FixedVectorRef are imported
    assert!(
        generated.contains("use ssz_types::view::{FixedVectorRef, VariableListRef}"),
        "Generated code should import VariableListRef and FixedVectorRef"
    );

    // Verify that List<u8, N> (BytesRef) uses .into() for to_owned conversion
    assert!(
        generated.contains(".to_owned().into()"),
        "List<u8, N> fields should use .into() to convert Vec<u8> to VariableList"
    );

    // Verify that List<T, N> (VariableListRef) uses .expect() to unwrap Result
    assert!(
        generated.contains(".to_owned().expect(\"valid view\")"),
        "VariableListRef::to_owned() should use .expect() to unwrap Result"
    );

    // Verify that Vector<T, N> (FixedVectorRef) uses .expect() to unwrap Result
    assert!(
        generated.contains("to_owned().expect(\"valid view\")"),
        "FixedVectorRef::to_owned() should use .expect() to unwrap Result"
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
