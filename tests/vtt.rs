#![cfg(any(feature = "alloc", feature = "std"))]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

use std::vec::Vec;

use fasrt::{
  types::*,
  vtt::{
    Align, Block, Cue, Hour, Line, LineAlign, LineValue, ParseVttError, Parser, Percentage,
    Position, PositionAlign, Size, Timestamp, Vertical,
  },
};

/// Helper: collect all blocks.
fn collect<'a>(input: &'a str) -> Result<Vec<Block<'a, &'a str>>, ParseVttError> {
  Parser::new(input).collect()
}

/// Helper: collect only cues.
fn collect_cues<'a>(input: &'a str) -> Result<Vec<Cue<'a, &'a str>>, ParseVttError> {
  let blocks = collect(input)?;
  Ok(
    blocks
      .into_iter()
      .filter_map(|b| match b {
        Block::Cue(c) => Some(c),
        _ => None,
      })
      .collect(),
  )
}

// ── Signature tests ─────────────────────────────────────────────────────────

#[test]
fn missing_signature() {
  let err = collect("not webvtt").unwrap_err();
  assert!(matches!(err, ParseVttError::MissingSignature));
}

#[test]
fn empty_input() {
  let err = collect("").unwrap_err();
  assert!(matches!(err, ParseVttError::MissingSignature));
}

#[test]
fn invalid_signature_no_space() {
  // "WEBVTTx" — character after WEBVTT is not space/tab
  let err = collect("WEBVTTx\n").unwrap_err();
  assert!(matches!(err, ParseVttError::InvalidSignature));
}

#[test]
fn bare_signature() {
  let blocks = collect("WEBVTT\n").unwrap();
  assert!(blocks.is_empty());
}

#[test]
fn signature_with_header_text() {
  let blocks = collect("WEBVTT Some header text\n\n").unwrap();
  assert!(blocks.is_empty());
}

#[test]
fn signature_with_tab() {
  let blocks = collect("WEBVTT\tsome text\n\n").unwrap();
  assert!(blocks.is_empty());
}

#[test]
fn signature_with_bom() {
  let blocks = collect("\u{feff}WEBVTT\n\n").unwrap();
  assert!(blocks.is_empty());
}

// ── Basic cue parsing ───────────────────────────────────────────────────────

#[test]
fn parse_single_cue() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000
Hello world!
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 1);
  assert_eq!(*cues[0].body_ref(), "Hello world!");
  assert_eq!(cues[0].header_ref().start().seconds(), Second::with(1));
  assert_eq!(cues[0].header_ref().end().seconds(), Second::with(4));
}

#[test]
fn parse_multiple_cues() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000
First

00:00:05.000 --> 00:00:08.000
Second
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 2);
  assert_eq!(*cues[0].body_ref(), "First");
  assert_eq!(*cues[1].body_ref(), "Second");
}

#[test]
fn parse_cue_with_identifier() {
  let vtt = "\
WEBVTT

intro
00:00:01.000 --> 00:00:04.000
Hello!
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 1);

  #[cfg(any(feature = "alloc", feature = "std"))]
  {
    let id = cues[0].header_ref().identifier().unwrap();
    assert_eq!(id.as_str(), "intro");
  }

  assert_eq!(*cues[0].body_ref(), "Hello!");
}

#[test]
fn parse_cue_multiline_body() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000
Line one
Line two
Line three
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 1);
  assert_eq!(*cues[0].body_ref(), "Line one\nLine two\nLine three");
}

#[test]
fn parse_cue_no_trailing_newline() {
  let vtt = "\
WEBVTT

00:00:01.500 --> 00:00:04.000
Hello!";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 1);
  assert_eq!(*cues[0].body_ref(), "Hello!");
}

#[test]
fn parse_cue_empty_body() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000

00:00:05.000 --> 00:00:08.000
Text
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 2);
  assert_eq!(*cues[0].body_ref(), "");
  assert_eq!(*cues[1].body_ref(), "Text");
}

