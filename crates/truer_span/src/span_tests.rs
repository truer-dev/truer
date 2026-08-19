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

#[test]
fn span_at_the_start_of_a_text_is_valid() {
    let span = Span::new(0, 0);
    assert_eq!((span.start(), span.end()), (0, 0));
}

#[test]
fn spans_sort_by_where_they_start() {
    let mut spans = [Span::new(5, 9), Span::new(1, 3), Span::new(3, 4)];
    spans.sort();
    assert_eq!(spans, [Span::new(1, 3), Span::new(3, 4), Span::new(5, 9)]);
}

#[test]
fn spans_that_start_together_sort_by_where_they_end() {
    let mut spans = [Span::new(1, 9), Span::new(1, 3)];
    spans.sort();
    assert_eq!(spans, [Span::new(1, 3), Span::new(1, 9)]);
}

#[test]
#[should_panic]
fn span_cannot_end_before_it_starts() {
    Span::new(7, 3);
}
