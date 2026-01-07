//! # SSZ Codegen
//!
//! A codegen tool that parses simplified Python SSZ (Simple Serialize) definitions using
//! `sizzle-parser` and generates Rust code for it utilizing `ssz_derive`'s derive macros.

use std::{collections::HashSet, error, fs, path::Path};

use prettyplease::unparse;
#[cfg(any(test, doctest))]
use serde as _;
use sizzle_parser::parse_str_schema;
use ssz as _;
use ssz_derive as _;
use ssz_primitives as _;
use ssz_types as _;
use syn::parse_str;
#[cfg(any(test, doctest))]
use toml as _;
use tree_hash as _;
use tree_hash_derive as _;

use crate::derive_config::DeriveConfig;

/// Controls how modules are generated in the output.
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
pub mod derive_config;
pub mod files;
pub mod pragma;
pub mod types;

/// Run the code generation process in a build script (build.rs).
///
/// This function:
/// 1. Reads all Pythonic SSZ definition files from the input directory
/// 2. Generates Rust code for each file separately
/// 3. Writes the generated code to the output directory, with the same file name but with a .rs
///    extension
/// 4. Outputs Cargo instructions to rerun the build script when any input file changes
///
/// # Arguments
///
/// * `entry_points` - Paths to the entrypoint SSZ definition files. Entry points can safely import
///   each other without causing duplicate definitions. If the same file is listed multiple times,
///   it will only be parsed once. The order of entry points does not affect duplicate prevention.
/// * `base_dir` - Path to the base directory of the SSZ definition files
/// * `crates` - A slice of strings representing the external crates you want to import in your ssz
///   schema
/// * `output_file_path` - Path where the generated Rust code files will be written
/// * `module_generation` - Module generation strategy.
///
/// # Example
///
/// ```ignore
/// // In build.rs
/// use ssz_codegen::build_ssz_files;
/// fn main() {
///     let out_dir = Path::new(env::var("OUT_DIR").unwrap().as_str()).join("generated_ssz.rs");
///     build_ssz_files(
///         &["test_1.ssz", "test_2.ssz"],
///         "specs/",
///         &["ssz_defs_1", "ssz_defs_2"],
///         out_dir.to_str().unwrap(),
///         ModuleGeneration::NestedModules, // Use default module generation
///     )
///     .expect("Failed to generate SSZ types");
/// }
/// ```
pub fn build_ssz_files(
    entry_points: &[&str],
    base_dir: &str,
    crates: &[&str],
    output_file_path: &str,
    module_generation: ModuleGeneration,
) -> Result<(), Box<dyn error::Error>> {
    let files = files::read_entrypoint_ssz(entry_points, base_dir)?;
    println!("cargo:rerun-if-changed={base_dir}");
    let (parsing_order, schema_map) = parse_str_schema(&files, crates)?;

    // Track which paths are actual entry points (vs imported dependencies)
    let entry_point_paths: HashSet<_> = files.keys().cloned().collect();

    let generation_mode = module_generation;
    let rust_code = codegen::schema_map_to_rust_code(
        &parsing_order,
        &schema_map,
        generation_mode,
        &derive_config::DeriveConfig::default_defaults(),
        &entry_point_paths,
    );
    let pretty_rust_code = prettyplease::unparse(&syn::parse_str(&rust_code.to_string())?);
    let output_path = Path::new(output_file_path);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, pretty_rust_code)?;
    Ok(())
}

/// Same as `build_ssz_files` but allows specifying a derive configuration and/or TOML file.
pub fn build_ssz_files_with_derives(
    entry_points: &[&str],
    base_dir: &str,
    crates: &[&str],
    output_file_path: &str,
    module_generation: ModuleGeneration,
    derives: Option<DeriveConfig>,
    derives_toml_path: Option<&str>,
) -> Result<(), Box<dyn error::Error>> {
    let files = files::read_entrypoint_ssz(entry_points, base_dir)?;
    println!("cargo:rerun-if-changed={base_dir}");
    if let Some(path) = derives_toml_path {
        println!("cargo:rerun-if-changed={path}");
    }

    let (parsing_order, schema_map) = parse_str_schema(&files, crates)?;

    // Track which paths are actual entry points (vs imported dependencies)
    let entry_point_paths: HashSet<_> = files.keys().cloned().collect();

    // Load config: start with defaults, merge optional in-memory, then TOML (replacing fields)
    let mut cfg = DeriveConfig::default_defaults();
    if let Some(user_cfg) = derives {
        cfg = user_cfg;
    }
    if let Some(toml_path) = derives_toml_path
        && let Ok(content) = fs::read_to_string(toml_path)
        && let Ok(file_cfg) = DeriveConfig::from_toml_str(&content)
    {
        cfg = file_cfg; // per-type entries already replace semantics
    }

    let rust_code = codegen::schema_map_to_rust_code(
        &parsing_order,
        &schema_map,
        module_generation,
        &cfg,
        &entry_point_paths,
    );
    let pretty_rust_code = unparse(&parse_str(&rust_code.to_string())?);
    let output_path = Path::new(output_file_path);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, pretty_rust_code)?;
    Ok(())
}
