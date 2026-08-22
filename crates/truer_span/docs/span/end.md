Returns the byte offset the span ends before. Spans are half-open, so this is one past the last byte
covered and the text from it onwards is what follows the span.

```rust
use truer_span::Span;

let source = "const café = 1;";
let name = Span::new(6, 11);

assert_eq!(&source[..name.end() as usize], "const café");
assert_eq!(&source[name.end() as usize..], " = 1;");
```
