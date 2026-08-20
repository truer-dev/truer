use super::{LineCol, LineIndex};

#[test]
fn line_col_of_empty_text_is_origin() {
    let index = LineIndex::new("");
    assert_eq!(index.line_col(0), Some(LineCol { line: 0, col: 0 }));
}
