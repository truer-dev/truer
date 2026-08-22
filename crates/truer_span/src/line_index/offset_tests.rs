use super::{LineCol, LineIndex};

#[test]
fn first_position_is_first_offset() {
    let index = LineIndex::new("");
    assert_eq!(index.offset(LineCol { line: 0, col: 0 }), Some(0));
}

#[test]
fn column_at_the_line_terminator_is_an_offset() {
    let index = LineIndex::new("a\nb");
    assert_eq!(index.offset(LineCol { line: 0, col: 1 }), Some(1));
}

#[test]
fn position_on_a_later_line_resolves_past_its_break() {
    let index = LineIndex::new("a\nb");
    assert_eq!(index.offset(LineCol { line: 1, col: 0 }), Some(2));
}

#[test]
fn caret_after_the_last_character_is_an_offset() {
    let index = LineIndex::new("abc");
    assert_eq!(index.offset(LineCol { line: 0, col: 3 }), Some(3));
}

#[test]
fn position_round_trips_from_the_offset_it_came_from() {
    let index = LineIndex::new("café x");
    let line_col = index.line_col(6).unwrap();
    assert_eq!(index.offset(line_col), Some(6));
}

#[test]
fn position_resolving_past_a_carriage_return_newline_pair() {
    let index = LineIndex::new("a\r\nb");
    assert_eq!(index.offset(LineCol { line: 1, col: 0 }), Some(3));
}

#[test]
fn final_empty_line_has_an_offset() {
    let index = LineIndex::new("a\n");
    assert_eq!(index.offset(LineCol { line: 1, col: 0 }), Some(2));
}

#[test]
fn line_past_the_end_has_no_offset() {
    let index = LineIndex::new("abc");
    assert_eq!(index.offset(LineCol { line: 1, col: 0 }), None);
}

#[test]
fn column_past_the_end_of_its_line_has_no_offset() {
    let index = LineIndex::new("ab\ncd");
    assert_eq!(index.offset(LineCol { line: 0, col: 3 }), None);
}

#[test]
fn column_past_the_end_of_the_last_line_has_no_offset() {
    let index = LineIndex::new("ab\ncd");
    assert_eq!(index.offset(LineCol { line: 1, col: 3 }), None);
}

#[test]
fn column_inside_a_character_has_no_offset() {
    let index = LineIndex::new("café");
    assert_eq!(index.offset(LineCol { line: 0, col: 4 }), None);
}

#[test]
fn column_that_overflows_the_offset_has_no_offset() {
    let index = LineIndex::new("ab\ncd");
    assert_eq!(
        index.offset(LineCol {
            line: 1,
            col: u32::MAX
        }),
        None
    );
}
