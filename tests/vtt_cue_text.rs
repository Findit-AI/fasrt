#![cfg(any(feature = "alloc", feature = "std"))]

use fasrt::vtt::cue::{CueParser, CueStr, CueText, CueToken, Node, Tag};

// ── CueParser (token iterator) tests ────────────────────────────────────────

#[test]
fn parse_plain_text() {
  let tokens: Vec<_> = CueParser::new("hello world").collect();
  assert_eq!(tokens.len(), 1);
  assert_eq!(tokens[0], CueToken::Text(CueStr::borrowed("hello world")));
}

#[test]
fn parse_bold_tag() {
  let tokens: Vec<_> = CueParser::new("<b>bold</b>").collect();
  assert_eq!(tokens.len(), 3);
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag {
      tag: Tag::Bold,
      classes,
      annotation: None
    } if classes.is_empty()
  ));
  assert_eq!(tokens[1], CueToken::Text(CueStr::borrowed("bold")));
  assert_eq!(tokens[2], CueToken::EndTag(Tag::Bold));
}

#[test]
fn parse_italic_tag() {
  let tokens: Vec<_> = CueParser::new("<i>italic</i>").collect();
  assert_eq!(tokens.len(), 3);
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag {
      tag: Tag::Italic,
      ..
    }
  ));
  assert_eq!(tokens[2], CueToken::EndTag(Tag::Italic));
}

#[test]
fn parse_underline_tag() {
  let tokens: Vec<_> = CueParser::new("<u>underline</u>").collect();
  assert_eq!(tokens.len(), 3);
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag {
      tag: Tag::Underline,
      ..
    }
  ));
}

#[test]
fn parse_class_with_classes() {
  let tokens: Vec<_> = CueParser::new("<c.loud.important>text</c>").collect();
  assert_eq!(tokens.len(), 3);
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag {
      tag: Tag::Class,
      classes: "loud.important",
      annotation: None,
    }
  ));
}

#[test]
fn parse_voice_tag() {
  let tokens: Vec<_> = CueParser::new("<v Roger Bingham>text</v>").collect();
  assert_eq!(tokens.len(), 3);
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag {
      tag: Tag::Voice,
      annotation: Some("Roger Bingham"),
      ..
    }
  ));
}

#[test]
fn parse_lang_tag() {
  let tokens: Vec<_> = CueParser::new("<lang en>hello</lang>").collect();
  assert_eq!(tokens.len(), 3);
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag {
      tag: Tag::Lang,
      annotation: Some("en"),
      ..
    }
  ));
}

#[test]
fn parse_ruby_tags() {
  let tokens: Vec<_> = CueParser::new("<ruby>base<rt>text</rt></ruby>").collect();
  assert_eq!(tokens.len(), 6);
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag { tag: Tag::Ruby, .. }
  ));
  assert_eq!(tokens[1], CueToken::Text(CueStr::borrowed("base")));
  assert!(matches!(
    &tokens[2],
    CueToken::StartTag {
      tag: Tag::RubyText,
      ..
    }
  ));
  assert_eq!(tokens[3], CueToken::Text(CueStr::borrowed("text")));
  assert_eq!(tokens[4], CueToken::EndTag(Tag::RubyText));
  assert_eq!(tokens[5], CueToken::EndTag(Tag::Ruby));
}

#[test]
fn parse_timestamp_tag() {
  let tokens: Vec<_> = CueParser::new("text<00:05.000>more").collect();
  assert_eq!(tokens.len(), 3);
  assert_eq!(tokens[0], CueToken::Text(CueStr::borrowed("text")));
  assert!(matches!(&tokens[1], CueToken::Timestamp(ts) if ts.to_duration().as_secs() == 5));
  assert_eq!(tokens[2], CueToken::Text(CueStr::borrowed("more")));
}

#[test]
fn parse_timestamp_tag_long_form() {
  let tokens: Vec<_> = CueParser::new("<01:02:03.456>").collect();
  assert_eq!(tokens.len(), 1);
  if let CueToken::Timestamp(ts) = &tokens[0] {
    assert_eq!(ts.to_duration().as_millis(), 3723456);
  } else {
    panic!("expected timestamp");
  }
}

#[test]
fn parse_entities() {
  let tokens: Vec<_> = CueParser::new("a&amp;b&lt;c&gt;d").collect();
  assert_eq!(tokens.len(), 1);
  // Raw text still contains entities
  assert_eq!(tokens[0].as_raw_text().unwrap(), "a&amp;b&lt;c&gt;d");
  // Normalized text has them decoded
  assert_eq!(tokens[0].as_normalized_text().unwrap(), "a&b<c>d");
  // Flag is set
  assert!(tokens[0].requires_normalization());
}

