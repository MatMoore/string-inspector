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
            let codepoint = format!("{:x}", character as u32);
            format!("{:width$}", codepoint, width = char_size * 3)
        }
    }
}