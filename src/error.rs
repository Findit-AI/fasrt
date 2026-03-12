use core::num::ParseIntError;
use derive_more::{IsVariant, TryUnwrap, Unwrap};

/// The error type for parsing minute components of timestamps.
#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Unwrap, TryUnwrap, thiserror::Error)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum ParseMinuteError {
  /// The minute component is not zero-padded to 2 digits.
  #[error("minute component is not zero-padded to 2 digits")]
  #[unwrap(ignore)]
  #[try_unwrap(ignore)]
  NotPadded,
  /// The minute component is out of range (not between 0-59).
  #[error("minute component must be between 0-59, but was {0}")]
  Overflow(u8),
  /// Not a valid number.
  #[error(transparent)]
  ParseInt(#[from] ParseIntError),
}

/// The error type for parsing second components of timestamps.
#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Unwrap, TryUnwrap, thiserror::Error)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum ParseSecondError {
  /// The second component is not zero-padded to 2 digits.
  #[error("second component is not zero-padded to 2 digits")]
  #[unwrap(ignore)]
  #[try_unwrap(ignore)]
  NotPadded,
  /// The second component is out of range (not between 0-59).
  #[error("second component must be between 0-59, but was {0}")]
  Overflow(u8),
  /// Not a valid number.
  #[error(transparent)]
  ParseInt(#[from] ParseIntError),
}

/// The error type for parsing hour components of timestamps.
///
/// This enum is shared by both SRT and WebVTT parsers:
/// - **SRT** hours are 2–3 digits (0–999): uses [`NotPadded`](Self::NotPadded)
///   and [`Overflow(u16)`](Self::Overflow).
/// - **WebVTT** hours are unbounded (`u64`): uses [`NotPadded`](Self::NotPadded)
///   for non-digit input and [`HourOverflow`](Self::HourOverflow) when the
///   value exceeds `u64::MAX`.
#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Unwrap, TryUnwrap, thiserror::Error)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum ParseHourError {
  /// The hour component is not zero-padded to 2 digits (SRT),
  /// or contains non-digit characters (VTT).
  #[error("hour component is not zero-padded to 2 digits")]
  #[unwrap(ignore)]
  #[try_unwrap(ignore)]
  NotPadded,
  /// The hour component is out of the SRT range (0–999).
  #[error("hour component must be between 0-999, but was {0}")]
  Overflow(u16),
  /// The hour component overflowed `u64` (VTT unbounded hours).
  #[error("hour component overflowed")]
  #[unwrap(ignore)]
  #[try_unwrap(ignore)]
  HourOverflow,
  /// Not a valid number.
  #[error(transparent)]
  ParseInt(#[from] ParseIntError),
}

/// The error type for parsing millisecond components of timestamps.
#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Unwrap, TryUnwrap, thiserror::Error)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum ParseMillisecondError {
  /// The millisecond component is not zero-padded to 3 digits.
  #[error("millisecond component is not zero-padded to 3 digits")]
  #[unwrap(ignore)]
  #[try_unwrap(ignore)]
  NotPadded,
  /// The millisecond component is out of range (not between 0-999).
  #[error("millisecond component must be between 0-999, but was {0}")]
  Overflow(u16),
  /// Not a valid number.
  #[error(transparent)]
  ParseInt(#[from] ParseIntError),
}

/// The error type for parsing index numbers of subtitles.
#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Unwrap, TryUnwrap, thiserror::Error)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum ParseIndexNumberError {
  /// The index number is zero, which is invalid (must be between 1-18446744073709551615).
  #[error("index number cannot be zero")]
  Zero,
  /// The index number is out of range (not between 1-18446744073709551615).
  #[error("index number must be between 1-18446744073709551615")]
  Overflow,
  /// Not a valid index number.
  #[error(transparent)]
  ParseInt(#[from] ParseIntError),
}
