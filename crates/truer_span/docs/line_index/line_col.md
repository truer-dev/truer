Returns the position an offset names, or `None` if it names none.

An offset is a caret between characters, so the offset equal to the length of the text is a position — the caret after the last character — and not an overrun.

`None` means the offset is past the end, or inside a multi-byte character, where Rust itself would refuse the slice. Neither is clamped or snapped to a boundary: which way to move such an offset is a policy, and it belongs to whoever did the arithmetic that produced it.

```rust
use truer_span::{LineCol, LineIndex};

let index = LineIndex::new("const café = 1;\nlet x = 2;\n");

assert_eq!(index.line_col(6), Some(LineCol { line: 0, col: 6 }));

// 28 is the length of the text: the caret past the trailing newline, on a line of its own.
assert_eq!(index.line_col(28), Some(LineCol { line: 2, col: 0 }));

assert_eq!(index.line_col(29), None);
assert_eq!(index.line_col(10), None); // between the two bytes of `é`
```
