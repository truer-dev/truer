Builds the index in a single scan. That scan is the only pass over the text: a million-line file is indexed once and answered by binary search afterwards.

A line break is `\n`, `\r\n`, or a lone `\r`. U+2028 and U+2029 are not, because the authority for that set is the client that renders a position — an editor, a terminal — and none of them splits on those two.

```rust
use truer_span::LineIndex;

let index = LineIndex::new("const café = 1;\nlet x = 2;\n");
assert_eq!(index.line_count(), 2);

// One break each, whichever spelling.
assert_eq!(LineIndex::new("a\nb\r\nc\rd").line_count(), 4);
```
