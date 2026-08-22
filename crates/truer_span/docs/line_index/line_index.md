An index over one text, converting a byte offset into the line and column each consumer asks for, and back again.

Building it is a single scan that captures everything the conversions will need — where each line starts, which lines end in a `\r\n`, and where the non-ASCII characters sit — so the index holds no borrow of the text and outlives the buffer it was built from.

```rust
use truer_span::{LineCol, LineIndex};

let index = {
    let source = String::from("const café = 1;\nlet x = 2;\n");
    LineIndex::new(&source)
};

assert_eq!(index.line_count(), 2);
assert_eq!(index.line_col(17), Some(LineCol { line: 1, col: 0 }));
```

The three coordinate systems, and why one index owns all of them, are in the [crate documentation](crate).
