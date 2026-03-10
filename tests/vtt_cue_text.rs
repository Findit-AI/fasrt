#![cfg(any(feature = "alloc", feature = "std"))]

use fasrt::vtt::cue_text::{CueParser, CueStr, CueText, CueToken, Node, Tag, TagNode};

// ── CueParser (token iterator) tests ────────────────────────────────────────

#[test]
fn parse_plain_text() {
  let tokens: Vec<_> = CueParser::new("hello world").collect();
  assert_eq!(tokens.len(), 1);
  assert_eq!(tokens[0], CueToken::Text(CueStr::Borrowed("hello world")));
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
  assert_eq!(tokens[1], CueToken::Text(CueStr::Borrowed("bold")));
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
  assert_eq!(tokens[1], CueToken::Text(CueStr::Borrowed("base")));
  assert!(matches!(
    &tokens[2],
    CueToken::StartTag {
      tag: Tag::RubyText,
      ..
    }
  ));
  assert_eq!(tokens[3], CueToken::Text(CueStr::Borrowed("text")));
  assert_eq!(tokens[4], CueToken::EndTag(Tag::RubyText));
  assert_eq!(tokens[5], CueToken::EndTag(Tag::Ruby));
}

#[test]
fn parse_timestamp_tag() {
  let tokens: Vec<_> = CueParser::new("text<00:05.000>more").collect();
  assert_eq!(tokens.len(), 3);
  assert_eq!(tokens[0], CueToken::Text(CueStr::Borrowed("text")));
  assert!(matches!(&tokens[1], CueToken::Timestamp(ts) if ts.to_duration().as_secs() == 5));
  assert_eq!(tokens[2], CueToken::Text(CueStr::Borrowed("more")));
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
  assert_eq!(tokens[0].as_text().unwrap(), "a&b<c>d");
}

#[test]
fn parse_entity_nbsp() {
  let tokens: Vec<_> = CueParser::new("hello&nbsp;world").collect();
  assert_eq!(tokens[0].as_text().unwrap(), "hello\u{00A0}world");
}

#[test]
fn parse_entity_lrm_rlm() {
  let tokens: Vec<_> = CueParser::new("a&lrm;b&rlm;c").collect();
  assert_eq!(tokens[0].as_text().unwrap(), "a\u{200E}b\u{200F}c");
}

#[test]
fn parse_unknown_entity_passthrough() {
  let tokens: Vec<_> = CueParser::new("a&unknown;b").collect();
  // Unknown entities are passed through as-is
  let text = tokens[0].as_text().unwrap();
  assert!(text.contains("&unknown;"));
}

#[test]
fn parse_unknown_tag_skipped() {
  let tokens: Vec<_> = CueParser::new("<unknown>text</unknown>").collect();
  // Unknown tags are skipped, text is still emitted
  assert_eq!(tokens.len(), 1);
  assert_eq!(tokens[0], CueToken::Text(CueStr::Borrowed("text")));
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
  assert_eq!(tokens[2], CueToken::Text(CueStr::Borrowed("bold italic")));
  assert_eq!(tokens[3], CueToken::EndTag(Tag::Italic));
  assert_eq!(tokens[4], CueToken::EndTag(Tag::Bold));
}

#[test]
fn parse_empty_input() {
  let tokens: Vec<_> = CueParser::new("").collect();
  assert!(tokens.is_empty());
}

#[test]
fn parse_text_no_entities_is_borrowed() {
  let tokens: Vec<_> = CueParser::new("just text").collect();
  assert!(matches!(&tokens[0], CueToken::Text(CueStr::Borrowed(_))));
}

// ── CueText (DOM tree) tests ────────────────────────────────────────────────

#[test]
fn tree_plain_text() {
  let tree = CueText::parse("hello");
  assert_eq!(tree.children().len(), 1);
  assert!(matches!(&tree.children()[0], Node::Text(t) if t.as_str() == "hello"));
}

#[test]
fn tree_bold_text() {
  let tree = CueText::parse("<b>bold</b>");
  assert_eq!(tree.children().len(), 1);
  match &tree.children()[0] {
    Node::Tag(tag) => {
      assert_eq!(tag.tag, Tag::Bold);
      assert_eq!(tag.children.len(), 1);
      assert!(matches!(&tag.children[0], Node::Text(t) if t.as_str() == "bold"));
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
      assert_eq!(outer.tag, Tag::Bold);
      assert_eq!(outer.children.len(), 1);
      match &outer.children[0] {
        Node::Tag(inner) => {
          assert_eq!(inner.tag, Tag::Italic);
          assert_eq!(inner.children.len(), 1);
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
  assert!(matches!(&tree.children()[0], Node::Text(t) if t.as_str() == "before "));
  assert!(matches!(
    &tree.children()[1],
    Node::Tag(TagNode { tag: Tag::Bold, .. })
  ));
  assert!(matches!(&tree.children()[2], Node::Text(t) if t.as_str() == " after"));
}

#[test]
fn tree_unclosed_tag() {
  let tree = CueText::parse("<b>unclosed");
  assert_eq!(tree.children().len(), 1);
  // Unclosed tag becomes a root child with its text content
  match &tree.children()[0] {
    Node::Tag(tag) => {
      assert_eq!(tag.tag, Tag::Bold);
      assert_eq!(tag.children.len(), 1);
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
      assert_eq!(tag.tag, Tag::Voice);
      assert_eq!(tag.annotation, Some("Speaker"));
      assert_eq!(tag.children.len(), 1);
    }
    _ => panic!("expected tag"),
  }
}

// ── Helper ──────────────────────────────────────────────────────────────────

trait CueTokenExt {
  fn as_text(&self) -> Option<&str>;
}

impl CueTokenExt for CueToken<'_> {
  fn as_text(&self) -> Option<&str> {
    match self {
      CueToken::Text(s) => Some(s.as_str()),
      _ => None,
    }
  }
}
