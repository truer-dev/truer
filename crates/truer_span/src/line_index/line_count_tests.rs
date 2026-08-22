use super::LineIndex;

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
