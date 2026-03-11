use core::{str::FromStr, time::Duration};

use derive_more::{Display, From, Into, IsVariant, TryUnwrap, Unwrap};

use crate::{
  error::ParseHourError,
  types::{Entry as GenericEntry, *},
  utils::u64_digits,
};

/// A single cue entry in a WebVTT file.
pub type Cue<'a, T> = GenericEntry<Header<'a>, T>;

/// The hour component of a WebVTT timestamp.
///
/// Per the W3C spec, WebVTT hours have no upper limit ("one or more digits").
/// This wraps a `u64` with no maximum constraint.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into)]
#[repr(transparent)]
pub struct Hour(pub(crate) u64);

impl FromStr for Hour {
  type Err = ParseHourError;

  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(match s {
      "0" => 0,
      "1" => 1,
      "2" => 2,
      "3" => 3,
      "4" => 4,
      "5" => 5,
      "6" => 6,
      "7" => 7,
      "8" => 8,
      "9" => 9,
      "00" => 0,
      "01" => 1,
      "02" => 2,
      "03" => 3,
      "04" => 4,
      "05" => 5,
      "06" => 6,
      "07" => 7,
      "08" => 8,
      "09" => 9,
      _ => return s.parse::<u64>().map(Self).map_err(Into::into),
    }))
  }
}

impl Hour {
  /// Create a new `Hour` with value 0.
  ///
  /// ```rust
  /// use fasrt::vtt::Hour;
  ///
  /// let hour = Hour::new();
  /// assert_eq!(hour.as_u64(), 0);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new() -> Self {
    Self(0)
  }

  /// Create a new `Hour` from a `u64`.
  ///
  /// ```rust
  /// use fasrt::vtt::Hour;
  ///
  /// let hour = Hour::with(12345);
  /// assert_eq!(hour.as_u64(), 12345);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with(value: u64) -> Self {
    Self(value)
  }

  /// Returns the inner `u64` value.
  ///
  /// ```rust
  /// use fasrt::vtt::Hour;
  ///
  /// let hour = Hour::with(42);
  /// assert_eq!(hour.as_u64(), 42);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_u64(&self) -> u64 {
    self.0
  }
}

impl core::fmt::Display for Hour {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if self.0 < 10 {
      write!(f, "0{}", self.0)
    } else {
      write!(f, "{}", self.0)
    }
  }
}

/// A WebVTT timestamp with millisecond precision.
///
/// Per the W3C spec, WebVTT timestamps use a dot `.` to separate seconds
/// from milliseconds (e.g. `01:30.000`), and hours are optional with no
/// upper limit.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display("{}", self.encode().as_str())]
pub struct Timestamp {
  /// Hours (unbounded per the W3C spec).
  hours: Hour,
  /// Milliseconds (0–999).
  millis: Millisecond,
  /// Minutes (0–59).
  minutes: Minute,
  /// Seconds (0–59).
  seconds: Second,
}

impl From<Duration> for Timestamp {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from(value: Duration) -> Self {
    Timestamp::from_duration(value)
  }
}

impl Default for Timestamp {
  /// ```rust
  /// use fasrt::vtt::{Timestamp, Hour};
  ///
  /// let timestamp = Timestamp::default();
  /// assert_eq!(timestamp.hours(), Hour::new());
  /// assert_eq!(u8::from(timestamp.minutes()), 0u8);
  /// assert_eq!(u8::from(timestamp.seconds()), 0u8);
  /// assert_eq!(u16::from(timestamp.millis()), 0u16);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn default() -> Self {
    Self::new()
  }
}

