use truer_span::Span;

#[test]
fn consumer_reaches_span_type() {
    let _span = Span::new(0, 0);
}
