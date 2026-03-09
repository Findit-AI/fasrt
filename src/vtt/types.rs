use core::time::Duration;

use derive_more::{Display, From, Into, IsVariant, TryUnwrap, Unwrap};

use crate::{
  types::{Entry as GenericEntry, *},
  utils::u64_digits,
};

/// A single cue entry in a WebVTT file.
pub type Cue<T> = GenericEntry<Header, T>;

/// The hour component of a WebVTT timestamp.
///
/// Per the W3C spec, WebVTT hours have no upper limit ("one or more digits").
/// This wraps a `u64` with no maximum constraint.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into)]
#[repr(transparent)]
pub struct Hour(pub(crate) u64);

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
pub struct Header {
  identifier: Option<CueId>,
  start: Timestamp,
  end: Timestamp,
  settings: Option<CueSettings>,
}

/// A cue identifier. Per the W3C spec, this is any text that does not
/// contain "-->", and is not empty.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CueId(CueIdInner);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CueIdInner {
  Borrowed(&'static str),
  #[cfg(any(feature = "alloc", feature = "std"))]
  Owned(std::string::String),
}

impl CueId {
  /// Create a `CueId` from a static string.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn from_static(s: &'static str) -> Self {
    Self(CueIdInner::Borrowed(s))
  }

  /// Create a `CueId` from an owned string.
  #[cfg(any(feature = "alloc", feature = "std"))]
  #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn from_string(s: std::string::String) -> Self {
    Self(CueIdInner::Owned(s))
  }

  /// Returns the string representation of this cue identifier.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn as_str(&self) -> &str {
    match &self.0 {
      CueIdInner::Borrowed(s) => s,
      #[cfg(any(feature = "alloc", feature = "std"))]
      CueIdInner::Owned(s) => s.as_str(),
    }
  }
}

impl core::fmt::Display for CueId {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Cue settings per the W3C WebVTT specification.
///
/// Each setting is optional. The spec defines these settings:
/// `vertical`, `line`, `position`, `size`, `align`, `region`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CueSettings {
  /// Writing direction (`vertical:rl` or `vertical:lr`).
  pub vertical: Option<Vertical>,
  /// Line position.
  pub line: Option<Line>,
  /// Text position.
  pub position: Option<Position>,
  /// Cue size as a percentage (0–100).
  pub size: Option<Size>,
  /// Text alignment.
  pub align: Option<Align>,
  /// Region identifier.
  pub region: Option<RegionId>,
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

/// Line position value.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, IsVariant, Unwrap, TryUnwrap)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum LineValue {
  /// A percentage value (0–100).
  #[display("{_0}%")]
  Percentage(u8),
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
  /// The line value.
  pub value: LineValue,
  /// Optional line alignment.
  pub alignment: Option<LineAlign>,
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
  /// The position percentage (0–100).
  pub value: u8,
  /// Optional position alignment.
  pub alignment: Option<PositionAlign>,
}

/// Size setting: percentage (0–100).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size(pub u8);

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegionId(RegionIdInner);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum RegionIdInner {
  Borrowed(&'static str),
  #[cfg(any(feature = "alloc", feature = "std"))]
  Owned(std::string::String),
}

impl RegionId {
  /// Create a `RegionId` from a static string.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn from_static(s: &'static str) -> Self {
    Self(RegionIdInner::Borrowed(s))
  }

  /// Create a `RegionId` from an owned string.
  #[cfg(any(feature = "alloc", feature = "std"))]
  #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn from_string(s: std::string::String) -> Self {
    Self(RegionIdInner::Owned(s))
  }

  /// Returns the string representation of this region identifier.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn as_str(&self) -> &str {
    match &self.0 {
      RegionIdInner::Borrowed(s) => s,
      #[cfg(any(feature = "alloc", feature = "std"))]
      RegionIdInner::Owned(s) => s.as_str(),
    }
  }
}

impl core::fmt::Display for RegionId {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl Header {
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
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn identifier(&self) -> Option<&CueId> {
    self.identifier.as_ref()
  }

  /// Sets the cue identifier.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_identifier(mut self, id: CueId) -> Self {
    self.identifier = Some(id);
    self
  }

  /// Sets the cue identifier.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_identifier(&mut self, id: CueId) -> &mut Self {
    self.identifier = Some(id);
    self
  }

  /// Returns the start timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn start(&self) -> Timestamp {
    self.start
  }

  /// Sets the start timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_start(mut self, start: Timestamp) -> Self {
    self.start = start;
    self
  }

  /// Sets the start timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_start(&mut self, start: Timestamp) -> &mut Self {
    self.start = start;
    self
  }

  /// Returns the end timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn end(&self) -> Timestamp {
    self.end
  }

  /// Sets the end timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_end(mut self, end: Timestamp) -> Self {
    self.end = end;
    self
  }

  /// Sets the end timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_end(&mut self, end: Timestamp) -> &mut Self {
    self.end = end;
    self
  }

  /// Returns the cue settings, if any.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn settings(&self) -> Option<&CueSettings> {
    self.settings.as_ref()
  }

  /// Sets the cue settings.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_settings(mut self, settings: CueSettings) -> Self {
    self.settings = Some(settings);
    self
  }

  /// Sets the cue settings.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_settings(&mut self, settings: CueSettings) -> &mut Self {
    self.settings = Some(settings);
    self
  }
}

/// A WebVTT block — either a cue, a note, a style, or a region definition.
#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Unwrap, TryUnwrap)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum Block<T> {
  /// A cue block containing timed text.
  Cue(Cue<T>),
  /// A NOTE block (comment).
  Note(T),
  /// A STYLE block (CSS).
  Style(T),
  /// A REGION definition block.
  Region(T),
}
