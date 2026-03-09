use logos::{Lexer, Logos};

use crate::error::*;

pub use types::{
  Align, Block, Cue, CueId, CueSettings, Header, Hour, Line, LineAlign, LineValue, Position,
  PositionAlign, RegionId, Size, Timestamp, Vertical,
};

mod types;

/// The error type for parsing WebVTT files.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseVttError {
  /// An error occurred while parsing the hour component of a timestamp.
  #[error(transparent)]
  ParseMinute(#[from] ParseMinuteError),
  /// An error occurred while parsing the second component of a timestamp.
  #[error(transparent)]
  ParseSecond(#[from] ParseSecondError),
  /// An error occurred while parsing the hour component of a timestamp.
  #[error(transparent)]
  ParseHour(#[from] ParseHourError),
  /// An error occurred while parsing the millisecond component of a timestamp.
  #[error(transparent)]
  ParseMillisecond(#[from] ParseMillisecondError),

  /// The file does not start with the required `WEBVTT` signature.
  #[error("missing WEBVTT signature")]
  MissingSignature,

  /// The character after `WEBVTT` is not a space, tab, or newline.
  #[error("invalid character after WEBVTT signature")]
  InvalidSignature,

  /// A timestamp could not be parsed.
  #[error("invalid timestamp: {0}")]
  InvalidTimestamp(&'static str),

  /// The timing line is malformed (missing `-->`).
  #[error("invalid timing line: missing '-->' separator")]
  InvalidTimingLine,

  /// The end timestamp is missing after `-->`.
  #[error("unclosed duration, missing end timestamp")]
  UnclosedDuration,

  /// An unknown error occurred.
  #[error("unexpected token: {0}")]
  Unknown(&'static str),
}

impl Default for ParseVttError {
  fn default() -> Self {
    Self::Unknown("unknown lexer error")
  }
}

/// Token produced by the WebVTT lexer.
#[derive(Debug, Logos, PartialEq)]
#[logos(error = ParseVttError)]
enum Token {
  /// A WebVTT timing line with optional hours:
  /// `[HH:]MM:SS.mmm --> [HH:]MM:SS.mmm`
  ///
  /// This matches the timing portion only; cue settings that follow
  /// on the same line are parsed separately from the remainder.
  ///
  /// Hours accept 1+ digits per the W3C spec.
  /// Whitespace around `-->` includes space, tab, and form feed (U+000C).
  #[regex(
    r"[0-9]+:[0-5][0-9]:[0-5][0-9]\.[0-9]{3}[ \t\x0C]+-->[ \t\x0C]+[0-9]+:[0-5][0-9]:[0-5][0-9]\.[0-9]{3}",
    parse_timing_long_long,
    priority = 4,
  )]
  #[regex(
    r"[0-5][0-9]:[0-5][0-9]\.[0-9]{3}[ \t\x0C]+-->[ \t\x0C]+[0-9]+:[0-5][0-9]:[0-5][0-9]\.[0-9]{3}",
    parse_timing_short_long,
    priority = 3
  )]
  #[regex(
    r"[0-9]+:[0-5][0-9]:[0-5][0-9]\.[0-9]{3}[ \t\x0C]+-->[ \t\x0C]+[0-5][0-9]:[0-5][0-9]\.[0-9]{3}",
    parse_timing_long_short,
    priority = 3
  )]
  #[regex(
    r"[0-5][0-9]:[0-5][0-9]\.[0-9]{3}[ \t\x0C]+-->[ \t\x0C]+[0-5][0-9]:[0-5][0-9]\.[0-9]{3}",
    parse_timing_short_short,
    priority = 2
  )]
  TimingLine((Timestamp, Timestamp)),
}

fn parse_timing_long_long(
  lex: &mut Lexer<'_, Token>,
) -> Result<(Timestamp, Timestamp), ParseVttError> {
  let s = lex.slice();
  let (start_str, end_str) = split_arrow(s)?;
  let start = parse_timestamp_long(start_str)?;
  let end = parse_timestamp_long(end_str)?;
  Ok((start, end))
}

fn parse_timing_short_long(
  lex: &mut Lexer<'_, Token>,
) -> Result<(Timestamp, Timestamp), ParseVttError> {
  let s = lex.slice();
  let (start_str, end_str) = split_arrow(s)?;
  let start = parse_timestamp_short(start_str)?;
  let end = parse_timestamp_long(end_str)?;
  Ok((start, end))
}

