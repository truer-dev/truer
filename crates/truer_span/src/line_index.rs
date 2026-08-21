#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: u32,
    pub col: u32,
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
        let is_last_line = line + 1 >= self.line_starts.len();
        let end = self.line_starts.get(line + 1).copied().unwrap_or(self.len);
        let offset = start + line_col.col;
        let in_bounds = if is_last_line {
            offset <= end
        } else {
            offset < end
        };
        in_bounds.then_some(offset)
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

#[cfg(test)]
#[path = "line_index_tests.rs"]
mod tests;
