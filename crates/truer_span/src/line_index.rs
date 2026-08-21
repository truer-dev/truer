#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: u32,
    pub col: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WideLineCol {
    pub line: u32,
    pub col: u32,
}

#[derive(Debug)]
pub enum WideEncoding {
    Utf16,
    Utf32,
}

#[derive(Debug)]
pub struct LineIndex {
    line_starts: Box<[u32]>,
    non_ascii_chars: Box<[(u32, u32)]>,
    len: u32,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        Self {
            line_starts: line_starts_of(text),
            non_ascii_chars: non_ascii_chars_of(text),
            len: text.len() as u32,
        }
    }

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

    pub fn to_wide(&self, encoding: WideEncoding, line_col: LineCol) -> Option<WideLineCol> {
        let offset = self.offset(line_col)?;
        let line_start = self.line_starts[line_col.line as usize];
        let reduction = self.wide_reduction(encoding, line_start, offset);
        Some(WideLineCol {
            line: line_col.line,
            col: line_col.col - reduction,
        })
    }

    pub fn to_narrow(&self, encoding: WideEncoding, wide: WideLineCol) -> Option<LineCol> {
        let line_start = *self.line_starts.get(wide.line as usize)?;
        let reduction =
            self.wide_reduction(encoding, line_start, line_start.saturating_add(wide.col));
        let col = wide.col + reduction;
        let offset = line_start.checked_add(col)?;
        if self.splits_a_character(offset) {
            return None;
        }
        Some(LineCol {
            line: wide.line,
            col,
        })
    }

    fn wide_reduction(&self, encoding: WideEncoding, line_start: u32, offset: u32) -> u32 {
        let units = units_for(encoding);
        self.non_ascii_chars
            .iter()
            .filter(|&&(start, _)| (line_start..offset).contains(&start))
            .map(|&(_, len)| len - units(len))
            .sum()
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

fn non_ascii_chars_of(text: &str) -> Box<[(u32, u32)]> {
    text.char_indices()
        .filter(|&(_, ch)| !ch.is_ascii())
        .map(|(i, ch)| (i as u32, ch.len_utf8() as u32))
        .collect()
}

fn utf16_units(len_utf8: u32) -> u32 {
    if len_utf8 > 3 { 2 } else { 1 }
}

fn units_for(encoding: WideEncoding) -> fn(u32) -> u32 {
    match encoding {
        WideEncoding::Utf16 => utf16_units,
        WideEncoding::Utf32 => |_| 1,
    }
}

#[cfg(test)]
#[path = "line_index_tests.rs"]
mod tests;