fn parse_timing_long_short(
  lex: &mut Lexer<'_, Token>,
) -> Result<(Timestamp, Timestamp), ParseVttError> {
  let s = lex.slice();
  let (start_str, end_str) = split_arrow(s)?;
  let start = parse_timestamp_long(start_str)?;
  let end = parse_timestamp_short(end_str)?;
  Ok((start, end))
}

fn parse_timing_short_short(
  lex: &mut Lexer<'_, Token>,
) -> Result<(Timestamp, Timestamp), ParseVttError> {
  let s = lex.slice();
  let (start_str, end_str) = split_arrow(s)?;
  let start = parse_timestamp_short(start_str)?;
  let end = parse_timestamp_short(end_str)?;
  Ok((start, end))
}

/// Split a timing line on `-->`, trimming whitespace around each part.
fn split_arrow(s: &str) -> Result<(&str, &str), ParseVttError> {
  let arrow = s.find("-->").ok_or(ParseVttError::InvalidTimingLine)?;
  let start = s[..arrow].trim();
  let end = s[arrow + 3..].trim();
  if start.is_empty() || end.is_empty() {
    return Err(ParseVttError::UnclosedDuration);
  }
  Ok((start, end))
}

/// Parse a long-form timestamp `HH:MM:SS.mmm` (hours required).
fn parse_timestamp_long(s: &str) -> Result<Timestamp, ParseVttError> {
  let dot = s.rfind('.').unwrap();
  let millis: crate::types::Millisecond = s[dot + 1..].parse()?;
  let hms = &s[..dot];
  let mut parts = hms.splitn(3, ':');
  let h: u64 = parts
    .next()
    .unwrap()
    .parse()
    .map_err(|_| ParseVttError::InvalidTimestamp("hours value too large"))?;
  let m: crate::types::Minute = parts.next().unwrap().parse()?;
  let sec: crate::types::Second = parts.next().unwrap().parse()?;
  Ok(Timestamp::from_hmsm(Hour::with(h), m, sec, millis))
}

/// Parse a short-form timestamp `MM:SS.mmm` (hours omitted, default to 0).
fn parse_timestamp_short(s: &str) -> Result<Timestamp, ParseVttError> {
  let dot = s.rfind('.').unwrap();
  let millis: crate::types::Millisecond = s[dot + 1..].parse()?;
  let ms = &s[..dot];
  let mut parts = ms.splitn(2, ':');
  let m: crate::types::Minute = parts.next().unwrap().parse()?;
  let sec: crate::types::Second = parts.next().unwrap().parse()?;
  Ok(Timestamp::from_hmsm(Hour::new(), m, sec, millis))
}

/// Lex the next token from a line. Returns `Ok(None)` when the line
/// produces no tokens.
fn lex(line: &str) -> Result<Option<(Token, usize)>, ParseVttError> {
  let mut lexer = Token::lexer(line);
  match lexer.next() {
    Some(result) => result.map(|t| Some((t, lexer.span().end))),
    None => Ok(None),
  }
}

/// Returns `true` if the line looks like a WebVTT timing line (contains `-->`).
#[cfg_attr(not(tarpaulin), inline(always))]
fn is_timing_line(line: &str) -> bool {
  line.contains("-->")
}

/// Strip BOM and leading VTT whitespace (space, tab, form feed).
#[cfg_attr(not(tarpaulin), inline(always))]
fn strip_leading(line: &str) -> &str {
  line
    .trim_start_matches('\u{feff}')
    .trim_start_matches([' ', '\t', '\x0c'])
}

