#![doc = include_str!("../README.md")]

mod line_index;
mod span;

pub use line_index::{LineCol, LineIndex, WideEncoding, WideLineCol};
pub use span::Span;
