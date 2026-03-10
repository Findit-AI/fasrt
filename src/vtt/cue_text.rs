//! Cue text parsing per the W3C WebVTT spec (§6.4).
//!
//! Provides a lazy [`CueParser`] iterator that yields [`CueToken`]s from raw
//! cue text, and (with `alloc`/`std`) a [`CueText`] DOM tree built on top.
use derive_more::{Display, IsVariant};

use core::fmt;

/// A recognized WebVTT cue text tag name.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, IsVariant)]
pub enum Tag {
  /// `<b>` — bold.
  #[display("b")]
  Bold,
  /// `<i>` — italic.
  #[display("i")]
  Italic,
  /// `<u>` — underline.
  #[display("u")]
  Underline,
  /// `<c>` — class span.
  #[display("c")]
  Class,
  /// `<ruby>` — ruby annotation container.
  #[display("ruby")]
  Ruby,
  /// `<rt>` — ruby text.
  #[display("rt")]
  RubyText,
  /// `<v>` — voice span.
  #[display("v")]
  Voice,
  /// `<lang>` — language span.
  #[display("lang")]
  Lang,
}

impl Tag {
  #[cfg_attr(not(tarpaulin), inline(always))]
  const fn from_str(s: &str) -> Option<Self> {
    Some(match s.as_bytes() {
      b"b" => Self::Bold,
      b"i" => Self::Italic,
      b"u" => Self::Underline,
      b"c" => Self::Class,
      b"ruby" => Self::Ruby,
      b"rt" => Self::RubyText,
      b"v" => Self::Voice,
      b"lang" => Self::Lang,
      _ => return None,
    })
  }
}

/// A token emitted by the [`CueParser`] iterator.
///
/// This is the low-level, zero-allocation representation of cue text.
/// Users who need a DOM tree can use [`CueText::parse`] (requires `alloc`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CueToken<'a> {
  /// A run of plain text (entities already decoded).
  ///
  /// If the original text contains no entities, the slice borrows directly
  /// from the input. Otherwise it holds decoded text.
  Text(CueStr<'a>),
  /// A start tag like `<b>`, `<c.classname>`, or `<v Speaker Name>`.
  StartTag {
    /// The tag name.
    tag: Tag,
    /// Dot-separated class names (e.g., `"loud.important"`), empty if none.
    classes: &'a str,
    /// Annotation text (for `<v>` and `<lang>`), `None` if absent.
    annotation: Option<&'a str>,
  },
  /// An end tag like `</b>`.
  EndTag(Tag),
  /// A timestamp tag like `<00:05.000>`.
  Timestamp(crate::vtt::Timestamp),
}

/// Text content that may or may not own its data.
///
/// When the raw cue text contains no entities, this borrows the original
/// slice. When entities are present, the decoded text is stored inline
/// in a stack buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CueStr<'a> {
  /// Borrowed directly from the input (no entities).
  Borrowed(&'a str),
  /// Decoded text with entities replaced.
  #[cfg(any(feature = "alloc", feature = "std"))]
  Owned(std::string::String),
}

impl CueStr<'_> {
  /// Returns the string content.
  pub fn as_str(&self) -> &str {
    match self {
      CueStr::Borrowed(s) => s,
      #[cfg(any(feature = "alloc", feature = "std"))]
      CueStr::Owned(s) => s.as_str(),
    }
  }
}

impl fmt::Display for CueStr<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Try to decode a WebVTT entity reference. Returns the decoded char(s) and
/// the number of bytes consumed (including the leading `&` and trailing `;`).
#[cfg(any(feature = "alloc", feature = "std"))]
fn decode_entity(s: &str) -> Option<(char, usize)> {
  // s starts right after `&`
  let end = s.find(';')?;
  let name = &s[..end];
  let ch = match name {
    "amp" => '&',
    "lt" => '<',
    "gt" => '>',
    "lrm" => '\u{200E}',
    "rlm" => '\u{200F}',
    "nbsp" => '\u{00A0}',
    _ => return None,
  };
  Some((ch, end + 1)) // +1 for the `;`
}