/// Parse cue settings from the remainder of a timing line.
fn parse_cue_settings(s: &str) -> CueSettings {
  let mut settings = CueSettings::default();
  if s.is_empty() {
    return settings;
  }

  for token in s.split([' ', '\t', '\x0c']) {
    if token.is_empty() {
      continue;
    }
    let Some((key, value)) = token.split_once(':') else {
      continue;
    };
    if value.is_empty() {
      // Empty value stops settings parsing per the spec
      break;
    }
    match key {
      "vertical" => match value {
        "rl" => {
          settings.set_vertical(Vertical::Rl);
        }
        "lr" => {
          settings.set_vertical(Vertical::Lr);
        }
        _ => {
          continue;
        }
      },
      "line" => {
        // value can be `N%`, `N`, optionally followed by `,start|center|end`
        if let Some((val_str, align_str)) = value.split_once(',') {
          let alignment = match align_str {
            "start" => LineAlign::Start,
            "center" => LineAlign::Center,
            "end" => LineAlign::End,
            _ => {
              continue;
            }
          };
          if let Some(v) = parse_line_value(val_str) {
            settings.set_line(Line::with_alignment(v, alignment));
          }
        } else if let Some(v) = parse_line_value(value) {
          settings.set_line(Line::new(v));
        }
      }
      "position" => {
        if let Some((val_str, align_str)) = value.split_once(',') {
          let alignment = match align_str {
            "line-left" => PositionAlign::LineLeft,
            "center" => PositionAlign::Center,
            "line-right" => PositionAlign::LineRight,
            "auto" => PositionAlign::Auto,
            _ => {
              continue;
            }
          };
          if let Some(pct) = parse_percentage(val_str) {
            settings.set_position(Position::with_alignment(pct, alignment));
          }
        } else if let Some(pct) = parse_percentage(value) {
          settings.set_position(Position::new(pct));
        }
      }
      "size" => {
        if let Some(pct) = parse_percentage(value) {
          settings.set_size(Size::new(pct));
        }
      }
      "align" => match value {
        "start" => {
          settings.set_align(Align::Start);
        }
        "center" => {
          settings.set_align(Align::Center);
        }
        "end" => {
          settings.set_align(Align::End);
        }
        "left" => {
          settings.set_align(Align::Left);
        }
        "right" => {
          settings.set_align(Align::Right);
        }
        _ => {
          continue;
        }
      },
      "region" => {
        #[cfg(any(feature = "alloc", feature = "std"))]
        {
          settings.set_region(RegionId::from_string(value.into()));
        }
        #[cfg(not(any(feature = "alloc", feature = "std")))]
        {
          // Without alloc, we can't store region IDs from parsed input
          let _ = value;
        }
      }
      _ => {
        // Unknown settings are ignored per the spec
      }
    }
  }

  settings
}

/// Parse a percentage value like "50%". Returns the number part.
fn parse_percentage(s: &str) -> Option<u8> {
  let s = s.strip_suffix('%')?;
  let n: u8 = s.parse().ok()?;
  if n > 100 { None } else { Some(n) }
}

/// Parse a line value: either `N%` (percentage) or `N` (line number, possibly negative).
fn parse_line_value(s: &str) -> Option<LineValue> {
  if let Some(pct) = parse_percentage(s) {
    Some(LineValue::Percentage(pct))
  } else {
    s.parse::<i32>().ok().map(LineValue::Number)
  }
}

/// Format cue settings to a string for writing.
#[cfg(feature = "std")]
fn format_cue_settings(settings: &CueSettings, buf: &mut std::vec::Vec<u8>) {
  use std::io::Write as _;

  if let Some(v) = settings.vertical() {
    let _ = write!(buf, " vertical:{v}");
  }
  if let Some(line) = settings.line() {
    match line.value() {
      LineValue::Percentage(p) => {
        let _ = write!(buf, " line:{p}%");
      }
      LineValue::Number(n) => {
        let _ = write!(buf, " line:{n}");
      }
    }
    if let Some(align) = line.alignment() {
      let _ = write!(buf, ",{align}");
    }
  }
  if let Some(pos) = settings.position() {
    let _ = write!(buf, " position:{}%", pos.value());
    if let Some(align) = pos.alignment() {
      let _ = write!(buf, ",{align}");
    }
  }
  if let Some(size) = settings.size() {
    let _ = write!(buf, " size:{}%", size.value());
  }
  if let Some(align) = settings.align() {
    let _ = write!(buf, " align:{align}");
  }
  if let Some(region) = settings.region() {
    let _ = write!(buf, " region:{region}");
  }
}

/// State machine states for the WebVTT parser.
enum State {
  /// Expecting the WEBVTT signature line.
  Signature,
  /// After signature, collecting optional header text until first blank line.
  Header,
  /// Expecting a block start (cue, NOTE, STYLE, REGION) or blank lines.
  BlockStart,
  /// Collecting cue body text.
  CueBody(CueBodyState),
  /// Collecting NOTE body text.
  NoteBody(usize, usize),
  /// Collecting STYLE body text.
  StyleBody(usize, usize),
  /// Collecting REGION body text.
  RegionBody(usize, usize),
  /// The iterator is exhausted or encountered an error.
  Done,
}

