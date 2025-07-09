//! Tests for the ssz_codegen crate.

use proc_macro2 as _;
use quote as _;
use ssz as _;
use ssz_derive as _;
use ssz_types as _;
use tree_hash as _;
use tree_hash_derive as _;
use typenum as _;

use sizzle_parser::SszSchema;
use sizzle_parser::parse_str_schema;
use std::path::Path;

fn read_schema(file_name: &Path) -> SszSchema {
    let content = std::fs::read_to_string(file_name).unwrap();
    parse_str_schema(&content).unwrap()
}

#[test]
fn test_codegen() {
    let schema_folder = Path::new("tests/input");
    let output_folder = Path::new("tests/output");
    let expected_output_folder = Path::new("tests/expected_output");

    let files = ssz_codegen::files::read_directory_ssz_files(schema_folder).unwrap();
    for (file_path, _content) in files {
        let path = schema_folder.join(file_path.clone());
        let schema = read_schema(&path);
        let rust_code = ssz_codegen::codegen::schema_to_rust_code(&schema);

        let rust_file_name = file_path.replace(".ssz", ".rs");
        let output_path = output_folder.join(rust_file_name.clone());
        if !output_path.exists() {
            std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
        }
        let pretty_rust_code =
            prettyplease::unparse(&syn::parse_str(&rust_code.to_string()).unwrap());
        std::fs::write(output_path, pretty_rust_code.clone()).unwrap();

        let expected_path = expected_output_folder.join(rust_file_name);
        let expected_code = std::fs::read_to_string(expected_path).unwrap();
        assert_eq!(pretty_rust_code, expected_code);
    }
}
