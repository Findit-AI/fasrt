/// A single subtitle entry in an SRT file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry<H, T> {
  header: H,
  /// The text lines of this subtitle (joined with newlines on display).
  body: T,
}

impl<H, T> Entry<H, T> {
  /// Create a new `Entry` with the given header and body.
  ///
  /// ```rust
  /// use fasrt::types::Entry;
  ///
  /// let entry = Entry::new("Header", "Body");
  /// assert_eq!(entry.header(), "Header");
  /// assert_eq!(entry.body(), "Body");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn new(header: H, body: T) -> Self {
    Self { header, body }
  }

  /// Returns the header of this subtitle entry, containing the index and timestamps.
  ///
  /// ```rust
  /// use fasrt::types::Entry;
  ///
  /// let entry = Entry::new("Header", "Body");
  /// assert_eq!(entry.header(), "Header");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn header(&self) -> H
  where
    H: Copy,
  {
    self.header
  }

  /// Returns the header of this subtitle entry, containing the index and timestamps.
  ///
  /// ```rust
  /// use fasrt::types::Entry;
  ///
  /// let entry = Entry::new("Header", "Body");
  /// assert_eq!(entry.header_ref(), &"Header");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn header_ref(&self) -> &H {
    &self.header
  }

  /// Returns the header of this subtitle entry, containing the index and timestamps.
  ///
  /// ```rust
  /// use fasrt::types::Entry;
  ///
  /// let mut entry = Entry::new("Header", "Body");
  /// *entry.header_mut() = "New Header";
  /// assert_eq!(entry.header_ref(), &"New Header");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn header_mut(&mut self) -> &mut H {
    &mut self.header
  }

  /// Sets the header of this subtitle entry, containing the index and timestamps.
  ///
  /// ```rust
  /// use fasrt::types::Entry;
  ///
  /// let entry = Entry::new("Header", "Body");
  /// let new_entry = entry.with_header("New Header");
  /// assert_eq!(new_entry.header(), "New Header");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_header(mut self, header: H) -> Self {
    self.set_header(header);
    self
  }

  /// Sets the header of this subtitle entry, containing the index and timestamps.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_header(&mut self, header: H) -> &mut Self {
    self.header = header;
    self
  }

  /// Returns the body of this subtitle entry, containing the text lines.
  ///
  /// ```rust
  /// use fasrt::types::Entry;
  ///
  /// let entry = Entry::new("Header", "Body");
  /// assert_eq!(entry.body(), "Body");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn body(&self) -> T
  where
    T: Copy,
  {
    self.body
  }

  /// Returns the body of this subtitle entry, containing the text lines.
  ///
  /// ```rust
  /// use fasrt::types::Entry;
  ///
  /// let entry = Entry::new("Header", "Body");
  /// assert_eq!(entry.body_ref(), &"Body");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn body_ref(&self) -> &T {
    &self.body
  }

  /// Returns the body of this subtitle entry, containing the text lines.
  ///
  /// ```rust
  /// use fasrt::types::Entry;
  ///
  /// let mut entry = Entry::new("Header", "Body");
  /// *entry.body_mut() = "New Body";
  /// assert_eq!(entry.body_ref(), &"New Body");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn body_mut(&mut self) -> &mut T {
    &mut self.body
  }

  /// Sets the body of this subtitle entry, containing the text lines.
  ///
  /// ```rust
  /// use fasrt::types::Entry;
  ///
  /// let entry = Entry::new("Header", "Body");
  /// let new_entry = entry.with_body("New Body");
  /// assert_eq!(new_entry.body(), "New Body");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn with_body(mut self, body: T) -> Self {
    self.set_body(body);
    self
  }

  /// Sets the body of this subtitle entry, containing the text lines.
  ///
  /// ```rust
  /// use fasrt::types::Entry;
  ///
  /// let mut entry = Entry::new("Header", "Body");
  /// entry.set_body("New Body");
  /// assert_eq!(entry.body(), "New Body");
  /// ```
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn set_body(&mut self, body: T) -> &mut Self {
    self.body = body;
    self
  }
}
