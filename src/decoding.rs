//! Things for decoding bytes into strings.
use colored::*;
use std::borrow::Cow;
use encoding::types::EncodingRef;

extern crate encoding;

use encoding::{Encoding, EncoderTrap};

const BYTE_DISPLAY_SIZE: u16 = 3;

/// A logical character that has been decoded from some code points.
#[derive(Debug, Clone)]
pub struct DecodedCharacter {
    pub character: char,
    pub bytes: Vec<u8>
}

impl DecodedCharacter {
    /// The number of columns required to format this character in the output.
    fn width(&self) -> usize {
        self.bytes.len() * BYTE_DISPLAY_SIZE as usize
    }

    /// Convert a raw character into a DecodedCharacter using a particular Encoding.
    ///
    /// # Limitations
    /// It's assumed that `encoding` is the same one used to decode the character.
    /// We use this to reencode the character, in order to work out which code units
    /// within the string actually belong to this character. This allows us to display bytes
    /// and characters/unicode code points side by side. However, if the input is a unicode replacement
    /// character, that means that there were code points in the input which could not be decoded,
    /// and this method won't be able to recover those.
    ///
    /// # Panics
    /// Panics if character is unrepresentable in the provided encoding,
    /// and that encoding cannot encode a unicode replacement character (U+FFFD).
    fn new(character: char, encoding: &dyn Encoding) -> DecodedCharacter {
        let bytes_for_character = encoding.encode(&character.to_string(), EncoderTrap::Replace).unwrap();
        DecodedCharacter { character, bytes: bytes_for_character }
    }

    /// Format the character in an easy to understand way.
    /// ASCII characters are rendered normally.
    /// Tabs, carriage returns and newlines are represented as escape sequences.
    /// All other characters are rendered as their unicode codepoints.
    ///
    /// # Limitations
    /// This is not guaranteed to work properly if the codepoint in hex is longer than the number of
    /// bytes used to represent it in the encoding; for example, latin characters in UTF-16.
    fn format_character(&self) -> String {
        let char_size = self.width();
        let character = self.character;

        match character {
            '\t' | '\r' | '\n' => {
                let escaped = character.escape_default();
                format!("{:width$} ", escaped, width = char_size)
            }
            '\u{20}'...'\u{7e}' => {
                format!("{:width$}", character, width = char_size)
            }
            _ => {
                let codepoint = format!("{:02x} ", character as u32);
                format!("{:width$}", codepoint, width = char_size)
            }
        }
    }

    /// Format the byte representation of the character using hex.
    fn format_bytes(&self) -> String {
        let mut buffer = String::new();
        for byte in self.bytes.iter() {
            let byte_hex = format!("{:02x} ", byte);
            buffer.push_str(&byte_hex)
        }
        buffer
    }
}

// The result of decoding one or more code units
// If there is a decoding error, we capture the first invalid code unit
// and then continue decoding.
// TODO: for UTF-16 a code unit is 2 bytes, not one byte, so this needs to
// be more flexible.
#[derive(Debug, Clone)]
pub enum Atom {
    Character(DecodedCharacter),
    InvalidCodeUnit(u8)
}

impl Atom {
    /// Format the byte representation of the character using hex.
    pub fn format_bytes(&self) -> String {
        match &self {
            Atom::Character(c) => {c.format_bytes()}
            Atom::InvalidCodeUnit(b) => {format!("{:02x} ", b)}
        }
    }

    /// Format the character in an easy to understand way.
    /// ASCII characters are rendered normally.
    /// Tabs, carriage returns and newlines are represented as escape sequences.
    /// All other characters are rendered as their unicode codepoints.
    ///
    /// # Limitations
    /// This is not guaranteed to work properly if the codepoint in hex is longer than the number of
    /// bytes used to represent it in the encoding; for example, latin characters in UTF-16.
    pub fn format_character(&self) -> String {
        match &self {
            Atom::Character(c) => {c.format_character()}
            Atom::InvalidCodeUnit(_) => {format!("\u{FFFD} ")}
        }
    }

    /// Convert to the regular rust char type.
    pub fn to_char(&self) -> char {
        match &self {
            Atom::Character(c) => {c.character}
            Atom::InvalidCodeUnit(_) => {'\u{FFFD}'}
        }
    }

    /// Convert to a vector of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        match &self {
            Atom::Character(c) => {c.bytes.clone()}
            Atom::InvalidCodeUnit(b) => {vec![b.clone()]}
        }
    }

    /// The number of columns required to format this character in the output.
    pub fn width(&self) -> usize {
        match & self {
            Atom::Character(c) => {c.width()}
            Atom::InvalidCodeUnit(_) => {2}
        }
    }
}

/// A string that has been decoded using a particular character encoding.
pub struct DecodedString {
    pub encoding: &'static dyn Encoding,
    pub atoms: Vec<Atom>
}

