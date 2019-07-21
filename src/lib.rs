use colored::*;
use std::io;
use std::io::Read;
use std::os::unix::ffi::OsStringExt;
use std::borrow::Cow;

extern crate encoding;

use encoding::{Encoding, DecoderTrap, EncoderTrap};
use encoding::all::ISO_8859_1;

const LABEL_SIZE: u16 = 7; // "bytes: / chars:" labels
const BYTE_DISPLAY_SIZE: u16 = 3;

fn highlight_non_ascii(input: &str) -> String {
    let mut output = String::new();

    for character in input.chars() {
        if character.is_ascii() {
            output.push_str(&character.to_string().green().to_string());
        } else {
            output.push_str(&character.to_string().red().to_string());
        }
    }

    output
}

fn format_bytes(utf8_bytes: &Vec<u8>) -> String {
    let mut buffer = String::new();
    for byte in utf8_bytes {
        let byte_hex = format!("{:02x} ", byte);
        buffer.push_str(&byte_hex)
    }
    buffer
}

fn format_character(character: char, encoding: &Encoding) -> String {
    let char_size = character_display_width(character, encoding);

    match character {
        '\t' | '\r' | '\n' => {
            let escaped = character.escape_default();
            format!("{:width$} ", escaped, width = char_size)
        }
        '\u{20}'...'\u{7e}' => {
            format!("{:width$}", character, width = char_size)
        }
        _ => {
            // TODO: this formatting will break if the codepoint in hex is longer than
            // the byte representation in hex
            let codepoint = format!("{:02x} ", character as u32);
            format!("{:width$}", codepoint, width = char_size)
        }
    }
}

pub fn parse_input(mut args: std::env::ArgsOs) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    if args.len() < 2 {
        eprintln!("No arguments passed to program: reading text from standard input...");
        io::stdin().read_to_end(&mut result).expect("Unable to read from stdin");
    } else {
        args.next();

        let mut arg_bytes: Vec<Vec<u8>> = Vec::new();
        for arg in args {
            arg_bytes.push(arg.to_owned().into_vec());
        }

        result = arg_bytes.join(&0x20);
    }
    result
}

#[derive(Debug, Clone)]
pub struct DecodedCharacter {
    pub character: char,
    pub bytes: Vec<u8>
}

impl DecodedCharacter {
    pub fn width(&self) -> usize {
        self.bytes.len() * BYTE_DISPLAY_SIZE as usize
    }

    fn new(character: char, encoding: &dyn Encoding) -> DecodedCharacter {
        let bytes_for_character = encoding.encode(&character.to_string(), EncoderTrap::Replace).unwrap();
        DecodedCharacter { character, bytes: bytes_for_character }
    }
}

pub struct DecodedString {
    pub encoding: &'static dyn Encoding,
    pub characters: Vec<DecodedCharacter>
}

