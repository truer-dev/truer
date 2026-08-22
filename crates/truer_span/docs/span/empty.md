The empty span at one offset.

An offset is a caret position between characters, not the index of one, so this names a place
rather than a region: where a token was expected and none was written, or where an edit would
be inserted. The caret after the last character is such a place, so an empty span at the
length of the text is a position and not an overrun.

```rust
use truer_span::Span;

let source = "const café = 1";
let missing_semicolon = Span::empty(source.len() as u32);

assert_eq!(missing_semicolon, Span::new(15, 15));
assert!(missing_semicolon.is_empty());
```