struct CueBodyState {
  header: Header,
  start: usize,
  end: usize,
}

impl CueBodyState {
  #[cfg_attr(not(tarpaulin), inline(always))]
  const fn new(header: Header, start: usize, end: usize) -> Self {
    Self { header, start, end }
  }
}

/// A lazy, zero-copy WebVTT parser that yields blocks as an iterator.
///
/// Created via [`Parser::new`]. Each call to [`Iterator::next`] yields the
/// next parsed [`Block`], or an error if the input is malformed.
///
/// The parser follows the W3C WebVTT specification strictly.
///
/// # Example
///
/// ```
/// # #[cfg(not(any(feature = "std", feature = "alloc")))]
/// # fn main() {}
/// # #[cfg(any(feature = "std", feature = "alloc"))]
/// # fn main() {
/// # use std::vec::Vec;
///
/// use fasrt::vtt::{Parser, Block};
///
/// let vtt = "\
/// WEBVTT
///
/// 00:01.000 --> 00:04.000
/// Hello world!
///
/// 00:05.000 --> 00:08.000
/// Goodbye world!
/// ";
///
/// let blocks: Vec<_> = Parser::new(vtt).collect::<Result<_, _>>().unwrap();
/// assert_eq!(blocks.len(), 2);
/// match &blocks[0] {
///   Block::Cue(cue) => assert_eq!(*cue.body_ref(), "Hello world!"),
///   _ => panic!("expected cue"),
/// }
/// # }
/// ```
pub struct Parser<'a> {
  input: &'a str,
  lines: Lines<'a>,
  state: State,
  /// Whether we've seen the first cue. STYLE/REGION blocks are only
  /// allowed before the first cue per the spec.
  seen_cue: bool,
  /// A line to be reprocessed in the next iteration (used for error
  /// recovery when `-->` appears in cue body or a timing line fails).
  pending_line: Option<&'a str>,
}

impl<'a> Parser<'a> {
  /// Create a new WebVTT parser for the given input.
  pub fn new(input: &'a str) -> Self {
    Self {
      input,
      lines: Lines::new(input),
      state: State::Signature,
      seen_cue: false,
      pending_line: None,
    }
  }

  /// Get the next line, using pending_line if available.
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn next_line(&mut self) -> Option<&'a str> {
    self.pending_line.take().or_else(|| self.lines.next())
  }

  /// Skip lines until a blank line or a line containing `-->`.
  /// If a `-->` line is found, it is stored as pending for reprocessing.
  fn skip_to_block_boundary(&mut self) {
    loop {
      let Some(line) = self.lines.next() else {
        return;
      };
      if line.is_empty() {
        return;
      }
      if is_timing_line(line) {
        self.pending_line = Some(line);
        return;
      }
    }
  }
}

/// Try to lex a timing line. On success, returns the parsed header and
/// the byte offset within `self.input` where the body starts.
fn try_parse_timing(line: &str, input: &str) -> Option<(Header, usize)> {
  let stripped = strip_leading(line);
  if !is_timing_line(stripped) {
    return None;
  }

  match lex(stripped) {
    Ok(Some((Token::TimingLine((start, end)), matched_end))) => {
      let mut header = Header::new(start, end);
      let settings_str = stripped[matched_end..].trim();
      let settings = parse_cue_settings(settings_str);
      if settings != CueSettings::default() {
        header.set_settings(settings);
      }
      let offset = line.as_ptr() as usize - input.as_ptr() as usize + line.len();
      Some((header, offset))
    }
    _ => None,
  }
}

impl<'a> Iterator for Parser<'a> {
  type Item = Result<Block<&'a str>, ParseVttError>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.state {
        State::Done => return None,

        State::Signature => {
          let Some(line) = self.lines.next() else {
            self.state = State::Done;
            return Some(Err(ParseVttError::MissingSignature));
          };

          // Strip optional BOM
          let line = line.trim_start_matches('\u{feff}');

          // Must start with "WEBVTT"
          if !line.starts_with("WEBVTT") {
            self.state = State::Done;
            return Some(Err(ParseVttError::MissingSignature));
          }

          // After "WEBVTT", the next character (if any) must be space, tab, or nothing.
          let rest = &line[6..];
          if !rest.is_empty() && !rest.starts_with(' ') && !rest.starts_with('\t') {
            self.state = State::Done;
            return Some(Err(ParseVttError::InvalidSignature));
          }

          self.state = State::Header;
        }