#[test]
fn parse_entity_nbsp() {
  let tokens: Vec<_> = CueParser::new("hello&nbsp;world").collect();
  assert_eq!(
    tokens[0].as_normalized_text().unwrap(),
    "hello\u{00A0}world"
  );
}

#[test]
fn parse_entity_lrm_rlm() {
  let tokens: Vec<_> = CueParser::new("a&lrm;b&rlm;c").collect();
  assert_eq!(
    tokens[0].as_normalized_text().unwrap(),
    "a\u{200E}b\u{200F}c"
  );
}

#[test]
fn parse_unknown_entity_passthrough() {
  let tokens: Vec<_> = CueParser::new("a&unknown;b").collect();
  let text = tokens[0].as_normalized_text().unwrap();
  assert!(text.contains("&unknown;"));
}

#[test]
fn parse_unknown_tag_skipped() {
  let tokens: Vec<_> = CueParser::new("<unknown>text</unknown>").collect();
  // Unknown tags are skipped, text is still emitted
  assert_eq!(tokens.len(), 1);
  assert_eq!(tokens[0], CueToken::Text(CueStr::borrowed("text")));
}

#[test]
fn parse_nested_tags() {
  let tokens: Vec<_> = CueParser::new("<b><i>bold italic</i></b>").collect();
  assert_eq!(tokens.len(), 5);
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag { tag: Tag::Bold, .. }
  ));
  assert!(matches!(
    &tokens[1],
    CueToken::StartTag {
      tag: Tag::Italic,
      ..
    }
  ));
  assert_eq!(tokens[2], CueToken::Text(CueStr::borrowed("bold italic")));
  assert_eq!(tokens[3], CueToken::EndTag(Tag::Italic));
  assert_eq!(tokens[4], CueToken::EndTag(Tag::Bold));
}

#[test]
fn parse_empty_input() {
  let tokens: Vec<_> = CueParser::new("").collect();
  assert!(tokens.is_empty());
}

#[test]
fn parse_text_no_entities_not_normalized() {
  let tokens: Vec<_> = CueParser::new("just text").collect();
  if let CueToken::Text(cue_str) = &tokens[0] {
    assert!(!cue_str.requires_normalization());
  } else {
    panic!("expected text token");
  }
}

#[test]
fn parse_text_with_entities_requires_normalization() {
  let tokens: Vec<_> = CueParser::new("a&amp;b").collect();
  if let CueToken::Text(cue_str) = &tokens[0] {
    assert!(cue_str.requires_normalization());
    assert_eq!(cue_str.as_raw(), "a&amp;b");
    assert_eq!(cue_str.normalize(), "a&b");
  } else {
    panic!("expected text token");
  }
}

#[test]
fn parse_null_requires_normalization() {
  let tokens: Vec<_> = CueParser::new("hello\0world").collect();
  if let CueToken::Text(cue_str) = &tokens[0] {
    assert!(cue_str.requires_normalization());
    assert_eq!(cue_str.normalize(), "hello\u{FFFD}world");
  } else {
    panic!("expected text token");
  }
}

#[test]
fn normalize_is_lazy_and_cached() {
  let tokens: Vec<_> = CueParser::new("a&amp;b").collect();
  if let CueToken::Text(cue_str) = &tokens[0] {
    // First call computes the normalized form
    let first = cue_str.normalize();
    // Second call returns the same cached reference
    let second = cue_str.normalize();
    assert!(core::ptr::eq(first, second));
  } else {
    panic!("expected text token");
  }
}

// ── CueText (DOM tree) tests ────────────────────────────────────────────────

#[test]
fn tree_plain_text() {
  let tree = CueText::parse("hello");
  assert_eq!(tree.children().len(), 1);
  assert!(matches!(&tree.children()[0], Node::Text(t) if t.normalize() == "hello"));
}

#[test]
fn tree_bold_text() {
  let tree = CueText::parse("<b>bold</b>");
  assert_eq!(tree.children().len(), 1);
  match &tree.children()[0] {
    Node::Tag(tag) => {
      assert_eq!(tag.tag(), Tag::Bold);
      assert_eq!(tag.children().len(), 1);
      assert!(matches!(&tag.children()[0], Node::Text(t) if t.normalize() == "bold"));
    }
    _ => panic!("expected tag node"),
  }
}

