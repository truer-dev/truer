A position in a text as a 0-based line and a 0-based column, the column counting UTF-8 bytes from the start of its line.

Both fields are 0-based because every producer is, and so is the Language Server Protocol, the largest consumer; the 1-based `path:12:5` a terminal prints is the command line adding one as it formats. The column counts bytes rather than characters so that it stays an offset into its line — columns in UTF-16 or UTF-32 code units are a separate type, [`WideLineCol`].

```rust
use truer_span::{LineCol, LineIndex};

let index = LineIndex::new("const café = 1;\nlet x = 2;\n");

// Ten characters precede the caret after `café`, but eleven bytes do.
assert_eq!(index.line_col(11), Some(LineCol { line: 0, col: 11 }));
assert_eq!(index.line_col(17), Some(LineCol { line: 1, col: 0 }));
```