impl Timestamp {
  /// Create a new timestamp with all components set to zero.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new() -> Self {
    Self::from_hmsm(Hour::new(), Minute(0), Second(0), Millisecond(0))
  }

  /// Create a new timestamp from hours, minutes, seconds, and milliseconds.
  ///
  /// The hours component has no upper limit per the W3C WebVTT spec.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn from_hmsm(
    hours: Hour,
    minutes: Minute,
    seconds: Second,
    millis: Millisecond,
  ) -> Self {
    Self {
      hours,
      minutes,
      seconds,
      millis,
    }
  }

  /// Returns the hours component of this timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn hours(&self) -> Hour {
    self.hours
  }

  /// Returns the minutes component of this timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn minutes(&self) -> Minute {
    self.minutes
  }

  /// Returns the seconds component of this timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn seconds(&self) -> Second {
    self.seconds
  }

  /// Returns the milliseconds component of this timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn millis(&self) -> Millisecond {
    self.millis
  }

  /// Build a new timestamp with the hours field set to the given value.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_hours(mut self, hours: Hour) -> Self {
    self.set_hours(hours);
    self
  }

  /// Build a new timestamp with the minutes field set to the given value.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_minutes(mut self, minutes: Minute) -> Self {
    self.set_minutes(minutes);
    self
  }

  /// Build a new timestamp with the seconds field set to the given value.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_seconds(mut self, seconds: Second) -> Self {
    self.set_seconds(seconds);
    self
  }

  /// Build a new timestamp with the millis field set to the given value.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_millis(mut self, millis: Millisecond) -> Self {
    self.set_millis(millis);
    self
  }

  /// Set the hours field of this timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_hours(&mut self, hours: Hour) -> &mut Self {
    self.hours = hours;
    self
  }

  /// Set the minutes field of this timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_minutes(&mut self, minutes: Minute) -> &mut Self {
    self.minutes = minutes;
    self
  }

  /// Set the seconds field of this timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_seconds(&mut self, seconds: Second) -> &mut Self {
    self.seconds = seconds;
    self
  }

  /// Set the millis field of this timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_millis(&mut self, millis: Millisecond) -> &mut Self {
    self.millis = millis;
    self
  }

  /// Set all fields of this timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_hmsm(
    &mut self,
    hours: Hour,
    minutes: Minute,
    seconds: Second,
    millis: Millisecond,
  ) -> &mut Self {
    self.hours = hours;
    self.minutes = minutes;
    self.seconds = seconds;
    self.millis = millis;
    self
  }

  /// Convert this timestamp to a `Duration`.
  ///
  /// ```rust
  /// use core::time::Duration;
  /// use fasrt::vtt::{Timestamp, Hour};
  /// use fasrt::types::{Minute, Second, Millisecond};
  ///
  /// let timestamp = Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4));
  /// let duration = timestamp.to_duration();
  /// assert_eq!(duration, Duration::from_millis(1 * 3_600_000 + 2 * 60_000 + 3 * 1_000 + 4));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn to_duration(&self) -> Duration {
    let minutes = self.minutes.0 as u64;
    let seconds = self.seconds.0 as u64;
    let millis = self.millis.0 as u64;

    Duration::from_millis(
      self.hours().as_u64() * 3_600_000 + minutes * 60_000 + seconds * 1_000 + millis,
    )
  }

  /// Creates a timestamp from a [`Duration`].
  ///
  /// ```rust
  /// use core::time::Duration;
  /// use fasrt::vtt::{Timestamp, Hour};
  /// use fasrt::types::{Minute, Second, Millisecond};
  ///
  /// let dur = Duration::from_millis(1 * 3_600_000 + 2 * 60_000 + 3 * 1_000 + 4);
  /// let ts = Timestamp::from(dur);
  /// assert_eq!(ts.hours(), Hour::with(1));
  /// assert_eq!(ts.minutes(), Minute::with(2));
  /// assert_eq!(ts.seconds(), Second::with(3));
  /// assert_eq!(ts.millis(), Millisecond::with(4));
  ///
  /// // Round-trip
  /// assert_eq!(ts.to_duration(), dur);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn from_duration(dur: Duration) -> Self {
    let total_millis = dur.as_millis() as u64;
    let hours = total_millis / 3_600_000;
    let minutes = ((total_millis % 3_600_000) / 60_000) as u8;
    let seconds = ((total_millis % 60_000) / 1_000) as u8;
    let millis = (total_millis % 1_000) as u16;

    Self {
      hours: Hour::with(hours),
      minutes: Minute::with(minutes),
      seconds: Second::with(seconds),
      millis: Millisecond::with(millis),
    }
  }

  /// Returns the encoded length of this timestamp.
  ///
  /// When hours are 0, the short form `MM:SS.mmm` is used (no hours prefix).
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn encoded_len(&self) -> usize {
    let base = self.minutes().as_str().len() + 1 // MM:
      + self.seconds().as_str().len() + 1 // SS.
      + self.millis().as_str().len(); // mmm
    if self.hours.0 == 0 {
      base
    } else {
      // Hours: at least 2 digits, zero-padded
      let digits = u64_digits(self.hours.as_u64());
      let hours_len = if digits < 2 { 2 } else { digits };
      hours_len + 1 + base // HH+:
    }
  }

  /// Format this timestamp to a WebVTT timestamp string (uses `.` separator).
  ///
  /// When hours are 0, the short form `MM:SS.mmm` is emitted per the spec.
  /// When hours are non-zero, the long form `HH:MM:SS.mmm` is used with
  /// at least 2 digits for the hours component.
  ///
  /// ```rust
  /// use fasrt::vtt::{Timestamp, Hour};
  /// use fasrt::types::{Minute, Second, Millisecond};
  ///
  /// let ts = Timestamp::from_hmsm(Hour::with(0), Minute::with(2), Second::with(3), Millisecond::with(4));
  /// assert_eq!(ts.encode().as_str(), "02:03.004");
  ///
  /// let ts = Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4));
  /// assert_eq!(ts.encode().as_str(), "01:02:03.004");
  ///
  /// let ts = Timestamp::from_hmsm(Hour::with(12345), Minute::with(0), Second::with(0), Millisecond::with(0));
  /// assert_eq!(ts.encode().as_str(), "12345:00:00.000");
  ///
  /// let ts = Timestamp::from_hmsm(Hour::with(18446744073709551615), Minute::with(0), Second::with(0), Millisecond::with(0));
  /// assert_eq!(ts.encode().as_str(), "18446744073709551615:00:00.000");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn encode(&self) -> Buffer<31> {
    let mut buffer = Buffer::new();
    if self.hours.0 != 0 {
      // Pad to at least 2 digits
      if self.hours.0 < 10 {
        buffer.write_str("0");
      }
      buffer.fmt_u64(self.hours.0);
      buffer.write_str(":");
    }
    buffer.write_str(self.minutes().as_str());
    buffer.write_str(":");
    buffer.write_str(self.seconds().as_str());
    buffer.write_str(".");
    buffer.write_str(self.millis().as_str());
    buffer
  }
}

/// The header of a WebVTT cue, containing optional identifier, timestamps,
/// and optional cue settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
  identifier: Option<CueId<'a>>,
  start: Timestamp,
  end: Timestamp,
  settings: Option<CueOptions<'a>>,
}

/// A cue identifier. Per the W3C spec, this is any text that does not
/// contain "-->", and is not empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{_0}")]
#[repr(transparent)]
pub struct CueId<'a>(&'a str);

impl<'a> CueId<'a> {
  /// Create a `CueId` from a string slice.
  ///
  /// ```rust
  /// use fasrt::vtt::CueId;
  ///
  /// let id = CueId::new("intro");
  /// assert_eq!(id.as_str(), "intro");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(s: &'a str) -> Self {
    Self(s)
  }

  /// Returns the string representation of this cue identifier.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_str(&self) -> &'a str {
    self.0
  }
}

/// Cue settings per the W3C WebVTT specification.
///
/// Each setting is optional. The spec defines these settings:
/// `vertical`, `line`, `position`, `size`, `align`, `region`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CueOptions<'a> {
  vertical: Option<Vertical>,
  line: Option<Line>,
  position: Option<Position>,
  size: Option<Size>,
  align: Option<Align>,
  region: Option<RegionId<'a>>,
}

