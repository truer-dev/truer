use truer_span::{LineCol, LineIndex, Span, WideEncoding, WideLineCol};

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

#[test]
fn consumer_reaches_wide_encoding_type() {
    let _encoding = WideEncoding::Utf16;
}

#[test]
fn consumer_calls_every_method_the_index_exposes() {
    let index = LineIndex::new("a");
    let line_col = index.line_col(0).unwrap();
    index.offset(line_col).unwrap();
    let wide = index.to_wide(WideEncoding::Utf16, line_col).unwrap();
    let narrow = index.to_narrow(WideEncoding::Utf16, wide).unwrap();
    assert_eq!(narrow, line_col);
}
