//! Token trees are a second stage before parsing to simplify parsing
//! structures.

use thiserror::Error;

use crate::{
    names::Identifier,
    src_pos::SrcPos,
    token::{SrcToken, TaggedToken},
};

/// Token tree with an empty tag value.
pub type Toktr = TaggedToktr<()>;

pub(crate) type SrcToktr = TaggedToktr<SrcPos>;

/// Token tree with a tag.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaggedToktr<T> {
    // Keywords and structural elements.
    /// `import` keyword.
    Import(T),
    /// `as` keyword.
    As(T),
    /// `class` keyword.
    Class(T),
    /// `:` keyword.
    Colon(T),
    /// `=` keyword.
    Eq(T),
    /// `,` keyword.
    Comma(T),
    /// `.` keyword.
    Dot(T),
    /// `\n` newline.
    Newline(T),
    /// `null` keyword.
    Null(T),

    // Identifiers.
    /// An identifier.
    Identifier(T, Identifier),

    // Expressions.
    /// An integer literal.
    IntegerLiteral(T, u64),
    /// `<<` operator.
    Shl(T),
    /// `*` operator.
    Mul(T),

    // Token tree nodes with children.
    /// A bracket block.
    BracketBlock(T, Box<NodeData<T>>),
    /// A parenthesis block.
    ParenBlock(T, Box<NodeData<T>>),
    /// An indent block.
    IndentBlock(T, Box<NodeData<T>>),

    // Comments
    /// Regular comment (discarded during conversion).
    Comment(T, String),
    /// Doc comment (merged with consecutive lines).
    DocComment(T, String),
    /// Pragma comment (with whitespace trimmed).
    PragmaComment(T, String),

    // Misc
    /// Docstring (triple-quoted string).
    DocString(T, String),
}

impl<T> TaggedToktr<T> {
    /// Gets the tag of the token.
    pub fn tag(&self) -> &T {
        match self {
            Self::Import(t) => t,
            Self::As(t) => t,
            Self::Class(t) => t,
            Self::Colon(t) => t,
            Self::Eq(t) => t,
            Self::Comma(t) => t,
            Self::Dot(t) => t,
            Self::Newline(t) => t,
            Self::Null(t) => t,
            Self::Identifier(t, _) => t,
            Self::IntegerLiteral(t, _) => t,
            Self::Shl(t) => t,
            Self::Mul(t) => t,
            Self::BracketBlock(t, _) => t,
            Self::ParenBlock(t, _) => t,
            Self::IndentBlock(t, _) => t,
            Self::Comment(t, _) => t,
            Self::DocComment(t, _) => t,
            Self::PragmaComment(t, _) => t,
            Self::DocString(t, _) => t,
        }
    }

    /// Gets if the token is a block.
    pub fn is_block(&self) -> bool {
        matches!(
            self,
            Self::BracketBlock(_, _) | Self::ParenBlock(_, _) | Self::IndentBlock(_, _)
        )
    }

    /// Gets the node data of the token.
    pub fn node_data(&self) -> Option<&NodeData<T>> {
        match self {
            Self::BracketBlock(_, data)
            | Self::ParenBlock(_, data)
            | Self::IndentBlock(_, data) => Some(data),
            _ => None,
        }
    }
}

/// Node data containing the children of a node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeData<T> {
    children: Vec<TaggedToktr<T>>,
}

impl<T> NodeData<T> {
    pub fn new(children: Vec<TaggedToktr<T>>) -> Self {
        Self { children }
    }

    pub fn children(&self) -> &[TaggedToktr<T>] {
        &self.children
    }
}

#[derive(Debug, Error)]
pub enum ToktrError {
    #[error("expected {0:?} terminal but found {1:?} terminal")]
    UnfinishedBlock(BlockType, BlockType),

    #[error("end of sequence with unclosed block of type {0:?}")]
    UnclosedBlock(BlockType),

    #[error("not yet implemented")]
    Unimplemented,
}

/// Describes what syntactic structure the block is representing.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlockType {
    Root,
    Bracket,
    Paren,
    Indent,
}

struct ToktrBuilder {
    block_ty_stack: Vec<BlockType>,
    block_sp_stack: Vec<SrcPos>,
    contexts: Vec<Vec<SrcToktr>>,
}

impl ToktrBuilder {
    pub(crate) fn new() -> Self {
        Self {
            block_ty_stack: vec![BlockType::Root],
            block_sp_stack: vec![],
            contexts: vec![Vec::new()],
        }
    }

    fn top_block_mut(&mut self) -> &mut Vec<SrcToktr> {
        self.contexts.last_mut().expect("toktr: top_block_mut")
    }