impl<'a> CueOptions<'a> {
  /// Returns the writing direction setting.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Vertical};
  ///
  /// let s = CueOptions::default();
  /// assert_eq!(s.vertical(), None);
  ///
  /// let s = CueOptions::default().with_vertical(Vertical::Rl);
  /// assert_eq!(s.vertical(), Some(Vertical::Rl));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn vertical(&self) -> Option<Vertical> {
    self.vertical
  }

  /// Sets the writing direction (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Vertical};
  ///
  /// let s = CueOptions::default().with_vertical(Vertical::Lr);
  /// assert_eq!(s.vertical(), Some(Vertical::Lr));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_vertical(mut self, vertical: Vertical) -> Self {
    self.vertical = Some(vertical);
    self
  }

  /// Sets the writing direction.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Vertical};
  ///
  /// let mut s = CueOptions::default();
  /// s.set_vertical(Vertical::Rl);
  /// assert_eq!(s.vertical(), Some(Vertical::Rl));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_vertical(&mut self, vertical: Vertical) -> &mut Self {
    self.vertical = Some(vertical);
    self
  }

  /// Sets the writing direction from an `Option` (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Vertical};
  ///
  /// let s = CueOptions::default().maybe_vertical(Some(Vertical::Lr));
  /// assert_eq!(s.vertical(), Some(Vertical::Lr));
  ///
  /// let s = CueOptions::default().maybe_vertical(None);
  /// assert_eq!(s.vertical(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn maybe_vertical(mut self, vertical: Option<Vertical>) -> Self {
    self.vertical = vertical;
    self
  }

  /// Sets the writing direction from an `Option`.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Vertical};
  ///
  /// let mut s = CueOptions::default();
  /// s.update_vertical(Some(Vertical::Rl));
  /// assert_eq!(s.vertical(), Some(Vertical::Rl));
  /// s.update_vertical(None);
  /// assert_eq!(s.vertical(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn update_vertical(&mut self, vertical: Option<Vertical>) -> &mut Self {
    self.vertical = vertical;
    self
  }

  /// Returns the line position setting.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Line, LineValue, Percentage};
  ///
  /// let s = CueOptions::default();
  /// assert_eq!(s.line(), None);
  ///
  /// let s = CueOptions::default().with_line(Line::new(LineValue::Number(-1)));
  /// assert_eq!(s.line().unwrap().value(), LineValue::Number(-1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn line(&self) -> Option<&Line> {
    self.line.as_ref()
  }

  /// Sets the line position (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Line, LineValue, Percentage};
  ///
  /// let s = CueOptions::default().with_line(Line::new(LineValue::Percentage(Percentage::with(50.0))));
  /// assert_eq!(s.line().unwrap().value(), LineValue::Percentage(Percentage::with(50.0)));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_line(mut self, line: Line) -> Self {
    self.line = Some(line);
    self
  }

  /// Sets the line position.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Line, LineValue, Percentage};
  ///
  /// let mut s = CueOptions::default();
  /// s.set_line(Line::new(LineValue::Number(3)));
  /// assert_eq!(s.line().unwrap().value(), LineValue::Number(3));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_line(&mut self, line: Line) -> &mut Self {
    self.line = Some(line);
    self
  }

  /// Sets the line position from an `Option` (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Line, LineValue, Percentage};
  ///
  /// let s = CueOptions::default().maybe_line(Some(Line::new(LineValue::Number(5))));
  /// assert_eq!(s.line().unwrap().value(), LineValue::Number(5));
  ///
  /// let s = CueOptions::default().maybe_line(None);
  /// assert_eq!(s.line(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn maybe_line(mut self, line: Option<Line>) -> Self {
    self.line = line;
    self
  }

  /// Sets the line position from an `Option`.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Line, LineValue, Percentage};
  ///
  /// let mut s = CueOptions::default();
  /// s.update_line(Some(Line::new(LineValue::Percentage(Percentage::with(25.0)))));
  /// assert!(s.line().is_some());
  /// s.update_line(None);
  /// assert_eq!(s.line(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn update_line(&mut self, line: Option<Line>) -> &mut Self {
    self.line = line;
    self
  }

  /// Returns the text position setting.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Percentage, Position};
  ///
  /// let s = CueOptions::default();
  /// assert_eq!(s.position(), None);
  ///
  /// let s = CueOptions::default().with_position(Position::new(Percentage::with(50.0)));
  /// assert_eq!(s.position().unwrap().value(), Percentage::with(50.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn position(&self) -> Option<&Position> {
    self.position.as_ref()
  }

  /// Sets the text position (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Percentage, Position, PositionAlign};
  ///
  /// let s = CueOptions::default()
  ///   .with_position(Position::with_alignment(Percentage::with(25.0), PositionAlign::LineLeft));
  /// let pos = s.position().unwrap();
  /// assert_eq!(pos.value(), Percentage::with(25.0));
  /// assert_eq!(pos.alignment(), Some(PositionAlign::LineLeft));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_position(mut self, position: Position) -> Self {
    self.position = Some(position);
    self
  }

  /// Sets the text position.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Percentage, Position};
  ///
  /// let mut s = CueOptions::default();
  /// s.set_position(Position::new(Percentage::with(75.0)));
  /// assert_eq!(s.position().unwrap().value(), Percentage::with(75.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_position(&mut self, position: Position) -> &mut Self {
    self.position = Some(position);
    self
  }

  /// Sets the text position from an `Option` (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Percentage, Position};
  ///
  /// let s = CueOptions::default().maybe_position(Some(Position::new(Percentage::with(60.0))));
  /// assert_eq!(s.position().unwrap().value(), Percentage::with(60.0));
  ///
  /// let s = CueOptions::default().maybe_position(None);
  /// assert_eq!(s.position(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn maybe_position(mut self, position: Option<Position>) -> Self {
    self.position = position;
    self
  }

  /// Sets the text position from an `Option`.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Percentage, Position};
  ///
  /// let mut s = CueOptions::default();
  /// s.update_position(Some(Position::new(Percentage::with(40.0))));
  /// assert!(s.position().is_some());
  /// s.update_position(None);
  /// assert_eq!(s.position(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn update_position(&mut self, position: Option<Position>) -> &mut Self {
    self.position = position;
    self
  }

  /// Returns the cue size setting.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Percentage, Size};
  ///
  /// let s = CueOptions::default();
  /// assert_eq!(s.size(), None);
  ///
  /// let s = CueOptions::default().with_size(Size::new(Percentage::with(80.0)));
  /// assert_eq!(s.size().unwrap().value(), Percentage::with(80.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn size(&self) -> Option<Size> {
    self.size
  }

  /// Sets the cue size (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Percentage, Size};
  ///
  /// let s = CueOptions::default().with_size(Size::new(Percentage::with(100.0)));
  /// assert_eq!(s.size().unwrap().value(), Percentage::with(100.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_size(mut self, size: Size) -> Self {
    self.size = Some(size);
    self
  }

  /// Sets the cue size.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Percentage, Size};
  ///
  /// let mut s = CueOptions::default();
  /// s.set_size(Size::new(Percentage::with(50.0)));
  /// assert_eq!(s.size().unwrap().value(), Percentage::with(50.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_size(&mut self, size: Size) -> &mut Self {
    self.size = Some(size);
    self
  }

  /// Sets the cue size from an `Option` (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Percentage, Size};
  ///
  /// let s = CueOptions::default().maybe_size(Some(Size::new(Percentage::with(60.0))));
  /// assert_eq!(s.size().unwrap().value(), Percentage::with(60.0));
  ///
  /// let s = CueOptions::default().maybe_size(None);
  /// assert_eq!(s.size(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn maybe_size(mut self, size: Option<Size>) -> Self {
    self.size = size;
    self
  }

  /// Sets the cue size from an `Option`.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Percentage, Size};
  ///
  /// let mut s = CueOptions::default();
  /// s.update_size(Some(Size::new(Percentage::with(30.0))));
  /// assert!(s.size().is_some());
  /// s.update_size(None);
  /// assert_eq!(s.size(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn update_size(&mut self, size: Option<Size>) -> &mut Self {
    self.size = size;
    self
  }

  /// Returns the text alignment setting.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Align};
  ///
  /// let s = CueOptions::default();
  /// assert_eq!(s.align(), None);
  ///
  /// let s = CueOptions::default().with_align(Align::Center);
  /// assert_eq!(s.align(), Some(Align::Center));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn align(&self) -> Option<Align> {
    self.align
  }

  /// Sets the text alignment (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Align};
  ///
  /// let s = CueOptions::default().with_align(Align::Start);
  /// assert_eq!(s.align(), Some(Align::Start));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_align(mut self, align: Align) -> Self {
    self.align = Some(align);
    self
  }

  /// Sets the text alignment.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Align};
  ///
  /// let mut s = CueOptions::default();
  /// s.set_align(Align::End);
  /// assert_eq!(s.align(), Some(Align::End));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_align(&mut self, align: Align) -> &mut Self {
    self.align = Some(align);
    self
  }

  /// Sets the text alignment from an `Option` (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Align};
  ///
  /// let s = CueOptions::default().maybe_align(Some(Align::Left));
  /// assert_eq!(s.align(), Some(Align::Left));
  ///
  /// let s = CueOptions::default().maybe_align(None);
  /// assert_eq!(s.align(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn maybe_align(mut self, align: Option<Align>) -> Self {
    self.align = align;
    self
  }

  /// Sets the text alignment from an `Option`.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, Align};
  ///
  /// let mut s = CueOptions::default();
  /// s.update_align(Some(Align::Right));
  /// assert_eq!(s.align(), Some(Align::Right));
  /// s.update_align(None);
  /// assert_eq!(s.align(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn update_align(&mut self, align: Option<Align>) -> &mut Self {
    self.align = align;
    self
  }

  /// Returns the region identifier setting.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, RegionId};
  ///
  /// let s = CueOptions::default();
  /// assert!(s.region().is_none());
  ///
  /// let s = CueOptions::default().with_region(RegionId::new("r1"));
  /// assert_eq!(s.region().unwrap().as_str(), "r1");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn region(&self) -> Option<&RegionId<'a>> {
    self.region.as_ref()
  }

  /// Sets the region identifier (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, RegionId};
  ///
  /// let s = CueOptions::default().with_region(RegionId::new("header"));
  /// assert_eq!(s.region().unwrap().as_str(), "header");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_region(mut self, region: RegionId<'a>) -> Self {
    self.region = Some(region);
    self
  }

  /// Sets the region identifier.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, RegionId};
  ///
  /// let mut s = CueOptions::default();
  /// s.set_region(RegionId::new("footer"));
  /// assert_eq!(s.region().unwrap().as_str(), "footer");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_region(&mut self, region: RegionId<'a>) -> &mut Self {
    self.region = Some(region);
    self
  }

  /// Sets the region identifier from an `Option` (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, RegionId};
  ///
  /// let s = CueOptions::default().maybe_region(Some(RegionId::new("r1")));
  /// assert_eq!(s.region().unwrap().as_str(), "r1");
  ///
  /// let s = CueOptions::default().maybe_region(None);
  /// assert!(s.region().is_none());
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn maybe_region(mut self, region: Option<RegionId<'a>>) -> Self {
    self.region = region;
    self
  }

  /// Sets the region identifier from an `Option`.
  ///
  /// ```rust
  /// use fasrt::vtt::{CueOptions, RegionId};
  ///
  /// let mut s = CueOptions::default();
  /// s.update_region(Some(RegionId::new("nav")));
  /// assert!(s.region().is_some());
  /// s.update_region(None);
  /// assert!(s.region().is_none());
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn update_region(&mut self, region: Option<RegionId<'a>>) -> &mut Self {
    self.region = region;
    self
  }
}

