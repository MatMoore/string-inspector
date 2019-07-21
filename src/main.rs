use std::env;
use terminal_size::{Width, terminal_size};
use encoding::all::UTF_8;
use encoding::all::ISO_8859_1;

fn main() {
    let buffer = string_inspector::parse_input(env::args_os());
    let size = terminal_size().map(|(Width(w), _)| w);
    if size.is_none() {
        eprintln!("Unable to determine terminal size: wrapping output at 80 characters and disabling colors.");
        colored::control::set_override(false);
    }

    let size = size.unwrap_or(80) as usize;

    let foo = string_inspector::DecodedString::decode(&buffer, UTF_8);
    let bar = string_inspector::DecodedString::decode(&buffer, ISO_8859_1);

    match (foo, bar) {
        (Ok(utf8_decoding), Ok(iso_8859_1_decoding)) => {
            string_inspector::display_decoding(&utf8_decoding, size);
            println!("");
            string_inspector::display_decoding(&iso_8859_1_decoding, size);
        }
        _ => {
            panic!("Unable to interpret input. This is a bug.");
        }
    }
}