use std::num::NonZeroU64;
use std::vec::Vec;

use logos::{Lexer, Logos};

use crate::{
  error::*,
  types::{SrtEntry, SrtHeader, SrtTimestamp},
};

/// The error type for parsing SRT files.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseSrtError {
  /// An error occurred while parsing the hour component of a timestamp.
  #[error(transparent)]
  ParseMinute(#[from] ParseMinuteError),
  /// An error occurred while parsing the minute component of a timestamp.
  #[error(transparent)]
  ParseSecond(#[from] ParseSecondError),
  /// An error occurred while parsing the second component of a timestamp.
  #[error(transparent)]
  ParseHour(#[from] ParseHourError),
  /// An error occurred while parsing the millisecond component of a timestamp.
  #[error(transparent)]
  ParseMillisecond(#[from] ParseMillisecondError),
  /// An error occurred while parsing the index number of a subtitle.
  #[error(transparent)]
  ParseIndex(#[from] ParseIndexNumberError),

  /// Expected an end timestamp after the arrow token "-->".
  #[error("unclosed duration, missing end timestamp")]
  UnclosedDuration,

  /// Unopened duration, missing start timestamp before the arrow token "-->".
  #[error("unopened duration, missing start timestamp")]
  UnopenedDuration,

  /// Expected a header (timeline) line after the index.
  #[error("expected header line (e.g. '00:00:01,000 --> 00:00:04,000') after index {0}")]
  ExpectedHeader(NonZeroU64),

  /// An unknown lexer error occurred.
  #[error("unexpected token: {0}")]
  Unknown(&'static str),
}

impl Default for ParseSrtError {
  fn default() -> Self {
    Self::Unknown("unknown lexer error")
  }
}

/// Token produced by the lexer.
#[derive(Debug, Logos, PartialEq)]
#[logos(
  error = ParseSrtError,
  extras = Option<Self>,
)]
enum Token {
  /// Header "HH:MM:SS,mmm --> HH:MM:SS,mmm"
  #[regex(
    r"[0-9]{1,3}:[0-5][0-9]:[0-5][0-9],[0-9]{3} --> [0-9]{1,3}:[0-5][0-9]:[0-5][0-9],[0-9]{3}",
    parse_header
  )]
  Header(SrtHeader),

  /// A number (subtitle index).
  #[regex(r"[0-9]+", parse_number, priority = 3)]
  Number(NonZeroU64),
}

fn parse_number(s: &mut Lexer<'_, Token>) -> Result<NonZeroU64, ParseSrtError> {
  let slice = s.slice().trim();
  if slice.len() > 20 {
    // Longer than max u64 value, so definitely an overflow.
    return Err(ParseIndexNumberError::Overflow.into());
  }

  if slice == "0" {
    // Zero is not a valid index number.
    return Err(ParseIndexNumberError::Zero.into());
  }

  slice
    .parse::<u64>()
    .map_err(|e| ParseIndexNumberError::ParseInt(e).into())
    .and_then(|num| NonZeroU64::new(num).ok_or(ParseIndexNumberError::Zero.into()))
}

fn parse_header(s: &mut Lexer<'_, Token>) -> Result<SrtHeader, ParseSrtError> {
  let s = s.slice().trim();
  let mut parts = s.split(" --> ");
  match (parts.next(), parts.next()) {
    (Some(start), Some(end)) => {
      let start = parse_timestamp(start)?;
      let end = parse_timestamp(end)?;
      Ok(SrtHeader::new(start, end))
    }
    _ => panic!("logos regex should guarantee this never happens"),
  }
}

fn parse_timestamp(s: &str) -> Result<SrtTimestamp, ParseSrtError> {
  let mut parts = s.split(",");

  match (parts.next(), parts.next()) {
    (Some(hms), Some(millis)) => {
      let mut hms_parts = hms.split(':');

      let (h, m, s) = match (hms_parts.next(), hms_parts.next(), hms_parts.next()) {
        (Some(h), Some(m), Some(s)) => (h.parse()?, m.parse()?, s.parse()?),
        _ => panic!("logos regex should guarantee this never happens"),
      };
      let millis = millis.parse()?;
      Ok(SrtTimestamp::from_hmsm(h, m, s, millis))
    }
    _ => panic!("logos regex should guarantee this never happens"),
  }
}

/// State machine states for the SRT parser.
enum State {
  /// Expecting a subtitle index number (or blank lines / BOM to skip).
  Index,
  /// Got an index, expecting the header (timeline) line.
  Header { index: NonZeroU64 },
  /// Got the header, collecting body text lines.
  Body {
    header: SrtHeader,
    body_start: usize,
    body_end: usize,
  },
}

/// Parse an SRT string into a list of [`SrtEntry`]s with borrowed text bodies.
///
/// The parser uses a line-by-line state machine. Index numbers and timeline
/// headers are parsed quickly through logos; text lines are collected as
/// zero-copy `&str` slices from the input.
///
/// # Errors
///
/// Returns a [`ParseSrtError`] if the input is malformed.
///
/// # Example
///
/// ```
/// use fasrt::srt::parse;
///
/// let srt = "\
/// 1
/// 00:00:01,000 --> 00:00:04,000
/// Hello world!
///
/// 2
/// 00:00:05,000 --> 00:00:08,000
/// Goodbye world!
/// ";
///
/// let subs = parse(srt).unwrap();
/// assert_eq!(subs.len(), 2);
/// assert_eq!(*subs[0].body(), "Hello world!");
/// assert_eq!(*subs[1].body(), "Goodbye world!");
/// ```
pub fn parse(input: &str) -> Result<Vec<SrtEntry<&str>>, ParseSrtError> {
  let mut entries = Vec::new();
  let mut state = State::Index;

  for line in Lines::new(input) {
    let trimmed = line.trim_start_matches('\u{feff}');

    match state {
      State::Index => {
        // Skip blank lines between entries
        if trimmed.is_empty() {
          continue;
        }

        // Parse the index number using logos
        let index = lex_index(trimmed)?;
        state = State::Header { index };
      }
      State::Header { index } => {
        // Parse the header/timeline using logos
        let mut header = lex_header(trimmed, index)?;
        header.set_index(index);
        state = State::Body {
          header,
          body_start: line.as_ptr() as usize - input.as_ptr() as usize + line.len(),
          body_end: line.as_ptr() as usize - input.as_ptr() as usize + line.len(),
        };
      }
      State::Body {
        ref header,
        ref mut body_start,
        ref mut body_end,
      } => {
        if trimmed.is_empty() {
          // Blank line → finalize this entry
          let body = body_slice(input, *body_start, *body_end);
          entries.push(make_entry(header, body));
          state = State::Index;
        } else {
          let line_offset = line.as_ptr() as usize - input.as_ptr() as usize;
          if *body_start == *body_end {
            // First text line
            *body_start = line_offset;
          }
          *body_end = line_offset + line.len();
        }
      }
    }
  }

  // Handle the last entry if the file doesn't end with a blank line
  if let State::Body {
    header,
    body_start,
    body_end,
  } = state
  {
    let body = body_slice(input, body_start, body_end);
    entries.push(make_entry(&header, body));
  }

  Ok(entries)
}

/// Extract the body slice from the input, or empty string if no text lines.
#[cfg_attr(not(tarpaulin), inline(always))]
fn body_slice(input: &str, start: usize, end: usize) -> &str {
  if start >= end { "" } else { &input[start..end] }
}

#[cfg_attr(not(tarpaulin), inline(always))]
fn make_entry<'a>(header: &SrtHeader, body: &'a str) -> SrtEntry<&'a str> {
  SrtEntry::new(header.clone(), body)
}