#[test]
fn parse_cue_crlf() {
  let vtt = "WEBVTT\r\n\r\n00:00:01.000 --> 00:00:04.000\r\nHello CRLF!\r\n";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 1);
  assert_eq!(*cues[0].body_ref(), "Hello CRLF!");
}

// ── Timestamp format tests ──────────────────────────────────────────────────

#[test]
fn parse_short_timestamp_mm_ss() {
  let vtt = "\
WEBVTT

01:30.500 --> 02:00.000
Short form
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 1);
  let start = cues[0].header_ref().start();
  assert_eq!(start.hours(), Hour::new());
  assert_eq!(start.minutes(), Minute::with(1));
  assert_eq!(start.seconds(), Second::with(30));
  assert_eq!(start.millis(), Millisecond::with(500));
}

#[test]
fn parse_large_hours() {
  let vtt = "\
WEBVTT

100:59:59.999 --> 200:00:00.000
Large hours
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues[0].header_ref().start().hours(), Hour::with(100));
  assert_eq!(cues[0].header_ref().end().hours(), Hour::with(200));
}

#[test]
fn parse_very_large_hours() {
  let vtt = "\
WEBVTT

99999:59:59.999 --> 100000:00:00.000
Unbounded hours
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues[0].header_ref().start().hours(), Hour::with(99999));
  assert_eq!(cues[0].header_ref().end().hours(), Hour::with(100000));
}

#[test]
fn timestamp_encode_large_hours() {
  let ts = Timestamp::from_hmsm(
    Hour::with(99999),
    Minute::with(59),
    Second::with(59),
    Millisecond::with(999),
  );
  assert_eq!(ts.encode().as_str(), "99999:59:59.999");
}

// ── Cue settings ────────────────────────────────────────────────────────────

#[test]
fn parse_cue_with_settings() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000 vertical:rl line:50% position:10% size:80% align:center
Styled cue
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 1);
  let settings = cues[0].header_ref().settings().unwrap();
  assert_eq!(settings.vertical(), Some(Vertical::Rl));
  assert_eq!(
    settings.line(),
    Some(&Line::new(LineValue::Percentage(Percentage::with(50))))
  );
  assert_eq!(
    settings.position(),
    Some(&Position::new(Percentage::with(10)))
  );
  assert_eq!(settings.size(), Some(Size::new(Percentage::with(80))));
  assert_eq!(settings.align(), Some(Align::Center));
}

#[test]
fn parse_cue_line_with_alignment() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000 line:50%,start
Test
";

  let cues = collect_cues(vtt).unwrap();
  let settings = cues[0].header_ref().settings().unwrap();
  assert_eq!(
    settings.line(),
    Some(&Line::with_alignment(
      LineValue::Percentage(Percentage::with(50)),
      LineAlign::Start
    ))
  );
}

#[test]
fn parse_cue_line_number() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000 line:-1
Test
";

  let cues = collect_cues(vtt).unwrap();
  let settings = cues[0].header_ref().settings().unwrap();
  assert_eq!(settings.line(), Some(&Line::new(LineValue::Number(-1))));
}

#[test]
fn parse_cue_position_with_alignment() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000 position:50%,line-left
Test
";

  let cues = collect_cues(vtt).unwrap();
  let settings = cues[0].header_ref().settings().unwrap();
  assert_eq!(
    settings.position(),
    Some(&Position::with_alignment(
      Percentage::with(50),
      PositionAlign::LineLeft
    ))
  );
}

#[test]
fn parse_all_align_values() {
  for (value, expected) in [
    ("start", Align::Start),
    ("center", Align::Center),
    ("end", Align::End),
    ("left", Align::Left),
    ("right", Align::Right),
  ] {
    let vtt = std::format!("WEBVTT\n\n00:00:01.000 --> 00:00:04.000 align:{value}\nTest\n");
    let cues = collect_cues(&vtt).unwrap();
    assert_eq!(
      cues[0].header_ref().settings().unwrap().align(),
      Some(expected)
    );
  }
}

