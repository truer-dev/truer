use crate::Span;

#[doc = include_str!("../docs/line_index/line_col_type.md")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: u32,
    pub col: u32,
}

#[doc = include_str!("../docs/line_index/wide_line_col.md")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WideLineCol {
    pub line: u32,
    pub col: u32,
}

#[doc = include_str!("../docs/line_index/wide_encoding.md")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WideEncoding {
    Utf16,
    Utf32,
}

#[doc = include_str!("../docs/line_index/line_index.md")]
#[derive(Debug)]
pub struct LineIndex {
    line_starts: Box<[u32]>,
    crlf_lines: Box<[u32]>,
    non_ascii_chars: Box<[(u32, u32)]>,
    len: u32,
}

impl LineIndex {
    #[doc = include_str!("../docs/line_index/new.md")]
    pub fn new(text: &str) -> Self {
        let line_starts = line_starts_of(text);
        let crlf_lines = crlf_lines_of(text, &line_starts);
        Self {
            line_starts,
            crlf_lines,
            non_ascii_chars: non_ascii_chars_of(text),
            len: text.len() as u32,
        }
    }

    #[doc = include_str!("../docs/line_index/line_col.md")]
    pub fn line_col(&self, offset: u32) -> Option<LineCol> {
        if offset > self.len || self.splits_a_character(offset) {
            return None;
        }
        let line = self.line_starts.partition_point(|&start| start <= offset) - 1;
        Some(LineCol {
            line: line as u32,
            col: offset - self.line_starts[line],
        })
    }

    #[doc = include_str!("../docs/line_index/offset.md")]
    pub fn offset(&self, line_col: LineCol) -> Option<u32> {
        let line = line_col.line as usize;
        let start = *self.line_starts.get(line)?;
        let end_exclusive = self
            .line_starts
            .get(line + 1)
            .copied()
            .unwrap_or(self.len + 1);
        let offset = start.checked_add(line_col.col)?;
        (offset < end_exclusive && !self.splits_a_character(offset)).then_some(offset)
    }

    #[doc = include_str!("../docs/line_index/to_wide.md")]
    pub fn to_wide(&self, encoding: WideEncoding, line_col: LineCol) -> Option<WideLineCol> {
        let offset = self.offset(line_col)?;
        let line_start = self.line_starts[line_col.line as usize];
        let reduction = self.wide_reduction(encoding, line_start, offset);
        Some(WideLineCol {
            line: line_col.line,
            col: line_col.col - reduction,
        })
    }

    #[doc = include_str!("../docs/line_index/to_narrow.md")]
    pub fn to_narrow(&self, encoding: WideEncoding, wide: WideLineCol) -> Option<LineCol> {
        let line = wide.line as usize;
        let line_start = *self.line_starts.get(line)?;
        let end_exclusive = self
            .line_starts
            .get(line + 1)
            .copied()
            .unwrap_or(self.len + 1);
        let col = self.narrow_column(encoding, line_start, wide.col);
        let offset = line_start.checked_add(col)?;
        if offset >= end_exclusive || self.splits_a_character(offset) {
            return None;
        }
        Some(LineCol {
            line: wide.line,
            col,
        })
    }

    #[doc = include_str!("../docs/line_index/line_count.md")]
    pub fn line_count(&self) -> u32 {
        let count = self.line_starts.len() as u32;
        let final_line_is_empty = self.line_starts.last() == Some(&self.len);
        if final_line_is_empty {
            count - 1
        } else {
            count
        }
    }

    #[doc = include_str!("../docs/line_index/line_span.md")]
    pub fn line_span(&self, line: u32) -> Option<Span> {
        let start = *self.line_starts.get(line as usize)?;
        let end = match self.line_starts.get(line as usize + 1) {
            Some(&next_start) => next_start - self.terminator_width(line),
            None => self.len,
        };
        Some(Span::new(start, end))
    }

    fn terminator_width(&self, line: u32) -> u32 {
        match self.crlf_lines.binary_search(&line) {
            Ok(_) => 2,
            Err(_) => 1,
        }
    }

    fn narrow_column(&self, encoding: WideEncoding, line_start: u32, wide_col: u32) -> u32 {
        let mut reduction = 0;
        for &(start, len) in self.non_ascii_chars_from(line_start) {
            if start - line_start - reduction >= wide_col {
                break;
            }
            reduction += len - units_for(encoding, len);
        }
        wide_col + reduction
    }

    fn wide_reduction(&self, encoding: WideEncoding, line_start: u32, offset: u32) -> u32 {
        self.non_ascii_chars_from(line_start)
            .take_while(|&&(start, _)| start < offset)
            .map(|&(_, len)| len - units_for(encoding, len))
            .sum()
    }

    fn non_ascii_chars_from(&self, offset: u32) -> std::slice::Iter<'_, (u32, u32)> {
        let first = self
            .non_ascii_chars
            .partition_point(|&(start, _)| start < offset);
        self.non_ascii_chars[first..].iter()
    }

    fn splits_a_character(&self, offset: u32) -> bool {
        match self
            .non_ascii_chars
            .binary_search_by_key(&offset, |&(start, _)| start)
        {
            Ok(_) | Err(0) => false,
            Err(i) => {
                let (start, len) = self.non_ascii_chars[i - 1];
                offset < start + len
            }
        }
    }
}

fn line_starts_of(text: &str) -> Box<[u32]> {
    let bytes = text.as_bytes();
    let mut line_starts = vec![0];
    line_starts.extend(
        bytes
            .iter()
            .enumerate()
            .filter(|&(i, &byte)| ends_a_line(bytes, i, byte))
            .map(|(i, _)| i as u32 + 1),
    );
    line_starts.into_boxed_slice()
}

fn ends_a_line(bytes: &[u8], i: usize, byte: u8) -> bool {
    byte == b'\n' || (byte == b'\r' && bytes.get(i + 1) != Some(&b'\n'))
}

fn crlf_lines_of(text: &str, line_starts: &[u32]) -> Box<[u32]> {
    let bytes = text.as_bytes();
    line_starts
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(_, &start)| ends_with_a_pair(bytes, start))
        .map(|(i, _)| i as u32 - 1)
        .collect()
}

fn ends_with_a_pair(bytes: &[u8], line_start: u32) -> bool {
    let i = line_start as usize;
    i >= 2 && bytes[i - 1] == b'\n' && bytes[i - 2] == b'\r'
}

fn non_ascii_chars_of(text: &str) -> Box<[(u32, u32)]> {
    text.char_indices()
        .filter(|&(_, ch)| !ch.is_ascii())
        .map(|(i, ch)| (i as u32, ch.len_utf8() as u32))
        .collect()
}

fn units_for(encoding: WideEncoding, len_utf8: u32) -> u32 {
    match encoding {
        WideEncoding::Utf16 if len_utf8 > 3 => 2,
        _ => 1,
    }
}

#[cfg(test)]
#[path = "line_index_tests.rs"]
mod tests;
