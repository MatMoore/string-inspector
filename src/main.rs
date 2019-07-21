use std::env;
use terminal_size::{Width, terminal_size};
use encoding::all::UTF_8;
use encoding::all::ISO_8859_1;

fn main() {
    let buffer = string_inspector::parse_input(env::args_os());
    let size = terminal_size();

    if let Some((Width(w), _)) = size {
        let foo = string_inspector::DecodedString::decode(&buffer, UTF_8);
        let bar = string_inspector::DecodedString::decode(&buffer, ISO_8859_1);

        match (foo, bar) {
            (Ok(utf8_decoding), Ok(iso_8859_1_decoding)) => {
                string_inspector::display_decoding(&utf8_decoding, w as usize);
                println!("");
                string_inspector::display_decoding(&iso_8859_1_decoding, w as usize);
            }
            _ => {
                panic!("oh no");
            }
        }
    } else {
        eprintln!("Unable to get terminal size");
    }
}