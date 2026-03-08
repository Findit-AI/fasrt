use std::num::NonZeroU64;

use logos::{Lexer, Logos};

use crate::{
  error::*,
  types::{SrtHeader, SrtTimestamp},
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

fn parse_number<'a>(s: &mut Lexer<'a, Token>) -> Result<NonZeroU64, ParseSrtError> {
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

fn parse_header<'a>(s: &mut Lexer<'a, Token>) -> Result<SrtHeader, ParseSrtError> {
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

fn parse_timestamp<'a>(s: &str) -> Result<SrtTimestamp, ParseSrtError> {
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

enum State {
  Init,
}