    fn top_ty(&self) -> &BlockType {
        self.block_ty_stack.last().expect("toktr: top_ty")
    }

    pub(crate) fn push_token(&mut self, tt: SrcToktr) {
        self.top_block_mut().push(tt);
    }

    pub(crate) fn push_block(&mut self, ty: BlockType, sp: SrcPos) {
        self.contexts.push(Vec::new());
        self.block_ty_stack.push(ty);
        self.block_sp_stack.push(sp);
    }

    pub(crate) fn try_pop_block(&mut self, ty: BlockType) -> Result<SrcToktr, ToktrError> {
        // This shouldn't be allowed.
        if ty == BlockType::Root {
            panic!("toktr: tried to pop root");
        }

        if *self.top_ty() != ty {
            return Err(ToktrError::UnfinishedBlock(*self.top_ty(), ty));
        }

        // Construct the new node.
        self.block_ty_stack.pop();
        let sp = self.block_sp_stack.pop().unwrap();
        let body_toks = self.contexts.pop().expect("toktr: try_finish_block");
        let data = NodeData::new(body_toks);
        let tt = match ty {
            BlockType::Bracket => SrcToktr::BracketBlock(sp, Box::new(data)),
            BlockType::Paren => SrcToktr::ParenBlock(sp, Box::new(data)),
            BlockType::Indent => SrcToktr::IndentBlock(sp, Box::new(data)),
            _ => unreachable!(),
        };

        Ok(tt)
    }

    pub(crate) fn finish(mut self) -> Result<Vec<SrcToktr>, ToktrError> {
        if self.contexts.len() != 1 {
            return Err(ToktrError::UnclosedBlock(*self.top_ty()));
        }

        let toks = self.contexts.pop().unwrap();
        Ok(toks)
    }
}

pub(crate) fn parse_tokens_to_toktrs(tokens: &[SrcToken]) -> Result<Vec<SrcToktr>, ToktrError> {
    let mut i = 0;

    let mut builder = ToktrBuilder::new();

    while i < tokens.len() {
        let cur = &tokens[i];

        let tt = match cur {
            TaggedToken::Null(sp) => TaggedToktr::Null(*sp),
            TaggedToken::Class(sp) => TaggedToktr::Class(*sp),
            TaggedToken::Import(sp) => TaggedToktr::Import(*sp),
            TaggedToken::As(sp) => TaggedToktr::As(*sp),
            TaggedToken::Colon(sp) => TaggedToktr::Colon(*sp),
            TaggedToken::Eq(sp) => TaggedToktr::Eq(*sp),
            TaggedToken::Comma(sp) => TaggedToktr::Comma(*sp),
            TaggedToken::Dot(sp) => TaggedToktr::Dot(*sp),
            TaggedToken::Newline(sp) => TaggedToktr::Newline(*sp),
            TaggedToken::Identifier(sp, ident) => TaggedToktr::Identifier(*sp, ident.clone()),
            TaggedToken::IntegerLiteral(sp, v) => TaggedToktr::IntegerLiteral(*sp, *v),
            TaggedToken::Shl(sp) => TaggedToktr::Shl(*sp),
            TaggedToken::Mul(sp) => TaggedToktr::Mul(*sp),
            TaggedToken::Comment(sp, text) => TaggedToktr::Comment(*sp, text.clone()),
            TaggedToken::DocComment(sp, text) => {
                // Merge consecutive doc comments, but don't skip newlines
                let mut merged = text.clone();
                let mut j = i + 1;
                let sp_start = *sp;
                let mut last_doc_idx = i; // Track the index of the last DocComment we merged
                let mut newlines_found = 0;

                // Look ahead for consecutive doc comments (possibly with newlines between)
                while j < tokens.len() {
                    match &tokens[j] {
                        TaggedToken::Newline(_) => {
                            newlines_found += 1;
                            j += 1;
                            continue;
                        }
                        TaggedToken::DocComment(_, text) => {
                            // Separate the two doc comments with newlines
                            // If there are newlines between them, preserve them; otherwise just one
                            let num_separators = if newlines_found > 0 {
                                newlines_found
                            } else {
                                1
                            };
                            for _ in 0..num_separators {
                                merged.push('\n');
                            }
                            newlines_found = 0;
                            merged.push_str(text);
                            last_doc_idx = j; // Update to this DocComment's index
                            j += 1;
                        }
                        _ => break,
                    }
                }

                // Skip past all merged doc comments, but leave newlines to be processed normally
                if last_doc_idx > i {
                    i = last_doc_idx; // Skip to the last DocComment
                }
                TaggedToktr::DocComment(sp_start, merged)
            }
            TaggedToken::PragmaComment(sp, text) => {
                // Trim leading/trailing whitespace from pragma comments
                let trimmed = text.trim().to_owned();
                TaggedToktr::PragmaComment(*sp, trimmed)
            }

            TaggedToken::OpenBracket(sp) => {
                builder.push_block(BlockType::Bracket, *sp);
                i += 1;
                continue;
            }

            TaggedToken::CloseBracket(_) => builder.try_pop_block(BlockType::Bracket)?,

            TaggedToken::OpenParen(sp) => {
                builder.push_block(BlockType::Paren, *sp);
                i += 1;
                continue;
            }
            TaggedToken::CloseParen(_) => builder.try_pop_block(BlockType::Paren)?,

            TaggedToken::Indent(sp) => {
                builder.push_block(BlockType::Indent, *sp);
                i += 1;
                continue;
            }
            TaggedToken::Deindent(_) => builder.try_pop_block(BlockType::Indent)?,
        };

        builder.push_token(tt);

        i += 1;
    }

    builder.finish()
}

