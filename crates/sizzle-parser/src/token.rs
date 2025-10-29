//! First stage tokenizer.
//!
//! This varies from a textbook tokenizer in that it tracks indentation in order
//! to emit special `Indent` and `Deindent` tokens.  These are treated like
//! other tokens used for blocks to create token tree nodes with children.

use thiserror::Error;

use crate::{
    names::{Identifier, NameError, is_valid_ident_continuing_char, is_valid_ident_initial_char},
    src_pos::{PosTbl, SrcPos},
};

/// Token without an empty tag value.
pub type Token = TaggedToken<()>;

/// Token tagged with a srcpos.
pub(crate) type SrcToken = TaggedToken<SrcPos>;

/// Token with a tag.
///
/// The tag can be used for something like a span location in the original
/// source or to assign an identifier across structures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaggedToken<T> {
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

    // Structural, these are treated specially in token trees later.
    /// `[` open bracket.
    OpenBracket(T),
    /// `]` close bracket.
    CloseBracket(T),
    /// `(` open parenthesis.
    OpenParen(T),
    /// `)` close parenthesis.
    CloseParen(T),
    /// `indent` token.
    Indent(T),
    /// `deindent` token.
    Deindent(T),
    /// A decorator token (e.g., @derive(...))
    Decorator(T, String),
}

impl<T> TaggedToken<T> {
    /// Returns the tag on the token.
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
            Self::Identifier(t, _) => t,
            Self::IntegerLiteral(t, _) => t,
            Self::Shl(t) => t,
            Self::Mul(t) => t,
            Self::OpenBracket(t) => t,
            Self::CloseBracket(t) => t,
            Self::OpenParen(t) => t,
            Self::CloseParen(t) => t,
            Self::Indent(t) => t,
            Self::Deindent(t) => t,
            Self::Null(t) => t,
            Self::Decorator(t, _) => t,
        }
    }

    /// Converts the token to a untagged token.
    pub fn to_untagged(&self) -> Token {
        match self {
            Self::Import(_) => Token::Import(()),
            Self::As(_) => Token::As(()),
            Self::Class(_) => Token::Class(()),
            Self::Colon(_) => Token::Colon(()),
            Self::Eq(_) => Token::Eq(()),
            Self::Comma(_) => Token::Comma(()),
            Self::Dot(_) => Token::Dot(()),
            Self::Newline(_) => Token::Newline(()),
            Self::Identifier(_, ident) => Token::Identifier((), ident.clone()),
            Self::IntegerLiteral(_, v) => Token::IntegerLiteral((), *v),
            Self::Shl(_) => Token::Shl(()),
            Self::Mul(_) => Token::Mul(()),
            Self::OpenBracket(_) => Token::OpenBracket(()),
            Self::CloseBracket(_) => Token::CloseBracket(()),
            Self::OpenParen(_) => Token::OpenParen(()),
            Self::CloseParen(_) => Token::CloseParen(()),
            Self::Indent(_) => Token::Indent(()),
            Self::Deindent(_) => Token::Deindent(()),
            Self::Null(_) => Token::Null(()),
            Self::Decorator(_, content) => Token::Decorator((), content.clone()),
        }
    }
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("unexpected char '{0}' at pos {1}")]
    UnexpectedChar(char, usize),

    #[error("unexpected end of input")]
    UnexpectedEnd,

    #[error("invalid indent at {0} (was {1:?})")]
    InvalidIndent(usize, Indent),

    #[error("unrecognizable indent at {0}")]
    UnrecognizableIndent(usize),

    #[error("invalid integer '{0}'")]
    InvalidInt(String),

    #[error("invalid name: {0}")]
    InvalidName(#[from] NameError),
}

/// Describes how we're interpreting indentation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Indent {
    Spaces(u8),
    Tab,
}

pub(crate) struct TokenSeqBuilder {
    indent_ty: Option<Indent>,
    indent_level: usize,
    output: Vec<SrcToken>,
}

impl TokenSeqBuilder {
    pub(crate) fn new() -> Self {
        Self {
            indent_ty: None,
            indent_level: 0,
            output: Vec::new(),
        }
    }

    fn _indent_level(&self) -> usize {
        self.indent_level
    }

