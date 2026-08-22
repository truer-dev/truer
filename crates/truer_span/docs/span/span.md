A region of source text, as a pair of byte offsets into UTF-8.

Spans are half-open, `[start, end)`: the end offset is the caret *after* the region rather than the last byte in it, which is what lets a span index a slice directly and what lets an empty span name a position. A span's start never exceeds its end — [`Span::new`] checks that in debug builds.

```rust
use truer_span::Span;

let source = "const café = 1;";
let name = Span::new(6, 11);

assert_eq!(&source[name.start() as usize..name.end() as usize], "café");
assert_eq!(name.end() - name.start(), 5); // bytes, and `é` takes two
```

Ordering is by `start` first and `end` second, so a slice of spans sorts into the order a reader meets them in the file.

A `Span` names a region, never which text it belongs to; the three coordinate systems it is converted into are in the [crate documentation](crate).