/// A lazy, zero-copy cue text parser.
///
/// Yields [`CueToken`]s from raw WebVTT cue text. This is the low-level
/// parsing layer that does no allocation in `no_std` (text nodes that
/// contain entities will use [`CueStr::Owned`] when `alloc` is available,
/// otherwise entities are passed through as-is).
///
/// # Example
///
/// ```rust
/// use fasrt::vtt::cue_text::{CueParser, CueToken, Tag, CueStr};
///
/// let tokens: Vec<_> = CueParser::new("<b>bold</b>").collect();
/// assert_eq!(tokens.len(), 3);
/// assert!(matches!(&tokens[0], CueToken::StartTag { tag: Tag::Bold, .. }));
/// assert!(matches!(&tokens[1], CueToken::Text(_)));
/// assert!(matches!(&tokens[2], CueToken::EndTag(Tag::Bold)));
/// ```
pub struct CueParser<'a> {
  input: &'a str,
  pos: usize,
}

impl<'a> CueParser<'a> {
  /// Create a new cue text parser for the given raw cue text.
  pub const fn new(input: &'a str) -> Self {
    Self { input, pos: 0 }
  }

  /// Parse a text run up to the next `<` or end of input.
  fn parse_text(&mut self) -> CueToken<'a> {
    let start = self.pos;
    let bytes = self.input.as_bytes();
    let mut has_entity = false;

    let mut i = self.pos;
    while i < bytes.len() {
      if bytes[i] == b'<' {
        break;
      }
      if bytes[i] == b'&' {
        has_entity = true;
      }
      i += 1;
    }
    self.pos = i;

    let raw = &self.input[start..i];

    if !has_entity {
      return CueToken::Text(CueStr::Borrowed(raw));
    }

    // Decode entities
    #[cfg(any(feature = "alloc", feature = "std"))]
    {
      let mut decoded = std::string::String::with_capacity(raw.len());
      let mut j = 0;
      let raw_bytes = raw.as_bytes();
      while j < raw_bytes.len() {
        if raw_bytes[j] == b'&' {
          if let Some((ch, consumed)) = decode_entity(&raw[j + 1..]) {
            decoded.push(ch);
            j += 1 + consumed;
            continue;
          }
        }
        decoded.push(raw[j..].chars().next().unwrap());
        j += raw[j..].chars().next().unwrap().len_utf8();
      }
      CueToken::Text(CueStr::Owned(decoded))
    }

    #[cfg(not(any(feature = "alloc", feature = "std")))]
    {
      CueToken::Text(CueStr::Borrowed(raw))
    }
  }

  /// Parse a tag (everything between `<` and `>`).
  fn parse_tag(&mut self) -> Option<CueToken<'a>> {
    // Skip the `<`
    self.pos += 1;
    let bytes = self.input.as_bytes();

    // Find the closing `>`
    let tag_start = self.pos;
    while self.pos < bytes.len() && bytes[self.pos] != b'>' {
      self.pos += 1;
    }

    let tag_content = &self.input[tag_start..self.pos];

    // Skip the `>`
    if self.pos < bytes.len() {
      self.pos += 1;
    }

    if tag_content.is_empty() {
      return None;
    }

    // End tag: </tagname>
    if let Some(name) = tag_content.strip_prefix('/') {
      let name = name.trim();
      return Tag::from_str(name).map(CueToken::EndTag);
    }

    // Timestamp tag: <HH:MM:SS.mmm> or <MM:SS.mmm>
    if tag_content
      .as_bytes()
      .first()
      .is_some_and(|b| b.is_ascii_digit())
    {
      if let Ok(ts) = super::parse_timestamp(tag_content) {
        return Some(CueToken::Timestamp(ts));
      }
      return None;
    }

    // Start tag: <tagname.class1.class2 annotation>
    // Split on first space/tab to separate tag+classes from annotation
    let (tag_part, annotation) = match tag_content.find([' ', '\t']) {
      Some(idx) => {
        let ann = tag_content[idx + 1..].trim();
        (
          &tag_content[..idx],
          if ann.is_empty() { None } else { Some(ann) },
        )
      }
      None => (tag_content, None),
    };

    // Split tag_part on first `.` to get tag name and classes
    let (tag_name, classes) = match tag_part.find('.') {
      Some(idx) => (&tag_part[..idx], &tag_part[idx + 1..]),
      None => (tag_part, ""),
    };

    Tag::from_str(tag_name).map(|tag| CueToken::StartTag {
      tag,
      classes,
      annotation,
    })
  }
}

impl<'a> Iterator for CueParser<'a> {
  type Item = CueToken<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pos >= self.input.len() {
      return None;
    }