#[test]
fn tree_nested_tags() {
  let tree = CueText::parse("<b><i>text</i></b>");
  assert_eq!(tree.children().len(), 1);
  match &tree.children()[0] {
    Node::Tag(outer) => {
      assert_eq!(outer.tag(), Tag::Bold);
      assert_eq!(outer.children().len(), 1);
      match &outer.children()[0] {
        Node::Tag(inner) => {
          assert_eq!(inner.tag(), Tag::Italic);
          assert_eq!(inner.children().len(), 1);
        }
        _ => panic!("expected inner tag"),
      }
    }
    _ => panic!("expected outer tag"),
  }
}

#[test]
fn tree_mixed_text_and_tags() {
  let tree = CueText::parse("before <b>bold</b> after");
  assert_eq!(tree.children().len(), 3);
  assert!(matches!(&tree.children()[0], Node::Text(t) if t.normalize() == "before "));
  assert!(matches!(
    &tree.children()[1],
    Node::Tag(t) if t.tag() == Tag::Bold
  ));
  assert!(matches!(&tree.children()[2], Node::Text(t) if t.normalize() == " after"));
}

#[test]
fn tree_unclosed_tag() {
  let tree = CueText::parse("<b>unclosed");
  assert_eq!(tree.children().len(), 1);
  match &tree.children()[0] {
    Node::Tag(tag) => {
      assert_eq!(tag.tag(), Tag::Bold);
      assert_eq!(tag.children().len(), 1);
    }
    _ => panic!("expected tag node"),
  }
}

#[test]
fn tree_with_timestamp() {
  let tree = CueText::parse("text<00:05.000>more");
  assert_eq!(tree.children().len(), 3);
  assert!(matches!(&tree.children()[0], Node::Text(_)));
  assert!(matches!(&tree.children()[1], Node::Timestamp(_)));
  assert!(matches!(&tree.children()[2], Node::Text(_)));
}

#[test]
fn tree_voice_with_annotation() {
  let tree = CueText::parse("<v Speaker>dialogue</v>");
  assert_eq!(tree.children().len(), 1);
  match &tree.children()[0] {
    Node::Tag(tag) => {
      assert_eq!(tag.tag(), Tag::Voice);
      assert_eq!(tag.annotation(), Some("Speaker"));
      assert_eq!(tag.children().len(), 1);
    }
    _ => panic!("expected tag"),
  }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

trait CueTokenExt {
  fn as_raw_text(&self) -> Option<&str>;
  fn as_normalized_text(&self) -> Option<&str>;
  fn requires_normalization(&self) -> bool;
}

impl CueTokenExt for CueToken<'_> {
  fn as_raw_text(&self) -> Option<&str> {
    match self {
      CueToken::Text(s) => Some(s.as_raw()),
      _ => None,
    }
  }
  fn as_normalized_text(&self) -> Option<&str> {
    match self {
      CueToken::Text(s) => Some(s.normalize()),
      _ => None,
    }
  }
  fn requires_normalization(&self) -> bool {
    match self {
      CueToken::Text(s) => s.requires_normalization(),
      _ => false,
    }
  }
}

// ── Numeric character reference tests ─────────────────────────────────────────

#[test]
fn parse_numeric_char_ref_decimal() {
  let tokens: Vec<_> = CueParser::new("&#65;").collect();
  assert_eq!(tokens[0].as_normalized_text().unwrap(), "A");
}

#[test]
fn parse_numeric_char_ref_hex() {
  let tokens: Vec<_> = CueParser::new("&#x41;").collect();
  assert_eq!(tokens[0].as_normalized_text().unwrap(), "A");
}

#[test]
fn parse_numeric_char_ref_hex_uppercase() {
  let tokens: Vec<_> = CueParser::new("&#X42;").collect();
  assert_eq!(tokens[0].as_normalized_text().unwrap(), "B");
}

#[test]
fn parse_numeric_char_ref_null_replaced() {
  let tokens: Vec<_> = CueParser::new("&#0;").collect();
  assert_eq!(tokens[0].as_normalized_text().unwrap(), "\u{FFFD}");
}

#[test]
fn parse_numeric_char_ref_invalid_codepoint() {
  // U+FFFFFF is not a valid Unicode codepoint
  let tokens: Vec<_> = CueParser::new("&#xFFFFFF;").collect();
  assert_eq!(tokens[0].as_normalized_text().unwrap(), "\u{FFFD}");
}

