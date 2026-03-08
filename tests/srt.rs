#![cfg(any(feature = "alloc", feature = "std"))]

use core::num::NonZeroU64;
use std::vec::Vec;

use fasrt::{
  srt::{Entry, Options, ParseSrtError, Parser, Timestamp},
  types::*,
};

/// Helper: collect the strict parser into a Vec.
fn collect(input: &str) -> Result<Vec<Entry<&str>>, ParseSrtError> {
  Parser::strict(input).collect()
}

/// Helper: collect the lossy parser into a Vec.
fn collect_lossy(input: &str) -> Result<Vec<Entry<&str>>, ParseSrtError> {
  Parser::lossy(input).collect()
}

/// Helper: collect with specific options.
fn collect_with(input: &str, opts: Options) -> Result<Vec<Entry<&str>>, ParseSrtError> {
  Parser::with_options(input, opts).collect()
}

// ── Strict mode tests ──────────────────────────────────────────────────────

#[test]
fn parse_basic() {
  let srt = "\
1
00:00:01,000 --> 00:00:04,000
Hello world!

2
00:00:05,000 --> 00:00:08,000
Goodbye world!
";

  let subs = collect(srt).unwrap();
  assert_eq!(subs.len(), 2);

  let h0 = subs[0].header();
  assert_eq!(h0.index(), NonZeroU64::new(1));
  assert_eq!(h0.start().hours(), Hour::with(0));
  assert_eq!(h0.start().minutes(), Minute::with(0));
  assert_eq!(h0.start().seconds(), Second::with(1));
  assert_eq!(h0.start().millis(), Millisecond::with(0));
  assert_eq!(h0.end().seconds(), Second::with(4));
  assert_eq!(*subs[0].body(), "Hello world!");

  let h1 = subs[1].header();
  assert_eq!(h1.index(), NonZeroU64::new(2));
  assert_eq!(*subs[1].body(), "Goodbye world!");
}

#[test]
fn parse_multiline_text() {
  let srt = "\
1
00:00:01,000 --> 00:00:04,000
Line one
Line two
Line three

";

  let subs = collect(srt).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(*subs[0].body(), "Line one\nLine two\nLine three");
}

#[test]
fn parse_no_trailing_newline() {
  let srt = "\
1
00:00:01,500 --> 00:00:04,000
Hello!";

  let subs = collect(srt).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(*subs[0].body(), "Hello!");
}

#[test]
fn parse_with_bom() {
  let srt = "\u{feff}1
00:00:01,000 --> 00:00:04,000
BOM test
";

  let subs = collect(srt).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(*subs[0].body(), "BOM test");
}

#[test]
fn parse_large_hours() {
  let srt = "\
1
100:59:59,999 --> 200:00:00,000
Large hours
";

  let subs = collect(srt).unwrap();
  assert_eq!(subs[0].header().start().hours(), Hour::with(100));
  assert_eq!(subs[0].header().end().hours(), Hour::with(200));
}

#[test]
fn parse_empty_input() {
  let subs = collect("").unwrap();
  assert!(subs.is_empty());
}

#[test]
fn parse_only_whitespace() {
  let subs = collect("\n\n\n").unwrap();
  assert!(subs.is_empty());
}

#[test]
fn parse_error_bad_index() {
  let srt = "\
0
00:00:01,000 --> 00:00:04,000
Hello
";

  let err = collect(srt).unwrap_err();
  assert!(matches!(err, ParseSrtError::ParseIndex(_)));
}

#[test]
fn parse_error_missing_header() {
  let srt = "\
1
not a timeline
";

  let err = collect(srt).unwrap_err();
  assert!(
    matches!(
      err,
      ParseSrtError::ExpectedHeader(_) | ParseSrtError::Unknown(_)
    ),
    "unexpected error: {err:?}"
  );
}

#[test]
fn parse_many_entries() {
  let mut srt = String::new();
  for i in 1..=100u32 {
    srt.push_str(&format!(
      "{i}\n00:00:{s:02},000 --> 00:00:{e:02},000\nLine {i}\n\n",
      s = (i % 60),
      e = ((i + 1) % 60),
    ));
  }
  let subs = collect(&srt).unwrap();
  assert_eq!(subs.len(), 100);
}

