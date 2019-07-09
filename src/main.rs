//use std::io;
//use std::io::Read;

fn main() {
    //let mut buffer = String::new();
    //io::stdin().read_to_string(&mut buffer).expect("Unable to read from stdin");
    //println!("{}", buffer);
    let buffer = "hello world, here is\t£1";
    let mut byte_buffer = [0; 4];

    // First print the raw bytes
    // this should be unambiguous and each one always takes up a fixed width of 3
    for byte in buffer.bytes() {
        print!("{:02x} ", byte)
    }
    println!("");

    // Now the characters. This assumes a single character cannot be more
    // than 3 "characters" wide, which breaks down if we print control characters directly.
    // So we should print escape characters for these.
    // Ascii escapes can fit within 3 chars e.g. "\t " but unicode escapes cannot e.g. "u+feff BOM"
    for character in buffer.chars() {
        let char_size = character.encode_utf8(&mut byte_buffer).len();

        match character {
            '\t' | '\r' | '\n' => {
                print!("{:width$} ", character.escape_default(), width = char_size * 3);
            }
            '\u{20}'...'\u{7e}' => {
                print!("{:width$}", character, width = char_size * 3);
            }
            _ => {
                // TODO: this formatting will break if the codepoint in hex is longer than
                // the byte representation in hex
                let codepoint = format!("{:x}", character as u32);
                print!("{:width$}", codepoint, width = char_size * 3);
            }
        }
    }
    println!("");
    println!("{}", buffer);
}
