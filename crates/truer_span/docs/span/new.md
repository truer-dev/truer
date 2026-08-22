The span between two byte offsets.

```rust
use truer_span::Span;

let name = Span::new(6, 11);

assert_eq!((name.start(), name.end()), (6, 11));
```

# Panics

In debug builds, when `start` is greater than `end`. The check is a `debug_assert!` rather
than an `assert!` because it runs once per token: worth its cost while the code that produces
spans is being written, not in every release build afterwards.
