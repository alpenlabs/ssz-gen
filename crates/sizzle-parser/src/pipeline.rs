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
#[derive(Debug, Clone, Eq, PartialEq, Error)]
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
            ast::parse_module_from_toktrs(&toktrs, path, &mut module_manager, Some(files))?;
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
    fn test_schema_without_trailing_newline() {
        // Test that parsing works when the file doesn't end with a newline
        // This was causing an infinite loop in parse_class_body
        const SCHEMA: &str = "class Point2d(Container):\n  x_coord: uint32\n  y_coord: uint32";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let result = parse_str_schema(&files, &[]);
        assert!(
            result.is_ok(),
            "Schema without trailing newline should parse successfully"
        );
    }

    #[test]
    fn test_assignment_without_trailing_newline() {
        // Test that parsing works when an assignment doesn't end with a newline
        const SCHEMA: &str = "MyAlias = uint32";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let result = parse_str_schema(&files, &[]);
        assert!(
            result.is_ok(),
            "Assignment without trailing newline should parse successfully"
        );
    }

    #[test]
    fn test_import_without_trailing_newline() {
        // Test that parsing works when an import doesn't end with a newline
        const SCHEMA: &str = "import ssz_external";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let result = parse_str_schema(&files, &["ssz_external"]);
        assert!(
            result.is_ok(),
            "Import without trailing newline should parse successfully"
        );
    }

    #[test]
    fn test_complex_schema_without_trailing_newline() {
        // Test a more complex schema without trailing newline
        const SCHEMA: &str = r"
Epoch = uint32
SomeVec = List[Epoch, 1337]

class Header(Container):
    slot: uint64
    epoch: Epoch
    vec: SomeVec";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let result = parse_str_schema(&files, &[]);
        assert!(
            result.is_ok(),
            "Complex schema without trailing newline should parse successfully"
        );
    }

    #[test]
    fn test_add_sub_operators() {
        // Test the add and subtract operators with both literal and symbolic operands
        const SCHEMA: &str = r"
BASE = 1024
PLUS_ONE = BASE + 1
MINUS_ONE = BASE - 1
LITERAL_ADD = 10 + 5
LITERAL_SUB = 10 - 5

class MyContainer(Container):
    field_a: List[byte, PLUS_ONE]
    field_b: List[byte, MINUS_ONE]
    field_c: List[byte, LITERAL_ADD]
    field_d: List[byte, LITERAL_SUB]
";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let (_, schema_map) =
            parse_str_schema(&files, &[]).expect("test: parse schema with add/sub operators");

        let schema = schema_map
            .get(Path::new("test.ssz"))
            .expect("test: get schema");

        // Verify we have all the constants
        let constants = schema.constants();
        assert_eq!(constants.len(), 5, "Should have 5 constants");

        // Check BASE constant
        assert_eq!(constants[0].name().0, "BASE");
        assert_eq!(constants[0].value().eval(), 1024);

        // Check PLUS_ONE constant (1024 + 1 = 1025)
        assert_eq!(constants[1].name().0, "PLUS_ONE");
        assert_eq!(constants[1].value().eval(), 1025);

        // Check MINUS_ONE constant (1024 - 1 = 1023)
        assert_eq!(constants[2].name().0, "MINUS_ONE");
        assert_eq!(constants[2].value().eval(), 1023);

        // Check LITERAL_ADD constant (10 + 5 = 15)
        assert_eq!(constants[3].name().0, "LITERAL_ADD");
        assert_eq!(constants[3].value().eval(), 15);

        // Check LITERAL_SUB constant (10 - 5 = 5)
        assert_eq!(constants[4].name().0, "LITERAL_SUB");
        assert_eq!(constants[4].value().eval(), 5);

        // Verify the class was created
        let classes = schema.classes();
        assert_eq!(classes.len(), 1, "Should have 1 class");
        assert_eq!(classes[0].name().0, "MyContainer");
        assert_eq!(classes[0].fields().len(), 4, "Should have 4 fields");

        eprintln!("Successfully tested add/sub operators with evaluated values: {schema:#?}");
    }

    #[test]
    fn test_issue_49_example() {
        // Test the actual example from issue #49
        const SCHEMA: &str = r"
### Maximum length of the predicate condition bytes
MAX_CONDITION_LEN = 1 << 10

### One additional byte for the PredicateTypeId
MAX_PREDICATE_LEN = MAX_CONDITION_LEN + 1

class Predicate(Container):
    condition: List[byte, MAX_CONDITION_LEN]
    full_data: List[byte, MAX_PREDICATE_LEN]
";

        let files = HashMap::from([(Path::new("test.ssz").to_path_buf(), SCHEMA.to_string())]);

        let (_, schema_map) =
            parse_str_schema(&files, &[]).expect("test: parse schema with issue #49 example");

        let schema = schema_map
            .get(Path::new("test.ssz"))
            .expect("test: get schema");

        // Verify constants
        let constants = schema.constants();
        assert_eq!(constants.len(), 2, "Should have 2 constants");

        // Check MAX_CONDITION_LEN constant (1 << 10 = 1024)
        assert_eq!(constants[0].name().0, "MAX_CONDITION_LEN");
        assert_eq!(constants[0].value().eval(), 1024);

        // Check MAX_PREDICATE_LEN constant (1024 + 1 = 1025)
        assert_eq!(constants[1].name().0, "MAX_PREDICATE_LEN");
        assert_eq!(constants[1].value().eval(), 1025);

        // Verify the class
        let classes = schema.classes();
        assert_eq!(classes.len(), 1, "Should have 1 class");
        assert_eq!(classes[0].name().0, "Predicate");
        assert_eq!(classes[0].fields().len(), 2, "Should have 2 fields");

        eprintln!("Successfully parsed issue #49 example: {schema:#?}");
    }
}
