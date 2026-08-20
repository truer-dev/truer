use super::{LineCol, LineIndex};

#[test]
fn line_col_of_empty_text_is_origin() {
    let index = LineIndex::new("");
    assert_eq!(index.line_col(0), Some(LineCol { line: 0, col: 0 }));
}

#[test]
fn column_counts_bytes_before_it_on_its_line() {
    let index = LineIndex::new("const café = 1;");
    assert_eq!(index.line_col(6), Some(LineCol { line: 0, col: 6 }));
}

#[test]
fn column_counts_bytes_not_characters() {
    let index = LineIndex::new("café x");
    assert_eq!(index.line_col(6), Some(LineCol { line: 0, col: 6 }));
}