    fn infer_indent_level(&mut self, indent: &[char], at: usize) -> Result<usize, TokenError> {
        match self.indent_ty {
            Some(i @ Indent::Spaces(n)) => {
                if is_all_spaces(indent) {
                    let found_spaces = indent.len();
                    if found_spaces.is_multiple_of(n as usize) {
                        let ind_cnt = found_spaces / n as usize;
                        Ok(ind_cnt)
                    } else {
                        Err(TokenError::InvalidIndent(at, i))
                    }
                } else {
                    Err(TokenError::InvalidIndent(at, i))
                }
            }
            Some(i @ Indent::Tab) => {
                if is_all_tabs(indent) {
                    let found_tabs = indent.len();
                    Ok(found_tabs)
                } else {
                    Err(TokenError::InvalidIndent(at, i))
                }
            }
            None => {
                let is_spaces = is_all_spaces(indent);
                let is_tabs = is_all_tabs(indent);

                // Doesn't matter what it is, this is just zero.
                if indent.is_empty() {
                    return Ok(0);
                }

                if is_spaces {
                    self.indent_ty = Some(Indent::Spaces(indent.len() as u8));
                    Ok(1)
                } else if is_tabs {
                    self.indent_ty = Some(Indent::Tab);
                    Ok(indent.len())
                } else {
                    Err(TokenError::UnrecognizableIndent(at))
                }
            }
        }
    }

    fn push_token(&mut self, t: SrcToken) {
        self.output.push(t);
    }

    /// Updates the indentation level, producing indent tokens as necessary.
    fn update_indent_level(&mut self, level: usize, sp: SrcPos) {
        let diff = level as isize - self.indent_level as isize;
        match diff {
            0 => {}

            // Deindentating.
            d if d < 0 => {
                for _ in 0..(-d) {
                    self.push_token(TaggedToken::Deindent(sp));
                }
                self.indent_level = level;
            }

            // Indenting.
            d if d > 0 => {
                for _ in 0..d {
                    self.push_token(TaggedToken::Indent(sp));
                }
                self.indent_level = level;
            }

            _ => unreachable!(),
        }
    }

    fn finish(mut self, sp: SrcPos) -> Result<Vec<SrcToken>, TokenError> {
        // Automatically close the rest of the indents.
        for _ in 0..self.indent_level {
            self.push_token(TaggedToken::Deindent(sp));
        }

        Ok(self.output)
    }
}

fn is_all_spaces<'c>(iter: impl IntoIterator<Item = &'c char>) -> bool {
    iter.into_iter().all(|c| *c == ' ')
}

fn is_all_tabs<'c>(iter: impl IntoIterator<Item = &'c char>) -> bool {
    iter.into_iter().all(|c| *c == '\t')
}

pub(crate) fn parse_char_array_to_tokens(s: &[char]) -> Result<Vec<SrcToken>, TokenError> {
    let sp_tbl = PosTbl::generate(s.iter().copied());

    let mut builder = TokenSeqBuilder::new();

    let mut i = 0;
    while i < s.len() {
        let cur = s[i];
        let sp = sp_tbl.expect_srcpos(i);

        #[cfg(test)]
        eprintln!(
            "considering {cur:?} (indent level {})",
            builder._indent_level()
        );

        let next = s.get(i + 1).copied();

        // Handle simple cases and whitespace.
        match cur {
            ' ' => {
                i += 1;
                continue;
            }

            // Newlines are the special case since this is where we have to figure out whitespace!
            '\n' => {
                builder.push_token(SrcToken::Newline(sp));
                i += 1;

                // Now i is the first one after the newline.  If this isn't the
                // end of the buffer, let's do some work to figure out if we
                // should do intendents.
                if i < s.len() {
                    let new_cur = s[i];
                    let new_sp = sp_tbl.expect_srcpos(i);
                    if new_cur == '\n' {
                        // Just get it next time.
                        continue;
                    }

                    let j = find_satisfying_range(s, i, |c| c.is_ascii_whitespace());

                    let ws_span = &s[i..j];
                    let cnt = builder.infer_indent_level(ws_span, i)?;

                    builder.update_indent_level(cnt, new_sp);
                    i = j;
                }

                continue;
            }

            ':' => builder.push_token(SrcToken::Colon(sp)),
            '=' => builder.push_token(SrcToken::Eq(sp)),
            ',' => builder.push_token(SrcToken::Comma(sp)),
            '.' => builder.push_token(SrcToken::Dot(sp)),
            '*' => builder.push_token(SrcToken::Mul(sp)),
            '[' => builder.push_token(SrcToken::OpenBracket(sp)),
            ']' => builder.push_token(SrcToken::CloseBracket(sp)),
            '(' => builder.push_token(SrcToken::OpenParen(sp)),
            ')' => builder.push_token(SrcToken::CloseParen(sp)),
            '@' => {
                // Parse decorator until newline
                let j = find_satisfying_range(s, i + 1, |c| c != '\n');
                let decorator_content = s[i..j].iter().collect::<String>();
                builder.push_token(SrcToken::Decorator(sp, decorator_content));
                i = j;
                continue;
            }
            '#' => {
                // Skip comment to end of line
                let j = find_satisfying_range(s, i + 1, |c| c != '\n');
                i = j;
                continue;
            }

            '<' => {
                if let Some(next) = next {
                    if next == '<' {
                        builder.push_token(SrcToken::Shl(sp));
                        i += 2;
                        continue;
                    } else {
                        return Err(TokenError::UnexpectedChar(next, i + 1));
                    }
                } else {
                    return Err(TokenError::UnexpectedEnd);
                }
            }

            c if is_valid_ident_initial_char(c) => {
                let j = find_satisfying_range(s, i + 1, is_valid_ident_continuing_char);
                let s = s[i..j].iter().collect::<String>();

                // Keywords are like identifiers, but separated out.
                if let Some(kwtok) = try_parse_keyword(&s, sp) {
                    builder.push_token(kwtok);
                } else {
                    let ident = Identifier::try_from(s)?;
                    builder.push_token(SrcToken::Identifier(sp, ident));
                }

                i = j;
                continue;
            }

            c if c.is_numeric() => {
                let j = find_satisfying_range(s, i + 1, char::is_numeric);
                let arr = s[i..j].iter().collect::<String>();
                let v = arr
                    .parse::<u64>()
                    .map_err(|_| TokenError::InvalidInt(arr))?;
                builder.push_token(SrcToken::IntegerLiteral(sp, v));
                i = j;
                continue;
            }

            _ => return Err(TokenError::UnexpectedChar(cur, i)),
        }

        i += 1;
    }

    builder.finish(sp_tbl.expect_end())
}

