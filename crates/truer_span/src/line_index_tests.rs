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

#[test]
fn offset_on_newline_ends_the_line_it_terminates() {
    let index = LineIndex::new("a\nb");
    assert_eq!(index.line_col(1), Some(LineCol { line: 0, col: 1 }));
}

#[test]
fn offset_after_newline_starts_the_next_line() {
    let index = LineIndex::new("a\nb");
    assert_eq!(index.line_col(2), Some(LineCol { line: 1, col: 0 }));
}

#[test]
fn carriage_return_and_newline_together_are_one_break() {
    let index = LineIndex::new("a\r\nb");
    assert_eq!(index.line_col(3), Some(LineCol { line: 1, col: 0 }));
}

#[test]
fn lone_carriage_return_is_a_break() {
    let index = LineIndex::new("a\rb");
    assert_eq!(index.line_col(2), Some(LineCol { line: 1, col: 0 }));
}

#[test]
fn carriage_return_before_a_pair_is_its_own_break() {
    let index = LineIndex::new("a\r\r\nb");
    assert_eq!(index.line_col(4), Some(LineCol { line: 2, col: 0 }));
}
