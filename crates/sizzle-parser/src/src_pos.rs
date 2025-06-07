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
    pub fn new(line: u16, col: u16) -> Self {
        Self { line, col }
    }

    pub fn dummy() -> Self {
        Self::new(0, 0)
    }

    pub fn line(&self) -> u16 {
        self.line
    }

    pub fn col(&self) -> u16 {
        self.col
    }
}

#[derive(Clone)]
pub struct PosTbl {
    chars: Vec<LineColPos>,
}

impl PosTbl {
    pub fn new(chars: Vec<LineColPos>) -> Self {
        Self { chars }
    }

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

    pub fn get(&self, idx: usize) -> Option<LineColPos> {
        self.chars.get(idx).copied()
    }

    pub fn get_srcpos(&self, idx: usize) -> Option<SrcPos> {
        self.get(idx).map(|lcp| SrcPos::new(idx as i32, lcp))
    }

    pub fn expect_srcpos(&self, idx: usize) -> SrcPos {
        self.get_srcpos(idx).expect("srcpos: out of bounds")
    }

    pub fn expect_end(&self) -> SrcPos {
        let last_lcp = *self.chars.last().expect("srcpos: empty");
        SrcPos::new(self.chars.len() as i32, last_lcp)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SrcPos {
    /// The byte index in the source file.
    off: i32,

    /// The line and column.
    lc: LineColPos,
}

impl SrcPos {
    pub fn new(off: i32, lc: LineColPos) -> Self {
        Self { off, lc }
    }

    pub fn dummy() -> Self {
        Self::new(-1, LineColPos::dummy())
    }

    pub fn off(&self) -> i32 {
        self.off
    }

    pub fn line_col_pos(&self) -> LineColPos {
        self.lc
    }

    pub fn line(&self) -> u16 {
        self.lc.line
    }

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SrcSpan {
    start: SrcPos,
    end: SrcPos,
}

impl SrcSpan {
    pub fn new(start: SrcPos, end: SrcPos) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> SrcPos {
        self.start
    }

    pub fn end(&self) -> SrcPos {
        self.end
    }
}
