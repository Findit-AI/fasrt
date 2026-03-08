use derive_more::{Display, From};

use core::time::Duration;

use crate::{types::*, utils::u64_digits};

/// A single subtitle entry in an SRT file.
pub type SrtEntry<T> = Entry<SrtHeader, T>;

/// A timestamp in an SRT file, with millisecond precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From)]
#[display("{}:{}:{},{}", hours, minutes, seconds, millis)]
pub struct SrtTimestamp {
  /// Hours (0–999).
  hours: Hour,
  /// Millisecondeconds (0–999).
  millis: Millisecond,
  /// Minutes (0–59).
  minutes: Minute,
  /// Seconds (0–59).
  seconds: Second,
}

impl Default for SrtTimestamp {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn default() -> Self {
    Self::new()
  }
}

impl SrtTimestamp {
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
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_hours(mut self, hours: Hour) -> Self {
    self.hours = hours;
    self
  }

  /// Build a new timestamp with the minutes field set to the given value.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_minutes(mut self, minutes: Minute) -> Self {
    self.minutes = minutes;
    self
  }

  /// Build a new timestamp with the seconds field set to the given value.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_seconds(mut self, seconds: Second) -> Self {
    self.seconds = seconds;
    self
  }

  /// Build a new timestamp with the millis field set to the given value.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_millis(mut self, millis: Millisecond) -> Self {
    self.millis = millis;
    self
  }

  /// Set the hours, minutes, seconds, and milliseconds fields of this timestamp.
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
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn to_duration(&self) -> Duration {
    let hours = self.hours.0 as u64;
    let minutes = self.minutes.0 as u64;
    let seconds = self.seconds.0 as u64;
    let millis = self.millis.0 as u64;

    Duration::from_millis(hours * 3_600_000 + minutes * 60_000 + seconds * 1_000 + millis)
  }

  /// Returns the encoded length of this timestamp.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn encoded_len(&self) -> usize {
    self.hours().as_str().len() + 1 // HH:
    + self.minutes().as_str().len() + 1 // MM:
    + self.seconds().as_str().len() + 1 // SS, or SS.
    + self.millis().as_str().len() // mmm
  }

  /// Format this timestamp to a SRT timestamp string.
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
pub struct SrtHeader {
  index: Option<NonZeroU64>,
  start: SrtTimestamp,
  end: SrtTimestamp,
}

impl SrtHeader {
  /// Create a new `SrtHeader` with the given index, start time, and end time.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(start: SrtTimestamp, end: SrtTimestamp) -> Self {
    Self {
      index: None,
      start,
      end,
    }
  }

  /// Returns the index of this subtitle header, or `None` if it was missing.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn index(&self) -> Option<NonZeroU64> {
    self.index
  }

  /// Sets the index of this subtitle header.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_index(mut self, index: NonZeroU64) -> Self {
    self.index = Some(index);
    self
  }

  /// Sets the index of this subtitle header.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_index(&mut self, index: NonZeroU64) -> &mut Self {
    self.index = Some(index);
    self
  }

  /// Sets the index of this subtitle header.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn maybe_index(mut self, index: Option<NonZeroU64>) -> Self {
    self.index = index;
    self
  }

  /// Sets the index of this subtitle header.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn update_index(&mut self, index: Option<NonZeroU64>) -> &mut Self {
    self.index = index;
    self
  }

  /// Returns the start time of this subtitle header.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn start(&self) -> SrtTimestamp {
    self.start
  }

  /// Sets the start time of this subtitle header.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_start(mut self, start: SrtTimestamp) -> Self {
    self.start = start;
    self
  }

  /// Sets the start time of this subtitle header.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_start(&mut self, start: SrtTimestamp) -> &mut Self {
    self.start = start;
    self
  }

  /// Returns the end time of this subtitle header.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn end(&self) -> SrtTimestamp {
    self.end
  }

  /// Sets the end time of this subtitle header.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with_end(mut self, end: SrtTimestamp) -> Self {
    self.end = end;
    self
  }

  /// Sets the end time of this subtitle header.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn set_end(&mut self, end: SrtTimestamp) -> &mut Self {
    self.end = end;
    self
  }

  /// Returns the encoded length of this subtitle header.
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
