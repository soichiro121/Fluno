use crate::ast::node::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    pub name: String,
    pub source: String,
    line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        let source = source.into();
        let mut line_starts = vec![0];
        for (idx, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(idx + ch.len_utf8());
            }
        }
        Self {
            name: name.into(),
            source,
            line_starts,
        }
    }

    pub fn byte_offset(&self, span: Span) -> Option<usize> {
        let line_start = *self.line_starts.get(span.line.saturating_sub(1))?;
        Some(line_start + span.column.saturating_sub(1))
    }

    pub fn line_text(&self, line: usize) -> Option<&str> {
        let start = *self.line_starts.get(line.saturating_sub(1))?;
        let end = self
            .line_starts
            .get(line)
            .copied()
            .unwrap_or(self.source.len());
        Some(self.source[start..end].trim_end_matches(['\r', '\n']))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_offsets_and_lines() {
        let file = SourceFile::new("test.fln", "fn main() {\n  let x = 1;\n}\n");
        assert_eq!(file.byte_offset(Span::new(2, 3, 3)), Some(14));
        assert_eq!(file.line_text(2), Some("  let x = 1;"));
    }
}
