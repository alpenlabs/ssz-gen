//! Pythinic SSZ definition parser.

mod ast;

mod builtins;

mod token;
pub use token::{TaggedToken, Token};

mod names;
pub use names::{ConstName, FieldName, Identifier, TypeName};

mod token_tree;
pub use token_tree::{TaggedToktr, Toktr};

mod src_pos;
pub use src_pos::{LineColPos, PosTbl, SrcPos, SrcSpan};

mod schema;
pub use schema::{AliasDef, ClassDef, ClassFieldDef, SszSchema};

mod ty_resolver;

pub mod tysys;

mod gobbler;

mod pipeline;
pub use pipeline::{parse_str_schema, SszError};