#[test]
fn parse_crlf() {
  let srt = "1\r\n00:00:01,000 --> 00:00:04,000\r\nHello CRLF!\r\n\r\n";

  let subs = collect(srt).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(*subs[0].body(), "Hello CRLF!");
}

#[test]
fn parse_multiple_blank_lines_between() {
  let srt = "\
1
00:00:01,000 --> 00:00:02,000
First


2
00:00:03,000 --> 00:00:04,000
Second
";

  let subs = collect(srt).unwrap();
  assert_eq!(subs.len(), 2);
  assert_eq!(*subs[0].body(), "First");
  assert_eq!(*subs[1].body(), "Second");
}

#[test]
fn parse_leading_blank_lines() {
  let srt = "\n\n\n1\n00:00:01,000 --> 00:00:04,000\nHello\n";

  let subs = collect(srt).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(*subs[0].body(), "Hello");
}

#[test]
fn parse_no_body_text() {
  let srt = "\
1
00:00:01,000 --> 00:00:04,000

2
00:00:05,000 --> 00:00:08,000
Text
";

  let subs = collect(srt).unwrap();
  assert_eq!(subs.len(), 2);
  assert_eq!(*subs[0].body(), "");
  assert_eq!(*subs[1].body(), "Text");
}

#[test]
fn parse_lazy_stops_on_error() {
  let srt = "\
1
00:00:01,000 --> 00:00:04,000
Good

0
00:00:05,000 --> 00:00:08,000
Bad
";

  let mut iter = Parser::strict(srt);
  let first = iter.next().unwrap();
  assert!(first.is_ok());
  assert_eq!(*first.unwrap().body(), "Good");

  let second = iter.next().unwrap();
  assert!(second.is_err());

  assert!(iter.next().is_none());
}

#[test]
fn parse_lazy_single_entry() {
  let srt = "\
1
00:00:00,000 --> 00:00:01,000
Only one
";

  let mut iter = Parser::strict(srt);
  let entry = iter.next().unwrap().unwrap();
  assert_eq!(*entry.body(), "Only one");
  assert!(iter.next().is_none());
}

// ── Strict: monotonic index tests ──────────────────────────────────────────

#[test]
fn strict_monotonic_ok() {
  let srt = "\
1
00:00:01,000 --> 00:00:02,000
A

3
00:00:03,000 --> 00:00:04,000
B

10
00:00:05,000 --> 00:00:06,000
C
";

  let subs = collect(srt).unwrap();
  assert_eq!(subs.len(), 3);
}

#[test]
fn strict_monotonic_duplicate_index() {
  let srt = "\
1
00:00:01,000 --> 00:00:02,000
A

1
00:00:03,000 --> 00:00:04,000
B
";

  let mut iter = Parser::strict(srt);
  assert!(iter.next().unwrap().is_ok());
  let err = iter.next().unwrap().unwrap_err();
  assert!(
    matches!(err, ParseSrtError::NonMonotonicIndex { last: 1, got: 1 }),
    "unexpected error: {err:?}"
  );
}

#[test]
fn strict_monotonic_decreasing_index() {
  let srt = "\
5
00:00:01,000 --> 00:00:02,000
A

3
00:00:03,000 --> 00:00:04,000
B
";

  let mut iter = Parser::strict(srt);
  assert!(iter.next().unwrap().is_ok());
  let err = iter.next().unwrap().unwrap_err();
  assert!(
    matches!(err, ParseSrtError::NonMonotonicIndex { last: 5, got: 3 }),
    "unexpected error: {err:?}"
  );
}

// ── Lossy mode tests (all options enabled) ─────────────────────────────────

#[test]
fn lossy_valid_input_same_as_strict() {
  let srt = "\
1
00:00:01,000 --> 00:00:04,000
Hello

2
00:00:05,000 --> 00:00:08,000
World
";

  let strict = collect(srt).unwrap();
  let lossy = collect_lossy(srt).unwrap();
  assert_eq!(strict, lossy);
}

#[test]
fn lossy_missing_index() {
  let srt = "\
00:00:01,000 --> 00:00:04,000
No index entry
";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(subs[0].header().index(), None);
  assert_eq!(*subs[0].body(), "No index entry");
}

