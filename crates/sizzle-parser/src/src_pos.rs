//! Source code position logic.

use std::fmt::{Debug, Display};

/// Describes a line and column position in some source string.
///
/// This is both 0 indexed, so the first line is 0 and the first col on a line is 0.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct LineColPos {
    line: u16,
    col: u16,
}

impl LineColPos {
    /// Creates a new line and column position.
    pub fn new(line: u16, col: u16) -> Self {
        Self { line, col }
    }

    /// Creates a dummy line and column position.
    pub fn dummy() -> Self {
        Self::new(0, 0)
    }

    /// Gets the line of the line and column position.
    pub fn line(&self) -> u16 {
        self.line
    }

    /// Gets the column of the line and column position.
    pub fn col(&self) -> u16 {
        self.col
    }
}

/// A table of line and column positions for a source string.
#[derive(Clone, Debug)]
pub struct PosTbl {
    chars: Vec<LineColPos>,
}

impl PosTbl {
    /// Creates a new line and column position table.
    pub fn new(chars: Vec<LineColPos>) -> Self {
        Self { chars }
    }

    /// Generates a line and column position table from a source string.
    pub fn generate(src: impl Iterator<Item = char>) -> Self {
        let mut buf = Vec::new();

        let mut line = 0;
        let mut chr = 0;

        for c in src {
            let lcp = if c == '\n' {
                let p = LineColPos::new(line, chr);
                line += 1;
                chr = 0;
                p
            } else {
                let p = LineColPos::new(line, chr);
                chr += 1;
                p
            };

            buf.push(lcp);
        }

        Self::new(buf)
    }

    /// Gets the line and column position for a given byte index.
    pub fn get(&self, idx: usize) -> Option<LineColPos> {
        self.chars.get(idx).copied()
    }

    /// Gets the source position for a given byte index.
    pub fn get_srcpos(&self, idx: usize) -> Option<SrcPos> {
        self.get(idx).map(|lcp| SrcPos::new(idx as i32, lcp))
    }

    /// Gets the source position for a given byte index, panicking if out of bounds.
    pub fn expect_srcpos(&self, idx: usize) -> SrcPos {
        self.get_srcpos(idx).expect("srcpos: out of bounds")
    }

    /// Gets the source position for the end of the source string, panicking if the source string is empty.
    pub fn expect_end(&self) -> SrcPos {
        let last_lcp = *self.chars.last().expect("srcpos: empty");
        SrcPos::new(self.chars.len() as i32, last_lcp)
    }
}

/// A source position.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SrcPos {
    /// The byte index in the source file.
    off: i32,

    /// The line and column.
    lc: LineColPos,
}

impl SrcPos {
    /// Creates a new source position.
    pub fn new(off: i32, lc: LineColPos) -> Self {
        Self { off, lc }
    }

    /// Creates a dummy source position.
    pub fn dummy() -> Self {
        Self::new(-1, LineColPos::dummy())
    }

    /// Gets the byte index of the source position.
    pub fn off(&self) -> i32 {
        self.off
    }

    /// Gets the line and column position of the source position.
    pub fn line_col_pos(&self) -> LineColPos {
        self.lc
    }

    /// Gets the line of the source position.
    pub fn line(&self) -> u16 {
        self.lc.line
    }

    /// Gets the column of the source position.
    pub fn col(&self) -> u16 {
        self.lc.col
    }
}

impl Display for SrcPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "L{}:C{}", self.line(), self.col())
    }
}

impl Debug for SrcPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "L{}:C{}:#{}", self.line(), self.col(), self.off())
    }
}

/// A source span.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SrcSpan {
    start: SrcPos,
    end: SrcPos,
}

impl SrcSpan {
    /// Creates a new source span.
    pub fn new(start: SrcPos, end: SrcPos) -> Self {
        Self { start, end }
    }

    /// Gets the start of the source span.
    pub fn start(&self) -> SrcPos {
        self.start
    }

    /// Gets the end of the source span.
    pub fn end(&self) -> SrcPos {
        self.end
    }
}