#[test]
fn parse_vertical_lr() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000 vertical:lr
Test
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(
    cues[0].header_ref().settings().unwrap().vertical(),
    Some(Vertical::Lr)
  );
}

#[test]
fn parse_no_settings_means_none() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000
Test
";

  let cues = collect_cues(vtt).unwrap();
  assert!(cues[0].header_ref().settings().is_none());
}

// ── NOTE blocks ─────────────────────────────────────────────────────────────

#[test]
fn parse_note_block() {
  let vtt = "\
WEBVTT

NOTE
This is a comment

00:00:01.000 --> 00:00:04.000
Hello
";

  let blocks = collect(vtt).unwrap();
  assert_eq!(blocks.len(), 2);
  match &blocks[0] {
    Block::Note(text) => assert_eq!(*text, "This is a comment"),
    _ => panic!("expected Note block"),
  }
  match &blocks[1] {
    Block::Cue(cue) => assert_eq!(*cue.body_ref(), "Hello"),
    _ => panic!("expected Cue block"),
  }
}

#[test]
fn parse_note_inline() {
  let vtt = "\
WEBVTT

NOTE This is an inline comment

00:00:01.000 --> 00:00:04.000
Hello
";

  let blocks = collect(vtt).unwrap();
  assert_eq!(blocks.len(), 2);
  match &blocks[0] {
    Block::Note(text) => assert_eq!(*text, "This is an inline comment"),
    _ => panic!("expected Note block"),
  }
}

#[test]
fn parse_note_multiline() {
  let vtt = "\
WEBVTT

NOTE
Line 1
Line 2
Line 3

00:00:01.000 --> 00:00:04.000
Hello
";

  let blocks = collect(vtt).unwrap();
  match &blocks[0] {
    Block::Note(text) => assert_eq!(*text, "Line 1\nLine 2\nLine 3"),
    _ => panic!("expected Note block"),
  }
}

#[test]
fn parse_note_empty() {
  let vtt = "\
WEBVTT

NOTE

00:00:01.000 --> 00:00:04.000
Hello
";

  let blocks = collect(vtt).unwrap();
  assert_eq!(blocks.len(), 2);
  match &blocks[0] {
    Block::Note(text) => assert_eq!(*text, ""),
    _ => panic!("expected Note block"),
  }
}

// ── STYLE blocks ────────────────────────────────────────────────────────────

#[test]
fn parse_style_block() {
  let vtt = "\
WEBVTT

STYLE
::cue {
  background-image: linear-gradient(to bottom, dimgray, lightgray);
  color: papayawhip;
}

00:00:01.000 --> 00:00:04.000
Styled
";

  let blocks = collect(vtt).unwrap();
  assert_eq!(blocks.len(), 2);
  match &blocks[0] {
    Block::Style(text) => {
      assert!(text.contains("::cue"));
      assert!(text.contains("papayawhip"));
    }
    _ => panic!("expected Style block"),
  }
}

#[test]
fn style_after_cue_ignored() {
  // STYLE blocks after a cue should not be parsed as STYLE
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000
First

STYLE
::cue { color: red; }

00:00:05.000 --> 00:00:08.000
Second
";

  let blocks = collect(vtt).unwrap();
  // The STYLE line after a cue is not a valid STYLE block.
  // Per spec, it would be treated as something else (possibly a cue identifier).
  let cues: Vec<_> = blocks
    .iter()
    .filter(|b| matches!(b, Block::Cue(_)))
    .collect();
  assert_eq!(cues.len(), 2);
  // No style blocks should be present
  let styles: Vec<_> = blocks
    .iter()
    .filter(|b| matches!(b, Block::Style(_)))
    .collect();
  assert!(styles.is_empty());
}

