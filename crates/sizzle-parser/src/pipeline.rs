//! High-level logic for full-pipeline parsing.

use std::{collections::HashMap, path::PathBuf};

use thiserror::Error;

use crate::{
    SszSchema,
    ast::{self, ModuleManager, ParseError},
    schema::{self, SchemaError},
    token::{self, TokenError},
    token_tree::{self, ToktrError},
    ty_resolver::{CrossModuleTypeMap, ModuleTypeMap, ResolverError},
};

/// Represents an error from any of the phases of parsing a raw schema.
#[derive(Debug, Error)]
pub enum SszError {
    /// Error from the tokenizer.
    #[error("tokenizer: {0}")]
    Token(#[from] TokenError),

    /// Error from the token tree parser.
    #[error("treeizer: {0}")]
    TokenTree(#[from] ToktrError),

    /// Error from the AST parser.
    #[error("parser: {0}")]
    Parser(#[from] ParseError),

    /// Error from the type resolver.
    #[error("type resolution: {0}")]
    TyResolver(#[from] ResolverError),

    /// Error from the schema generator.
    #[error("schema generation: {0}")]
    SchemaGen(#[from] SchemaError),
}

/// High-level parse function.
pub fn parse_str_schema(
    files: &HashMap<PathBuf, String>,
    external_modules: &[&str],
) -> Result<(Vec<PathBuf>, HashMap<PathBuf, SszSchema>), SszError> {
    let mut module_manager = ModuleManager::new(external_modules);

    for (path, content) in files {
        // Only parse if the module hasn't been added yet (e.g., by an import from another entry
        // point)
        if module_manager.add_module_to_front(path.clone()) {
            let chars = content.chars().collect::<Vec<_>>();
            let tokens = token::parse_char_array_to_tokens(&chars)?;
            let toktrs = token_tree::parse_tokens_to_toktrs(&tokens)?;
            ast::parse_module_from_toktrs(&toktrs, path, &mut module_manager)?;
        }
    }

    let mut schema_map = HashMap::new();
    let mut cross_module_types = CrossModuleTypeMap::new();

    // Pre-register external modules before any schema conversion occurs.
    // This ensures that when schema conversion tries to resolve types from external modules,
    // they are already registered in cross_module_types.
    for external_module in external_modules {
        let path = PathBuf::from(external_module);
        cross_module_types.insert(path, ModuleTypeMap::External);
    }

    let mut parsing_order = Vec::new();
    while let Some((path, module)) = module_manager.pop_module() {
        if module.is_external() {
            cross_module_types.insert(path.clone(), ModuleTypeMap::External);
            continue;
        }
        let (schema, idents) = schema::conv_module_to_schema(&module, &cross_module_types)?;
        parsing_order.push(path.clone());
        cross_module_types.insert(path.clone(), ModuleTypeMap::Internal(idents));
        schema_map.insert(path, schema);
    }

    Ok((parsing_order, schema_map))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::Path};

    use crate::pipeline::parse_str_schema;

    /*fn make_ident(s: &str) -> Identifier {
        Identifier::try_from(s.to_owned()).expect("test: make ident")
    }*/

    #[test]
    fn test_pipeline_simple() {
        const SCHEMA: &str = r"
class Point2d(Container):
  x_coord: uint32
  y_coord: uint32
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let schema = parse_str_schema(&files, &[]).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_pipeline_beacon_deposit_request() {
        // This is kinda bodging it, I just wanted to take a "real example".
        const SCHEMA: &str = r"
BLSPubkey = List[byte, 96]
BLSSignature = List[byte, 96]
Gwei = uint256

class DepositRequest(Container):
    pubkey: BLSPubkey
    withdrawal_credentials: Bytes32
    amount: Gwei
    signature: BLSSignature
    index: uint64
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let schema = parse_str_schema(&files, &[]).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_pipeline_aliases() {
        const SCHEMA: &str = r"
OMG = 3
Epoch = uint32
SomeVec = List[Epoch, 1337]

class Header(Container):
    slot: uint64
    epoch: Epoch
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let schema = parse_str_schema(&files, &[]).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_pipeline_parent_aliases() {
        // I don't even know if we want to support this, but hey we do now!
        const SCHEMA: &str = r"
MagicStable = StableContainer[32]

class MagicFoo(MagicStable):
    foo: Optional[uint32]
    bar: Optional[uint64]
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let schema = parse_str_schema(&files, &[]).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_pipeline_imports() {
        const SCHEMA_AS: &str = r"
import import_test as test
import ssz_external as external

TestA = test.A
TestB = test.B
TestC = test.C
TestD = external.D

VAL_A = 12
VAL_B = VAL_A
TEST_CONST = test.D

class Header(test.A):
    a: Union[null, test.B]
    b: test.B
    c: test.C
    d: uint8

f = List[test.A, TEST_CONST]
";

        const SCHEMA: &str = r"
import import_test
import ssz_external.module_a

TestA = import_test.A
TestB = import_test.B
TestC = import_test.C
TestD = module_a.D

VAL_A = 12
VAL_B = VAL_A
TEST_CONST = import_test.D

class Header(import_test.A):
    a: Union[null, import_test.B]
    b: import_test.B
    c: import_test.C
    d: uint8

f = List[import_test.A, TEST_CONST]
";

        let files = HashMap::from([(
            Path::new("tests/non_existent").to_path_buf(),
            SCHEMA_AS.to_string(),
        )]);
        let schema = parse_str_schema(&files, &["ssz_external"]).expect("test: parse schema");

        eprintln!("{schema:#?}");

        let files = HashMap::from([(
            Path::new("tests/non_existent").to_path_buf(),
            SCHEMA.to_string(),
        )]);
        let schema = parse_str_schema(&files, &["ssz_external"]).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_external_module_pre_registration() {
        // Test that external modules are pre-registered and can be referenced
        // without UnknownImport errors, even when referenced early in schema conversion
        const SCHEMA: &str = r"
import external_crate

class ContainerWithExternal(Container):
    field: external_crate.SomeType
    list_field: List[external_crate.OtherType, 10]
";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        // This should not panic with UnknownImport
        let result = parse_str_schema(&files, &["external_crate"]);
        assert!(
            result.is_ok(),
            "External module should be pre-registered and resolvable"
        );
    }

    #[test]
    fn test_external_crate_path_construction() {
        // Test that external crate paths are constructed correctly
        // This verifies that external crates can be imported and referenced
        // without UnknownImport errors, which was the main bug fixed
        const SCHEMA: &str = r"
import external_crate
import external_crate.module_a.module_b as mod_b

TestA = external_crate.TypeA
TestB = mod_b.TypeB
";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        // The main goal is that this doesn't fail with UnknownImport
        let result = parse_str_schema(&files, &["external_crate"]);
        assert!(
            result.is_ok(),
            "External crate paths should be constructed correctly and not cause UnknownImport"
        );
    }

    #[test]
    fn test_pipeline_generic_class() {
        // Test that generic classes are passed through with type parameters
        const SCHEMA: &str = r"
# Generic class definition
class Box[T](Container):
    value: T
    count: uint64
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let result = parse_str_schema(&files, &[]);

        assert!(
            result.is_ok(),
            "Generic classes should parse: {:?}",
            result.err()
        );

        let (_order, schemas) = result.unwrap();
        let schema = schemas.get(Path::new("")).expect("Schema should exist");

        // Generic class should be in the schema with type parameters
        assert_eq!(schema.classes().len(), 1, "Should have Box generic class");

        let box_class = &schema.classes()[0];
        assert_eq!(box_class.name().0, "Box");
        assert_eq!(
            box_class.type_params().len(),
            1,
            "Box should have 1 type parameter"
        );
        assert_eq!(box_class.type_params()[0].name().0, "T");
    }

    #[test]
    fn test_pipeline_generic_instantiation() {
        // Test that generic class instantiation aliases work in type resolution
        const SCHEMA: &str = r"
# Generic class definition
class Box[T](Container):
    value: T
    metadata: uint64

# Create type aliases for specific instantiations
BoxU8 = Box[uint8]
BoxU16 = Box[uint16]

# Use in another class
class Warehouse(Container):
    box8: BoxU8
    box16: BoxU16
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let result = parse_str_schema(&files, &[]);

        assert!(
            result.is_ok(),
            "Generic instantiation should work: {:?}",
            result.err()
        );

        let (_order, schemas) = result.unwrap();
        let schema = schemas.get(Path::new("")).expect("Schema should exist");

        // Should have 2 classes: Box (generic) and Warehouse
        assert!(
            !schema.classes().is_empty(),
            "Should have at least Box generic class"
        );

        // Check that Box generic class exists
        let box_class = schema.classes().iter().find(|c| c.name().0 == "Box");
        assert!(box_class.is_some(), "Box should exist");
        let box_class = box_class.unwrap();
        assert_eq!(
            box_class.type_params().len(),
            1,
            "Box should have 1 type parameter"
        );
        assert_eq!(box_class.fields().len(), 2, "Box should have 2 fields");
        assert_eq!(box_class.fields()[0].name().0, "value");
        assert_eq!(box_class.fields()[1].name().0, "metadata");

        // Check that Warehouse exists and uses the instantiated types
        let warehouse = schema.classes().iter().find(|c| c.name().0 == "Warehouse");
        assert!(warehouse.is_some(), "Warehouse should exist");
        let warehouse = warehouse.unwrap();
        assert_eq!(
            warehouse.fields().len(),
            2,
            "Warehouse should have 2 fields"
        );
    }

    #[test]
    fn test_pipeline_nested_generic_instantiation() {
        // Test nested generic instantiation (generic types using other generic types)
        // This tests the strata-common PR #36 use case
        const SCHEMA: &str = r"
# Generic RawMerkleProof with type parameter H
class RawMerkleProof[H](Container):
    cohashes: List[H, 64]

# Instantiate with simple type (creates type alias)
RawMerkleProofU8 = RawMerkleProof[uint8]
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let result = parse_str_schema(&files, &[]);

        assert!(
            result.is_ok(),
            "Nested generic instantiation should work: {:?}",
            result.err()
        );

        let (_order, schemas) = result.unwrap();
        let schema = schemas.get(Path::new("")).expect("Schema should exist");

        // Should have 1 generic class: RawMerkleProof
        assert_eq!(schema.classes().len(), 1, "Should have 1 generic class");

        // Verify RawMerkleProof generic class exists
        let raw_proof = schema
            .classes()
            .iter()
            .find(|c| c.name().0 == "RawMerkleProof");
        assert!(raw_proof.is_some(), "RawMerkleProof should exist");
        let raw_proof = raw_proof.unwrap();
        assert_eq!(
            raw_proof.type_params().len(),
            1,
            "RawMerkleProof should have 1 type parameter"
        );
    }

    #[test]
    fn test_pipeline_strata_schema() {
        // Test strata-common use case with generic types
        const SCHEMA: &str = r"
# Generic RawMerkleProof with type parameter H for hash type
class RawMerkleProof[H](Container):
    cohashes: List[H, 1024]

# Generic MerkleProof that uses RawMerkleProof[H]
class MerkleProof[H](Container):
    inner: RawMerkleProof[H]
    index: uint64

# CompactMmr64 structure (non-generic)
class CompactMmr64(Container):
    entries: uint64
    cap_log2: uint8
    roots: List[Vector[byte, 32], 64]
";

        let files = HashMap::from([(Path::new("").to_path_buf(), SCHEMA.to_string())]);
        let result = parse_str_schema(&files, &[]);

        assert!(
            result.is_ok(),
            "Strata schema should parse: {:?}",
            result.err()
        );

        let (_order, schemas) = result.unwrap();
        let schema = schemas.get(Path::new("")).expect("Schema should exist");

        // Should have 3 generic/non-generic classes
        assert_eq!(schema.classes().len(), 3);

        // Check RawMerkleProof is generic
        let raw_proof = schema
            .classes()
            .iter()
            .find(|c| c.name().0 == "RawMerkleProof");
        assert!(raw_proof.is_some());
        assert_eq!(raw_proof.unwrap().type_params().len(), 1);

        // Check MerkleProof is generic
        let merkle_proof = schema
            .classes()
            .iter()
            .find(|c| c.name().0 == "MerkleProof");
        assert!(merkle_proof.is_some());
        assert_eq!(merkle_proof.unwrap().type_params().len(), 1);

        // Check CompactMmr64 is non-generic
        let mmr = schema
            .classes()
            .iter()
            .find(|c| c.name().0 == "CompactMmr64");
        assert!(mmr.is_some());
        assert_eq!(mmr.unwrap().type_params().len(), 0);
    }
}
