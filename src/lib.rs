use colored::*;
use std::io;
use std::io::Read;

fn highlight_non_ascii(input: &str) -> String {
    let mut output = String::new();

    for character in input.chars() {
        if character.is_ascii() {
            output.push(character);
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

pub fn parse_input(args: &[String]) -> String {
    if args.len() < 2 {
        eprintln!("No arguments passed to program: reading text from standard input...");
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Unable to read from stdin");
        buffer
    } else {
        args[1..].join(" ")
    }
}

pub fn run(string: &str) {
    let mut color_toggle = true;

    println!("[utf-8]");

    print!(" bytes: ");
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

    print!(" chars: ");
    for character in string.chars() {
        if color_toggle {
            print!("{}", format_character(character).green());
        } else {
            print!("{}", format_character(character).blue());
        }

        color_toggle = !color_toggle;
    }
    println!("");

    print!("output: ");
    println!("{}", highlight_non_ascii(string));
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