impl DecodedString {
    /// Decode a sequence of bytes using a particular encoding.
    ///
    /// Any characters that cannot be encoded will be represented using unicode replacement characters (U+FFFD).
    ///
    /// # Errors
    /// Returns an error if anything goes wrong with the underlying decoder. This shouldn't actually happen(?)
    pub fn decode(string: &[u8], encoding: EncodingRef) -> Result<DecodedString, Cow<'static, str>> {
        let mut result = Vec::new();
        let mut decoder = encoding.raw_decoder();
        let mut remaining = string;
        let mut string_writer = String::new();

        loop {
            let (offset, error) = decoder.raw_feed(remaining, &mut string_writer);

            // Consume the processed characters
            let mut new_chars: Vec<Atom> = string_writer.chars().map(|c| Atom::Character(DecodedCharacter::new(c, encoding))).collect();
            result.append(&mut new_chars);
            string_writer.clear();

            if let Some(_) = error {
                // Handle the first unprocessed code unit and shrink the input slice
                result.push(Atom::InvalidCodeUnit(remaining[offset]));
                let next = offset + 1;
                remaining = &remaining[next..];
            } else {
                break;
            }
        }

        Ok(DecodedString {encoding: encoding, atoms: result})
    }

    /// Format the byte representation of the string using hex.
    pub fn format_bytes(&self) -> String {
        self.toggle_color(self.atoms.iter().map(Atom::format_bytes))
    }

    /// Format the string in an easy to understand way.
    /// ASCII characters are rendered normally.
    /// Tabs, carriage returns and newlines are represented as escape sequences.
    /// All other characters are rendered as their unicode codepoints.
    ///
    /// # Limitations
    /// This is not guaranteed to work properly if codepoints in hex are longer than the number of
    /// bytes used to represent it in the encoding; for example, latin characters in UTF-16.
    pub fn format_characters(&self) -> String {
        self.toggle_color(self.atoms.iter().map(Atom::format_character))
    }

    fn toggle_color<I>(&self, iterator: I) -> String
    where I: Iterator<Item = String>
    {
        let mut color_toggle = true;
        let mut buffer = String::new();

        for string in iterator {
            if color_toggle {
                buffer.push_str(&string.green().to_string());
            } else {
                buffer.push_str(&string.blue().to_string());
            }
            color_toggle = !color_toggle;
        }
        buffer
    }

    /// Convert to a regular string.
    pub fn to_string(&self) -> String {
        self.atoms.iter().map(Atom::to_char).collect()
    }

    /// Split into chunks so that the output of [format_bytes](#method.format_bytes) and [format_characters](#method.format_characters)
    /// fit within `max_line_width` characters for each chunk.
    pub fn wrap_lines(&self, max_line_width: usize) -> Vec<DecodedString> {
        let mut lines = Vec::new();
        let mut characters_in_line = Vec::new();
        let mut line_size = 0;

        for character in self.atoms.iter() {
            let char_output_width = character.width();
            if line_size + char_output_width > max_line_width as usize {
                lines.push(DecodedString {atoms: characters_in_line, encoding: self.encoding});
                characters_in_line = Vec::new();
                line_size = 0;
            }

            characters_in_line.push(character.clone());
            line_size += character.width();
        }

        if characters_in_line.len() > 0 {
            lines.push(DecodedString {atoms: characters_in_line, encoding: self.encoding});
        }

        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use encoding::all::UTF_8;

    #[test]
    fn ascii_printables() {
        colored::control::set_override(false);
        let decoding = DecodedString::decode("!aA1".as_bytes(), UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "21 61 41 31 ");
        assert_eq!(decoding.format_characters(), "!  a  A  1  ");
    }

    #[test]
    fn ascii_escapables() {
        colored::control::set_override(false);
        let decoding = DecodedString::decode("\n\r\t".as_bytes(), UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "0a 0d 09 ");
        assert_eq!(decoding.format_characters(), "\\n \\r \\t ");
    }

    #[test]
    fn ascii_non_printables() {
        colored::control::set_override(false);
        let decoding = DecodedString::decode("\u{00}\u{7f}".as_bytes(), UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "00 7f ");
        assert_eq!(decoding.format_characters(), "00 7f ");
    }

    #[test]
    fn extra_latin_letters() {
        colored::control::set_override(false);
        let decoding = DecodedString::decode("éß".as_bytes(), UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "c3 a9 c3 9f ");
        assert_eq!(decoding.format_characters(), "e9    df    ");
    }

    #[test]
    fn overlong_utf8_code_units_are_not_decoded() {
        // The bytes C0 and C1 are not valid in UTF8.
        // The only way you could get these as leading bytes is
        // by padding codeboints below U+007F with leading zeros (overlong encoding)
        // which does not happen in normal UTF8.
        colored::control::set_override(false);
        let decoding = DecodedString::decode(&[0xc0], UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "c0 ");
        assert_eq!(decoding.format_characters(), "\u{FFFD} ");
    }

    #[test]
    fn modified_utf8_null_byte_is_not_decoded_in_utf8() {
        // A special case is "Modified UTF-8" which uses overlong encoding
        // only for U+0000, so that strings can be safely processed by
        // null-terminated string functions. mUTF-8 encodes U+0000 as c0 80.
        // In regular UTF-8, this is completely invalid, because 80 is only
        // meaningful as a continuation byte.
        colored::control::set_override(false);
        let decoding = DecodedString::decode(&[0xc0, 0x80], UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "c0 80 ");
        assert_eq!(decoding.format_characters(), "\u{FFFD} \u{FFFD} ");
    }

    #[test]
    fn cannot_escape_unicode_space_in_utf8() {
        // Leading bytes from F5 to FD are invalid because they would encode points
        // larger than the U+10FFFF limit of Unicode
        colored::control::set_override(false);
        let decoding = DecodedString::decode(&[0xf5, 0x80, 0x80, 0x80], UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "f5 80 80 80 ");
        assert_eq!(decoding.format_characters(), "\u{FFFD} \u{FFFD} \u{FFFD} \u{FFFD} ");
    }

    #[test]
    fn edge_of_unicode_in_utf8() {
        // U+10FFFF is the highest character in unicode. It is in a private use area.
        colored::control::set_override(false);
        let decoding = DecodedString::decode(&[0xF4, 0x8f, 0xBF, 0xBF], UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "f4 8f bf bf ");
        assert_eq!(decoding.format_characters(), "10ffff      ");
    }

    #[test]
    fn just_outside_of_unicode_in_utf8() {
        // A leading byte of F4 may or may not be a valid UTF-8 sequence.
        // The first invalid one corresponds to U+110000, which does not exist.
        colored::control::set_override(false);
        let decoding = DecodedString::decode(&[0xF4, 0x90, 0x80, 0x80], UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "f4 90 80 80 ");
        assert_eq!(decoding.format_characters(), "\u{FFFD} \u{FFFD} \u{FFFD} \u{FFFD} ");
    }

    #[test]
    fn nobody_expects_the_unexpected_continuation_byte() {
        colored::control::set_override(false);
        let decoding = DecodedString::decode(&[0xC2, 0xA3, 0xA3, 0xA3], UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "c2 a3 a3 a3 ");
        assert_eq!(decoding.format_characters(), "a3    \u{FFFD} \u{FFFD} ");
    }

    #[test]
    fn display_width_single_byte() {
        let decoded_character = DecodedCharacter {character: 'a', bytes: "a".as_bytes().to_owned()};
        assert_eq!(decoded_character.width(), 3);
    }

    #[test]
    fn display_width_two_bytes() {
        let decoded_character = DecodedCharacter {character: 'ß', bytes: "ß".as_bytes().to_owned()};
        assert_eq!(decoded_character.width(), 6);
    }

    #[test]
    fn line_wrapping_if_it_fits() {
        colored::control::set_override(false);
        let text = "aaaaa";
        let screen_width = 15;
        let decoding = DecodedString::decode(text.as_bytes(), UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "61 61 61 61 61 ");
        assert_eq!(decoding.format_characters(), "a  a  a  a  a  ");

        let lines = decoding.wrap_lines(screen_width);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].format_bytes(), "61 61 61 61 61 ");
        assert_eq!(lines[0].format_characters(), "a  a  a  a  a  ");
    }

    #[test]
    fn line_wrapping_wraps_to_exact_number_of_lines() {
        colored::control::set_override(false);
        let text = "aaaaabbbbb";
        let screen_width = 15;
        let decoding = DecodedString::decode(text.as_bytes(), UTF_8).unwrap();
        let lines = decoding.wrap_lines(screen_width);

        assert_eq!(lines.len(), 2);

        assert_eq!(lines[0].format_bytes(), "61 61 61 61 61 ");
        assert_eq!(lines[0].format_characters(), "a  a  a  a  a  ");

        assert_eq!(lines[1].format_bytes(), "62 62 62 62 62 ");
        assert_eq!(lines[1].format_characters(), "b  b  b  b  b  ");
    }

    #[test]
    fn line_wrapping_wraps_to_inexact_number_of_lines() {
        colored::control::set_override(false);
        let text = "aaaaabbbbbcc";
        let screen_width = 15;
        let decoding = DecodedString::decode(text.as_bytes(), UTF_8).unwrap();
        let lines = decoding.wrap_lines(screen_width);

        assert_eq!(lines.len(), 3);

        assert_eq!(lines[0].format_bytes(), "61 61 61 61 61 ");
        assert_eq!(lines[0].format_characters(), "a  a  a  a  a  ");

        assert_eq!(lines[1].format_bytes(), "62 62 62 62 62 ");
        assert_eq!(lines[1].format_characters(), "b  b  b  b  b  ");

        assert_eq!(lines[2].format_bytes(), "63 63 ");
        assert_eq!(lines[2].format_characters(), "c  c  ");
    }
}