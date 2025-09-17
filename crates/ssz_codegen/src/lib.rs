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

use sizzle_parser::parse_str_schema;
use std::path::Path;

/// Controls how modules are generated in the output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModuleGeneration {
    /// Generate a single flat module with all definitions at the root level
    SingleModule,

    /// Generate flat modules without deep nesting (one level per file)
    FlatModules,

    /// Generate nested modules
    #[default]
    NestedModules,
}

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
/// * `entry_points` - Paths to the entrypoint SSZ definition files
/// * `base_dir` - Path to the base directory of the SSZ definition files
/// * `crates` - A slice of strings representing the external crates you want to import in your ssz schema
/// * `output_file_path` - Path where the generated Rust code files will be written
/// * `module_generation` - Optional module generation strategy, defaults to NestedModules if None
///
/// # Example
///
/// ```ignore
/// // In build.rs
/// use ssz_codegen::build_ssz_files;
/// fn main() {
///     let out_dir = Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join("generated_ssz.rs");
///     build_ssz_files(
///         &["test_1.ssz", "test_2.ssz"],
///         "specs/",
///         &["ssz_defs_1", "ssz_defs_2"],
///         out_dir.to_str().unwrap(),
///         None, // Use default module generation
///     )
///     .expect("Failed to generate SSZ types");
/// }
/// ```
pub fn build_ssz_files(
    entry_points: &[&str],
    base_dir: &str,
    crates: &[&str],
    output_file_path: &str,
    module_generation: Option<ModuleGeneration>,
) -> Result<(), Box<dyn std::error::Error>> {
    let files = files::read_entrypoint_ssz(entry_points, base_dir)?;
    println!("cargo:rerun-if-changed={base_dir}");
    let (parsing_order, schema_map) = parse_str_schema(&files, crates)?;
    let generation_mode = module_generation.unwrap_or_default();
    let rust_code = codegen::schema_map_to_rust_code(&parsing_order, &schema_map, generation_mode);
    let pretty_rust_code = prettyplease::unparse(&syn::parse_str(&rust_code.to_string())?);
    let output_path = Path::new(output_file_path);
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(output_path, pretty_rust_code)?;
    Ok(())
}
