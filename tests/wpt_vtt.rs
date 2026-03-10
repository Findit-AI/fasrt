//! Tests derived from the W3C WebVTT Web Platform Tests (wpt-file-parsing).
//!
//! These tests mirror the JavaScript assertions in webvtt/wpt-file-parsing/*.js
//! against the corresponding .vtt files. Cue-parsing tests (entities, tags,
//! tree-building) are out of scope for this text-level parser.

#![cfg(any(feature = "std", feature = "alloc"))]

use fasrt::vtt::{Align, Block, Cue, Hour, ParseVttError, Parser, Percentage, Size, Vertical};

/// Helper: collect all blocks (ignoring errors after signature).
fn collect<'a>(input: &'a str) -> Result<Vec<Block<'a, &'a str>>, ParseVttError> {
  Parser::new(input).collect()
}

/// Helper: collect only cues.
fn collect_cues<'a>(input: &'a str) -> Result<Vec<Cue<'a, &'a str>>, ParseVttError> {
  let blocks = collect(input)?;
  Ok(
    blocks
      .into_iter()
      .filter_map(|b| match b {
        Block::Cue(c) => Some(c),
        _ => None,
      })
      .collect(),
  )
}

fn load_vtt(name: &str) -> std::string::String {
  std::fs::read_to_string(std::format!(
    "{}/tests/webvtt/wpt-file-parsing/{name}",
    env!("CARGO_MANIFEST_DIR")
  ))
  .unwrap()
}

// ── arrows.vtt ──────────────────────────────────────────────────────────────