/// Writing direction for a cue.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, IsVariant)]
pub enum Vertical {
  /// Right-to-left (`vertical:rl`).
  #[display("rl")]
  Rl,
  /// Left-to-right (`vertical:lr`).
  #[display("lr")]
  Lr,
}

/// A percentage value (0.0–100.0).
///
/// This is a newtype wrapper around `f64` that guarantees the value is
/// within the valid range for WebVTT percentage settings.
/// Per the W3C spec, percentages may include decimals (e.g., `50.5%`).
///
/// ```rust
/// use fasrt::vtt::Percentage;
///
/// let pct = Percentage::with(50.0);
/// assert_eq!(pct.value(), 50.0);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Into)]
#[repr(transparent)]
pub struct Percentage(f64);

impl Eq for Percentage {}

impl core::hash::Hash for Percentage {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.0.to_bits().hash(state);
  }
}

impl PartialOrd for Percentage {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Percentage {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.0.total_cmp(&other.0)
  }
}

impl core::fmt::Display for Percentage {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if self.0 == (self.0 as i64) as f64 {
      write!(f, "{}", self.0 as i64)
    } else {
      write!(f, "{}", self.0)
    }
  }
}

impl Percentage {
  /// Create a new `Percentage` with value 0.
  ///
  /// ```rust
  /// use fasrt::vtt::Percentage;
  ///
  /// let pct = Percentage::new();
  /// assert_eq!(pct.value(), 0.0);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new() -> Self {
    Self(0.0)
  }

  /// Create a new `Percentage` from an `f64`.
  ///
  /// # Panics
  /// Panics if the value is not in the range 0.0..=100.0.
  ///
  /// ```rust
  /// use fasrt::vtt::Percentage;
  ///
  /// let pct = Percentage::with(100.0);
  /// assert_eq!(pct.value(), 100.0);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with(value: f64) -> Self {
    assert!(
      (0.0..=100.0).contains(&value),
      "Percentage value must be between 0.0 and 100.0"
    );
    Self(value)
  }

  /// Try to create a new `Percentage` from an `f64`, returning `None` if the value is out of range.
  ///
  /// ```rust
  /// use fasrt::vtt::Percentage;
  ///
  /// assert_eq!(Percentage::try_with(50.0), Some(Percentage::with(50.0)));
  /// assert_eq!(Percentage::try_with(101.0), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn try_with(value: f64) -> Option<Self> {
    if (0.0..=100.0).contains(&value) {
      Some(Self(value))
    } else {
      None
    }
  }

  /// Returns the inner `f64` value.
  ///
  /// ```rust
  /// use fasrt::vtt::Percentage;
  ///
  /// let pct = Percentage::with(75.0);
  /// assert_eq!(pct.value(), 75.0);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn value(&self) -> f64 {
    self.0
  }
}