/// Use the logos lexer to parse a line as a subtitle index number.
fn lex_index(line: &str) -> Result<NonZeroU64, ParseSrtError> {
  let mut lexer = Token::lexer(line);
  match lexer.next() {
    Some(Ok(Token::Number(n))) => Ok(n),
    Some(Err(e)) => Err(e),
    _ => Err(ParseSrtError::Unknown("expected subtitle index number")),
  }
}

/// Use the logos lexer to parse a line as a header (timeline).
fn lex_header(line: &str, index: NonZeroU64) -> Result<SrtHeader, ParseSrtError> {
  let mut lexer = Token::lexer(line);
  match lexer.next() {
    Some(Ok(Token::Header(header))) => Ok(header),
    Some(Err(e)) => Err(e),
    _ => Err(ParseSrtError::ExpectedHeader(index)),
  }
}

/// A line iterator that yields lines without the line terminator,
/// but preserves the ability to compute offsets into the original input.
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

    let remaining = &self.input[self.pos..];
    let line_end = remaining
      .find('\n')
      .map(|i| {
        // Handle \r\n
        let end = if i > 0 && remaining.as_bytes()[i - 1] == b'\r' {
          i - 1
        } else {
          i
        };
        (end, i + 1) // (line content end, next line start)
      })
      .unwrap_or((remaining.len(), remaining.len()));

    let line = &self.input[self.pos..self.pos + line_end.0];
    self.pos += line_end.1;
    Some(line)
  }
}
