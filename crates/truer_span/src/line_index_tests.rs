use super::{LineCol, LineIndex, WideEncoding, WideLineCol};

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

#[test]
fn large_text_is_indexed_and_queried_without_quadratic_cost() {
    let text = "x\n".repeat(1_000_000);
    let index = LineIndex::new(&text);
    for line in (0..1_000_000u32).step_by(100) {
        assert_eq!(index.line_col(line * 2), Some(LineCol { line, col: 0 }));
    }
}

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

#[test]
fn ascii_position_is_unchanged_in_utf16() {
    let index = LineIndex::new("abc");
    let line_col = LineCol { line: 0, col: 2 };
    assert_eq!(
        index.to_wide(WideEncoding::Utf16, line_col),
        Some(WideLineCol { line: 0, col: 2 })
    );
}

#[test]
fn two_byte_character_shortens_the_utf16_column() {
    let index = LineIndex::new("café x");
    let line_col = LineCol { line: 0, col: 6 };
    assert_eq!(
        index.to_wide(WideEncoding::Utf16, line_col),
        Some(WideLineCol { line: 0, col: 5 })
    );
}

#[test]
fn three_byte_character_shortens_the_utf16_column_further() {
    let index = LineIndex::new("€ x");
    let line_col = LineCol { line: 0, col: 4 };
    assert_eq!(
        index.to_wide(WideEncoding::Utf16, line_col),
        Some(WideLineCol { line: 0, col: 2 })
    );
}

#[test]
fn character_outside_the_basic_plane_counts_twice_in_utf16() {
    let index = LineIndex::new("𝄞x");
    let line_col = LineCol { line: 0, col: 4 };
    assert_eq!(
        index.to_wide(WideEncoding::Utf16, line_col),
        Some(WideLineCol { line: 0, col: 2 })
    );
}

#[test]
fn character_outside_the_basic_plane_counts_once_in_utf32() {
    let index = LineIndex::new("𝄞x");
    let line_col = LineCol { line: 0, col: 4 };
    assert_eq!(
        index.to_wide(WideEncoding::Utf32, line_col),
        Some(WideLineCol { line: 0, col: 1 })
    );
}

#[test]
fn columns_on_a_later_line_are_converted_against_that_line() {
    let index = LineIndex::new("é\né x");
    let line_col = LineCol { line: 1, col: 3 };
    assert_eq!(
        index.to_wide(WideEncoding::Utf16, line_col),
        Some(WideLineCol { line: 1, col: 2 })
    );
}

#[test]
fn line_past_the_end_has_no_wide_position() {
    let index = LineIndex::new("abc");
    let line_col = LineCol { line: 5, col: 0 };
    assert_eq!(index.to_wide(WideEncoding::Utf16, line_col), None);
}

#[test]
fn column_past_the_end_of_its_line_has_no_wide_position() {
    let index = LineIndex::new("abc");
    let line_col = LineCol { line: 0, col: 9 };
    assert_eq!(index.to_wide(WideEncoding::Utf16, line_col), None);
}

#[test]
fn column_inside_a_character_has_no_wide_position() {
    let index = LineIndex::new("café");
    let line_col = LineCol { line: 0, col: 4 };
    assert_eq!(index.to_wide(WideEncoding::Utf16, line_col), None);
}
