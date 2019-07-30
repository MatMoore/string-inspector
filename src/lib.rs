//! Utilities for inspecting unicode strings.
//!
//! # Usage
//! Use [DecodedString](decoding/struct.DecodedString.html) to wrap a sequence of bytes and a [rust-encoding](https://lifthrasiir.github.io/rust-encoding/) encoding.
//! ```
//! let bytes = [0x41, 0x42, 0x43];
//! let string = string_inspector::DecodedString::decode(&bytes, encoding::all::ISO_8859_2).unwrap();
//!
//! assert_eq!("ABC", string.to_string());
//! assert_eq!("\u{1b}[32mA  \u{1b}[0m\u{1b}[34mB  \u{1b}[0m\u{1b}[32mC  \u{1b}[0m", string.format_characters());
//! assert_eq!("\u{1b}[32m41 \u{1b}[0m\u{1b}[34m42 \u{1b}[0m\u{1b}[32m43 \u{1b}[0m", string.format_bytes());
//! ```
//!
//! [DecodedString](decoding/struct.DecodedString.html) contains a sequence of [Atoms](decoding/enum.Atom.html).
//! Atoms represent either a valid character or an invalid code unit in the original string.
//! ```
//! # let bytes = [0x41, 0x42, 0x43];
//! # let string = string_inspector::DecodedString::decode(&bytes, encoding::all::ISO_8859_2).unwrap();
//! assert_eq!(3, string.atoms.len());
//! ```
//!
//! Atoms can be easily converted to their character representation or byte representation:
//! ```
//! # let bytes = [0x41, 0x42, 0x43];
//! # let string = string_inspector::DecodedString::decode(&bytes, encoding::all::ISO_8859_2).unwrap();
//! assert_eq!('A', string.atoms[0].to_char());
//! assert_eq!(vec![0x41], string.atoms[0].to_bytes());
//! ```
//!
//! The unicode replacement character � (U+FFFD) is used if the input contains invalid code units:
//! ```
//! let bytes = [0x41, 0x42, 0x43, 0xC0];
//! let string = string_inspector::DecodedString::decode(&bytes, encoding::all::UTF_8).unwrap();
//!
//! assert_eq!('\u{FFFD}', string.atoms[3].to_char());
//! assert_eq!(vec![0xC0], string.atoms[3].to_bytes());
//! ```
pub mod cli;
pub mod decoding;

pub use decoding::Atom;
pub use decoding::DecodedCharacter;
pub use decoding::DecodedString;