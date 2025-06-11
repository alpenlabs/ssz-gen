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

pub mod codegen;
pub mod files;
pub mod types;
