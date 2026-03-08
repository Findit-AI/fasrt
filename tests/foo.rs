// use fasrt::{ParseError, Subtitle, Timestamp, parse, to_string};

// #[test]
// fn parse_basic() {
//   let srt = "\
// 1
// 00:00:01,000 --> 00:00:04,000
// Hello world!

// 2
// 00:00:05,000 --> 00:00:08,000
// Goodbye world!
// ";

//   let subs = parse(srt).unwrap();
//   assert_eq!(subs.len(), 2);

//   assert_eq!(subs[0].index, 1);
//   assert_eq!(subs[0].start, Timestamp::new(0, 0, 1, 0));
//   assert_eq!(subs[0].end, Timestamp::new(0, 0, 4, 0));
//   assert_eq!(subs[0].text, vec!["Hello world!"]);

//   assert_eq!(subs[1].index, 2);
//   assert_eq!(subs[1].start, Timestamp::new(0, 0, 5, 0));
//   assert_eq!(subs[1].end, Timestamp::new(0, 0, 8, 0));
//   assert_eq!(subs[1].text, vec!["Goodbye world!"]);
// }

// #[test]
// fn parse_multiline_text() {
//   let srt = "\
// 1
// 00:00:01,000 --> 00:00:04,000
// Line one
// Line two
// Line three

// ";

//   let subs = parse(srt).unwrap();
//   assert_eq!(subs.len(), 1);
//   assert_eq!(subs[0].text, vec!["Line one", "Line two", "Line three"]);
// }

// #[test]
// fn parse_no_trailing_newline() {
//   let srt = "\
// 1
// 00:00:01,500 --> 00:00:04,000
// Hello!";

//   let subs = parse(srt).unwrap();
//   assert_eq!(subs.len(), 1);
//   assert_eq!(subs[0].text, vec!["Hello!"]);
// }

// #[test]
// fn parse_with_bom() {
//   let srt = "\u{feff}1
// 00:00:01,000 --> 00:00:04,000
// BOM test
// ";

//   let subs = parse(srt).unwrap();
//   assert_eq!(subs.len(), 1);
//   assert_eq!(subs[0].text, vec!["BOM test"]);
// }

// #[test]
// fn parse_dot_separator() {
//   let srt = "\
// 1
// 00:00:01.000 --> 00:00:04.500
// Dot separator
// ";

//   let subs = parse(srt).unwrap();
//   assert_eq!(subs[0].start, Timestamp::new(0, 0, 1, 0));
//   assert_eq!(subs[0].end, Timestamp::new(0, 0, 4, 500));
// }

// #[test]
// fn parse_large_hours() {
//   let srt = "\
// 1
// 100:59:59,999 --> 200:00:00,000
// Large hours
// ";

//   let subs = parse(srt).unwrap();
//   assert_eq!(subs[0].start, Timestamp::new(100, 59, 59, 999));
//   assert_eq!(subs[0].end, Timestamp::new(200, 0, 0, 0));
// }

// #[test]
// fn roundtrip() {
//   let srt = "\
// 1
// 00:00:01,000 --> 00:00:04,000
// Hello world!

// 2
// 00:00:05,000 --> 00:00:08,000
// Goodbye world!
// ";

//   let subs = parse(srt).unwrap();
//   let output = to_string(&subs);
//   let subs2 = parse(&output).unwrap();
//   assert_eq!(subs, subs2);
// }

// #[test]
// fn timestamp_display() {
//   let ts = Timestamp::new(1, 2, 3, 45);
//   assert_eq!(ts.to_string(), "01:02:03,045");
// }

// #[test]
// fn timestamp_millis_roundtrip() {
//   let ts = Timestamp::new(1, 23, 45, 678);
//   let ms = ts.to_millis();
//   assert_eq!(Timestamp::from_millis(ms), ts);
// }

// #[test]
// fn timestamp_ordering() {
//   let a = Timestamp::new(0, 0, 1, 0);
//   let b = Timestamp::new(0, 0, 2, 0);
//   assert!(a < b);
// }

// #[test]
// fn subtitle_display() {
//   let sub = Subtitle {
//     index: 1,
//     start: Timestamp::new(0, 0, 1, 0),
//     end: Timestamp::new(0, 0, 4, 0),
//     text: vec!["Hello!".into()],
//   };
//   let s = sub.to_string();
//   assert_eq!(s, "1\n00:00:01,000 --> 00:00:04,000\nHello!");
// }

// #[test]
// fn parse_empty_input() {
//   let subs = parse("").unwrap();
//   assert!(subs.is_empty());
// }

// #[test]
// fn parse_only_whitespace() {
//   let subs = parse("\n\n\n").unwrap();
//   assert!(subs.is_empty());
// }

// #[test]
// fn parse_error_missing_arrow() {
//   let srt = "\
// 1
// 00:00:01,000 00:00:04,000
// Hello
// ";

//   let err = parse(srt).unwrap_err();
//   assert_eq!(err, ParseError::ExpectedArrow);
// }

// #[test]
// fn parse_many_entries() {
//   let mut srt = String::new();
//   for i in 1..=100 {
//     srt.push_str(&format!(
//       "{i}\n00:00:{s:02},000 --> 00:00:{e:02},000\nLine {i}\n\n",
//       s = (i % 60),
//       e = ((i + 1) % 60),
//     ));
//   }
//   let subs = parse(&srt).unwrap();
//   assert_eq!(subs.len(), 100);
// }
