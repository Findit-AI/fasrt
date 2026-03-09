use derive_more::{Display, From};

use core::{num::NonZeroU64, time::Duration};

use crate::{
  types::{Entry as GenericEntry, *},
  utils::u64_digits,
};

/// A single subtitle entry in an SRT file.
pub type Entry<T> = GenericEntry<Header, T>;

/// A timestamp in an SRT file, with millisecond precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From)]
#[display("{}:{}:{},{}", hours, minutes, seconds, millis)]
pub struct Timestamp {
  /// Hours (0–999).
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
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::{Hour, Minute, Second, Millisecond};
  ///
  /// let timestamp = Timestamp::default();
  /// assert_eq!(timestamp.hours(), Hour::with(0));
  /// assert_eq!(timestamp.minutes(), Minute::with(0));
  /// assert_eq!(timestamp.seconds(), Second::with(0));
  /// assert_eq!(timestamp.millis(), Millisecond::with(0));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn default() -> Self {
    Self::new()
  }
}

impl Timestamp {
  /// Create a new timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new() -> Self {
    Self::from_hmsm(Hour(0), Minute(0), Second(0), Millisecond(0))
  }

  /// Create a new timestamp from hours, minutes, seconds, and milliseconds.
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
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::Hour;
  ///
  /// let timestamp = Timestamp::default().with_hours(Hour::with(1));
  /// assert_eq!(timestamp.hours(), Hour::with(1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_hours(mut self, hours: Hour) -> Self {
    self.set_hours(hours);
    self
  }

  /// Build a new timestamp with the minutes field set to the given value.
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::Minute;
  ///
  /// let timestamp = Timestamp::default().with_minutes(Minute::with(1));
  /// assert_eq!(timestamp.minutes(), Minute::with(1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_minutes(mut self, minutes: Minute) -> Self {
    self.set_minutes(minutes);
    self
  }

  /// Build a new timestamp with the seconds field set to the given value.
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::Second;
  ///
  /// let timestamp = Timestamp::default().with_seconds(Second::with(1));
  /// assert_eq!(timestamp.seconds(), Second::with(1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_seconds(mut self, seconds: Second) -> Self {
    self.set_seconds(seconds);
    self
  }

  /// Build a new timestamp with the millis field set to the given value.
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::Millisecond;
  ///
  /// let timestamp = Timestamp::default().with_millis(Millisecond::with(1));
  /// assert_eq!(timestamp.millis(), Millisecond::with(1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_millis(mut self, millis: Millisecond) -> Self {
    self.set_millis(millis);
    self
  }

  /// Set the hours field of this timestamp.
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::Hour;
  ///
  /// let mut timestamp = Timestamp::default();
  /// timestamp.set_hours(Hour::with(1));
  /// assert_eq!(timestamp.hours(), Hour::with(1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_hours(&mut self, hours: Hour) -> &mut Self {
    self.hours = hours;
    self
  }

  /// Set the minutes field of this timestamp.
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::Minute;
  ///
  /// let mut timestamp = Timestamp::default();
  /// timestamp.set_minutes(Minute::with(1));
  /// assert_eq!(timestamp.minutes(), Minute::with(1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_minutes(&mut self, minutes: Minute) -> &mut Self {
    self.minutes = minutes;
    self
  }

  /// Set the seconds field of this timestamp.
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::Second;
  ///
  /// let mut timestamp = Timestamp::default();
  /// timestamp.set_seconds(Second::with(1));
  /// assert_eq!(timestamp.seconds(), Second::with(1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_seconds(&mut self, seconds: Second) -> &mut Self {
    self.seconds = seconds;
    self
  }

  /// Set the millis field of this timestamp.
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::Millisecond;
  ///
  /// let mut timestamp = Timestamp::default();
  /// timestamp.set_millis(Millisecond::with(1));
  /// assert_eq!(timestamp.millis(), Millisecond::with(1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_millis(&mut self, millis: Millisecond) -> &mut Self {
    self.millis = millis;
    self
  }

  /// Set the hours, minutes, seconds, and milliseconds fields of this timestamp.
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::{Hour, Minute, Second, Millisecond};
  ///
  /// let mut timestamp = Timestamp::default();
  /// timestamp.set_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4));
  /// assert_eq!(timestamp.hours(), Hour::with(1));
  /// assert_eq!(timestamp.minutes(), Minute::with(2));
  /// assert_eq!(timestamp.seconds(), Second::with(3));
  /// assert_eq!(timestamp.millis(), Millisecond::with(4));
  /// ```
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
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::{Hour, Minute, Second, Millisecond};
  ///
  /// let timestamp = Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4));
  /// let duration = timestamp.to_duration();
  /// assert_eq!(duration, Duration::from_millis(1 * 3_600_000 + 2 * 60_000 + 3 * 1_000 + 4));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn to_duration(&self) -> Duration {
    let hours = self.hours.0 as u64;
    let minutes = self.minutes.0 as u64;
    let seconds = self.seconds.0 as u64;
    let millis = self.millis.0 as u64;

    Duration::from_millis(hours * 3_600_000 + minutes * 60_000 + seconds * 1_000 + millis)
  }

  /// Returns the encoded length of this timestamp.
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::{Hour, Minute, Second, Millisecond};
  ///
  /// let timestamp = Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4));
  /// assert_eq!(timestamp.encoded_len(), 12);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn encoded_len(&self) -> usize {
    self.hours().as_str().len() + 1 // HH:
    + self.minutes().as_str().len() + 1 // MM:
    + self.seconds().as_str().len() + 1 // SS, or SS.
    + self.millis().as_str().len() // mmm
  }

  /// Format this timestamp to a SRT timestamp string.
  ///
  /// ```rust
  /// use fasrt::srt::Timestamp;
  /// use fasrt::types::{Hour, Minute, Second, Millisecond};
  ///
  /// let timestamp = Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4));
  /// assert_eq!(timestamp.encode().as_str(), "01:02:03,004");
  ///
  /// let timestamp = Timestamp::from_hmsm(Hour::with(122), Minute::with(34), Second::with(56), Millisecond::with(789));
  /// assert_eq!(timestamp.encode().as_str(), "122:34:56,789");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn encode(&self) -> Buffer<14> {
    let mut buffer = Buffer::new();
    buffer.write_str(self.hours().as_str());
    buffer.write_str(":");
    buffer.write_str(self.minutes().as_str());
    buffer.write_str(":");
    buffer.write_str(self.seconds().as_str());
    buffer.write_str(",");
    buffer.write_str(self.millis().as_str());
    buffer
  }
}

