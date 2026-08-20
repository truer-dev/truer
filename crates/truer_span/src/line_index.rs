#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: u32,
    pub col: u32,
}

#[derive(Debug)]
pub struct LineIndex;

impl LineIndex {
    pub fn new(_text: &str) -> Self {
        Self
    }

    pub fn line_col(&self, offset: u32) -> Option<LineCol> {
        Some(LineCol {
            line: 0,
            col: offset,
        })
    }
}

#[cfg(test)]
#[path = "line_index_tests.rs"]
mod tests;