        State::Header => {
          // Skip header text until we hit a blank line or a --> line
          let Some(line) = self.lines.next() else {
            self.state = State::Done;
            return None;
          };

          if line.is_empty() {
            self.state = State::BlockStart;
          } else if is_timing_line(line) {
            // Per the JS ref parser: if header contains -->, jump to block processing
            self.pending_line = Some(line);
            self.state = State::BlockStart;
          }
          // Otherwise skip header text lines
        }

        State::BlockStart => {
          let Some(line) = self.next_line() else {
            self.state = State::Done;
            return None;
          };

          let trimmed = line.trim_start_matches('\u{feff}');
          if trimmed.is_empty() {
            continue;
          }

          // Check for NOTE block
          if trimmed == "NOTE" || trimmed.starts_with("NOTE ") || trimmed.starts_with("NOTE\t") {
            let after_note = if trimmed == "NOTE" {
              ""
            } else {
              trimmed[5..].trim_start()
            };

            if after_note.is_empty() {
              let offset = line.as_ptr() as usize - self.input.as_ptr() as usize + line.len();
              self.state = State::NoteBody(offset, offset);
            } else {
              let body_start = after_note.as_ptr() as usize - self.input.as_ptr() as usize;
              let body_end = body_start + after_note.len();
              self.state = State::NoteBody(body_start, body_end);
            }
            continue;
          }

          // Check for STYLE block (only before first cue)
          if (trimmed == "STYLE" || trimmed.starts_with("STYLE ") || trimmed.starts_with("STYLE\t"))
            && !self.seen_cue
          {
            let offset = line.as_ptr() as usize - self.input.as_ptr() as usize + line.len();
            self.state = State::StyleBody(offset, offset);
            continue;
          }

          // Check for REGION block (only before first cue)
          if (trimmed == "REGION"
            || trimmed.starts_with("REGION ")
            || trimmed.starts_with("REGION\t"))
            && !self.seen_cue
          {
            let offset = line.as_ptr() as usize - self.input.as_ptr() as usize + line.len();
            self.state = State::RegionBody(offset, offset);
            continue;
          }

          // Check if this line is a timing line (cue without identifier)
          if is_timing_line(trimmed) {
            if let Some((header, offset)) = try_parse_timing(line, self.input) {
              self.state = State::CueBody(CueBodyState::new(header, offset, offset));
              self.seen_cue = true;
            } else {
              // Timing parse failed — skip to next block boundary
              self.skip_to_block_boundary();
            }
            continue;
          }

          // This line could be a cue identifier. The next line should be a timing line.
          let identifier_line = trimmed;
          let Some(next_line) = self.lines.next() else {
            self.state = State::Done;
            return None;
          };

          if is_timing_line(next_line) {
            if let Some((mut header, offset)) = try_parse_timing(next_line, self.input) {
              #[cfg(any(feature = "alloc", feature = "std"))]
              {
                header.set_identifier(CueId::from_string(identifier_line.into()));
              }
              #[cfg(not(any(feature = "alloc", feature = "std")))]
              {
                let _ = identifier_line;
              }

              self.state = State::CueBody(CueBodyState::new(header, offset, offset));
              self.seen_cue = true;
            } else {
              // Timing parse failed after identifier — skip to next block boundary
              self.skip_to_block_boundary();
            }
          } else if next_line.is_empty() {
            // Identifier line followed by blank — not a valid cue, skip
            continue;
          } else {
            // Non-timing, non-blank line after identifier.
            // Per the JS ref parser, this line is reprocessed as a new block start.
            self.pending_line = Some(next_line);
            continue;
          }
        }

        State::CueBody(ref mut body) => {
          let CueBodyState { header, start, end } = body;
          let Some(line) = self.lines.next() else {
            let body_text = body_slice(self.input, *start, *end);
            let entry = Cue::new(header.clone(), body_text);
            self.state = State::Done;
            return Some(Ok(Block::Cue(entry)));
          };

          if line.is_empty() {
            let body_text = body_slice(self.input, *start, *end);
            let entry = Cue::new(header.clone(), body_text);
            self.state = State::BlockStart;
            return Some(Ok(Block::Cue(entry)));
          }

          // Per the W3C spec, if a body line contains `-->`, the cue ends
          // and the line is reprocessed as a potential timing line.
          if is_timing_line(line) {
            let body_text = body_slice(self.input, *start, *end);
            let entry = Cue::new(header.clone(), body_text);
            self.pending_line = Some(line);
            self.state = State::BlockStart;
            return Some(Ok(Block::Cue(entry)));
          }

          let line_offset = line.as_ptr() as usize - self.input.as_ptr() as usize;
          if *start == *end {
            *start = line_offset;
          }
          *end = line_offset + line.len();
        }

