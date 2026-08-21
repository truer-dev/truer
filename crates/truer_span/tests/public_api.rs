use truer_span::{LineIndex, Span};

#[test]
fn consumer_reaches_span_type() {
    let _span = Span::new(0, 0);
}

#[test]
fn consumer_reaches_line_index_type() {
    let _index = LineIndex::new("");
}