#[cfg(test)]
mod tests {
    use super::parse_tokens_to_toktrs;
    use crate::{
        Identifier,
        src_pos::SrcPos,
        token::{SrcToken, parse_char_array_to_tokens},
    };

    #[test]
    fn test_parse_simple() {
        let tokens = vec![
            SrcToken::Class(SrcPos::dummy()),
            SrcToken::Colon(SrcPos::dummy()),
            SrcToken::Eq(SrcPos::dummy()),
            SrcToken::Comma(SrcPos::dummy()),
        ];

        let tt = parse_tokens_to_toktrs(&tokens).expect("test: invoke parse_tokens_to_toktrs");

        eprintln!("{tt:#?}");
    }

    #[test]
    fn test_parse_tree() {
        let foo_name = Identifier::try_from("Foo".to_owned()).expect("test: parse ident name");
        let container_name =
            Identifier::try_from("Container".to_owned()).expect("test: parse ident name");
        let vector_name =
            Identifier::try_from("Vector".to_owned()).expect("test: parse ident name");
        let bar_name = Identifier::try_from("bar".to_owned()).expect("test: parse ident name");

        let sp = SrcPos::dummy();

        let tokens = vec![
            SrcToken::Class(sp),
            SrcToken::Identifier(sp, foo_name),
            SrcToken::OpenParen(sp),
            SrcToken::Identifier(sp, container_name),
            SrcToken::CloseParen(sp),
            SrcToken::Colon(sp),
            SrcToken::Indent(sp),
            SrcToken::Identifier(sp, bar_name),
            SrcToken::Colon(sp),
            SrcToken::Identifier(sp, vector_name),
            SrcToken::Deindent(sp),
        ];

        let tt = parse_tokens_to_toktrs(&tokens).expect("test: invoke parse_tokens_to_toktrs");

        eprintln!("{tt:#?}");
    }

    #[test]
    fn test_parse_nested_tree() {
        let foo_name = Identifier::try_from("Foo".to_owned()).expect("test: parse ident");
        let stable_container_name =
            Identifier::try_from("StableContainer".to_owned()).expect("test: parse ident");
        let vector_name = Identifier::try_from("Vector".to_owned()).expect("test: parse ident");
        let bar_name = Identifier::try_from("bar".to_owned()).expect("test: parse ident");

        let sp = SrcPos::dummy();

        let tokens = vec![
            SrcToken::Class(sp),
            SrcToken::Identifier(sp, foo_name),
            SrcToken::OpenParen(sp),
            SrcToken::Identifier(sp, stable_container_name.clone()),
            SrcToken::OpenBracket(sp),
            SrcToken::IntegerLiteral(sp, 32),
            SrcToken::CloseBracket(sp),
            SrcToken::CloseParen(sp),
            SrcToken::Colon(sp),
            SrcToken::Indent(sp),
            SrcToken::Identifier(sp, bar_name),
            SrcToken::Colon(sp),
            SrcToken::Identifier(sp, vector_name),
            SrcToken::Deindent(sp),
        ];

        let tt = parse_tokens_to_toktrs(&tokens).expect("test: invoke parse_tokens_to_toktrs");

        eprintln!("{tt:#?}");
    }

    #[test]
    fn test_full_parse_stable_container() {
        let s = r"
class Foo(StableContainer[16]):
    x_coordinate: Optional[uint32]
    y_coordinate: Optional[uint64]
";

        let arr = s.chars().collect::<Vec<_>>();

        let toks = parse_char_array_to_tokens(&arr).expect("test: tokenize string");

        eprintln!("tokens {toks:#?}");

        let tt = parse_tokens_to_toktrs(&toks).expect("test: treeize tokens");

        eprintln!("tree {tt:#?}");
    }
}