// ── REGION blocks ───────────────────────────────────────────────────────────

#[test]
fn parse_region_block() {
  let vtt = "\
WEBVTT

REGION
id:fred
width:40%
lines:3

00:00:01.000 --> 00:00:04.000
Hello
";

  let blocks = collect(vtt).unwrap();
  assert_eq!(blocks.len(), 2);
  match &blocks[0] {
    Block::Region(text) => {
      assert!(text.contains("id:fred"));
      assert!(text.contains("width:40%"));
    }
    _ => panic!("expected Region block"),
  }
}

#[test]
fn region_after_cue_ignored() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000
First

REGION
id:test

00:00:05.000 --> 00:00:08.000
Second
";

  let blocks = collect(vtt).unwrap();
  let regions: Vec<_> = blocks
    .iter()
    .filter(|b| matches!(b, Block::Region(_)))
    .collect();
  assert!(regions.is_empty());
}

// ── Edge cases ──────────────────────────────────────────────────────────────

#[test]
fn parse_multiple_blank_lines_between() {
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:02.000
First


00:00:03.000 --> 00:00:04.000
Second
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 2);
}

#[test]
fn parse_header_text_lines_ignored() {
  let vtt = "\
WEBVTT
This is header text that should be ignored
Another header line

00:00:01.000 --> 00:00:04.000
Cue text
";

  let cues = collect_cues(vtt).unwrap();
  assert_eq!(cues.len(), 1);
  assert_eq!(*cues[0].body_ref(), "Cue text");
}

#[test]
fn parse_note_after_cue_ok() {
  // NOTE blocks can appear anywhere (before or after cues)
  let vtt = "\
WEBVTT

00:00:01.000 --> 00:00:04.000
First

NOTE This is after a cue

00:00:05.000 --> 00:00:08.000
Second
";

  let blocks = collect(vtt).unwrap();
  assert_eq!(blocks.len(), 3);
  assert!(matches!(&blocks[0], Block::Cue(_)));
  assert!(matches!(&blocks[1], Block::Note(_)));
  assert!(matches!(&blocks[2], Block::Cue(_)));
}

#[test]
fn timestamp_encode_uses_dot() {
  let ts = Timestamp::from_hmsm(
    Hour::with(1),
    Minute::with(23),
    Second::with(45),
    Millisecond::with(678),
  );
  assert_eq!(ts.encode().as_str(), "01:23:45.678");
}

