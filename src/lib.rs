//! A blazing fast SRT subtitle parser and writer in Rust.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

mod srt;

/// The types module contains the public types used by this crate.
pub mod types;

/// The error types.
pub mod error;

/// Utility functions
pub mod utils;

// use std::format;
// use std::string::String;
// use std::vec::Vec;

// use core::fmt;

// use logos::Logos;

// use srt::Token;

// /// An error that can occur while parsing an SRT file.
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum ParseError {
//   /// Expected a subtitle index number.
//   ExpectedIndex,
//   /// Expected a start timestamp.
//   ExpectedStartTimestamp,
//   /// Expected the "-->" arrow between timestamps.
//   ExpectedArrow,
//   /// Expected an end timestamp.
//   ExpectedEndTimestamp,
//   /// A timestamp string could not be parsed.
//   InvalidTimestamp,
//   /// A lexer error occurred.
//   LexerError,
// }

// impl fmt::Display for ParseError {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     match self {
//       Self::ExpectedIndex => write!(f, "expected subtitle index"),
//       Self::ExpectedStartTimestamp => write!(f, "expected start timestamp"),
//       Self::ExpectedArrow => write!(f, "expected '-->'"),
//       Self::ExpectedEndTimestamp => write!(f, "expected end timestamp"),
//       Self::InvalidTimestamp => write!(f, "invalid timestamp"),
//       Self::LexerError => write!(f, "lexer error"),
//     }
//   }
// }

// impl core::error::Error for ParseError {}

// /// A peekable wrapper around the logos lexer that buffers one token + its slice.
// struct TokenStream<'a> {
//   lexer: logos::Lexer<'a, Token<'a>>,
//   peeked: Option<Option<Result<Token<'a>, ()>>>,
// }

// impl<'a> TokenStream<'a> {
//   fn new(input: &'a str) -> Self {
//     Self {
//       lexer: Token::lexer(input),
//       peeked: None,
//     }
//   }

//   fn next_tok(&mut self) -> Option<Result<Token<'a>, ()>> {
//     if let Some(peeked) = self.peeked.take() {
//       return peeked;
//     }
//     let result = self.lexer.next()?;
//     Some(result)
//   }

//   fn peek(&mut self) -> Option<&Result<Token<'a>, ()>> {
//     if self.peeked.is_none() {
//       let next = self.next_tok();
//       self.peeked = Some(next);
//     }
//     self.peeked.as_ref().unwrap().as_ref()
//   }
// }

// /// Parse an SRT string into a list of [`Subtitle`]s.
// ///
// /// # Errors
// ///
// /// Returns a [`ParseError`] if the input is malformed.
// ///
// /// # Example
// ///
// /// ```
// /// use fasrt::parse;
// ///
// /// let srt = "\
// /// 1
// /// 00:00:01,000 --> 00:00:04,000
// /// Hello world!
// ///
// /// 2
// /// 00:00:05,000 --> 00:00:08,000
// /// Goodbye world!
// /// ";
// ///
// /// let subs = parse(srt).unwrap();
// /// assert_eq!(subs.len(), 2);
// /// assert_eq!(subs[0].text, vec!["Hello world!"]);
// /// assert_eq!(subs[1].text, vec!["Goodbye world!"]);
// /// ```
// pub fn parse(input: &str) -> Result<Vec<Subtitle>, ParseError> {
//   let mut subs = Vec::new();
//   let mut stream = TokenStream::new(input);

//   loop {
//     // Skip blank lines between entries
//     skip_newlines(&mut stream);

//     // Check if there's another entry
//     let Some(result) = stream.next_tok() else {
//       break;
//     };

//     let tok = result.map_err(|_| ParseError::LexerError)?;
//     match tok {
//       Token::Number(number) => {
//         let sub = parse_entry(&mut stream, number)?;
//         subs.push(sub);
//       }
//       _ => return Err(ParseError::ExpectedIndex),
//     }
//   }

//   Ok(subs)
// }

// /// Skip consecutive newline tokens.
// fn skip_newlines(stream: &mut TokenStream<'_>) {
//   while let Some(Ok(Token::Newline)) = stream.peek() {
//     stream.next_tok();
//   }
// }

// /// Parse a single subtitle entry after the index number has been consumed.
// fn parse_entry(stream: &mut TokenStream<'_>, index: u64) -> Result<Subtitle, ParseError> {
//   // Expect newline after index
//   match stream.next_tok() {
//     Some(Ok(Token::Newline)) => {}
//     _ => return Err(ParseError::ExpectedStartTimestamp),
//   }

//   // Start timestamp
//   let start = match stream.next_tok() {
//     Some(Ok(Token::Timestamp(timestamp))) => timestamp,
//     _ => return Err(ParseError::ExpectedStartTimestamp),
//   };

//   // Arrow
//   match stream.next_tok() {
//     Some(Ok(Token::Arrow)) => {}
//     _ => return Err(ParseError::ExpectedArrow),
//   }

//   // End timestamp
//   let end = match stream.next_tok() {
//     Some(Ok(Token::Timestamp(timestamp))) => timestamp,
//     _ => return Err(ParseError::ExpectedEndTimestamp),
//   };

//   // Consume the newline after the timecode line
//   match stream.next_tok() {
//     Some(Ok(Token::Newline)) => {}
//     None => {
//       return Ok(Subtitle {
//         index,
//         start,
//         end,
//         text: Vec::new(),
//       });
//     }
//     _ => return Err(ParseError::LexerError),
//   }

//   // Collect text lines until blank line (double newline) or EOF
//   let mut text: Vec<String> = Vec::new();
//   let mut current_line = String::new();

//   loop {
//     match stream.peek() {
//       None => {
//         if !current_line.is_empty() {
//           text.push(current_line);
//         }
//         break;
//       }
//       Some(&Ok(Token::Newline)) => {
//         stream.next_tok();
//         if current_line.is_empty() {
//           // Blank line or second newline in a row → end of subtitle
//           break;
//         }
//         text.push(core::mem::take(&mut current_line));
//       }
//       Some(&Ok(Token::Text(_))) => {
//         let (_, slice) = stream.next_tok().unwrap();
//         if !current_line.is_empty() {
//           current_line.push(' ');
//         }
//         current_line.push_str(slice);
//       }
//       Some(&Ok(Token::Number(number))) => {
//         let (_, slice) = stream.next_tok().unwrap();
//         if !current_line.is_empty() {
//           current_line.push(' ');
//         }
//         current_line.push_str(slice);
//       }
//       Some(&Ok(Token::Timestamp(timestamp))) => {
//         let (_, slice) = stream.next_tok().unwrap();
//         if !current_line.is_empty() {
//           current_line.push(' ');
//         }
//         current_line.push_str(slice);
//       }
//       Some(&Ok(Token::Arrow)) => {
//         stream.next_tok();
//         if !current_line.is_empty() {
//           current_line.push(' ');
//         }
//         current_line.push_str("-->");
//       }
//       Some(&Err(_)) => {
//         stream.next_tok();
//         return Err(ParseError::LexerError);
//       }
//     }
//   }

//   Ok(Subtitle {
//     index,
//     start,
//     end,
//     text,
//   })
// }

// /// Serialize a slice of [`Subtitle`]s back to an SRT string.
// ///
// /// # Example
// ///
// /// ```
// /// use fasrt::{Subtitle, Timestamp, to_string};
// ///
// /// let subs = vec![
// ///   Subtitle {
// ///     index: 1,
// ///     start: Timestamp::new(0, 0, 1, 0),
// ///     end: Timestamp::new(0, 0, 4, 0),
// ///     text: vec!["Hello!".into()],
// ///   },
// /// ];
// /// let out = to_string(&subs);
// /// assert!(out.contains("00:00:01,000 --> 00:00:04,000"));
// /// ```
// pub fn to_string(subs: &[Subtitle]) -> String {
//   let mut out = String::new();
//   for (i, sub) in subs.iter().enumerate() {
//     if i > 0 {
//       out.push('\n');
//     }
//     out.push_str(&format!("{sub}"));
//     out.push('\n');
//   }
//   out
// }