/// Line position value.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, IsVariant, Unwrap, TryUnwrap)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum LineValue {
  /// A percentage value (0–100).
  #[display("{_0}%")]
  Percentage(Percentage),
  /// A line number (can be negative).
  #[display("{_0}")]
  Number(i32),
}

/// Line alignment.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, IsVariant)]
pub enum LineAlign {
  /// `start`
  #[display("start")]
  Start,
  /// `center`
  #[display("center")]
  Center,
  /// `end`
  #[display("end")]
  End,
}

/// Line setting: value and optional alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line {
  value: LineValue,
  alignment: Option<LineAlign>,
}

impl Line {
  /// Create a new `Line` with the given value and no alignment.
  ///
  /// ```rust
  /// use fasrt::vtt::{Line, LineValue, Percentage};
  ///
  /// let line = Line::new(LineValue::Percentage(Percentage::with(50.0)));
  /// assert_eq!(line.value(), LineValue::Percentage(Percentage::with(50.0)));
  /// assert_eq!(line.alignment(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(value: LineValue) -> Self {
    Self {
      value,
      alignment: None,
    }
  }

  /// Create a new `Line` with the given value and alignment.
  ///
  /// ```rust
  /// use fasrt::vtt::{Line, LineAlign, LineValue, Percentage};
  ///
  /// let line = Line::with_alignment(LineValue::Number(-1), LineAlign::Start);
  /// assert_eq!(line.value(), LineValue::Number(-1));
  /// assert_eq!(line.alignment(), Some(LineAlign::Start));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_alignment(value: LineValue, alignment: LineAlign) -> Self {
    Self {
      value,
      alignment: Some(alignment),
    }
  }

  /// Returns the line value.
  ///
  /// ```rust
  /// use fasrt::vtt::{Line, LineValue, Percentage};
  ///
  /// let line = Line::new(LineValue::Number(3));
  /// assert_eq!(line.value(), LineValue::Number(3));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn value(&self) -> LineValue {
    self.value
  }

  /// Sets the line value.
  ///
  /// ```rust
  /// use fasrt::vtt::{Line, LineValue, Percentage};
  ///
  /// let mut line = Line::new(LineValue::Number(1));
  /// line.set_value(LineValue::Percentage(Percentage::with(80.0)));
  /// assert_eq!(line.value(), LineValue::Percentage(Percentage::with(80.0)));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_value(&mut self, value: LineValue) -> &mut Self {
    self.value = value;
    self
  }

  /// Returns the line alignment.
  ///
  /// ```rust
  /// use fasrt::vtt::{Line, LineAlign, LineValue, Percentage};
  ///
  /// let line = Line::new(LineValue::Number(0));
  /// assert_eq!(line.alignment(), None);
  ///
  /// let line = Line::with_alignment(LineValue::Number(0), LineAlign::Center);
  /// assert_eq!(line.alignment(), Some(LineAlign::Center));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn alignment(&self) -> Option<LineAlign> {
    self.alignment
  }

  /// Sets the line alignment.
  ///
  /// ```rust
  /// use fasrt::vtt::{Line, LineAlign, LineValue, Percentage};
  ///
  /// let mut line = Line::new(LineValue::Percentage(Percentage::with(50.0)));
  /// line.set_alignment(LineAlign::End);
  /// assert_eq!(line.alignment(), Some(LineAlign::End));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_alignment(&mut self, alignment: LineAlign) -> &mut Self {
    self.alignment = Some(alignment);
    self
  }

  /// Sets the line alignment from an `Option` (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{Line, LineAlign, LineValue, Percentage};
  ///
  /// let line = Line::new(LineValue::Number(0)).maybe_alignment(Some(LineAlign::Start));
  /// assert_eq!(line.alignment(), Some(LineAlign::Start));
  ///
  /// let line = Line::new(LineValue::Number(0)).maybe_alignment(None);
  /// assert_eq!(line.alignment(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn maybe_alignment(mut self, alignment: Option<LineAlign>) -> Self {
    self.alignment = alignment;
    self
  }

  /// Sets the line alignment from an `Option`.
  ///
  /// ```rust
  /// use fasrt::vtt::{Line, LineAlign, LineValue, Percentage};
  ///
  /// let mut line = Line::new(LineValue::Percentage(Percentage::with(50.0)));
  /// line.update_alignment(Some(LineAlign::Center));
  /// assert_eq!(line.alignment(), Some(LineAlign::Center));
  /// line.update_alignment(None);
  /// assert_eq!(line.alignment(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn update_alignment(&mut self, alignment: Option<LineAlign>) -> &mut Self {
    self.alignment = alignment;
    self
  }
}