fn try_parse_keyword(s: &str, sp: SrcPos) -> Option<SrcToken> {
    Some(match s {
        "import" => SrcToken::Import(sp),
        "as" => SrcToken::As(sp),
        "class" => SrcToken::Class(sp),
        "null" => SrcToken::Null(sp),
        _ => return None,
    })
}

/// Finds the end of a range of chars matching a condition.  The value returned
/// can be used to slice into the passed array (along with the start index) to
/// construct a subslice that all satisfies the condition.
fn find_satisfying_range(arr: &[char], start: usize, cond: impl Fn(char) -> bool) -> usize {
    let mut at = start;

    while at < arr.len() {
        if !cond(arr[at]) {
            break;
        }
        at += 1;
    }

    at
}

#[cfg(test)]
mod tests {
    use super::{TokenSeqBuilder, parse_char_array_to_tokens};

    #[test]
    fn test_whitespace_spaces() {
        let mut builder = TokenSeqBuilder::new();

        let cnt = builder
            .infer_indent_level(&[' ', ' ', ' ', ' '], 5)
            .unwrap();
        assert_eq!(cnt, 1);

        let cnt = builder
            .infer_indent_level(&[' ', ' ', ' ', ' '], 5)
            .unwrap();
        assert_eq!(cnt, 1);

        let cnt = builder
            .infer_indent_level(&[' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], 5)
            .unwrap();
        assert_eq!(cnt, 2);

        let _ = builder
            .infer_indent_level(&[' ', ' ', ' ', ' ', ' ', ' ', ' '], 5)
            .expect_err("test: should have errored");

        let _ = builder
            .infer_indent_level(&['\t'], 5)
            .expect_err("test: should have errored");
    }

    #[test]
    fn test_whitespace_tabs() {
        let mut builder = TokenSeqBuilder::new();

        let cnt = builder.infer_indent_level(&['\t'], 5).unwrap();
        assert_eq!(cnt, 1);

        let cnt = builder.infer_indent_level(&['\t', '\t'], 5).unwrap();
        assert_eq!(cnt, 2);

        let _ = builder
            .infer_indent_level(&[' ', ' '], 5)
            .expect_err("test: should have errored");
    }

    #[test]
    fn test_parse_const() {
        let s = "FOO_BAR = 1234";

        let chars = s.chars().collect::<Vec<_>>();

        let toks =
            parse_char_array_to_tokens(&chars).expect("test: invoke parse_char_array_to_tokens");

        eprintln!("{toks:#?}");
    }

    #[test]
    fn test_parse_whitespace_consts() {
        let s = "\nFOO = 123\n\n\nBAR = 555\nBAZ = 999";

        let chars = s.chars().collect::<Vec<_>>();

        let toks =
            parse_char_array_to_tokens(&chars).expect("test: invoke parse_char_array_to_tokens");

        eprintln!("{toks:#?}");
    }

    #[test]
    fn test_parse_shl() {
        let s = "\nFOO = 10 << 30";

        let chars = s.chars().collect::<Vec<_>>();

        let toks =
            parse_char_array_to_tokens(&chars).expect("test: invoke parse_char_array_to_tokens");

        eprintln!("{toks:#?}");
    }

    #[test]
    fn test_parse_container_def() {
        let s = "class Point(Container):\n  x_pos: int32\n  y_pos: int32\n";

        let chars = s.chars().collect::<Vec<_>>();

        let toks =
            parse_char_array_to_tokens(&chars).expect("test: invoke parse_char_array_to_tokens");

        eprintln!("{toks:#?}");
    }
}
