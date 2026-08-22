# Truer Span

A region of source text as a pair of byte offsets, and one index that converts it into the
line, column and UTF-16 coordinates its consumers ask for.

## Why

Every diagnostic truer emits points at a region of text, and three consumers want that region
in three incompatible coordinate systems. A parser hands out byte offsets, because that is
what slicing a `&str` takes. The command line prints `path:12:5`, 1-based line and column.
The Language Server Protocol asks for 0-based lines and columns counted in UTF-16 code units,
a unit no other part of the program uses.

Left to themselves, each consumer writes its own conversion, and the conversions disagree at
the first character outside ASCII: one counts `é` as a single column, another as the two bytes
it occupies, a third as the one UTF-16 unit the editor is waiting for. The caret lands beside
the error rather than under it, or a rename edits the wrong bytes — and it only goes wrong in
a file somebody else wrote. This crate owns the region type and the single index that converts
between the three, so there is one place to get it right.

## Install

```sh
cargo add truer_span
```

## Usage

```rust
use truer_span::{LineCol, LineIndex, Span, WideEncoding, WideLineCol};

let source = "const café = 1;\nlet x = 2;\n";
let index = LineIndex::new(source);

// A diagnostic points at `café`.
let span = Span::new(6, 11);
assert_eq!(index.line_col(span.start()), Some(LineCol { line: 0, col: 6 }));

// The command line adds one to each field and prints `path:1:7`. The editor wants the
// same position counted in UTF-16 code units, where `é` is one unit rather than two bytes.
let line_col = index.line_col(span.end()).unwrap();
assert_eq!(
    index.to_wide(WideEncoding::Utf16, line_col),
    Some(WideLineCol { line: 0, col: 10 }),
);

let line = index.line_span(0).unwrap();
assert_eq!(&source[line.start() as usize..line.end() as usize], "const café = 1;");
```

## Dependencies

None, and not even a lifetime. `LineIndex::new` takes the text, captures what it will need —
the line starts, which lines end in a `\r\n`, and where the non-ASCII characters sit — and
holds no borrow of it afterwards. So this crate is a leaf: it builds on its own, every other
truer crate can depend on it without a cycle, and there is nothing here to audit or upgrade
but the standard library.

## Behaviour

- Offsets are byte offsets into UTF-8 text, held as `u32`. Files too large for that are
  rejected before they reach this crate.
- An offset is a caret position *between* characters, not the index of one, so `offset == len`
  is a position and spans are half-open: `Span::new(3, 7)` covers four bytes.
- `LineCol` is 0-based in both fields. The 1-based `path:12:5` is the command line adding one
  as it formats, not a second convention living here.
- Columns count UTF-8 bytes within their line. UTF-16 and UTF-32 columns are a separate type,
  `WideLineCol`, reached through `WideEncoding`.
- A line break is `\n`, `\r\n`, or a lone `\r`. U+2028 and U+2029 are not: the authority is the
  client that renders the position, and no consumer splits on them.
- `line_span` excludes the line terminator, because its caller prints that line under a caret.
- Out of range, or inside a multi-byte character, is `None`, never a clamped or snapped
  answer. Clamping is a policy the caller owns.
- The index has a final empty line so that `offset == len` maps somewhere, but `line_count`
  does not count it. `"a\n"` has a `line_span(1)` and a `line_count()` of 1 — so iterate on
  the count, not on `line_span` returning `Some`.

## Not in this crate

- **Reading files, and the byte order mark.** `truer_vfs` strips the mark at the read seam, so
  no text arriving here has one.
- **Which text a span belongs to.** A `Span` names a region, never a file; `truer_diagnostics`
  owns `FileId` and the `@include` chain.
- **Clamping an out-of-range position.** That is a policy, and it belongs to `truer_lsp`.
- **Display columns.** Tab expansion and East Asian width belong where the caret is drawn.
- **The lines a `Span` covers.** `line_count` is whole-text; what a node's span covers is the
  consumer's policy. Spans are half-open, so the last line is the line of `end - 1`.

Per-method detail — parameters, return values, and what makes each of them `None` — is in the
reference documentation on [docs.rs](https://docs.rs/truer_span).
