use truer_span::{LineCol, LineIndex, Span};

#[test]
fn consumer_reaches_span_type() {
    let _span = Span::new(0, 0);
}

#[test]
fn consumer_reaches_line_index_type() {
    let _index = LineIndex::new("");
}

#[test]
fn consumer_reaches_line_col_type() {
    let _line_col = LineCol { line: 0, col: 0 };
}
