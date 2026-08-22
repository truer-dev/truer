Which unit a wide column counts. Both exist because the Language Server Protocol negotiates a position encoding: UTF-16 is the one it defaults to, and UTF-32 is on offer. There is no UTF-8 variant — that column is a plain [`LineCol`].

The two part ways above the Basic Multilingual Plane, where a character is a UTF-16 surrogate pair but still one code point.

```rust
use truer_span::{LineCol, LineIndex, WideEncoding, WideLineCol};

// `𝄞` is four bytes, two UTF-16 units, one code point.
let index = LineIndex::new("𝄞x");
let after_clef = LineCol { line: 0, col: 4 };

assert_eq!(
    index.to_wide(WideEncoding::Utf16, after_clef),
    Some(WideLineCol { line: 0, col: 2 }),
);
assert_eq!(
    index.to_wide(WideEncoding::Utf32, after_clef),
    Some(WideLineCol { line: 0, col: 1 }),
);
```
