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

#[test]
fn offset_at_the_end_of_a_text_is_addressable() {
    let index = LineIndex::new("abc");
    assert_eq!(index.line_col(3), Some(LineCol { line: 0, col: 3 }));
}

#[test]
fn text_ending_in_a_newline_has_a_final_empty_line() {
    let index = LineIndex::new("a\n");
    assert_eq!(index.line_col(2), Some(LineCol { line: 1, col: 0 }));
}

#[test]
fn offset_past_the_end_has_no_position() {
    let index = LineIndex::new("abc");
    assert_eq!(index.line_col(4), None);
}

#[test]
fn offset_inside_a_character_has_no_position() {
    let index = LineIndex::new("café");
    assert_eq!(index.line_col(4), None);
}

#[test]
fn offset_inside_a_character_on_a_later_line_has_no_position() {
    let index = LineIndex::new("a\né");
    assert_eq!(index.line_col(3), None);
}

#[test]
fn line_separator_does_not_break_a_line() {
    let index = LineIndex::new("a\u{2028}b");
    assert_eq!(index.line_col(4), Some(LineCol { line: 0, col: 4 }));
}

#[test]
fn paragraph_separator_does_not_break_a_line() {
    let index = LineIndex::new("a\u{2029}b");
    assert_eq!(index.line_col(4), Some(LineCol { line: 0, col: 4 }));
}

#[test]
fn tab_occupies_one_column() {
    let index = LineIndex::new("\tx");
    assert_eq!(index.line_col(1), Some(LineCol { line: 0, col: 1 }));
}

#[test]
fn lines_beyond_the_first_two_are_found() {
    let index = LineIndex::new("a\nb\nc\nd");
    assert_eq!(index.line_col(6), Some(LineCol { line: 3, col: 0 }));
}
