use super::Span;

#[test]
fn new_reports_the_offsets_it_was_built_from() {
    let span = Span::new(3, 7);
    assert_eq!((span.start(), span.end()), (3, 7));
}

#[test]
fn spans_over_the_same_offsets_are_equal() {
    assert_eq!(Span::new(3, 7), Span::new(3, 7));
}

#[test]
fn span_that_starts_where_it_ends_is_empty() {
    assert!(Span::new(5, 5).is_empty());
}

#[test]
fn span_covering_text_is_not_empty() {
    assert!(!Span::new(3, 7).is_empty());
}

#[test]
fn empty_span_is_built_at_one_offset() {
    assert_eq!(Span::empty(5), Span::new(5, 5));
}