/// Position alignment.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, IsVariant)]
pub enum PositionAlign {
  /// `line-left`
  #[display("line-left")]
  LineLeft,
  /// `center`
  #[display("center")]
  Center,
  /// `line-right`
  #[display("line-right")]
  LineRight,
  /// `auto`
  #[display("auto")]
  Auto,
}

/// Position setting: percentage value and optional alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
  value: Percentage,
  alignment: Option<PositionAlign>,
}

impl Position {
  /// Create a new `Position` with the given percentage and no alignment.
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Position};
  ///
  /// let pos = Position::new(Percentage::with(50.0));
  /// assert_eq!(pos.value(), Percentage::with(50.0));
  /// assert_eq!(pos.alignment(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(value: Percentage) -> Self {
    Self {
      value,
      alignment: None,
    }
  }

  /// Create a new `Position` with the given percentage and alignment.
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Position, PositionAlign};
  ///
  /// let pos = Position::with_alignment(Percentage::with(25.0), PositionAlign::LineLeft);
  /// assert_eq!(pos.value(), Percentage::with(25.0));
  /// assert_eq!(pos.alignment(), Some(PositionAlign::LineLeft));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_alignment(value: Percentage, alignment: PositionAlign) -> Self {
    Self {
      value,
      alignment: Some(alignment),
    }
  }

  /// Returns the position percentage.
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Position};
  ///
  /// let pos = Position::new(Percentage::with(75.0));
  /// assert_eq!(pos.value(), Percentage::with(75.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn value(&self) -> Percentage {
    self.value
  }

  /// Sets the position percentage.
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Position};
  ///
  /// let mut pos = Position::new(Percentage::with(10.0));
  /// pos.set_value(Percentage::with(90.0));
  /// assert_eq!(pos.value(), Percentage::with(90.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_value(&mut self, value: Percentage) -> &mut Self {
    self.value = value;
    self
  }

  /// Returns the position alignment.
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Position, PositionAlign};
  ///
  /// let pos = Position::new(Percentage::with(50.0));
  /// assert_eq!(pos.alignment(), None);
  ///
  /// let pos = Position::with_alignment(Percentage::with(50.0), PositionAlign::Center);
  /// assert_eq!(pos.alignment(), Some(PositionAlign::Center));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn alignment(&self) -> Option<PositionAlign> {
    self.alignment
  }

  /// Sets the position alignment.
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Position, PositionAlign};
  ///
  /// let mut pos = Position::new(Percentage::with(50.0));
  /// pos.set_alignment(PositionAlign::LineRight);
  /// assert_eq!(pos.alignment(), Some(PositionAlign::LineRight));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_alignment(&mut self, alignment: PositionAlign) -> &mut Self {
    self.alignment = Some(alignment);
    self
  }

  /// Sets the position alignment from an `Option` (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Position, PositionAlign};
  ///
  /// let pos = Position::new(Percentage::with(50.0)).maybe_alignment(Some(PositionAlign::Center));
  /// assert_eq!(pos.alignment(), Some(PositionAlign::Center));
  ///
  /// let pos = Position::new(Percentage::with(50.0)).maybe_alignment(None);
  /// assert_eq!(pos.alignment(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn maybe_alignment(mut self, alignment: Option<PositionAlign>) -> Self {
    self.alignment = alignment;
    self
  }

  /// Sets the position alignment from an `Option`.
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Position, PositionAlign};
  ///
  /// let mut pos = Position::new(Percentage::with(50.0));
  /// pos.update_alignment(Some(PositionAlign::Auto));
  /// assert_eq!(pos.alignment(), Some(PositionAlign::Auto));
  /// pos.update_alignment(None);
  /// assert_eq!(pos.alignment(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn update_alignment(&mut self, alignment: Option<PositionAlign>) -> &mut Self {
    self.alignment = alignment;
    self
  }
}

/// Size setting: percentage (0–100).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size(Percentage);

impl Size {
  /// Create a new `Size` from a percentage.
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Size};
  ///
  /// let size = Size::new(Percentage::with(80.0));
  /// assert_eq!(size.value(), Percentage::with(80.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(value: Percentage) -> Self {
    Self(value)
  }

  /// Returns the size percentage.
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Size};
  ///
  /// let size = Size::new(Percentage::with(100.0));
  /// assert_eq!(size.value(), Percentage::with(100.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn value(&self) -> Percentage {
    self.0
  }

  /// Sets the size percentage.
  ///
  /// ```rust
  /// use fasrt::vtt::{Percentage, Size};
  ///
  /// let mut size = Size::new(Percentage::with(50.0));
  /// size.set_value(Percentage::with(75.0));
  /// assert_eq!(size.value(), Percentage::with(75.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_value(&mut self, value: Percentage) -> &mut Self {
    self.0 = value;
    self
  }
}

/// Text alignment.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, IsVariant)]
pub enum Align {
  /// `start`
  #[display("start")]
  Start,
  /// `center`
  #[display("center")]
  Center,
  /// `end`
  #[display("end")]
  End,
  /// `left`
  #[display("left")]
  Left,
  /// `right`
  #[display("right")]
  Right,
}

/// Region identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("{_0}")]
#[repr(transparent)]
pub struct RegionId<'a>(&'a str);

impl<'a> RegionId<'a> {
  /// Create a `RegionId` from a string slice.
  ///
  /// ```rust
  /// use fasrt::vtt::RegionId;
  ///
  /// let id = RegionId::new("header");
  /// assert_eq!(id.as_str(), "header");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(s: &'a str) -> Self {
    Self(s)
  }

  /// Returns the string representation of this region identifier.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_str(&self) -> &'a str {
    self.0
  }
}

/// Scroll direction for a region.
#[derive(Debug, Default, Display, Clone, Copy, PartialEq, Eq, Hash, IsVariant)]
pub enum Scroll {
  /// No scrolling (`scroll:none`).
  #[display("none")]
  #[default]
  None,
  /// Scroll up (`scroll:up`).
  #[display("up")]
  Up,
}
/// An anchor point as a pair of percentages (x, y).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Anchor {
  x: Percentage,
  y: Percentage,
}

