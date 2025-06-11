//! High-level logic for full-pipeline parsing.

use thiserror::Error;

use crate::{
    SszSchema,
    ast::{self, ParseError},
    schema::{self, SchemaError},
    token::{self, TokenError},
    token_tree::{self, ToktrError},
    ty_resolver::ResolverError,
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
pub fn parse_str_schema(s: &str) -> Result<SszSchema, SszError> {
    let s_chars = s.chars().collect::<Vec<_>>();

    let tokens = token::parse_char_array_to_tokens(&s_chars)?;
    let toktrs = token_tree::parse_tokens_to_toktrs(&tokens)?;
    let ast_mod = ast::parse_module_from_toktrs(&toktrs)?;
    let schema = schema::conv_module_to_schema(&ast_mod)?;

    Ok(schema)
}

#[cfg(test)]
mod tests {
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

        let schema = parse_str_schema(SCHEMA).expect("test: parse schema");

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

        let schema = parse_str_schema(SCHEMA).expect("test: parse schema");

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

        let schema = parse_str_schema(SCHEMA).expect("test: parse schema");

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

        let schema = parse_str_schema(SCHEMA).expect("test: parse schema");

        eprintln!("{schema:#?}");
    }
}
