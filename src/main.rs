use std::env;
use terminal_size::{Width, terminal_size};
use encoding::all::UTF_8;

fn main() {
    let buffer = string_inspector::parse_input(env::args_os());
    let size = terminal_size();

    if let Some((Width(w), _)) = size {
        let input_string = String::from_utf8_lossy(&buffer);
        string_inspector::display_decoding(&input_string, w, UTF_8);

        println!("");
        string_inspector::display_iso_8859_1_encoding(&buffer, w);
    } else {
        eprintln!("Unable to get terminal size");
    }
}