#[test]
fn timestamp_encode_omits_zero_hours() {
  let ts = Timestamp::from_hmsm(
    Hour::new(),
    Minute::with(1),
    Second::with(30),
    Millisecond::with(500),
  );
  assert_eq!(ts.encode().as_str(), "01:30.500");
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

// ── Writer tests (std only) ─────────────────────────────────────────────────

#[cfg(feature = "std")]
mod writer {
  use fasrt::{
    types::*,
    vtt::{
      Align, Block, Cue, CueId, CueOptions, Header, Hour, Parser, Percentage, Size, Timestamp,
      Writer,
    },
  };

  fn ts(s: u8, ms: u16) -> Timestamp {
    Timestamp::from_hmsm(
      Hour::new(),
      Minute::with(0),
      Second::with(s),
      Millisecond::with(ms),
    )
  }

  fn simple_cue(start_s: u8, end_s: u8, body: &str) -> Block<'static, String> {
    let header = Header::new(ts(start_s, 0), ts(end_s, 0));
    Block::Cue(Cue::new(header, body.to_string()))
  }

  fn write_to_string(f: impl FnOnce(&mut Writer<&mut Vec<u8>>)) -> String {
    let mut buf = Vec::new();
    let mut w = Writer::new(&mut buf);
    f(&mut w);
    String::from_utf8(buf).unwrap()
  }

  #[test]
  fn write_single_cue() {
    let out = write_to_string(|w| {
      w.write(&simple_cue(1, 4, "Hello world!")).unwrap();
    });
    assert_eq!(out, "WEBVTT\n\n00:01.000 --> 00:04.000\nHello world!\n");
  }

  #[test]
  fn write_multiple_cues() {
    let blocks = vec![simple_cue(1, 4, "First"), simple_cue(5, 8, "Second")];
    let out = write_to_string(|w| {
      w.write_all(&blocks).unwrap();
    });
    assert_eq!(
      out,
      "WEBVTT\n\n00:01.000 --> 00:04.000\nFirst\n\n00:05.000 --> 00:08.000\nSecond\n"
    );
  }

  #[test]
  fn write_cue_with_identifier() {
    let out = write_to_string(|w| {
      let header = Header::new(ts(1, 0), ts(4, 0)).with_identifier(CueId::new("intro"));
      let block = Block::Cue(Cue::new(header, "Hello!".to_string()));
      w.write(&block).unwrap();
    });
    assert_eq!(out, "WEBVTT\n\nintro\n00:01.000 --> 00:04.000\nHello!\n");
  }

  #[test]
  fn write_cue_with_settings() {
    let out = write_to_string(|w| {
      let header = Header::new(ts(1, 0), ts(4, 0)).with_settings(
        CueOptions::default()
          .with_align(Align::Center)
          .with_size(Size::new(Percentage::with(80))),
      );
      let block = Block::Cue(Cue::new(header, "Styled".to_string()));
      w.write(&block).unwrap();
    });
    assert!(out.contains("size:80%"));
    assert!(out.contains("align:center"));
  }

  #[test]
  fn write_note_block() {
    let out = write_to_string(|w| {
      let block: Block<'static, String> = Block::Note("This is a comment".to_string());
      w.write(&block).unwrap();
    });
    assert_eq!(out, "WEBVTT\n\nNOTE\nThis is a comment\n");
  }

  #[test]
  fn write_style_block() {
    let out = write_to_string(|w| {
      let block: Block<'static, String> = Block::Style("::cue { color: red; }".to_string());
      w.write(&block).unwrap();
    });
    assert_eq!(out, "WEBVTT\n\nSTYLE\n::cue { color: red; }\n");
  }

  #[test]
  fn write_region_block() {
    let out = write_to_string(|w| {
      let block: Block<'static, String> = Block::Region("id:fred\nwidth:40%".to_string());
      w.write(&block).unwrap();
    });
    assert_eq!(out, "WEBVTT\n\nREGION\nid:fred\nwidth:40%\n");
  }

  #[test]
  fn write_custom_header() {
    let out = write_to_string(|w| {
      w.write_header(Some("My Video Subtitles")).unwrap();
      w.write(&simple_cue(1, 4, "Hello")).unwrap();
    });
    assert!(out.starts_with("WEBVTT My Video Subtitles\n"));
  }

  #[test]
  fn write_empty() {
    let out = write_to_string(|_w| {
      // Write nothing
    });
    assert_eq!(out, "");
  }

  #[test]
  fn write_into_inner() {
    let buf = Vec::new();
    let mut w = Writer::new(buf);
    w.write(&simple_cue(1, 2, "test")).unwrap();
    let inner = w.into_inner();
    assert!(!inner.is_empty());
  }

  #[test]
  fn write_roundtrip() {
    let original = "\
WEBVTT

00:01.000 --> 00:04.000
Hello world!

00:05.000 --> 00:08.000
Goodbye world!
";

    let blocks: Vec<Block<'_, &str>> = Parser::new(original).collect::<Result<_, _>>().unwrap();

    let mut buf = Vec::new();
    let mut w = Writer::new(&mut buf);
    w.write_header(None).unwrap();
    for block in &blocks {
      w.write(block).unwrap();
    }
    let written = String::from_utf8(buf).unwrap();

    assert_eq!(written, original);
  }
}
