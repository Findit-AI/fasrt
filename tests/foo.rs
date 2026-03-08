use std::num::NonZeroU64;

use fasrt::srt::{ParseSrtError, parse};
use fasrt::types::*;

/// Helper: collect the iterator into a Vec, unwrapping all Results.
fn collect(input: &str) -> Result<Vec<SrtEntry<&str>>, ParseSrtError> {
  parse(input).collect()
}

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
  // First entry is valid, second has a bad index
  let srt = "\
1
00:00:01,000 --> 00:00:04,000
Good

0
00:00:05,000 --> 00:00:08,000
Bad
";

  let mut iter = parse(srt);
  let first = iter.next().unwrap();
  assert!(first.is_ok());
  assert_eq!(*first.unwrap().body(), "Good");

  let second = iter.next().unwrap();
  assert!(second.is_err());

  // Iterator is exhausted after error
  assert!(iter.next().is_none());
}

#[test]
fn parse_lazy_single_entry() {
  let srt = "\
1
00:00:00,000 --> 00:00:01,000
Only one
";

  let mut iter = parse(srt);
  let entry = iter.next().unwrap().unwrap();
  assert_eq!(*entry.body(), "Only one");
  assert!(iter.next().is_none());
}

#[test]
fn header_encode_roundtrip() {
  let ts = SrtTimestamp::from_hmsm(
    Hour::with(1),
    Minute::with(23),
    Second::with(45),
    Millisecond::with(678),
  );
  assert_eq!(ts.encode().as_str(), "01:23:45,678");
}

#[test]
fn timestamp_to_duration() {
  let ts = SrtTimestamp::from_hmsm(
    Hour::with(1),
    Minute::with(0),
    Second::with(0),
    Millisecond::with(0),
  );
  assert_eq!(ts.to_duration().as_millis(), 3_600_000);
}