#[test]
fn parse_numeric_char_ref_no_digits() {
  let tokens: Vec<_> = CueParser::new("&#;").collect();
  let text = tokens[0].as_normalized_text().unwrap();
  assert!(text.contains("&#"));
}

#[test]
fn parse_numeric_char_ref_hex_no_digits() {
  let tokens: Vec<_> = CueParser::new("&#x;").collect();
  let text = tokens[0].as_normalized_text().unwrap();
  assert!(text.contains("&#x"));
}

#[test]
fn parse_ampersand_followed_by_non_alnum() {
  let tokens: Vec<_> = CueParser::new("a&!b").collect();
  assert_eq!(tokens[0].as_normalized_text().unwrap(), "a&!b");
}

#[test]
fn parse_trailing_ampersand_hash() {
  let tokens: Vec<_> = CueParser::new("a&#").collect();
  let text = tokens[0].as_normalized_text().unwrap();
  assert!(text.contains("&#"));
}

#[test]
fn parse_numeric_ref_without_semicolon() {
  let tokens: Vec<_> = CueParser::new("&#65x").collect();
  let text = tokens[0].as_normalized_text().unwrap();
  // Should decode &#65 as 'A' and output 'x'
  assert!(text.contains('A'));
}

// ── CueStr Clone and Debug tests ─────────────────────────────────────────────

#[test]
fn cue_str_clone() {
  let tokens: Vec<_> = CueParser::new("a&amp;b").collect();
  if let CueToken::Text(cue_str) = &tokens[0] {
    let cloned = cue_str.clone();
    assert_eq!(cloned.as_raw(), cue_str.as_raw());
    assert_eq!(
      cloned.requires_normalization(),
      cue_str.requires_normalization()
    );
  }
}

#[test]
fn cue_str_debug() {
  let tokens: Vec<_> = CueParser::new("test").collect();
  if let CueToken::Text(cue_str) = &tokens[0] {
    let debug = format!("{:?}", cue_str);
    assert!(debug.contains("CueStr"));
    assert!(debug.contains("test"));
  }
}

// ── Unterminated tag tests ────────────────────────────────────────────────────

#[test]
fn parse_unterminated_bold() {
  let tokens: Vec<_> = CueParser::new("<b").collect();
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag { tag: Tag::Bold, .. }
  ));
}

#[test]
fn parse_unterminated_italic_with_class() {
  let tokens: Vec<_> = CueParser::new("<i.highlight").collect();
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag {
      tag: Tag::Italic,
      ..
    }
  ));
}

#[test]
fn parse_unterminated_voice() {
  let tokens: Vec<_> = CueParser::new("<v Speaker").collect();
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag {
      tag: Tag::Voice,
      annotation: Some("Speaker"),
      ..
    }
  ));
}

#[test]
fn parse_unterminated_ruby() {
  let tokens: Vec<_> = CueParser::new("<ruby").collect();
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag { tag: Tag::Ruby, .. }
  ));
}

#[test]
fn parse_unterminated_rt() {
  let tokens: Vec<_> = CueParser::new("<ruby><rt").collect();
  assert_eq!(tokens.len(), 2);
  assert!(matches!(
    &tokens[1],
    CueToken::StartTag {
      tag: Tag::RubyText,
      ..
    }
  ));
}

#[test]
fn parse_unterminated_lang() {
  let tokens: Vec<_> = CueParser::new("<lang en").collect();
  assert!(matches!(
    &tokens[0],
    CueToken::StartTag {
      tag: Tag::Lang,
      annotation: Some("en"),
      ..
    }
  ));
}

#[test]
fn parse_unterminated_unknown() {
  let tokens: Vec<_> = CueParser::new("<xyz").collect();
  assert!(tokens.is_empty());
}

#[test]
fn parse_unterminated_empty() {
  let tokens: Vec<_> = CueParser::new("<").collect();
  assert!(tokens.is_empty());
}

#[test]
fn parse_unterminated_ruby_wrong_char() {
  let tokens: Vec<_> = CueParser::new("<rubyX").collect();
  assert!(tokens.is_empty());
}

#[test]
fn parse_unterminated_rt_wrong_char() {
  let tokens: Vec<_> = CueParser::new("<rtX").collect();
  assert!(tokens.is_empty());
}

#[test]
fn parse_unterminated_lang_wrong_char() {
  let tokens: Vec<_> = CueParser::new("<langX").collect();
  assert!(tokens.is_empty());
}

// ── CueText DOM tree edge cases ──────────────────────────────────────────────