impl Anchor {
  /// Create a new `Anchor` with the given x and y percentages.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(x: Percentage, y: Percentage) -> Self {
    Self { x, y }
  }

  /// Returns the x percentage.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn x(&self) -> Percentage {
    self.x
  }

  /// Returns the y percentage.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn y(&self) -> Percentage {
    self.y
  }
}

/// A parsed WebVTT REGION definition.
///
/// Per the W3C spec, a region has an id, width, number of visible lines,
/// region anchor, viewport anchor, and scroll direction.
///
/// ```rust
/// use fasrt::vtt::{Region, RegionId, Percentage, Anchor, Scroll};
///
/// let region = Region::new(RegionId::new("nav"))
///   .with_width(Percentage::with(40.0))
///   .with_lines(3)
///   .with_scroll(Scroll::Up);
/// assert_eq!(region.id().as_str(), "nav");
/// assert_eq!(region.width(), Percentage::with(40.0));
/// assert_eq!(region.lines(), 3);
/// assert_eq!(region.scroll(), Scroll::Up);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Region<'a> {
  id: RegionId<'a>,
  width: Percentage,
  lines: u32,
  region_anchor: Anchor,
  viewport_anchor: Anchor,
  scroll: Scroll,
}

impl Default for Region<'_> {
  fn default() -> Self {
    Self::new(RegionId::new(""))
  }
}

impl<'a> Region<'a> {
  /// Create a new `Region` with the given id and default settings.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn new(id: RegionId<'a>) -> Self {
    Self {
      id,
      width: Percentage::with(100.0),
      lines: 3,
      region_anchor: Anchor::new(Percentage::new(), Percentage::with(100.0)),
      viewport_anchor: Anchor::new(Percentage::new(), Percentage::with(100.0)),
      scroll: Scroll::None,
    }
  }

  /// Returns the region identifier.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn id(&self) -> RegionId<'a> {
    self.id
  }

  /// Sets the region identifier.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_id(&mut self, id: RegionId<'a>) -> &mut Self {
    self.id = id;
    self
  }

  /// Returns the region width.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn width(&self) -> Percentage {
    self.width
  }

  /// Sets the region width (builder pattern).
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_width(mut self, width: Percentage) -> Self {
    self.width = width;
    self
  }

  /// Sets the region width.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_width(&mut self, width: Percentage) -> &mut Self {
    self.width = width;
    self
  }

  /// Returns the number of visible lines.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn lines(&self) -> u32 {
    self.lines
  }

  /// Sets the number of visible lines (builder pattern).
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_lines(mut self, lines: u32) -> Self {
    self.lines = lines;
    self
  }

  /// Sets the number of visible lines.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_lines(&mut self, lines: u32) -> &mut Self {
    self.lines = lines;
    self
  }

  /// Returns the region anchor.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn region_anchor(&self) -> Anchor {
    self.region_anchor
  }

  /// Sets the region anchor (builder pattern).
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_region_anchor(mut self, anchor: Anchor) -> Self {
    self.region_anchor = anchor;
    self
  }

  /// Sets the region anchor.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_region_anchor(&mut self, anchor: Anchor) -> &mut Self {
    self.region_anchor = anchor;
    self
  }

  /// Returns the viewport anchor.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn viewport_anchor(&self) -> Anchor {
    self.viewport_anchor
  }

  /// Sets the viewport anchor (builder pattern).
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_viewport_anchor(mut self, anchor: Anchor) -> Self {
    self.viewport_anchor = anchor;
    self
  }

  /// Sets the viewport anchor.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_viewport_anchor(&mut self, anchor: Anchor) -> &mut Self {
    self.viewport_anchor = anchor;
    self
  }

  /// Returns the scroll direction.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn scroll(&self) -> Scroll {
    self.scroll
  }

  /// Sets the scroll direction (builder pattern).
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_scroll(mut self, scroll: Scroll) -> Self {
    self.scroll = scroll;
    self
  }

  /// Sets the scroll direction.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_scroll(&mut self, scroll: Scroll) -> &mut Self {
    self.scroll = scroll;
    self
  }
}

