use super::LineIndex;
use crate::Span;

#[test]
fn line_span_covers_its_text() {
    let index = LineIndex::new("ab\ncd");
    assert_eq!(index.line_span(0), Some(Span::new(0, 2)));
}

#[test]
fn line_span_excludes_a_carriage_return_newline_pair() {
    let index = LineIndex::new("ab\r\ncd");
    assert_eq!(index.line_span(0), Some(Span::new(0, 2)));
}

#[test]
fn line_span_excludes_a_lone_carriage_return() {
    let index = LineIndex::new("ab\rcd");
    assert_eq!(index.line_span(0), Some(Span::new(0, 2)));
}

#[test]
fn last_line_span_ends_at_the_end_of_the_text() {
    let index = LineIndex::new("ab\ncd");
    assert_eq!(index.line_span(1), Some(Span::new(3, 5)));
}

#[test]
fn final_empty_line_is_an_empty_span() {
    let index = LineIndex::new("a\n");
    assert_eq!(index.line_span(1), Some(Span::new(2, 2)));
}

#[test]
fn empty_text_has_one_empty_span() {
    let index = LineIndex::new("");
    assert_eq!(index.line_span(0), Some(Span::new(0, 0)));
}

#[test]
fn line_span_counts_bytes_not_characters() {
    let index = LineIndex::new("café\nx");
    assert_eq!(index.line_span(0), Some(Span::new(0, 5)));
}

#[test]
fn line_past_the_end_has_no_span() {
    let index = LineIndex::new("abc");
    assert_eq!(index.line_span(1), None);
}

#[test]
fn later_line_span_excludes_its_terminator() {
    let index = LineIndex::new("a\r\nb\r\nc");
    assert_eq!(index.line_span(1), Some(Span::new(3, 4)));
}
