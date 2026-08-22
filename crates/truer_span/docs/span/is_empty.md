Whether the span covers no bytes.

An empty span is a position rather than a region, so a renderer has nothing to underline and
draws a caret between two characters instead.

```rust
use truer_span::Span;

let name = Span::new(6, 11);

assert!(!name.is_empty());
assert!(Span::empty(name.start()).is_empty());
```
