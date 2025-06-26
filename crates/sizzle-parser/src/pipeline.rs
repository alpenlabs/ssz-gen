//! High-level logic for full-pipeline parsing.

use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;

use crate::{
    SszSchema,
    ast::{self, ModuleManager, ParseError},
    schema::{self, SchemaError},
    token::{self, TokenError},
    token_tree::{self, ToktrError},
    ty_resolver::{CrossModuleTypeMap, ResolverError},
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
) -> Result<(Vec<PathBuf>, HashMap<PathBuf, SszSchema>), SszError> {
    let mut module_manager = ModuleManager::new();

    for (path, content) in files {
        let chars = content.chars().collect::<Vec<_>>();
        let tokens = token::parse_char_array_to_tokens(&chars)?;
        let toktrs = token_tree::parse_tokens_to_toktrs(&tokens)?;

        // TODO: Inserts at positon 0 in Vec, it's ok for now since we don't expect too many imports
        // but if it becomes a bottleneck we can fix it.
        module_manager.add_module_to_front(path.clone());
        ast::parse_module_from_toktrs(&toktrs, path, &mut module_manager)?;
    }

    let mut schema_map = HashMap::new();
    let mut cross_module_types = CrossModuleTypeMap::new();
    let mut parsing_order = Vec::new();
    while let Some((path, module)) = module_manager.pop_module() {
        let (schema, idents) = schema::conv_module_to_schema(&module, &cross_module_types)?;
        parsing_order.push(path.clone());
        cross_module_types.insert(path.clone(), idents);
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
        let schema = parse_str_schema(&files).expect("test: parse schema");

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
        let schema = parse_str_schema(&files).expect("test: parse schema");

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
        let schema = parse_str_schema(&files).expect("test: parse schema");

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
        let schema = parse_str_schema(&files).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }

    #[test]
    fn test_pipeline_imports() {
        const SCHEMA_AS: &str = r"
import import_test as test

TestA = test.A
TestB = test.B
TestC = test.C

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

TestA = import_test.A
TestB = import_test.B
TestC = import_test.C

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
        let schema = parse_str_schema(&files).expect("test: parse schema");

        eprintln!("{schema:#?}");

        let files = HashMap::from([(
            Path::new("tests/non_existent").to_path_buf(),
            SCHEMA.to_string(),
        )]);
        let schema = parse_str_schema(&files).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }
}
