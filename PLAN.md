# W3C WebVTT Strict Compliance — Implementation Plan

This plan covers the four remaining items to bring the VTT parser into full
W3C spec compliance:

1. NULL (U+0000) preprocessing
2. Float percentages
3. Full REGION definition parsing
4. Cue text parsing

Each section lists the spec requirement, the design, affected files, and tests.

---

## 1. NULL (U+0000) Preprocessing

### Spec requirement

> Before parsing, replace every U+0000 NULL in the input with U+FFFD
> REPLACEMENT CHARACTER.

### Design

- This **cannot** be done zero-copy on the borrowed `&str` — a NULL byte
  would need to be replaced in-place or a new allocation made.
- **Option A (recommended):** Gate behind `alloc`/`std` feature. If the input
  contains a NULL, allocate a `Cow<'a, str>` (or just `String`) with
  replacements. If no NULLs, borrow the original.
- **Option B:** Document that the caller must pre-process NULLs. The parser
  asserts/panics on NULL.
- Use `Cow<'a, str>` to avoid allocation in the common case (no NULLs).

### Affected files

| File | Change |
|------|--------|
| `src/vtt.rs` | `Parser::new()` scans for `\0`; if found, replace into owned `String` stored in parser. The `Lines` iterator and all slices then borrow from the (possibly-owned) buffer. Change `input: &'a str` → internal `Cow<'a, str>`. |

### Tests

- Input with embedded NULLs → verify they become U+FFFD in cue body text.
- Input without NULLs → verify no allocation (stays borrowed).

---

## 2. Float Percentages

### Spec requirement

> A WebVTT percentage consists of:
> 1. One or more ASCII digits.
> 2. Optionally: a `.` followed by one or more ASCII digits.
> 3. A `%`.
>
> Must be in range 0..100.

The current `Percentage` type wraps a `u8` and only accepts integers.

### Design

- Change `Percentage` inner type from `u8` to `f32` (or `f64`; `f32` is
  sufficient for 0.0–100.0 with sub-percent precision).
- Keep `#[repr(transparent)]`.
- Update API:
  - `new() -> Self` (0.0)
  - `with(value: f32) -> Self` (panics if out of range)
  - `try_with(value: f32) -> Option<Self>`
  - `value() -> f32`
  - Keep `Display` impl formatting (e.g., `50` → `"50"`, `50.5` → `"50.5"`).
- Update `parse_percentage` in `src/vtt.rs` to parse `f32` instead of `u8`.
- The `Into<u8>` derive must be removed; replace with `Into<f32>`.

### Affected files

| File | Change |
|------|--------|
| `src/vtt/types.rs` | `Percentage(f32)` — update `new`, `with`, `try_with`, `value`, derives. Remove `Eq`/`Ord`/`Hash` (f32 is not `Eq`); add manual `PartialEq`/`PartialOrd` or use `ordered-float`. |
| `src/vtt.rs` | `parse_percentage` parses `f32`, range-checks 0.0..=100.0. |
| `tests/vtt.rs` | Update all `Percentage::with(N)` to `Percentage::with(N.0)` or `Percentage::with(N as f32)`. Add tests for `50.5%`, `0.1%`, `99.99%`. |
| `tests/wpt_vtt.rs` | Same percentage updates. |

### Alternative: keep integer path

If you want to avoid `f32` complexity (`Eq`/`Hash` loss), you could store the
percentage as a fixed-point `u32` in hundredths (0–10000 → 0.00%–100.00%).
This preserves `Eq`/`Ord`/`Hash`. API: `value_hundredths() -> u32`,
`value_f32() -> f32`.

---

## 3. Full REGION Definition Parsing

### Spec requirement

REGION blocks currently return raw text (`Block::Region(&str)`). The spec
requires parsing the body into a structured `Region` object with:

| Setting | Type | Default |
|---------|------|---------|
| `id` | string (no spaces, no `-->`) | `""` |
| `width` | percentage | `100%` |
| `lines` | integer (>= 0) | `3` |
| `regionanchor` | `(percentage, percentage)` | `(0%, 100%)` |
| `viewportanchor` | `(percentage, percentage)` | `(0%, 100%)` |
| `scroll` | `none` \| `up` | `none` |

Settings are `key:value` pairs, one per line. Unknown settings are ignored.

### Design

- New type `Region<'a>` in `src/vtt/types.rs`:
  ```rust
  pub struct Region<'a> {
    id: RegionId<'a>,
    width: Percentage,
    lines: u32,
    region_anchor: (Percentage, Percentage),
    viewport_anchor: (Percentage, Percentage),
    scroll: Scroll,
  }

  pub enum Scroll {
    None,
    Up,
  }
  ```
- Builder/setter pattern consistent with `CueOptions`.
- Add `Block::Region(Region<'a>)` variant instead of `Block::Region(T)`.
  - **Breaking change** — the `Region` variant switches from raw text to a
    parsed struct. Alternatively, add a second variant
    `Block::ParsedRegion(Region<'a>)` and keep the raw variant, but this adds
    complexity.
  - Recommended: just change `Block::Region` to hold `Region<'a>`. This is
    pre-1.0 so breaking changes are acceptable.
- New fn `parse_region_settings<'a>(body: &'a str) -> Region<'a>` in
  `src/vtt.rs`, similar to `parse_cue_settings`.
- In the `RegionBody` state, instead of returning raw text, call
  `parse_region_settings` and return `Block::Region(region)`.
- Writer: serialize `Region` back to `key:value` lines.

### Affected files

