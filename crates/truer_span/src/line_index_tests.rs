use super::{LineCol, LineIndex, WideEncoding, WideLineCol};

const FIXTURE: &str = "const café = 1;\nlet x = 2;\n";
const UNTERMINATED_FIXTURE: &str = "const café = 1;\nlet x = 2;";

fn char_boundary_offsets(text: &str) -> impl Iterator<Item = u32> {
    (0..=text.len() as u32).filter(|&offset| text.is_char_boundary(offset as usize))
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
