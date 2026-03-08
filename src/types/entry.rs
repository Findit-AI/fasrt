/// A single subtitle entry in an SRT file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry<H, T> {
  header: H,
  /// The text lines of this subtitle (joined with newlines on display).
  body: T,
}

impl<H, T> Entry<H, T> {
  /// Create a new `Entry` with the given header and body.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(header: H, body: T) -> Self {
    Self { header, body }
  }

  /// Returns the header of this subtitle entry, containing the index and timestamps.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn header(&self) -> &H {
    &self.header
  }

  /// Sets the header of this subtitle entry, containing the index and timestamps.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_header(mut self, header: H) -> Self {
    self.header = header;
    self
  }

  /// Sets the header of this subtitle entry, containing the index and timestamps.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_header(&mut self, header: H) -> &mut Self {
    self.header = header;
    self
  }

  /// Returns the body of this subtitle entry, containing the text lines.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn body(&self) -> &T {
    &self.body
  }

  /// Sets the body of this subtitle entry, containing the text lines.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_body(mut self, body: T) -> Self {
    self.body = body;
    self
  }

  /// Sets the body of this subtitle entry, containing the text lines.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_body(&mut self, body: T) -> &mut Self {
    self.body = body;
    self
  }
}
