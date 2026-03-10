# W3C WebVTT Strict Compliance — Implementation Plan

This plan covers the four items to bring the VTT parser into full W3C spec
compliance. Items 2–4 are complete; item 1 is deferred.

---

## 1. NULL (U+0000) Preprocessing — DEFERRED

### Spec requirement

> Before parsing, replace every U+0000 NULL in the input with U+FFFD
> REPLACEMENT CHARACTER.

### Status

**Not implemented as a full input preprocessing step.** NULL replacement is
handled lazily inside `CueStr::normalize()` for cue text content. A full
input-level preprocessor (replacing NULLs before the main VTT parser runs)
has not been added, as it would require either:

- A `Cow<'a, str>` wrapper to preserve zero-copy when no NULLs are present, or
- Documenting that callers must pre-process NULLs themselves.

This can be added later if needed.

---

## 2. Float Percentages — DONE

### What was implemented

- Changed `Percentage` inner type from `u8` to `f64`.
- Kept `#[repr(transparent)]`.
- Manual `Eq`/`Hash`/`Ord` implementations using `to_bits()` and `total_cmp()`,
  safe because values are guaranteed to be in 0.0..=100.0 (no NaN/Infinity).
- `Display` impl: shows `50` for whole numbers, `50.5` for decimals.
- Updated `parse_percentage` in `src/vtt.rs` to parse `f64`.

### Files changed

- `src/vtt/types.rs` — `Percentage(f64)`, manual trait impls.
- `src/vtt.rs` — `parse_percentage` updated.
- `tests/vtt.rs`, `tests/wpt_vtt.rs` — all `Percentage::with(N)` → `Percentage::with(N.0)`.

---

## 3. Full REGION Definition Parsing — DONE

### What was implemented

- New types: `Region<'a>`, `RegionId<'a>`, `Scroll`, `Anchor`.
- `Block::Region(T)` changed to `Block::Region(Region<'a>)` (breaking).
- `parse_region_settings()` parses `key:value` pairs (id, width, lines,
  regionanchor, viewportanchor, scroll).
- Writer serializes `Region` back to `key:value` lines via `format_region`.
- Builder/setter pattern consistent with other types.

### Files changed

- `src/vtt/types.rs` — new types, `Block` enum change.
- `src/vtt.rs` — `parse_region_settings()`, `parse_anchor()`, `format_region()`.
- `tests/vtt.rs` — region tests check parsed fields.

---

## 4. Cue Text Parsing — DONE

### What was implemented

Two-layer design as requested:

**Layer 1 — `CueParser` (zero-alloc, `no_std`):**
- Logos DFA-backed iterator yielding `CueToken`s.
- `RawCueToken` lexer directly classifies tags (`StartBold`, `EndItalic`, etc.)
  at the DFA level — no string-based tag-name lookup.
- Text nodes stored as `CueStr` with lazy normalization.

**Layer 2 — `CueText` DOM tree (`alloc`/`std`):**
- `Node` enum: `Text(CueStr)`, `Timestamp`, `Tag(TagNode)`.
- `TagNode` with private fields, full getter/setter/builder API.
- Stack-based tree builder handles unclosed and mis-nested tags gracefully.

**`CueStr` — lazy normalization:**
- Struct with `raw: &'a str`, `requires_normalization: bool`, and
  `OnceCell<String>` (alloc-gated).
- Parser **never allocates** — just sets the flag when `&` or `\0` is found.
- `normalize()` lazily decodes entities and replaces NULLs, caching the result.
- Entity decoding also uses a logos `NormToken` DFA for fast matching.

**Supported entities:** `&amp;`, `&lt;`, `&gt;`, `&lrm;`, `&rlm;`, `&nbsp;`.

**Supported tags:** `<b>`, `<i>`, `<u>`, `<c>`, `<ruby>`, `<rt>`, `<v>`, `<lang>`,
with classes (`.class1.class2`) and annotations (` annotation text`).

### Files changed

- `src/vtt/cue_text.rs` (new) — `Tag`, `CueStr`, `CueToken`, `CueParser`,
  `RawCueToken`, `NormToken`, `CueText`, `Node`, `TagNode`.
- `src/vtt.rs` — `pub mod cue_text;`, `pub(crate)` for `parse_timestamp`.
- `tests/vtt_cue_text.rs` (new) — 28 tests covering tokenizer, entity
  decoding, NULL handling, lazy caching, and DOM tree building.

---

## Implementation Order (actual)

```
2. Float percentages      ✅
3. Full REGION parsing    ✅ (depended on #2)
4. Cue text parsing       ✅
1. NULL preprocessing     ⏳ (deferred — partial coverage via CueStr::normalize)
```

---

## Resolved Design Decisions

1. **Float percentage representation:** `f64` with manual `Eq`/`Hash`/`Ord`
   via `to_bits()`/`total_cmp()`.

2. **`Block::Region` breaking change:** Replaced raw-text variant with
   `Region<'a>` struct. Acceptable as pre-1.0.

3. **Cue text parsing API:** Standalone `CueParser::new()` iterator +
   `CueText::parse()` tree builder. Does not change `Cue<'a, T>`.

4. **Entity handling allocation:** `CueStr` with `OnceCell`-based lazy
   normalization. Parser never allocates; users call `normalize()` when needed.
