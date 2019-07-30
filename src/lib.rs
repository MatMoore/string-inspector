//! Utilities for inspecting unicode strings.
//!
//! # Usage
//! Use [DecodedString](decoding/struct.DecodedString.html) to wrap a sequence of bytes and a [rust-encoding](https://lifthrasiir.github.io/rust-encoding/) encoding.
//! ```
//! let bytes = [65, 66, 67];
//! let string = string_inspector::DecodedString::decode(&bytes, encoding::all::ISO_8859_2).unwrap();
//!
//! assert_eq!("ABC", string.to_string());
//! assert_eq!("\u{1b}[32mA  \u{1b}[0m\u{1b}[34mB  \u{1b}[0m\u{1b}[32mC  \u{1b}[0m", string.format_characters());
//! assert_eq!("\u{1b}[32m41 \u{1b}[0m\u{1b}[34m42 \u{1b}[0m\u{1b}[32m43 \u{1b}[0m", string.format_bytes());
//! ```
//!
//! [DecodedString](decoding/struct.DecodedString.html) contains a sequence of [DecodedCharacters](decoding/struct.DecodedCharacter.html), which retain the original byte representation:
//! ```
//! let bytes = [65, 66, 67];
//! let string = string_inspector::DecodedString::decode(&bytes, encoding::all::ISO_8859_2).unwrap();
//!
//! assert_eq!(3, string.characters.len());
//! assert_eq!('A', string.characters[0].to_char());
//! assert_eq!(vec![65], string.characters[0].to_bytes());
//! ```
//!
pub mod cli;
pub mod decoding;

pub use decoding::DecodedUnit;
pub use decoding::DecodedCharacter;
pub use decoding::DecodedString;