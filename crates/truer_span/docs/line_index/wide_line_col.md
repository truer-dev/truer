A position whose column counts UTF-16 or UTF-32 code units rather than UTF-8 bytes. The line is the same 0-based line as in [`LineCol`].

Which of the two encodings a column was counted in is not recorded here, so the value means something only beside the [`WideEncoding`] that produced it: read a UTF-16 column back as UTF-32 and [`LineIndex::to_narrow`] answers with a different position, or with none.

```rust
use truer_span::{LineCol, LineIndex, WideEncoding, WideLineCol};

let index = LineIndex::new("const café = 1;\nlet x = 2;\n");
let after_cafe = LineCol { line: 0, col: 11 };

// `é` is two bytes but one UTF-16 unit, so the editor's column is one short of the byte column.
assert_eq!(
    index.to_wide(WideEncoding::Utf16, after_cafe),
    Some(WideLineCol { line: 0, col: 10 }),
);
```
