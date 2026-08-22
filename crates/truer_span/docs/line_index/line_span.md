Returns the byte span of a line's text, without its terminator. The caller is printing that line under a caret, and a trailing `\r\n` inside the underline helps nobody.

Its domain is one line wider than [`LineIndex::line_count`]: a text ending in a break has a final empty line, which this answers for so that the offset at the end of the text still has one. Iterate over `0..line_count()`, not until this returns `None`.

```rust
use truer_span::{LineIndex, Span};

let source = "const café = 1;\nlet x = 2;\n";
let index = LineIndex::new(source);

let first = index.line_span(0).unwrap();
assert_eq!(&source[first.start() as usize..first.end() as usize], "const café = 1;");
assert_eq!(index.line_span(1), Some(Span::new(17, 27)));

// Two lines, but three spans: the caret past the trailing newline is on one of its own.
assert_eq!(index.line_count(), 2);
assert_eq!(index.line_span(2), Some(Span::new(28, 28)));
assert_eq!(index.line_span(3), None);
```