#[test]
fn wpt_arrows() {
  let vtt = load_vtt("arrows.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 6, "expected 6 cues, got {}", cues.len());
  for (i, cue) in cues.iter().enumerate() {
    assert_eq!(
      cue.header_ref().identifier(),
      None,
      "cue {i} should have no identifier (per WPT, id is empty string)"
    );
    assert_eq!(
      *cue.body_ref(),
      std::format!("text{i}"),
      "cue {i} text mismatch"
    );
  }
}

// ── header-garbage.vtt ──────────────────────────────────────────────────────

#[test]
fn wpt_header_garbage() {
  let vtt = load_vtt("header-garbage.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 1);
  assert_eq!(*cues[0].body_ref(), "text");
  assert_eq!(cues[0].header_ref().start().hours(), Hour::new());
  assert_eq!(cues[0].header_ref().end().to_duration().as_secs(), 1);
}

// ── header-space.vtt ────────────────────────────────────────────────────────

#[test]
fn wpt_header_space() {
  let vtt = load_vtt("header-space.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 1);
  assert_eq!(*cues[0].body_ref(), "text");
}

// ── header-tab.vtt ──────────────────────────────────────────────────────────

#[test]
fn wpt_header_tab() {
  let vtt = load_vtt("header-tab.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 1);
  assert_eq!(*cues[0].body_ref(), "text");
}

// ── header-timings.vtt ──────────────────────────────────────────────────────

#[test]
fn wpt_header_timings() {
  // Timing line in header (no blank line after WEBVTT) → still parsed as cue
  let vtt = load_vtt("header-timings.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 1);
  assert_eq!(*cues[0].body_ref(), "text");
  assert_eq!(cues[0].header_ref().start().to_duration().as_millis(), 0);
  assert_eq!(cues[0].header_ref().end().to_duration().as_secs(), 1);
}

// ── ids.vtt ─────────────────────────────────────────────────────────────────

#[test]
fn wpt_ids() {
  let vtt = load_vtt("ids.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 5);
  let expected_ids = [" leading space", "trailing space ", "-- >", "->", " "];
  for (i, expected_id) in expected_ids.iter().enumerate() {
    let id = cues[i]
      .header_ref()
      .identifier()
      .unwrap_or_else(|| panic!("cue {i} should have id"));
    assert_eq!(id.as_str(), *expected_id, "cue {i} id mismatch");
  }
}

// ── newlines.vtt ────────────────────────────────────────────────────────────

#[test]
fn wpt_newlines() {
  let vtt = load_vtt("newlines.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 4);
  let expected = [
    ("cr", "text0"),
    ("lf", "text1"),
    ("crlf", "text2"),
    ("lfcr", "text3"),
  ];
  for (i, (id, text)) in expected.iter().enumerate() {
    let cue_id = cues[i]
      .header_ref()
      .identifier()
      .unwrap_or_else(|| panic!("cue {i} should have id"));
    assert_eq!(cue_id.as_str(), *id, "cue {i} id mismatch");
    assert_eq!(*cues[i].body_ref(), *text, "cue {i} text mismatch");
  }
}

// ── signature tests ─────────────────────────────────────────────────────────

#[test]
fn wpt_signature_bom() {
  let vtt = load_vtt("signature-bom.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 0);
}

#[test]
fn wpt_signature_no_newline() {
  let vtt = load_vtt("signature-no-newline.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 0);
}

#[test]
fn wpt_signature_space_no_newline() {
  let vtt = load_vtt("signature-space-no-newline.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 0);
}

#[test]
fn wpt_signature_space() {
  let vtt = load_vtt("signature-space.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 0);
}

#[test]
fn wpt_signature_tab_no_newline() {
  let vtt = load_vtt("signature-tab-no-newline.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 0);
}

#[test]
fn wpt_signature_tab() {
  let vtt = load_vtt("signature-tab.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 0);
}

#[test]
fn wpt_signature_timings() {
  let vtt = load_vtt("signature-timings.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 0);
}

// ── timings tests ───────────────────────────────────────────────────────────

#[test]
fn wpt_timings_60() {
  let vtt = load_vtt("timings-60.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 2);
  // 60 in minutes/seconds is invalid, but 60 in hours is valid
  assert_eq!(*cues[0].body_ref(), "text1");
  assert_eq!(cues[0].header_ref().start().hours(), Hour::new());
  assert_eq!(cues[0].header_ref().end().hours(), Hour::with(60));

  assert_eq!(*cues[1].body_ref(), "text2");
  assert_eq!(cues[1].header_ref().start().hours(), Hour::with(60));
}

#[test]
fn wpt_timings_eof() {
  let vtt = load_vtt("timings-eof.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 0);
}

#[test]
fn wpt_timings_garbage() {
  let vtt = load_vtt("timings-garbage.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 0);
}

#[test]
fn wpt_timings_negative() {
  // "Negative" means end < start, which is still parsed
  let vtt = load_vtt("timings-negative.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 4);
  let expected = [
    (0, 0),             // 00:00:00.000 --> 00:00:00.000
    (1000, 999),        // 00:00:01.000 --> 00:00:00.999
    (60000, 59999),     // 00:01:00.000 --> 00:00:59.999
    (3600000, 3599999), // 01:00:00.000 --> 00:59:59.999
  ];
  for (i, (start_ms, end_ms)) in expected.iter().enumerate() {
    assert_eq!(
      cues[i].header_ref().start().to_duration().as_millis(),
      *start_ms as u128,
      "cue {i} start mismatch"
    );
    assert_eq!(
      cues[i].header_ref().end().to_duration().as_millis(),
      *end_ms as u128,
      "cue {i} end mismatch"
    );
  }
}

#[test]
fn wpt_timings_omitted_hours() {
  let vtt = load_vtt("timings-omitted-hours.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 3);
  for (i, cue) in cues.iter().enumerate() {
    assert_eq!(*cue.body_ref(), std::format!("text{i}"), "cue {i} text");
    assert_eq!(
      cue.header_ref().start().to_duration().as_millis(),
      0,
      "cue {i} start should be 0"
    );
    assert_eq!(
      cue.header_ref().end().to_duration().as_secs(),
      1,
      "cue {i} end should be 1s"
    );
  }
}

#[test]
fn wpt_timings_too_long() {
  let vtt = load_vtt("timings-too-long.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 2);
  assert_eq!(*cues[0].body_ref(), "text0");
  assert_eq!(*cues[1].body_ref(), "text1");
  for cue in &cues {
    assert_eq!(cue.header_ref().start().to_duration().as_millis(), 0);
    assert_eq!(cue.header_ref().end().to_duration().as_secs(), 1);
  }
}

#[test]
fn wpt_timings_too_short() {
  let vtt = load_vtt("timings-too-short.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 2);
  assert_eq!(*cues[0].body_ref(), "text0");
  assert_eq!(*cues[1].body_ref(), "text1");
  for cue in &cues {
    assert_eq!(cue.header_ref().start().to_duration().as_millis(), 0);
    assert_eq!(cue.header_ref().end().to_duration().as_secs(), 1);
  }
}

// ── settings-align.vtt ──────────────────────────────────────────────────────

#[test]
fn wpt_settings_align() {
  let vtt = load_vtt("settings-align.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 13);

  // Expected align values per the WPT JS assertions.
  // Note: WPT defaults to "center" when no align set. Our parser returns None
  // for missing settings, so we check Some(X) or None accordingly.
  let expected: [Option<Align>; 13] = [
    None,                // cue 0: no setting (WPT default: center)
    Some(Align::Start),  // cue 1: align:start
    Some(Align::Center), // cue 2: align:center
    Some(Align::End),    // cue 3: align:end
    Some(Align::Left),   // cue 4: align:left
    Some(Align::Right),  // cue 5: align:right
    Some(Align::End),    // cue 6: align:start align:end → last wins
    Some(Align::End),    // cue 7: align:end align:CENTER → CENTER invalid
    Some(Align::End),    // cue 8: align:end align: center → "align:" has empty value, stops parsing
    Some(Align::End),    // cue 9: align:end align: → empty value stops parsing
    Some(Align::End),    // cue 10: align:end align:middle → middle invalid
    Some(Align::End),    // cue 11: align:end align → no colon, ignored
    Some(Align::Center), // cue 12: many align:end then align:center at end → center
  ];

  for (i, expected_align) in expected.iter().enumerate() {
    let actual = cues[i].header_ref().settings().and_then(|s| s.align());
    assert_eq!(actual, *expected_align, "cue {i} align mismatch");
  }
}

// ── settings-vertical.vtt ───────────────────────────────────────────────────

#[test]
fn wpt_settings_vertical() {
  let vtt = load_vtt("settings-vertical.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 8);

  // Expected: [None, lr, rl, lr, None, None, None, None]
  // (None = default horizontal, cases 4-7 have invalid values)
  let expected: [Option<Vertical>; 8] = [
    None,
    Some(Vertical::Lr),
    Some(Vertical::Rl),
    Some(Vertical::Lr), // vertical:rl vertical:lr → last wins
    None,               // vertical: → empty value (stops parsing per spec)
    None,               // vertical:RL → case-sensitive
    None,               // vertical: lr → space makes it "vertical:" + " lr" (separate token)
    None,               // vertical:vertical-rl → invalid
  ];

  for (i, expected_v) in expected.iter().enumerate() {
    let actual = cues[i].header_ref().settings().and_then(|s| s.vertical());
    assert_eq!(actual, *expected_v, "cue {i} vertical mismatch");
  }
}

// ── settings-size.vtt ───────────────────────────────────────────────────────

#[test]
fn wpt_settings_size() {
  let vtt = load_vtt("settings-size.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 16);

  // Cues 0-6 have valid sizes, cues 7-15 have invalid/default sizes.

  // cue 0: no size setting
  assert_eq!(
    cues[0]
      .header_ref()
      .settings()
      .and_then(|s| s.size().map(|s| s.value())),
    None
  );
  // cue 1: size:2%
  assert_eq!(
    cues[1]
      .header_ref()
      .settings()
      .and_then(|s| s.size().map(|s| s.value())),
    Some(Percentage::with(2.0))
  );
  // cue 2: size:0%
  assert_eq!(
    cues[2]
      .header_ref()
      .settings()
      .and_then(|s| s.size().map(|s| s.value())),
    Some(Percentage::with(0.0))
  );
  // cue 3: size:00%
  assert_eq!(
    cues[3]
      .header_ref()
      .settings()
      .and_then(|s| s.size().map(|s| s.value())),
    Some(Percentage::with(0.0))
  );
  // cue 4: size:100%
  assert_eq!(
    cues[4]
      .header_ref()
      .settings()
      .and_then(|s| s.size().map(|s| s.value())),
    Some(Percentage::with(100.0))
  );
  // cue 5: size:50%
  assert_eq!(
    cues[5]
      .header_ref()
      .settings()
      .and_then(|s| s.size().map(|s| s.value())),
    Some(Percentage::with(50.0))
  );
  // cue 6: size:1.5%
  assert_eq!(
    cues[6]
      .header_ref()
      .settings()
      .and_then(|s| s.size().map(|s| s.value())),
    Some(Percentage::with(1.5))
  );
}

// ── settings-multiple.vtt ───────────────────────────────────────────────────

#[test]
fn wpt_settings_multiple() {
  let vtt = load_vtt("settings-multiple.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 2);

  // Cue 0: align:start line:1% vertical:lr size:50% position:25%
  let s0 = cues[0].header_ref().settings().unwrap();
  assert_eq!(s0.align(), Some(Align::Start));
  assert_eq!(s0.vertical(), Some(Vertical::Lr));
  assert_eq!(s0.size(), Some(Size::new(Percentage::with(50.0))));
  assert_eq!(
    s0.position().map(|p| p.value()),
    Some(Percentage::with(25.0))
  );

  // Cue 1: align:center line:1 vertical:rl size:0% position:100%
  let s1 = cues[1].header_ref().settings().unwrap();
  assert_eq!(s1.align(), Some(Align::Center));
  assert_eq!(s1.vertical(), Some(Vertical::Rl));
  assert_eq!(s1.size(), Some(Size::new(Percentage::with(0.0))));
  assert_eq!(
    s1.position().map(|p| p.value()),
    Some(Percentage::with(100.0))
  );
}

// ── whitespace-chars.vtt ────────────────────────────────────────────────────

#[test]
fn wpt_whitespace_chars() {
  let vtt = load_vtt("whitespace-chars.vtt");
  let cues = collect_cues(&vtt).unwrap();

  assert_eq!(cues.len(), 3, "expected 3 cues, got {}", cues.len());

  // Cue 0: id=spaces, text="   text0" (leading spaces preserved in body)
  let id0 = cues[0].header_ref().identifier().unwrap();
  assert_eq!(id0.as_str(), "spaces");
  assert_eq!(*cues[0].body_ref(), "   text0");

  // Cue 1: id=tabs, text="text1"
  let id1 = cues[1].header_ref().identifier().unwrap();
  assert_eq!(id1.as_str(), "tabs");
  assert_eq!(*cues[1].body_ref(), "text1");

  // Cue 2: id=form feed, text="text2"
  let id2 = cues[2].header_ref().identifier().unwrap();
  assert_eq!(id2.as_str(), "form feed");
  assert_eq!(*cues[2].body_ref(), "text2");
}

// ── stylesheets.vtt ─────────────────────────────────────────────────────────

#[test]
fn wpt_stylesheets() {
  let vtt = load_vtt("stylesheets.vtt");
  let blocks = collect(&vtt).unwrap();

  // Should have STYLE blocks
  let styles: Vec<_> = blocks
    .iter()
    .filter(|b| matches!(b, Block::Style(_)))
    .collect();
  assert!(!styles.is_empty(), "should have at least one STYLE block");
}

// ── nulls.vtt ───────────────────────────────────────────────────────────────

#[test]
fn wpt_nulls() {
  let vtt = load_vtt("nulls.vtt");
  let cues = collect_cues(&vtt).unwrap();

  // The JS test expects 7 cues. NULL bytes are replaced with U+FFFD in
  // the JS parser (preprocessing step), but our zero-copy parser preserves
  // them as-is. We test cue count.
  assert_eq!(cues.len(), 7, "expected 7 cues, got {}", cues.len());
  // First cue should have text "text0"
  assert_eq!(*cues[0].body_ref(), "text0");
}

// ── regions tests ───────────────────────────────────────────────────────────

#[test]
fn wpt_regions_old() {
  let vtt = load_vtt("regions-old.vtt");
  let cues = collect_cues(&vtt).unwrap();
  // Old region syntax is not recognized → 2 cues, no regions
  assert_eq!(cues.len(), 2);
}

// ── settings-region.vtt ─────────────────────────────────────────────────────

#[test]
fn wpt_settings_region() {
  let vtt = load_vtt("settings-region.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 9);
}

// ── settings-line.vtt ───────────────────────────────────────────────────────

#[test]
fn wpt_settings_line() {
  let vtt = load_vtt("settings-line.vtt");
  let cues = collect_cues(&vtt).unwrap();
  // The JS test expects 46 cues total
  assert_eq!(cues.len(), 46, "expected 46 cues, got {}", cues.len());
}

// ── settings-position.vtt ───────────────────────────────────────────────────

#[test]
fn wpt_settings_position() {
  let vtt = load_vtt("settings-position.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 22, "expected 22 cues, got {}", cues.len());
}

// ── cue-parsing/text.vtt ──────────────────────────────────────────────────────
// Tests cue text extraction at the file-parsing level.
// Entity/tag/tree-building tests are out of scope (renderer concern).

fn load_cue_parsing_vtt(name: &str) -> std::string::String {
  std::fs::read_to_string(std::format!(
    "{}/tests/webvtt/wpt-cue-parsing/{name}",
    env!("CARGO_MANIFEST_DIR")
  ))
  .unwrap()
}

#[test]
fn wpt_cue_parsing_text() {
  let vtt = load_cue_parsing_vtt("text.vtt");
  let cues = collect_cues(&vtt).unwrap();

  // text.json expects 5 cues
  assert_eq!(cues.len(), 5, "expected 5 cues, got {}", cues.len());

  // Cue 0: single line "text"
  assert_eq!(*cues[0].body_ref(), "text");

  // Cue 1: multi-line "text1\ntext2"
  assert_eq!(*cues[1].body_ref(), "text1\ntext2");

  // Cue 2: null byte — our parser preserves raw text (NULL replacement is a
  // preprocessing step per the spec). The file contains "foo\0bar".
  assert_eq!(*cues[2].body_ref(), "foo\u{0}bar");

  // Cue 3: unicode "✓"
  assert_eq!(*cues[3].body_ref(), "✓");

  // Cue 4: blank line splits the cue — only "text1" is in this cue.
  // "text2" after the blank line becomes a non-cue block (no timing).
  assert_eq!(*cues[4].body_ref(), "text1");
}

#[test]
fn wpt_cue_parsing_entities_count() {
  // entities.json has 25 cues. We verify the parser extracts all cues
  // correctly. Entity decoding is a renderer concern.
  let vtt = load_cue_parsing_vtt("entities.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 25, "expected 25 cues, got {}", cues.len());
}

#[test]
fn wpt_cue_parsing_tags_count() {
  // tags.json has 28 cues.
  let vtt = load_cue_parsing_vtt("tags.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 28, "expected 28 cues, got {}", cues.len());
}

#[test]
fn wpt_cue_parsing_timestamps_count() {
  // timestamps.json has 10 cues.
  let vtt = load_cue_parsing_vtt("timestamps.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 10, "expected 10 cues, got {}", cues.len());
}

#[test]
fn wpt_cue_parsing_tree_building_count() {
  // tree-building.json has 9 cues.
  let vtt = load_cue_parsing_vtt("tree-building.vtt");
  let cues = collect_cues(&vtt).unwrap();
  assert_eq!(cues.len(), 9, "expected 9 cues, got {}", cues.len());
}
