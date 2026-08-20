#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: u32,
    pub col: u32,
}

#[derive(Debug)]
pub struct LineIndex {
    line_starts: Vec<u32>,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        let bytes = text.as_bytes();
        let mut line_starts = vec![0];
        line_starts.extend(
            bytes
                .iter()
                .enumerate()
                .filter(|&(i, &byte)| {
                    byte == b'\n' || (byte == b'\r' && bytes.get(i + 1) != Some(&b'\n'))
                })
                .map(|(i, _)| i as u32 + 1),
        );
        Self { line_starts }
    }

    pub fn line_col(&self, offset: u32) -> Option<LineCol> {
        let line = self.line_starts.partition_point(|&start| start <= offset) - 1;
        Some(LineCol {
            line: line as u32,
            col: offset - self.line_starts[line],
        })
    }
}

#[cfg(test)]
#[path = "line_index_tests.rs"]
mod tests;