        State::NoteBody(ref mut start, ref mut end) => {
          let Some(line) = self.lines.next() else {
            let body_text = body_slice(self.input, *start, *end);
            self.state = State::Done;
            return Some(Ok(Block::Note(body_text)));
          };

          if line.is_empty() {
            let body_text = body_slice(self.input, *start, *end);
            self.state = State::BlockStart;
            return Some(Ok(Block::Note(body_text)));
          }

          let line_offset = line.as_ptr() as usize - self.input.as_ptr() as usize;
          if *start == *end {
            *start = line_offset;
          }
          *end = line_offset + line.len();
        }

        State::StyleBody(ref mut start, ref mut end) => {
          let Some(line) = self.lines.next() else {
            let body_text = body_slice(self.input, *start, *end);
            self.state = State::Done;
            return Some(Ok(Block::Style(body_text)));
          };

          if line.is_empty() {
            let body_text = body_slice(self.input, *start, *end);
            self.state = State::BlockStart;
            return Some(Ok(Block::Style(body_text)));
          }

          let line_offset = line.as_ptr() as usize - self.input.as_ptr() as usize;
          if *start == *end {
            *start = line_offset;
          }
          *end = line_offset + line.len();
        }

        State::RegionBody(ref mut start, ref mut end) => {
          let Some(line) = self.lines.next() else {
            let body_text = body_slice(self.input, *start, *end);
            self.state = State::Done;
            return Some(Ok(Block::Region(body_text)));
          };

          if line.is_empty() {
            let body_text = body_slice(self.input, *start, *end);
            self.state = State::BlockStart;
            return Some(Ok(Block::Region(body_text)));
          }

          let line_offset = line.as_ptr() as usize - self.input.as_ptr() as usize;
          if *start == *end {
            *start = line_offset;
          }
          *end = line_offset + line.len();
        }
      }
    }
  }
}

/// Extract a body slice from the input, or empty string if no text lines.
#[cfg_attr(not(tarpaulin), inline(always))]
fn body_slice(input: &str, start: usize, end: usize) -> &str {
  if start >= end { "" } else { &input[start..end] }
}

