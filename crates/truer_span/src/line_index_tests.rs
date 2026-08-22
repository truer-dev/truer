use super::{LineCol, LineIndex, WideEncoding, WideLineCol};
use crate::Span;

const FIXTURE: &str = "const café = 1;\nlet x = 2;\n";
const UNTERMINATED_FIXTURE: &str = "const café = 1;\nlet x = 2;";

fn char_boundary_offsets(text: &str) -> impl Iterator<Item = u32> {
    (0..=text.len() as u32).filter(|&offset| text.is_char_boundary(offset as usize))
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

#[test]
fn wide_position_converts_back() {
    let index = LineIndex::new("café x");
    let wide = WideLineCol { line: 0, col: 5 };
    assert_eq!(
        index.to_narrow(WideEncoding::Utf16, wide),
        Some(LineCol { line: 0, col: 6 })
    );
}

#[test]
fn position_after_a_surrogate_pair_converts_back() {
    let index = LineIndex::new("𝄞x");
    let wide = WideLineCol { line: 0, col: 2 };
    assert_eq!(
        index.to_narrow(WideEncoding::Utf16, wide),
        Some(LineCol { line: 0, col: 4 })
    );
}

#[test]
fn wide_column_inside_a_surrogate_pair_has_no_narrow_position() {
    let index = LineIndex::new("𝄞x");
    let wide = WideLineCol { line: 0, col: 1 };
    assert_eq!(index.to_narrow(WideEncoding::Utf16, wide), None);
}

#[test]
fn same_wide_column_is_a_position_in_utf32() {
    let index = LineIndex::new("𝄞x");
    let wide = WideLineCol { line: 0, col: 1 };
    assert_eq!(
        index.to_narrow(WideEncoding::Utf32, wide),
        Some(LineCol { line: 0, col: 4 })
    );
}

#[test]
fn wide_position_past_the_end_of_its_line_has_no_narrow_position() {
    let index = LineIndex::new("abc");
    let wide = WideLineCol { line: 0, col: 9 };
    assert_eq!(index.to_narrow(WideEncoding::Utf16, wide), None);
}

#[test]
fn wide_line_past_the_end_has_no_narrow_position() {
    let index = LineIndex::new("abc");
    let wide = WideLineCol { line: 1, col: 0 };
    assert_eq!(index.to_narrow(WideEncoding::Utf16, wide), None);
}

#[test]
fn line_span_covers_its_text() {
    let index = LineIndex::new("ab\ncd");
    assert_eq!(index.line_span(0), Some(Span::new(0, 2)));
}

#[test]
fn line_span_excludes_a_carriage_return_newline_pair() {
    let index = LineIndex::new("ab\r\ncd");
    assert_eq!(index.line_span(0), Some(Span::new(0, 2)));
}

#[test]
fn line_span_excludes_a_lone_carriage_return() {
    let index = LineIndex::new("ab\rcd");
    assert_eq!(index.line_span(0), Some(Span::new(0, 2)));
}

#[test]
fn last_line_span_ends_at_the_end_of_the_text() {
    let index = LineIndex::new("ab\ncd");
    assert_eq!(index.line_span(1), Some(Span::new(3, 5)));
}

#[test]
fn final_empty_line_is_an_empty_span() {
    let index = LineIndex::new("a\n");
    assert_eq!(index.line_span(1), Some(Span::new(2, 2)));
}

#[test]
fn empty_text_has_one_empty_span() {
    let index = LineIndex::new("");
    assert_eq!(index.line_span(0), Some(Span::new(0, 0)));
}

#[test]
fn line_span_counts_bytes_not_characters() {
    let index = LineIndex::new("café\nx");
    assert_eq!(index.line_span(0), Some(Span::new(0, 5)));
}

#[test]
fn line_past_the_end_has_no_span() {
    let index = LineIndex::new("abc");
    assert_eq!(index.line_span(1), None);
}

#[test]
fn later_line_span_excludes_its_terminator() {
    let index = LineIndex::new("a\r\nb\r\nc");
    assert_eq!(index.line_span(1), Some(Span::new(3, 4)));
}

#[test]
fn empty_text_has_no_lines() {
    let index = LineIndex::new("");
    assert_eq!(index.line_count(), 0);
}

#[test]
fn text_without_a_trailing_newline_counts_its_last_line() {
    let index = LineIndex::new("a");
    assert_eq!(index.line_count(), 1);
}

#[test]
fn trailing_newline_does_not_add_a_line() {
    let index = LineIndex::new("a\n");
    assert_eq!(index.line_count(), 1);
}

#[test]
fn blank_line_before_the_end_is_counted() {
    let index = LineIndex::new("a\n\n");
    assert_eq!(index.line_count(), 2);
}

#[test]
fn lone_carriage_return_adds_a_line() {
    let index = LineIndex::new("a\rb");
    assert_eq!(index.line_count(), 2);
}

#[test]
fn carriage_return_newline_pair_adds_one_line() {
    let index = LineIndex::new("a\r\nb");
    assert_eq!(index.line_count(), 2);
}

#[test]
fn trailing_lone_carriage_return_does_not_add_a_line() {
    let index = LineIndex::new("a\r");
    assert_eq!(index.line_count(), 1);
}

#[test]
fn trailing_carriage_return_newline_pair_does_not_add_a_line() {
    let index = LineIndex::new("a\r\n");
    assert_eq!(index.line_count(), 1);
}

#[test]
fn trailing_whitespace_only_line_is_counted() {
    let index = LineIndex::new("a\n ");
    assert_eq!(index.line_count(), 2);
}

#[test]
fn every_character_boundary_round_trips_through_line_and_column() {
    let text = FIXTURE;
    let index = LineIndex::new(text);
    for offset in char_boundary_offsets(text) {
        let line_col = index.line_col(offset).unwrap();
        assert_eq!(index.offset(line_col), Some(offset));
    }
}

#[test]
fn every_character_boundary_round_trips_through_utf16() {
    let text = FIXTURE;
    let index = LineIndex::new(text);
    for offset in char_boundary_offsets(text) {
        let line_col = index.line_col(offset).unwrap();
        let wide = index.to_wide(WideEncoding::Utf16, line_col).unwrap();
        assert_eq!(index.to_narrow(WideEncoding::Utf16, wide), Some(line_col));
    }
}

#[test]
fn position_never_decreases_as_offset_increases() {
    let text = FIXTURE;
    let index = LineIndex::new(text);
    let positions: Vec<_> = char_boundary_offsets(text)
        .map(|offset| {
            let line_col = index.line_col(offset).unwrap();
            (line_col.line, line_col.col)
        })
        .collect();
    for pair in positions.windows(2) {
        assert!(pair[0] <= pair[1], "{:?} then {:?}", pair[0], pair[1]);
    }
}

#[test]
fn multi_character_grapheme_is_measured_by_its_parts_not_as_one() {
    let index = LineIndex::new("👨‍👩‍👧‍👦\nlet a = 1;\n");
    assert_eq!(index.offset(LineCol { line: 1, col: 0 }), Some(26));
    assert_eq!(
        index.to_wide(WideEncoding::Utf16, LineCol { line: 0, col: 25 }),
        Some(WideLineCol { line: 0, col: 11 })
    );
}

#[test]
fn text_mixing_every_line_break_is_counted_once_per_break() {
    let index = LineIndex::new("a\nb\r\nc\rd");
    assert_eq!(index.line_count(), 4);
}

#[test]
fn line_span_agrees_with_the_position_of_its_start() {
    let text = FIXTURE;
    let index = LineIndex::new(text);
    for line in 0..index.line_count() {
        let start = index.line_span(line).unwrap().start();
        assert_eq!(index.offset(LineCol { line, col: 0 }), Some(start));
        assert_eq!(index.line_col(start), Some(LineCol { line, col: 0 }));
    }
}

#[test]
fn round_trips_hold_on_text_with_no_trailing_newline() {
    let text = UNTERMINATED_FIXTURE;
    let index = LineIndex::new(text);
    for offset in char_boundary_offsets(text) {
        let line_col = index.line_col(offset).unwrap();
        assert_eq!(index.offset(line_col), Some(offset));
        let wide = index.to_wide(WideEncoding::Utf16, line_col).unwrap();
        assert_eq!(index.to_narrow(WideEncoding::Utf16, wide), Some(line_col));
    }
}

#[test]
fn narrow_conversion_counts_units_not_bytes() {
    let index = LineIndex::new("€€x");
    assert_eq!(
        index.to_narrow(WideEncoding::Utf16, WideLineCol { line: 0, col: 2 }),
        Some(LineCol { line: 0, col: 6 })
    );
}
