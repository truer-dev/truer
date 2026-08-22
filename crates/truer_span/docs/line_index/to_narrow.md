Converts a wide column back into a byte column, the inverse of [`LineIndex::to_wide`] under the same [`WideEncoding`].

`None` where a byte column would be `None`, and in one case of its own: a wide column falling between the two UTF-16 units of a character outside the Basic Multilingual Plane. An editor can send such a column; snapping it to a boundary is a policy, and the caller owns it.

```rust
use truer_span::{LineCol, LineIndex, WideEncoding, WideLineCol};

let index = LineIndex::new("const café = 1;\nlet x = 2;\n");
let after_cafe = LineCol { line: 0, col: 11 };
let wide = index.to_wide(WideEncoding::Utf16, after_cafe).unwrap();

assert_eq!(index.to_narrow(WideEncoding::Utf16, wide), Some(after_cafe));

// Column 1 falls between the clef's two UTF-16 units; as UTF-32 it is the caret after it.
let clef = LineIndex::new("𝄞x");
let inside = WideLineCol { line: 0, col: 1 };
assert_eq!(clef.to_narrow(WideEncoding::Utf16, inside), None);
assert_eq!(
    clef.to_narrow(WideEncoding::Utf32, inside),
    Some(LineCol { line: 0, col: 4 }),
);
```
