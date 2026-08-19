#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub const fn start(&self) -> u32 {
        self.start
    }

    pub const fn end(&self) -> u32 {
        self.end
    }
}

#[cfg(test)]
#[path = "span_tests.rs"]
mod tests;
