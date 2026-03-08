use derive_more::{Display, From, Into};

use core::str::FromStr;

use crate::error::*;

use super::macros::*;

/// The minute component (0–59) of a timestamp.
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From, Into,
)]
#[display("{}", self.as_str())]
#[repr(transparent)]
pub struct Minute(pub(super) u8);

impl FromStr for Minute {
  type Err = ParseMinuteError;

  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    minute_from_str!(s)
  }
}

impl Minute {
  /// Create a new `Minute` with value 0.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new() -> Self {
    Self::with(0)
  }

  /// Create a new `Minute` from a `u8`.
  ///
  /// # Panics
  /// Panics if the value is greater than 59.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with(value: u8) -> Self {
    if value > 59 {
      panic!("Minute value must be between 0-59");
    }
    Self(value)
  }

  /// Try to create a new `Minute` from a `u8`, returning `None` if the value is out of range.
  pub const fn try_with(value: u8) -> Option<Self> {
    if value > 59 { None } else { Some(Self(value)) }
  }

  /// Returns the string representation of this `Minute`, zero-padded to 2 digits.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    minute_to_str!(self.0)
  }
}

/// The second component (0–59) of a timestamp.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Into)]
#[display("{}", self.as_str())]
#[repr(transparent)]
pub struct Second(pub(super) u8);

impl FromStr for Second {
  type Err = ParseSecondError;

  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    second_from_str!(s)
  }
}

impl Second {
  /// Create a new `Second` with value 0.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new() -> Self {
    Self::with(0)
  }

  /// Create a new `Second` from a `u8`.
  ///
  /// # Panics
  /// Panics if the value is greater than 59.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with(value: u8) -> Self {
    if value > 59 {
      panic!("Second value must be between 0-59");
    }
    Self(value)
  }

  /// Try to create a new `Second` from a `u8`, returning `None` if the value is out of range.
  pub const fn try_with(value: u8) -> Option<Self> {
    if value > 59 { None } else { Some(Self(value)) }
  }

  /// Returns the string representation of this `Second`, zero-padded to 2 digits.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    second_to_str!(self.0)
  }
}

/// The hour component (0–999) of a timestamp.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Into)]
#[display("{:0>3}", _0)]
#[repr(transparent)]
pub struct Hour(pub(super) u16);

impl FromStr for Hour {
  type Err = ParseHourError;

  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    hour_from_str!(s)
  }
}

impl Hour {
  /// Create a new `Hour` with value 0.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new() -> Self {
    Self::with(0)
  }

  /// Create a new `Hour` from a `u16`.
  ///
  /// # Panics
  /// Panics if the value is greater than 999.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with(value: u16) -> Self {
    if value > 999 {
      panic!("Hour value must be between 0-999");
    }
    Self(value)
  }

  /// Try to create a new `Hour` from a `u16`, returning `None` if the value is out of range.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn try_with(value: u16) -> Option<Self> {
    if value > 999 { None } else { Some(Self(value)) }
  }

  /// Returns the string representation of this `Hour`, zero-padded to 3 digits.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    hour_to_str!(self.0)
  }
}

/// The millisecond component (0–999) of a timestamp.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Into)]
#[display("{:0>3}", _0)]
#[repr(transparent)]
pub struct Millisecond(pub(super) u16);

impl FromStr for Millisecond {
  type Err = ParseMillisecondError;

  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    millisecond_from_str!(s)
  }
}

impl Millisecond {
  /// Create a new `Millisecond` with value 0.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new() -> Self {
    Self::with(0)
  }

  /// Create a new `Millisecond` from a `u16`.
  ///
  /// # Panics
  /// Panics if the value is greater than 999.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn with(value: u16) -> Self {
    if value > 999 {
      panic!("Millisecond value must be between 0-999");
    }
    Self(value)
  }

  /// Try to create a new `Millisecond` from a `u16`, returning `None` if the value is out of range.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn try_with(value: u16) -> Option<Self> {
    if value > 999 { None } else { Some(Self(value)) }
  }

  /// Returns the string representation of this `Millisecond`, zero-padded to 3 digits.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    millisecond_to_str!(self.0)
  }
}