impl<'a> Header<'a> {
  /// Create a new `Header` with the given start and end timestamps.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(start: Timestamp, end: Timestamp) -> Self {
    Self {
      identifier: None,
      start,
      end,
      settings: None,
    }
  }

  /// Returns the cue identifier, if any.
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, CueId};
  ///
  /// let header = Header::new(Timestamp::new(), Timestamp::new());
  /// assert!(header.identifier().is_none());
  ///
  /// let header = header.with_identifier(CueId::new("intro"));
  /// assert_eq!(header.identifier().unwrap().as_str(), "intro");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn identifier(&self) -> Option<&CueId<'a>> {
    self.identifier.as_ref()
  }

  /// Sets the cue identifier (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, CueId};
  ///
  /// let header = Header::new(Timestamp::new(), Timestamp::new())
  ///   .with_identifier(CueId::new("cue-1"));
  /// assert_eq!(header.identifier().unwrap().as_str(), "cue-1");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_identifier(mut self, id: CueId<'a>) -> Self {
    self.identifier = Some(id);
    self
  }

  /// Sets the cue identifier.
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, CueId};
  ///
  /// let mut header = Header::new(Timestamp::new(), Timestamp::new());
  /// header.set_identifier(CueId::new("cue-2"));
  /// assert_eq!(header.identifier().unwrap().as_str(), "cue-2");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_identifier(&mut self, id: CueId<'a>) -> &mut Self {
    self.identifier = Some(id);
    self
  }

  /// Sets the cue identifier from an `Option` (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, CueId};
  ///
  /// let header = Header::new(Timestamp::new(), Timestamp::new())
  ///   .maybe_identifier(Some(CueId::new("id")));
  /// assert_eq!(header.identifier().unwrap().as_str(), "id");
  ///
  /// let header = Header::new(Timestamp::new(), Timestamp::new())
  ///   .maybe_identifier(None);
  /// assert!(header.identifier().is_none());
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn maybe_identifier(mut self, id: Option<CueId<'a>>) -> Self {
    self.identifier = id;
    self
  }

  /// Sets the cue identifier from an `Option`.
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, CueId};
  ///
  /// let mut header = Header::new(Timestamp::new(), Timestamp::new());
  /// header.update_identifier(Some(CueId::new("x")));
  /// assert!(header.identifier().is_some());
  /// header.update_identifier(None);
  /// assert!(header.identifier().is_none());
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn update_identifier(&mut self, id: Option<CueId<'a>>) -> &mut Self {
    self.identifier = id;
    self
  }

  /// Returns the start timestamp.
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, Hour};
  /// use fasrt::types::{Minute, Second, Millisecond};
  ///
  /// let ts = Timestamp::from_hmsm(Hour::with(0), Minute::with(1), Second::with(0), Millisecond::with(0));
  /// let header = Header::new(ts, Timestamp::new());
  /// assert_eq!(header.start(), ts);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn start(&self) -> Timestamp {
    self.start
  }

  /// Sets the start timestamp (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, Hour};
  /// use fasrt::types::{Minute, Second, Millisecond};
  ///
  /// let ts = Timestamp::from_hmsm(Hour::with(0), Minute::with(2), Second::with(0), Millisecond::with(0));
  /// let header = Header::new(Timestamp::new(), Timestamp::new()).with_start(ts);
  /// assert_eq!(header.start(), ts);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_start(mut self, start: Timestamp) -> Self {
    self.start = start;
    self
  }

  /// Sets the start timestamp.
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, Hour};
  /// use fasrt::types::{Minute, Second, Millisecond};
  ///
  /// let mut header = Header::new(Timestamp::new(), Timestamp::new());
  /// let ts = Timestamp::from_hmsm(Hour::with(0), Minute::with(3), Second::with(0), Millisecond::with(0));
  /// header.set_start(ts);
  /// assert_eq!(header.start(), ts);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_start(&mut self, start: Timestamp) -> &mut Self {
    self.start = start;
    self
  }

  /// Returns the end timestamp.
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, Hour};
  /// use fasrt::types::{Minute, Second, Millisecond};
  ///
  /// let ts = Timestamp::from_hmsm(Hour::with(0), Minute::with(4), Second::with(0), Millisecond::with(0));
  /// let header = Header::new(Timestamp::new(), ts);
  /// assert_eq!(header.end(), ts);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn end(&self) -> Timestamp {
    self.end
  }

  /// Sets the end timestamp (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, Hour};
  /// use fasrt::types::{Minute, Second, Millisecond};
  ///
  /// let ts = Timestamp::from_hmsm(Hour::with(0), Minute::with(5), Second::with(0), Millisecond::with(0));
  /// let header = Header::new(Timestamp::new(), Timestamp::new()).with_end(ts);
  /// assert_eq!(header.end(), ts);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_end(mut self, end: Timestamp) -> Self {
    self.end = end;
    self
  }

  /// Sets the end timestamp.
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, Hour};
  /// use fasrt::types::{Minute, Second, Millisecond};
  ///
  /// let mut header = Header::new(Timestamp::new(), Timestamp::new());
  /// let ts = Timestamp::from_hmsm(Hour::with(0), Minute::with(6), Second::with(0), Millisecond::with(0));
  /// header.set_end(ts);
  /// assert_eq!(header.end(), ts);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_end(&mut self, end: Timestamp) -> &mut Self {
    self.end = end;
    self
  }

  /// Returns the cue settings, if any.
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, CueOptions, Align};
  ///
  /// let header = Header::new(Timestamp::new(), Timestamp::new());
  /// assert!(header.settings().is_none());
  ///
  /// let header = header.with_settings(CueOptions::default().with_align(Align::Center));
  /// assert_eq!(header.settings().unwrap().align(), Some(Align::Center));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn settings(&self) -> Option<&CueOptions<'a>> {
    self.settings.as_ref()
  }

  /// Sets the cue settings (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, CueOptions, Percentage, Size};
  ///
  /// let header = Header::new(Timestamp::new(), Timestamp::new())
  ///   .with_settings(CueOptions::default().with_size(Size::new(Percentage::with(80.0))));
  /// assert_eq!(header.settings().unwrap().size().unwrap().value(), Percentage::with(80.0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_settings(mut self, settings: CueOptions<'a>) -> Self {
    self.settings = Some(settings);
    self
  }

  /// Sets the cue settings.
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, CueOptions, Vertical};
  ///
  /// let mut header = Header::new(Timestamp::new(), Timestamp::new());
  /// header.set_settings(CueOptions::default().with_vertical(Vertical::Rl));
  /// assert_eq!(header.settings().unwrap().vertical(), Some(Vertical::Rl));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_settings(&mut self, settings: CueOptions<'a>) -> &mut Self {
    self.settings = Some(settings);
    self
  }

  /// Sets the cue settings from an `Option` (builder pattern).
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, CueOptions, Align};
  ///
  /// let header = Header::new(Timestamp::new(), Timestamp::new())
  ///   .maybe_settings(Some(CueOptions::default().with_align(Align::End)));
  /// assert_eq!(header.settings().unwrap().align(), Some(Align::End));
  ///
  /// let header = Header::new(Timestamp::new(), Timestamp::new())
  ///   .maybe_settings(None);
  /// assert!(header.settings().is_none());
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn maybe_settings(mut self, settings: Option<CueOptions<'a>>) -> Self {
    self.settings = settings;
    self
  }

  /// Sets the cue settings from an `Option`.
  ///
  /// ```rust
  /// use fasrt::vtt::{Header, Timestamp, CueOptions, Percentage, Size};
  ///
  /// let mut header = Header::new(Timestamp::new(), Timestamp::new());
  /// header.update_settings(Some(CueOptions::default().with_size(Size::new(Percentage::with(50.0)))));
  /// assert!(header.settings().is_some());
  /// header.update_settings(None);
  /// assert!(header.settings().is_none());
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn update_settings(&mut self, settings: Option<CueOptions<'a>>) -> &mut Self {
    self.settings = settings;
    self
  }
}

/// A WebVTT block — either a cue, a note, a style, or a region definition.
#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Unwrap, TryUnwrap)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum Block<'a, T> {
  /// A cue block containing timed text.
  Cue(Cue<'a, T>),
  /// A NOTE block (comment).
  Note(T),
  /// A STYLE block (CSS).
  Style(T),
  /// A REGION definition block.
  Region(Region<'a>),
}
