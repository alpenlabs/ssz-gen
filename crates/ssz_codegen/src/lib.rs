//! # SSZ Codegen
//!
//! A codegen tool that parses simplified Python SSZ (Simple Serialize) definitions using `sizzle-parser`
//! and generates Rust code for it utilizing `ssz_derive`'s derive macros.

use prettyplease as _;
use ssz as _;
use ssz_derive as _;
use ssz_types as _;
use tree_hash as _;
use tree_hash_derive as _;
use typenum as _;

use sizzle_parser::parse_str_schema;
use std::path::Path;

pub mod codegen;
pub mod files;
pub mod types;

/// Run the code generation process in a build script (build.rs).
///
/// This function:
/// 1. Reads all Pythonic SSZ definition files from the input directory
/// 2. Generates Rust code for each file separately
/// 3. Writes the generated code to the output directory, with the same file name but with a .rs extension
/// 4. Outputs Cargo instructions to rerun the build script when any input file changes
///
/// # Arguments
///
/// * `input_dir` - Path to the directory containing SSZ definition files
/// * `output_dir` - Path where the generated Rust code files will be written
///
/// # Example
///
/// ```ignore
/// // In build.rs
/// use ssz_codegen::build_ssz_files;
/// fn main() {
///     let out_dir = std::env::var("OUT_DIR").unwrap();
///     build_ssz_files("specs/", &out_dir).expect("Failed to generate SSZ types");
/// }
/// ```
pub fn build_ssz_files(
    input_dir: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(output_dir)?;
    let files = files::read_directory_ssz_files(Path::new(input_dir))?;

    for (file_path, content) in files {
        let schema = parse_str_schema(&content)?;
        let rust_code = codegen::schema_to_rust_code(&schema);
        let pretty_rust_code = prettyplease::unparse(&syn::parse_str(&rust_code.to_string())?);

        let rust_file_name = if let Some(pos) = file_path.rfind(".ssz") {
            let mut name = file_path.clone();
            name.replace_range(pos..pos + 4, ".rs");
            name
        } else {
            // Not supposed to happen since read_directory_ssz_files_recursive only reads files with .ssz extension
            panic!("File {file_path} does not have a .ssz extension");
        };

        let output_path = Path::new(output_dir).join(rust_file_name);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(output_path, pretty_rust_code)?;

        // Tell Cargo to rerun if this input file changes
        println!(
            "cargo:rerun-if-changed={}",
            Path::new(input_dir).join(file_path).display()
        );
    }

    // Also rerun if the input directory structure changes
    println!("cargo:rerun-if-changed={input_dir}");

    Ok(())
}