#[test]
fn lossy_missing_index_mixed() {
  let srt = "\
1
00:00:01,000 --> 00:00:04,000
With index

00:00:05,000 --> 00:00:08,000
Without index
";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 2);

  assert_eq!(subs[0].header().index(), NonZeroU64::new(1));
  assert_eq!(*subs[0].body(), "With index");

  assert_eq!(subs[1].header().index(), None);
  assert_eq!(*subs[1].body(), "Without index");
}

#[test]
fn lossy_orphan_text_skipped() {
  let srt = "\
1
00:00:01,000 --> 00:00:02,000
Someone

Orphan text

2
00:00:03,000 --> 00:00:04,000
Valid
";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 2);
  assert_eq!(*subs[0].body(), "Someone");
  assert_eq!(*subs[1].body(), "Valid");
}

#[test]
fn lossy_orphan_text_multiple_lines() {
  let srt = "\
1
00:00:01,000 --> 00:00:02,000
OK

random garbage
more garbage
still garbage

2
00:00:03,000 --> 00:00:04,000
Also OK
";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 2);
  assert_eq!(*subs[0].body(), "OK");
  assert_eq!(*subs[1].body(), "Also OK");
}

#[test]
fn lossy_incomplete_header_skipped() {
  let srt = "\
1
not a timeline
some body text

2
00:00:03,000 --> 00:00:04,000
Valid entry
";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(subs[0].header().index(), NonZeroU64::new(2));
  assert_eq!(*subs[0].body(), "Valid entry");
}

#[test]
fn lossy_incomplete_header_at_eof() {
  let srt = "\
1
not a timeline
";

  let subs = collect_lossy(srt).unwrap();
  assert!(subs.is_empty());
}

#[test]
fn lossy_incomplete_header_blank_immediately() {
  let srt = "\
1

2
00:00:01,000 --> 00:00:02,000
Valid
";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(subs[0].header().index(), NonZeroU64::new(2));
}

#[test]
fn lossy_bad_index_skipped() {
  let srt = "\
0
00:00:01,000 --> 00:00:04,000
Bad index

2
00:00:05,000 --> 00:00:08,000
Good
";

  let subs = collect_lossy(srt).unwrap();
  assert!(subs.iter().any(|s| *s.body() == "Good"));
}

#[test]
fn lossy_empty_input() {
  let subs = collect_lossy("").unwrap();
  assert!(subs.is_empty());
}

#[test]
fn lossy_only_orphan_text() {
  let srt = "\
hello
world
foo
";

  let subs = collect_lossy(srt).unwrap();
  assert!(subs.is_empty());
}

#[test]
fn lossy_header_without_index_at_eof() {
  let srt = "00:00:01,000 --> 00:00:04,000\nEOF entry";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(subs[0].header().index(), None);
  assert_eq!(*subs[0].body(), "EOF entry");
}

#[test]
fn lossy_orphan_text_then_header_accepted() {
  let srt = "\
bad header line
00:00:05,000 --> 00:00:08,000
body of broken cue

1
00:00:01,000 --> 00:00:02,000
Valid
";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 2);
  assert_eq!(subs[0].header().index(), None);
  assert_eq!(*subs[0].body(), "body of broken cue");
  assert_eq!(subs[1].header().index(), NonZeroU64::new(1));
  assert_eq!(*subs[1].body(), "Valid");
}

#[test]
fn lossy_multiple_orphan_lines_then_header() {
  let srt = "\
garbage
also garbage
00:00:01,000 --> 00:00:02,000
body

1
00:00:03,000 --> 00:00:04,000
OK
";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 2);
  assert_eq!(subs[0].header().index(), None);
  assert_eq!(*subs[0].body(), "body");
  assert_eq!(subs[1].header().index(), NonZeroU64::new(1));
  assert_eq!(*subs[1].body(), "OK");
}

#[test]
fn lossy_orphan_text_then_header_at_eof() {
  let srt = "\
bad header line
00:00:05,000 --> 00:00:08,000
orphan body";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(subs[0].header().index(), None);
  assert_eq!(*subs[0].body(), "orphan body");
}

