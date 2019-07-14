use std::env;
use terminal_size::{Width, terminal_size};

fn main() {
    let buffer = string_inspector::parse_input(env::args_os());
    let size = terminal_size();

    if let Some((Width(w), _)) = size {
        let input_string = String::from_utf8_lossy(&buffer);
        string_inspector::run_with_line_wrapping(&input_string, w);
    } else {
        eprintln!("Unable to get terminal size");
    }
}