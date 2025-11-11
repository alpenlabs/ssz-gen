// Test generic type generation
use std::{collections::HashMap, path::Path};
use sizzle_parser::parse_str_schema;
use ssz_codegen::generate_code;

fn main() {
    const SCHEMA: &str = r"
# Generic RawMerkleProof with type parameter H for hash type
#~# bound: H: MerkleHash
class RawMerkleProof[H](Container):
    cohashes: List[H, 1024]

# Generic MerkleProof that uses RawMerkleProof[H]
#~# bound: H: MerkleHash
class MerkleProof[H](Container):
    inner: RawMerkleProof[H]
    index: uint64

# CompactMmr64 structure (non-generic)
class CompactMmr64(Container):
    entries: uint64
    cap_log2: uint8
    roots: List[Vector[byte, 32], 64]
";

    let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);
    let (parsing_order, schemas) = parse_str_schema(&files, &[]).expect("Failed to parse schema");

    println!("Parsed {} schemas", schemas.len());

    for path in &parsing_order {
        let schema = schemas.get(path).unwrap();
        println!("\n=== Schema: {} ===", path.display());
        println!("Classes: {}", schema.classes().len());

        for class in schema.classes() {
            println!("\nClass: {}", class.name().0);
            println!("  Type params: {}", class.type_params().len());
            for tp in class.type_params() {
                println!("    - {} ({:?})", tp.name().0, tp.kind());
            }
            println!("  Fields: {}", class.fields().len());
            for field in class.fields() {
                println!("    - {}: {:?}", field.name().0, field.ty());
            }
        }
    }

    // Generate code
    println!("\n=== Generating Rust code ===");
    let generated = generate_code(&schemas, &parsing_order, ssz_codegen::ModuleGeneration::Single, None)
        .expect("Failed to generate code");

    println!("{}", generated);
}
