Converts a byte column into one counted in `encoding`'s code units. The line is carried through unchanged; only the column moves, and only past the non-ASCII characters preceding it on that line.

The position is resolved first, so `None` here means what it means for [`LineIndex::offset`]: no such line, a column past the end of its line, or a column inside a character.

```rust
use truer_span::{LineCol, LineIndex, WideEncoding, WideLineCol};

let index = LineIndex::new("const café = 1;\nlet x = 2;\n");

// `é` costs two bytes and one UTF-16 unit, so every column after it loses one.
assert_eq!(
    index.to_wide(WideEncoding::Utf16, LineCol { line: 0, col: 11 }),
    Some(WideLineCol { line: 0, col: 10 }),
);

// A line of ASCII converts to itself, whatever the lines above it hold.
assert_eq!(
    index.to_wide(WideEncoding::Utf16, LineCol { line: 1, col: 5 }),
    Some(WideLineCol { line: 1, col: 5 }),
);

// Column 10 falls between the two bytes of `é`, so it names no position at all.
assert_eq!(
    index.to_wide(WideEncoding::Utf16, LineCol { line: 0, col: 10 }),
    None,
);
```