impl DecodedString {
    pub fn decode(string: &[u8], encoding: &'static dyn Encoding) -> Result<DecodedString, Cow<'static, str>> {
        match encoding.decode(string, DecoderTrap::Replace) {
            Ok(result) => {
                let characters = result.chars().map(|c| DecodedCharacter::new(c, encoding)).collect();
                Ok(DecodedString {
                    encoding: encoding,
                    characters: characters
                })
            },
            Err(msg) => Err(msg)
        }
    }

    // FIXME: simplify these methods
    pub fn format_bytes(&self) -> String {
        format_bytes(&self.characters.iter().flat_map(|c| c.bytes.clone()).collect::<Vec<u8>>())
    }

    pub fn format_characters(&self) -> String {
        self.characters.iter().map(|c| format_character(c.character, self.encoding)).collect::<Vec<String>>().join("")
    }

    pub fn wrap_lines(&self, max_line_width: usize) -> Vec<DecodedString> {
        let mut lines = Vec::new();
        let mut characters_in_line = Vec::new();
        let mut line_size = 0;

        for character in self.characters.iter() {
            let char_output_width = character.width();
            if line_size + char_output_width > max_line_width as usize {
                lines.push(DecodedString {characters: characters_in_line, encoding: self.encoding});
                characters_in_line = Vec::new();
                line_size = 0;
            }

            characters_in_line.push(character.clone());
            line_size += character.width();
        }

        if characters_in_line.len() > 0 {
            lines.push(DecodedString {characters: characters_in_line, encoding: self.encoding});
        }

        lines
    }
}

// TODO
pub fn display_iso_8859_1_encoding(string: &[u8], screen_width: u16) {
    if let Ok(decoded_string) = ISO_8859_1.decode(string, DecoderTrap::Replace) {
        display_decoding(&decoded_string, screen_width, ISO_8859_1);
    } else {
        // TODO
        panic!("Unable to decode ISO_8859_1");
    }
}

fn character_display_width(character: char, encoding: &Encoding) -> usize {
    encoding.encode(&character.to_string(), EncoderTrap::Replace).unwrap().len() * 3
}

pub fn display_decoding(string: &str, width: u16, encoding: &Encoding) {
    let mut buffer = String::new();
    let mut line_length = 0;
    let mut first = true;
    let width = width - LABEL_SIZE;

    println!("[{}]", encoding.name());

    for character in string.chars() {
        let char_output_width = character_display_width(character, encoding);
        if line_length + char_output_width > width as usize {
            if first {
                first = false;
            } else {
                println!("");
            }
            display_decoded_chunk(&buffer, encoding);
            buffer.clear();
            line_length = 0;
        } else {
            line_length += char_output_width;
            buffer.push(character);
        }
    }

    if buffer.len() > 0 {
        if !first {
            println!("");
        }
        display_decoded_chunk(&buffer, encoding);
    }

    println!("");
    println!("{}", highlight_non_ascii(string));
}

pub fn display_decoded_chunk(string: &str, encoding: &Encoding) {
    let mut color_toggle = true;

    print!("bytes: ");
    for character in string.chars() {
        let bytes_for_character = encoding.encode(&character.to_string(), EncoderTrap::Replace).unwrap();
        if color_toggle {
            print!("{}", format_bytes(&bytes_for_character).green());
        } else {
            print!("{}", format_bytes(&bytes_for_character).blue());
        }

        color_toggle = !color_toggle;
    }
    println!("");

    color_toggle = true;

    print!("chars: ");
    for character in string.chars() {
        if color_toggle {
            print!("{}", format_character(character, encoding).green());
        } else {
            print!("{}", format_character(character, encoding).blue());
        }

        color_toggle = !color_toggle;
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;
    use encoding::all::UTF_8;

    #[test]
    fn ascii_printables() {
        let decoding = DecodedString::decode("!aA1".as_bytes(), UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "21 61 41 31 ");
        assert_eq!(decoding.format_characters(), "!  a  A  1  ");
    }

    #[test]
    fn ascii_escapables() {
        let decoding = DecodedString::decode("\n\r\t".as_bytes(), UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "0a 0d 09 ");
        assert_eq!(decoding.format_characters(), "\\n \\r \\t ");
    }

    #[test]
    fn ascii_non_printables() {
        let decoding = DecodedString::decode("\u{00}\u{7f}".as_bytes(), UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "00 7f ");
        assert_eq!(decoding.format_characters(), "00 7f ");
    }

    #[test]
    fn extra_latin_letters() {
        let decoding = DecodedString::decode("éß".as_bytes(), UTF_8).unwrap();
        assert_eq!(decoding.format_bytes(), "c3 a9 c3 9f ");
        assert_eq!(decoding.format_characters(), "e9    df    ");
    }

    #[test]
    fn display_width_single_byte() {
        assert_eq!(character_display_width('a', UTF_8), 3);
    }

    #[test]
    fn display_width_two_bytes() {
        assert_eq!(character_display_width('ß', UTF_8), 6);
    }

    #[test]
    fn line_wrapping_if_it_fits() {
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