/// A WebVTT file writer that writes blocks to a [`std::io::Write`] target.
///
/// # Example
///
/// ```
/// use fasrt::vtt::{Writer, Cue, Header, Timestamp, Block, Hour};
/// use fasrt::types::*;
///
/// let mut buf = Vec::new();
/// let mut writer = Writer::new(&mut buf);
///
/// let header = Header::new(
///   Timestamp::from_hmsm(Hour::new(), Minute::with(0), Second::with(1), Millisecond::with(0)),
///   Timestamp::from_hmsm(Hour::new(), Minute::with(0), Second::with(4), Millisecond::with(0)),
/// );
///
/// writer.write(&Block::Cue(Cue::new(header, "Hello world!"))).unwrap();
///
/// let output = String::from_utf8(buf).unwrap();
/// assert!(output.starts_with("WEBVTT\n"));
/// ```
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct Writer<W> {
  inner: W,
  has_written_signature: bool,
  has_written_block: bool,
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
const _: () = {
  use std::io::{self, Write};

  impl<W: Write> Writer<W> {
    /// Create a new writer wrapping the given [`std::io::Write`] target.
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub const fn new(inner: W) -> Self {
      Self {
        inner,
        has_written_signature: false,
        has_written_block: false,
      }
    }

    /// Write the WEBVTT signature with optional header text.
    ///
    /// This is called automatically on the first `write` call if not
    /// called explicitly.
    pub fn write_header(&mut self, header_text: Option<&str>) -> io::Result<()> {
      if self.has_written_signature {
        return Ok(());
      }
      self.has_written_signature = true;
      self.inner.write_all(b"WEBVTT")?;
      if let Some(text) = header_text {
        if !text.is_empty() {
          self.inner.write_all(b" ")?;
          self.inner.write_all(text.as_bytes())?;
        }
      }
      self.inner.write_all(b"\n")
    }

    /// Write a single WebVTT block.
    ///
    /// A blank line separator is emitted between blocks. The `WEBVTT`
    /// signature is automatically emitted before the first block.
    pub fn write<T: AsRef<str>>(&mut self, block: &Block<T>) -> io::Result<()> {
      if !self.has_written_signature {
        self.write_header(None)?;
      }

      // Blank line before each block (separator after signature / between blocks)
      self.inner.write_all(b"\n")?;

      match block {
        Block::Cue(cue) => {
          let header = cue.header_ref();

          // Optional cue identifier
          if let Some(id) = header.identifier() {
            self.inner.write_all(id.as_str().as_bytes())?;
            self.inner.write_all(b"\n")?;
          }

          // Timing line
          self
            .inner
            .write_all(header.start().encode().as_str().as_bytes())?;
          self.inner.write_all(b" --> ")?;
          self
            .inner
            .write_all(header.end().encode().as_str().as_bytes())?;

          // Optional cue settings
          if let Some(settings) = header.settings() {
            let mut settings_buf = std::vec::Vec::new();
            format_cue_settings(settings, &mut settings_buf);
            self.inner.write_all(&settings_buf)?;
          }

          self.inner.write_all(b"\n")?;

          // Body
          let body = cue.body_ref().as_ref();
          if !body.is_empty() {
            self.inner.write_all(body.as_bytes())?;
            self.inner.write_all(b"\n")?;
          }
        }
        Block::Note(text) => {
          let text = text.as_ref();
          if text.is_empty() {
            self.inner.write_all(b"NOTE\n")?;
          } else {
            self.inner.write_all(b"NOTE\n")?;
            self.inner.write_all(text.as_bytes())?;
            self.inner.write_all(b"\n")?;
          }
        }
        Block::Style(text) => {
          self.inner.write_all(b"STYLE\n")?;
          let text = text.as_ref();
          if !text.is_empty() {
            self.inner.write_all(text.as_bytes())?;
            self.inner.write_all(b"\n")?;
          }
        }
        Block::Region(text) => {
          self.inner.write_all(b"REGION\n")?;
          let text = text.as_ref();
          if !text.is_empty() {
            self.inner.write_all(text.as_bytes())?;
            self.inner.write_all(b"\n")?;
          }
        }
      }

      self.has_written_block = true;
      Ok(())
    }

    /// Write all blocks from an iterator.
    pub fn write_all<'a, T, I>(&mut self, blocks: I) -> io::Result<()>
    where
      T: AsRef<str> + 'a,
      I: IntoIterator<Item = &'a Block<T>>,
    {
      for block in blocks {
        self.write(block)?;
      }
      Ok(())
    }

    /// Flush the underlying writer.
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn flush(&mut self) -> io::Result<()> {
      self.inner.flush()
    }

    /// Consume the writer and return the inner [`std::io::Write`] target.
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn into_inner(self) -> W {
      self.inner
    }
  }
};

/// A line iterator that yields lines without the line terminator,
/// but preserves the ability to compute offsets into the original input.
///
/// Handles all three line terminator styles: `\n` (LF), `\r\n` (CRLF),
/// and standalone `\r` (CR).
struct Lines<'a> {
  input: &'a str,
  pos: usize,
}

impl<'a> Lines<'a> {
  fn new(input: &'a str) -> Self {
    Self { input, pos: 0 }
  }
}

impl<'a> Iterator for Lines<'a> {
  type Item = &'a str;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pos >= self.input.len() {
      return None;
    }

    let bytes = &self.input.as_bytes()[self.pos..];
    let mut i = 0;
    while i < bytes.len() {
      if bytes[i] == b'\n' {
        let line = &self.input[self.pos..self.pos + i];
        self.pos += i + 1;
        return Some(line);
      } else if bytes[i] == b'\r' {
        let line = &self.input[self.pos..self.pos + i];
        // Check for CRLF
        if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
          self.pos += i + 2;
        } else {
          self.pos += i + 1;
        }
        return Some(line);
      }
      i += 1;
    }

    // No line terminator found - rest of input
    let line = &self.input[self.pos..];
    self.pos = self.input.len();
    Some(line)
  }
}