| File | Change |
|------|--------|
| `src/vtt/types.rs` | New `Region<'a>`, `Scroll` types. Update `Block` enum. |
| `src/vtt.rs` | `parse_region_settings()`, update `State::RegionBody` to return `Block::Region(Region)`. Update writer. |
| `tests/vtt.rs` | Update region tests to check parsed fields. |
| `tests/wpt_vtt.rs` | Same. |

---

## 4. Cue Text Parsing

### Spec requirement

Cue text is currently returned as a raw `&str`. The W3C spec defines a
DOM-tree parser (§6.4) that recognizes:

**Tags:** `<b>`, `<i>`, `<u>`, `<c>`, `<ruby>`, `<rt>`, `<v>`, `<lang>`

**Tag syntax:**
- Start tags: `<tag.class1.class2 annotation>` (classes and annotation are optional)
- End tags: `</tag>`
- Timestamp tags: `<HH:MM:SS.mmm>` or `<MM:SS.mmm>`

**Entities:** `&amp;`, `&lt;`, `&gt;`, `&lrm;`, `&rlm;`, `&nbsp;`

**Tree structure:**
- Root node contains a list of child nodes
- Internal nodes (tag nodes) have: tag name, classes, annotation (for `<v>`/`<lang>`), children
- Leaf nodes: text nodes, timestamp nodes

### Design

This is the largest feature. Two approaches:

#### Approach A: Parsed DOM tree (full spec compliance)

New types in a `src/vtt/cue_text.rs` module:

```rust
/// A parsed cue text DOM tree.
pub struct CueText<'a> {
  children: Vec<Node<'a>>,  // requires alloc
}

pub enum Node<'a> {
  Text(&'a str),
  Timestamp(Timestamp),
  Tag(TagNode<'a>),
}

pub struct TagNode<'a> {
  tag: Tag,
  classes: Vec<&'a str>,      // requires alloc
  annotation: Option<&'a str>,
  children: Vec<Node<'a>>,    // requires alloc
}

pub enum Tag {
  Bold,      // <b>
  Italic,    // <i>
  Underline, // <u>
  Class,     // <c>
  Ruby,      // <ruby>
  RubyText,  // <rt>
  Voice,     // <v>
  Lang,      // <lang>
}
```

- Gate behind `alloc`/`std` feature (requires `Vec`).
- Parser: state machine per §6.4 — states: Data, Tag, StartTag,
  StartTagAnnotation, StartTagClass, EndTag, TimestampTag.
- Provide `CueText::parse(raw: &'a str) -> CueText<'a>` — zero-copy for
  text slices, only allocates `Vec`s for the tree structure.
- **Do NOT change `Cue<'a, T>`** — keep it generic over `T`. Instead provide:
  - `Cue::parse_body(&self) -> CueText<'a>` method, or
  - A separate `parse_cue_text(s: &str) -> CueText` standalone function.
  - This way the parser still returns raw `&str` bodies by default, and users
    opt in to DOM parsing.

#### Approach B: Flat event/token stream (no alloc, simpler)

Instead of a tree, yield a flat iterator of tokens:

```rust
pub enum CueToken<'a> {
  Text(&'a str),
  StartTag { tag: Tag, classes: &'a str, annotation: Option<&'a str> },
  EndTag(Tag),
  Timestamp(Timestamp),
}
```

- Zero-alloc, `no_std`-friendly.
- Users build their own tree if needed.
- Simpler to implement.

#### Recommendation

Implement **both**: the token iterator as the core (`no_std`), and the DOM
tree builder on top (gated behind `alloc`/`std`). The token iterator is the
parsing engine; the tree is a convenience layer.

### Entity handling

Replace recognized entities during text node extraction:
- `&amp;` → `&`
- `&lt;` → `<`
- `&gt;` → `>`
- `&lrm;` → U+200E
- `&rlm;` → U+200F
- `&nbsp;` → U+00A0

If a text segment contains entities, it cannot be zero-copy — use `Cow<'a, str>`
for text nodes.

### Affected files

| File | Change |
|------|--------|
| `src/vtt/cue_text.rs` (new) | `CueToken`, `CueTokenizer` iterator, `Tag` enum, entity handling. |
| `src/vtt/cue_text.rs` (new) | `CueText`, `Node`, `TagNode` (behind `alloc`/`std`). |
| `src/vtt/types.rs` | Re-export cue text types from `Block`/`Cue`. |
| `src/vtt.rs` | `pub mod cue_text;`, re-exports. |
| `tests/vtt_cue_text.rs` (new) | Comprehensive tests for tokenizer and tree builder. |

---

## Implementation Order

The features have these dependencies:

```
1. NULL preprocessing     (independent, small)
2. Float percentages      (independent, medium — touches many files)
3. Full REGION parsing    (depends on #2 for float percentages in anchors)
4. Cue text parsing       (independent of 1–3, largest feature)
```

Suggested order: **1 → 2 → 3 → 4**

---

## Open Questions

1. **Float percentage representation:** `f32` vs fixed-point `u32` hundredths?
   `f32` is simpler but loses `Eq`/`Hash`. Fixed-point keeps `Eq`/`Hash` but
   has a slightly awkward API.

2. **`Block::Region` breaking change:** Replace the raw-text variant with
   `Region<'a>` struct, or add a new variant alongside?

3. **Cue text parsing API:** Standalone function vs method on `Cue`? Should
   the `Parser` optionally parse cue text automatically (e.g., via a type
   parameter or feature flag)?

4. **Entity handling allocation:** Use `Cow<'a, str>` for text nodes, or
   require callers to handle entities separately?
