use super::Span;

#[test]
fn new_reports_the_offsets_it_was_built_from() {
    let span = Span::new(3, 7);
    assert_eq!((span.start(), span.end()), (3, 7));
}
