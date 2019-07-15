use colored::*;
use std::io;
use std::io::Read;
use std::os::unix::ffi::OsStringExt;

extern crate encoding;

use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;

const LABEL_SIZE: u16 = 7; // "bytes: / chars:" labels

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

fn format_utf8_bytes(character: char) -> String {
    let mut utf8_bytes = [0; 4];
    let utf8_bytes = character.encode_utf8(&mut utf8_bytes);

    let mut buffer = String::new();
    for byte in utf8_bytes.bytes() {
        let byte_hex = format!("{:02x} ", byte);
        buffer.push_str(&byte_hex)
    }
    buffer
}

fn format_character(character: char) -> String {
    let mut utf8_bytes = [0; 4];
    let char_size = character.encode_utf8(&mut utf8_bytes).len();

    match character {
        '\t' | '\r' | '\n' => {
            let escaped = character.escape_default();
            format!("{:width$} ", escaped, width = char_size * 3)
        }
        '\u{20}'...'\u{7e}' => {
            format!("{:width$}", character, width = char_size * 3)
        }
        _ => {
            // TODO: this formatting will break if the codepoint in hex is longer than
            // the byte representation in hex
            let codepoint = format!("{:02x} ", character as u32);
            format!("{:width$}", codepoint, width = char_size * 3)
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

// TODO
pub fn display_iso_8859_1_encoding(string: &Vec<u8>, screen_width: u16) {
    if let Ok(decoded_string) = ISO_8859_1.decode(string, DecoderTrap::Replace) {
        // TODO: refactor
        // this needs to understand how many bytes each character takes up no matter what the encoding
        display_decoding(&decoded_string, screen_width, ISO_8859_1);
    } else {
        // TODO
        panic!("Unable to decode ISO_8859_1");
    }

}

pub fn display_decoding(string: &str, width: u16, encoding: &Encoding) {
    let mut buffer = String::new();
    let mut line_length = 0;
    let mut first = true;
    let width = width - LABEL_SIZE;

    println!("[{}]", encoding.name());

    for character in string.chars() {
        let mut char_bytes = [0; 4];
        let char_output_width = character.encode_utf8(&mut char_bytes).len() * 3;
        if line_length + char_output_width > width as usize {
            if first {
                first = false;
            } else {
                println!("");
            }
            run(&buffer);
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
        run(&buffer);
    }

    println!("");
    println!("{}", highlight_non_ascii(string));
}

pub fn run(string: &str) {
    let mut color_toggle = true;

    print!("bytes: ");
    for character in string.chars() {
        if color_toggle {
            print!("{}", format_utf8_bytes(character).green());
        } else {
            print!("{}", format_utf8_bytes(character).blue());
        }

        color_toggle = !color_toggle;
    }
    println!("");

    color_toggle = true;

    print!("chars: ");
    for character in string.chars() {
        if color_toggle {
            print!("{}", format_character(character).green());
        } else {
            print!("{}", format_character(character).blue());
        }

        color_toggle = !color_toggle;
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_printables() {
        assert_eq!(format_utf8_bytes('!'), "21 ");
        assert_eq!(format_character('!'), "!  ");

        assert_eq!(format_utf8_bytes('a'), "61 ");
        assert_eq!(format_character('a'), "a  ");

        assert_eq!(format_utf8_bytes('A'), "41 ");
        assert_eq!(format_character('A'), "A  ");

        assert_eq!(format_utf8_bytes('1'), "31 ");
        assert_eq!(format_character('1'), "1  ");
    }

    #[test]
    fn ascii_escapables() {
        assert_eq!(format_utf8_bytes('\n'), "0a ");
        assert_eq!(format_character('\n'), "\\n ");

        assert_eq!(format_utf8_bytes('\r'), "0d ");
        assert_eq!(format_character('\r'), "\\r ");

        assert_eq!(format_utf8_bytes('\t'), "09 ");
        assert_eq!(format_character('\t'), "\\t ");
    }

    #[test]
    fn ascii_non_printables() {
        assert_eq!(format_utf8_bytes('\u{00}'), "00 ");
        assert_eq!(format_character('\u{00}'), "00 ");

        assert_eq!(format_utf8_bytes('\u{7f}'), "7f ");
        assert_eq!(format_character('\u{7f}'), "7f ");
    }

    #[test]
    fn extra_latin_letters() {
        assert_eq!(format_utf8_bytes('é'), "c3 a9 ");
        assert_eq!(format_character('é'),  "e9    ");

        assert_eq!(format_utf8_bytes('ß'), "c3 9f ");
        assert_eq!(format_character('ß'),  "df    ");
    }
}