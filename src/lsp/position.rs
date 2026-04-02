//! Byte offset ↔ LSP Position conversion.
//!
//! `LineIndex` pre-computes newline offsets for O(log n) lookups.

use lsp_types::Position;

use crate::ast::Span;

/// Pre-computed line start offsets for a source string.
///
/// `line_starts[i]` is the byte offset where line `i` begins.
/// Line 0 always starts at offset 0.
#[derive(Clone, Debug)]
pub struct LineIndex {
    line_starts: Vec<u32>,
}

impl LineIndex {
    /// Build a `LineIndex` from source text.
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0u32];
        for (i, b) in source.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push((i + 1) as u32);
            }
        }
        LineIndex { line_starts }
    }

    /// Convert a byte offset to an LSP `Position` (0-based line + character).
    pub fn position(&self, offset: u32) -> Position {
        let line = self
            .line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1);
        let col = offset - self.line_starts[line];
        Position::new(line as u32, col)
    }

    /// Convert an LSP `Position` back to a byte offset.
    pub fn offset(&self, position: Position) -> u32 {
        let line = position.line as usize;
        if line < self.line_starts.len() {
            self.line_starts[line] + position.character
        } else {
            // Past the last line — clamp to end.
            *self.line_starts.last().unwrap_or(&0)
        }
    }

    /// Convert a `Span` (byte offsets) to an LSP `Range`.
    pub fn range(&self, span: Span) -> lsp_types::Range {
        lsp_types::Range::new(self.position(span.start), self.position(span.end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line() {
        let idx = LineIndex::new("hello");
        assert_eq!(idx.position(0), Position::new(0, 0));
        assert_eq!(idx.position(3), Position::new(0, 3));
        assert_eq!(idx.position(5), Position::new(0, 5));
    }

    #[test]
    fn multiple_lines() {
        let src = "abc\ndef\nghi";
        let idx = LineIndex::new(src);
        // 'a' = 0, 'd' = 4, 'g' = 8
        assert_eq!(idx.position(0), Position::new(0, 0));
        assert_eq!(idx.position(3), Position::new(0, 3)); // '\n'
        assert_eq!(idx.position(4), Position::new(1, 0)); // 'd'
        assert_eq!(idx.position(6), Position::new(1, 2)); // 'f'
        assert_eq!(idx.position(8), Position::new(2, 0)); // 'g'
        assert_eq!(idx.position(10), Position::new(2, 2)); // 'i'
    }

    #[test]
    fn offset_roundtrip() {
        let src = "line one\nline two\nline three";
        let idx = LineIndex::new(src);
        for offset in 0..src.len() as u32 {
            let pos = idx.position(offset);
            let back = idx.offset(pos);
            assert_eq!(back, offset, "roundtrip failed at offset {}", offset);
        }
    }

    #[test]
    fn empty_source() {
        let idx = LineIndex::new("");
        assert_eq!(idx.position(0), Position::new(0, 0));
        assert_eq!(idx.offset(Position::new(0, 0)), 0);
        // Past-end position clamps gracefully.
        assert_eq!(idx.offset(Position::new(5, 0)), 0);
    }

    #[test]
    fn span_to_range() {
        let src = "abc\ndef\nghi";
        let idx = LineIndex::new(src);
        let span = Span::new(4, 7); // "def"
        let range = idx.range(span);
        assert_eq!(range.start, Position::new(1, 0));
        assert_eq!(range.end, Position::new(1, 3));
    }
}
