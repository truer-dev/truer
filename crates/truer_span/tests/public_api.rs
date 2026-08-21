use truer_span::{LineCol, LineIndex, Span, WideLineCol};

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

#[test]
fn consumer_reaches_wide_line_col_type() {
    let _wide_line_col = WideLineCol { line: 0, col: 0 };
}
