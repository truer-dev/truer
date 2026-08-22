Returns the offset a position names, the inverse of [`LineIndex::line_col`], or `None` if the position names none.

A column may sit at its line's terminator — the caret at the end of what the line prints — but not past it, so a column is bounded by its own line rather than by the length of the text. Columns count bytes, so one landing inside a multi-byte character is `None` for the same reason [`LineIndex::line_col`] rejects such an offset.

```rust
use truer_span::{LineCol, LineIndex};

let index = LineIndex::new("const café = 1;\nlet x = 2;\n");

assert_eq!(index.offset(LineCol { line: 1, col: 0 }), Some(17));

// Line 0 is fifteen characters but sixteen bytes; column 16 is the caret before its newline.
assert_eq!(index.offset(LineCol { line: 0, col: 16 }), Some(16));
assert_eq!(index.offset(LineCol { line: 0, col: 17 }), None);

let after_cafe = index.line_col(11).unwrap();
assert_eq!(index.offset(after_cafe), Some(11));
```