    if self.input.as_bytes()[self.pos] == b'<' {
      // Try to parse a tag; if it fails (unknown tag), skip it and continue
      let saved_pos = self.pos;
      if let Some(token) = self.parse_tag() {
        return Some(token);
      }
      // Unknown tag — if we advanced past it, try next token
      if self.pos > saved_pos && self.pos < self.input.len() {
        return self.next();
      }
      return None;
    }

    Some(self.parse_text())
  }
}

// ── DOM tree (requires alloc) ──────────────────────────────────────────────

#[cfg(any(feature = "alloc", feature = "std"))]
mod tree {
  use super::*;
  use std::vec::Vec;

  /// A node in the cue text DOM tree.
  #[derive(Debug, Clone, PartialEq, Eq)]
  pub enum Node<'a> {
    /// A text node.
    Text(CueStr<'a>),
    /// A timestamp node.
    Timestamp(crate::vtt::Timestamp),
    /// A tag node with children.
    Tag(TagNode<'a>),
  }

  /// A tag node in the cue text DOM tree.
  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct TagNode<'a> {
    /// The tag name.
    pub tag: Tag,
    /// Dot-separated class names, empty if none.
    pub classes: &'a str,
    /// Annotation text (for `<v>` and `<lang>`).
    pub annotation: Option<&'a str>,
    /// Child nodes.
    pub children: Vec<Node<'a>>,
  }

  /// A parsed WebVTT cue text DOM tree.
  ///
  /// Built from a [`CueParser`] token stream. The tree structure follows
  /// the W3C spec's cue text parsing algorithm (§6.4).
  ///
  /// # Example
  ///
  /// ```rust
  /// # #[cfg(any(feature = "alloc", feature = "std"))]
  /// # {
  /// use fasrt::vtt::cue_text::{CueText, Tag, Node, CueStr};
  ///
  /// let tree = CueText::parse("<b>hello</b> world");
  /// assert_eq!(tree.children().len(), 2);
  /// # }
  /// ```
  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct CueText<'a> {
    children: Vec<Node<'a>>,
  }

  impl<'a> CueText<'a> {
    /// Parse raw cue text into a DOM tree.
    pub fn parse(input: &'a str) -> Self {
      let tokens: Vec<_> = CueParser::new(input).collect();
      let mut root_children = Vec::new();
      let mut stack: Vec<TagNode<'a>> = Vec::new();

      for token in tokens {
        match token {
          CueToken::Text(text) => {
            let node = Node::Text(text);
            if let Some(parent) = stack.last_mut() {
              parent.children.push(node);
            } else {
              root_children.push(node);
            }
          }
          CueToken::Timestamp(ts) => {
            let node = Node::Timestamp(ts);
            if let Some(parent) = stack.last_mut() {
              parent.children.push(node);
            } else {
              root_children.push(node);
            }
          }
          CueToken::StartTag {
            tag,
            classes,
            annotation,
          } => {
            stack.push(TagNode {
              tag,
              classes,
              annotation,
              children: Vec::new(),
            });
          }
          CueToken::EndTag(tag) => {
            // Pop until we find the matching tag (or exhaust the stack)
            let mut popped: Vec<TagNode<'a>> = Vec::new();
            let mut matched = None;
            while let Some(node) = stack.pop() {
              if node.tag == tag {
                matched = Some(node);
                break;
              }
              popped.push(node);
            }
            if let Some(node) = matched {
              let completed = Node::Tag(TagNode {
                tag: node.tag,
                classes: node.classes,
                annotation: node.annotation,
                children: node.children,
              });
              let target = stack
                .last_mut()
                .map_or(&mut root_children, |p| &mut p.children);
              for p in popped {
                target.push(Node::Tag(p));
              }
              target.push(completed);
            } else {
              // No matching open tag — push popped nodes to root
              for p in popped {
                root_children.push(Node::Tag(p));
              }
            }
          }
        }
      }

      // Any unclosed tags become root children
      while let Some(node) = stack.pop() {
        let completed = Node::Tag(node);
        if let Some(parent) = stack.last_mut() {
          parent.children.push(completed);
        } else {
          root_children.push(completed);
        }
      }

      Self {
        children: root_children,
      }
    }

    /// Returns the root children of the DOM tree.
    pub fn children(&self) -> &[Node<'a>] {
      &self.children
    }
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
pub use tree::*;
