use super::{LineCol, LineIndex, WideEncoding, WideLineCol};

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
fn narrow_conversion_counts_units_not_bytes() {
    let index = LineIndex::new("€€x");
    assert_eq!(
        index.to_narrow(WideEncoding::Utf16, WideLineCol { line: 0, col: 2 }),
        Some(LineCol { line: 0, col: 6 })
    );
}
