#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    /// # Panics
    ///
    /// Panics in debug builds if `start > end`.
    pub const fn new(start: u32, end: u32) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }

    pub const fn empty(at: u32) -> Self {
        Self { start: at, end: at }
    }

    pub const fn start(&self) -> u32 {
        self.start
    }

    pub const fn end(&self) -> u32 {
        self.end
    }

    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[cfg(test)]
#[path = "span_tests.rs"]
mod tests;
