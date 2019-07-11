//use std::io;
//use std::io::Read;
fn main() {
    //let mut buffer = String::new();
    //io::stdin().read_to_string(&mut buffer).expect("Unable to read from stdin");
    //println!("{}", buffer);
    let buffer = "hello world, here is\t£1";

    for character in buffer.chars() {
        print!("{}", format_utf8_bytes(character));
    }
    println!("");

    for character in buffer.chars() {
        print!("{}", format_character(character));
    }
    println!("");

    println!("{}", buffer);
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