/// The header of a subtitle entry, containing the index and timestamps, but not the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
  index: Option<NonZeroU64>,
  start: Timestamp,
  end: Timestamp,
}

impl Header {
  /// Create a new `Header` with the given index, start time, and end time.
  ///
  /// ```rust
  /// use fasrt::srt::{Header, Timestamp};
  ///
  /// let header = Header::new(Timestamp::default(), Timestamp::default());
  /// assert_eq!(header.index(), None);
  /// assert_eq!(header.start(), Timestamp::default());
  /// assert_eq!(header.end(), Timestamp::default());
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(start: Timestamp, end: Timestamp) -> Self {
    Self {
      index: None,
      start,
      end,
    }
  }

  /// Returns the index of this subtitle header, or `None` if it was missing.
  ///
  /// ```rust
  /// use fasrt::srt::{Header, Timestamp};
  ///
  /// let header = Header::new(Timestamp::default(), Timestamp::default());
  /// assert_eq!(header.index(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn index(&self) -> Option<NonZeroU64> {
    self.index
  }

  /// Sets the index of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::srt::{Header, Timestamp};
  /// use core::num::NonZeroU64;
  ///
  /// let mut header = Header::new(Timestamp::default(), Timestamp::default());
  /// header.set_index(NonZeroU64::new(1).unwrap());
  /// assert_eq!(header.index(), NonZeroU64::new(1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_index(mut self, index: NonZeroU64) -> Self {
    self.set_index(index);
    self
  }

  /// Sets the index of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::srt::{Header, Timestamp};
  /// use core::num::NonZeroU64;
  ///
  /// let mut header = Header::new(Timestamp::default(), Timestamp::default())
  ///   .maybe_index(Some(NonZeroU64::new(1).unwrap()));
  /// assert_eq!(header.index(), NonZeroU64::new(1));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_index(&mut self, index: NonZeroU64) -> &mut Self {
    self.update_index(Some(index));
    self
  }

  /// Sets the index of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::srt::{Header, Timestamp};
  ///
  /// let header = Header::new(Timestamp::default(), Timestamp::default()).maybe_index(None);
  /// assert_eq!(header.index(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn maybe_index(mut self, index: Option<NonZeroU64>) -> Self {
    self.update_index(index);
    self
  }

  /// Sets the index of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::srt::{Header, Timestamp};
  ///
  /// let mut header = Header::new(Timestamp::default(), Timestamp::default());
  /// header.update_index(None);
  /// assert_eq!(header.index(), None);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn update_index(&mut self, index: Option<NonZeroU64>) -> &mut Self {
    self.index = index;
    self
  }

  /// Returns the start time of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::srt::{Header, Timestamp};
  ///
  /// let header = Header::new(Timestamp::default(), Timestamp::default());
  /// assert_eq!(header.start(), Timestamp::default());
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn start(&self) -> Timestamp {
    self.start
  }

  /// Sets the start time of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::{
  ///   srt::{Header, Timestamp},
  ///   types::{Hour, Minute, Second, Millisecond},
  /// };
  ///
  /// let header = Header::new(Timestamp::default(), Timestamp::default())
  ///   .with_start(Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4)));
  /// assert_eq!(header.start(), Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4)));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_start(mut self, start: Timestamp) -> Self {
    self.set_start(start);
    self
  }

  /// Sets the start time of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::{
  ///   srt::{Header, Timestamp},
  ///   types::{Hour, Minute, Second, Millisecond},
  /// };
  ///
  /// let mut header = Header::new(Timestamp::default(), Timestamp::default());
  /// header.set_start(Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4)));
  /// assert_eq!(header.start(), Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4)));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_start(&mut self, start: Timestamp) -> &mut Self {
    self.start = start;
    self
  }

  /// Returns the end time of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::srt::{Header, Timestamp};
  ///
  /// let header = Header::new(Timestamp::default(), Timestamp::default());
  /// assert_eq!(header.end(), Timestamp::default());
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn end(&self) -> Timestamp {
    self.end
  }

  /// Sets the end time of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::{
  ///   srt::{Header, Timestamp},
  ///   types::{Hour, Minute, Second, Millisecond},
  /// };
  ///
  /// let header = Header::new(Timestamp::default(), Timestamp::default())
  ///   .with_end(Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4)));
  /// assert_eq!(header.end(), Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4)));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_end(mut self, end: Timestamp) -> Self {
    self.set_end(end);
    self
  }

  /// Sets the end time of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::{
  ///   srt::{Header, Timestamp},
  ///   types::{Hour, Minute, Second, Millisecond},
  /// };
  ///
  /// let mut header = Header::new(Timestamp::default(), Timestamp::default());
  /// header.set_end(Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4)));
  /// assert_eq!(header.end(), Timestamp::from_hmsm(Hour::with(1), Minute::with(2), Second::with(3), Millisecond::with(4)));
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_end(&mut self, end: Timestamp) -> &mut Self {
    self.end = end;
    self
  }

  /// Returns the encoded length of this subtitle header.
  ///
  /// ```rust
  /// use fasrt::srt::{Header, Timestamp};
  ///
  /// let header = Header::new(Timestamp::default(), Timestamp::default());
  /// assert_eq!(header.encoded_len(), 30);
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn encoded_len(&self) -> usize {
    let start_len = self.start.encoded_len();
    let end_len = self.end.encoded_len();

    match self.index {
      Some(index) => {
        u64_digits(index.get()) + 1 // index\n
        + start_len + 5 + end_len // HHH:MM:ss,mmm --> HHH:MM:ss,mmm
        + 1 // newline after header
      }
      None => start_len + 5 + end_len + 1, // HHH:MM:ss,mmm --> HHH:MM:ss,mmm\n
    }
  }

  /// Format this timestamp to a SRT timestamp string.
  ///
  /// ```rust
  /// use fasrt::{srt::{Header, Timestamp}, types::Hour};
  /// use core::num::NonZeroU64;
  ///
  /// let header = Header::new(Timestamp::default(), Timestamp::default()).with_index(NonZeroU64::new(u64::MAX).unwrap());
  /// let encoded_len = header.encoded_len();
  /// let buf = header.encode();
  /// assert_eq!(buf.len(), encoded_len);
  /// assert_eq!(buf.as_str(), "18446744073709551615\n00:00:00,000 --> 00:00:00,000\n");
  ///
  /// let header = Header::new(Timestamp::default().with_hours(Hour::with(122)), Timestamp::default().with_hours(Hour::with(123))).with_index(NonZeroU64::new(u64::MAX).unwrap());
  /// let encoded_len = header.encoded_len();
  /// let buf = header.encode();
  /// assert_eq!(buf.len(), encoded_len);
  /// assert_eq!(buf.as_str(), "18446744073709551615\n122:00:00,000 --> 123:00:00,000\n");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn encode(&self) -> Buffer<54> {
    let mut buffer = Buffer::new();
    if let Some(index) = self.index {
      buffer.fmt_u64(index.get());
      buffer.write_str("\n");
    }
    buffer.write_str(self.start.encode().as_str());
    buffer.write_str(" --> ");
    buffer.write_str(self.end.encode().as_str());
    buffer.write_str("\n");
    buffer
  }
}