#[test]
fn lossy_non_monotonic_index_not_enforced() {
  // Lossy mode does NOT enforce monotonic indices.
  let srt = "\
5
00:00:01,000 --> 00:00:02,000
A

3
00:00:03,000 --> 00:00:04,000
B
";

  let subs = collect_lossy(srt).unwrap();
  assert_eq!(subs.len(), 2);
}

// ── Lossy: individual option tests ─────────────────────────────────────────

#[test]
fn lossy_option_missing_index_disabled() {
  // With allow_missing_index=false, a header without index is an error.
  let srt = "\
00:00:01,000 --> 00:00:04,000
No index
";

  let opts = Options::lossy().with_allow_missing_index(false);

  // The header line is not a number, and allow_missing_index=false, so
  // it falls through to ignore_orphan_text (skipped). No entries produced.
  let subs = collect_with(srt, opts).unwrap();
  assert!(subs.is_empty());
}

#[test]
fn lossy_option_orphan_text_disabled() {
  // With ignore_orphan_text=false, orphan text causes an error.
  let srt = "\
garbage

1
00:00:01,000 --> 00:00:02,000
Valid
";

  let opts = Options::lossy()
    .with_allow_missing_index(false)
    .with_ignore_orphan_text(false);

  let err = collect_with(srt, opts).unwrap_err();
  assert!(
    matches!(err, ParseSrtError::Unknown(_)),
    "unexpected: {err:?}"
  );
}

#[test]
fn lossy_option_broken_header_disabled() {
  // With ignore_broken_header=false, a broken header after an index is an error.
  let srt = "\
1
not a header
";

  let opts = Options::lossy().with_ignore_broken_header(false);

  let err = collect_with(srt, opts).unwrap_err();
  assert!(
    matches!(
      err,
      ParseSrtError::ExpectedHeader(_) | ParseSrtError::Unknown(_)
    ),
    "unexpected: {err:?}"
  );
}

#[test]
fn lossy_option_only_missing_index() {
  // Only allow_missing_index enabled; others off.
  let srt = "\
00:00:01,000 --> 00:00:04,000
No index
";

  let opts = Options::strict()
    .with_allow_missing_index(true)
    .with_monotonic_index(false);

  let subs = collect_with(srt, opts).unwrap();
  assert_eq!(subs.len(), 1);
  assert_eq!(subs[0].header().index(), None);
}

#[test]
fn monotonic_off_accepts_non_monotonic() {
  let srt = "\
5
00:00:01,000 --> 00:00:02,000
A

3
00:00:03,000 --> 00:00:04,000
B
";

  // Strict (default) fails with NonMonotonicIndex.
  let err = collect(srt).unwrap_err();
  assert!(matches!(err, ParseSrtError::NonMonotonicIndex { .. }));

  // With monotonic_index disabled, non-monotonic order is accepted.
  let opts = Options::strict().with_monotonic_index(false);
  let subs = collect_with(srt, opts).unwrap();
  assert_eq!(subs.len(), 2);
}

#[test]
fn lossy_monotonic_index_skips_non_monotonic() {
  // With monotonic_index=true, non-monotonic entries are silently skipped.
  let srt = "\
1
00:00:01,000 --> 00:00:02,000
First

5
00:00:03,000 --> 00:00:04,000
Second

3
00:00:05,000 --> 00:00:06,000
Should be skipped

7
00:00:07,000 --> 00:00:08,000
Third
";

  let opts = Options::lossy().with_monotonic_index(true);

  let subs = collect_with(srt, opts).unwrap();
  assert_eq!(subs.len(), 3);
  assert_eq!(subs[0].body(), &"First");
  assert_eq!(subs[1].body(), &"Second");
  assert_eq!(subs[2].body(), &"Third");
}

// ── Type / encoding tests ──────────────────────────────────────────────────

#[test]
fn header_encode_roundtrip() {
  let ts = Timestamp::from_hmsm(
    Hour::with(1),
    Minute::with(23),
    Second::with(45),
    Millisecond::with(678),
  );
  assert_eq!(ts.encode().as_str(), "01:23:45,678");
}

#[test]
fn timestamp_to_duration() {
  let ts = Timestamp::from_hmsm(
    Hour::with(1),
    Minute::with(0),
    Second::with(0),
    Millisecond::with(0),
  );
  assert_eq!(ts.to_duration().as_millis(), 3_600_000);
}
