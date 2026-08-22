Returns the number of lines a reader would count: [`str::lines`], with a lone `\r` ending a line as well.

A trailing break closes the last line rather than opening another, so `"a\n"` is one line and `""` is none. The index still holds a position on that final empty line — see [`LineIndex::line_span`].

```rust
use truer_span::LineIndex;

assert_eq!(LineIndex::new("const café = 1;\nlet x = 2;\n").line_count(), 2);
assert_eq!(LineIndex::new("").line_count(), 0);

// `str::lines` splits on `\n` alone, so it sees one line here.
assert_eq!(LineIndex::new("a\rb").line_count(), 2);
```