#[test]
fn tree_rt_outside_ruby_ignored() {
  let tree = CueText::parse("<rt>text</rt>");
  // <rt> outside <ruby> should be ignored per spec
  assert_eq!(tree.children().len(), 1);
  assert!(matches!(&tree.children()[0], Node::Text(t) if t.normalize() == "text"));
}

#[test]
fn tree_end_rt_outside_ruby_ignored() {
  let tree = CueText::parse("text</rt>more");
  // </rt> outside <ruby> should be ignored, text nodes are separate
  assert_eq!(tree.children().len(), 2);
  assert!(matches!(&tree.children()[0], Node::Text(t) if t.normalize() == "text"));
  assert!(matches!(&tree.children()[1], Node::Text(t) if t.normalize() == "more"));
}

#[test]
fn tree_into_children() {
  let tree = CueText::parse("hello");
  let children = tree.into_children();
  assert_eq!(children.len(), 1);
}

// ── Malformed timestamp rejection tests ──────────────────────────────────────
//
// These verify that malformed cue-text timestamp tags are safely rejected
// (treated as unknown tags or skipped) without panicking, even in debug builds.

/// Colons where digits are expected: `<:::.000>` matches the old loose regex
/// but is rejected by the tightened DFA.
#[test]
fn cue_text_rejects_colons_as_digits() {
  let tokens: Vec<_> = CueParser::new("<:::.000>").collect();
  assert!(
    !tokens.iter().any(|t| matches!(t, CueToken::Timestamp(_))),
    "colons-only should not parse as timestamp"
  );
}

/// Minutes out of range: `<99:99.000>` — rejected by the DFA (`[0-5][0-9]`).
#[test]
fn cue_text_rejects_out_of_range_minutes() {
  let tokens: Vec<_> = CueParser::new("<99:99.000>").collect();
  assert!(
    !tokens.iter().any(|t| matches!(t, CueToken::Timestamp(_))),
    "99:99 should not parse as timestamp"
  );
}

/// Seconds out of range: `<00:60.000>` — rejected by the DFA.
#[test]
fn cue_text_rejects_out_of_range_seconds() {
  let tokens: Vec<_> = CueParser::new("<00:60.000>").collect();
  assert!(
    !tokens.iter().any(|t| matches!(t, CueToken::Timestamp(_))),
    "60 seconds should not parse as timestamp"
  );
}

/// Non-digit bytes in hour position: `<ab:00:00.000>`.
#[test]
fn cue_text_rejects_non_digit_hours() {
  let tokens: Vec<_> = CueParser::new("<ab:00:00.000>").collect();
  assert!(
    !tokens.iter().any(|t| matches!(t, CueToken::Timestamp(_))),
    "non-digit hours should not parse as timestamp"
  );
}

/// Empty hour prefix: `<:00:00.000>` — colon where a digit is expected.
#[test]
fn cue_text_rejects_empty_hour_prefix() {
  let tokens: Vec<_> = CueParser::new("<:00:00.000>").collect();
  assert!(
    !tokens.iter().any(|t| matches!(t, CueToken::Timestamp(_))),
    "empty hour prefix should not parse as timestamp"
  );
}

/// Very large hours that would overflow u64: 30-digit hour value.
#[test]
fn cue_text_rejects_overflowing_hours() {
  // 30 digits exceeds u64::MAX (20 digits)
  let tag = format!("<{}:00:00.000>", "9".repeat(30));
  let tokens: Vec<_> = CueParser::new(&tag).collect();
  assert!(
    !tokens.iter().any(|t| matches!(t, CueToken::Timestamp(_))),
    "overflowing hours should not parse as timestamp"
  );
}

/// Unterminated timestamp tag with valid format goes through `parse_timestamp_cue`.
#[test]
fn cue_text_unterminated_valid_timestamp() {
  // `<00:05.000` without closing `>` — handled by try_parse_unterminated
  let tokens: Vec<_> = CueParser::new("<00:05.000").collect();
  assert!(
    tokens.iter().any(|t| matches!(t, CueToken::Timestamp(_))),
    "unterminated but valid timestamp should parse"
  );
}

/// Unterminated timestamp with invalid digits goes through `parse_timestamp_cue`
/// and is safely rejected.
#[test]
fn cue_text_unterminated_invalid_timestamp() {
  let tokens: Vec<_> = CueParser::new("<99:99.000").collect();
  assert!(
    !tokens.iter().any(|t| matches!(t, CueToken::Timestamp(_))),
    "unterminated invalid timestamp should be rejected"
  );
}
