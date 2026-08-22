Returns the byte offset the span begins at, which is the first byte it covers.

```rust
use truer_span::Span;

let source = "const café = 1;";
let name = Span::new(6, 11);

assert_eq!(&source[name.start() as usize..], "café = 1;");
```
