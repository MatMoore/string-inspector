use std::env;
use terminal_size::{Width, terminal_size};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let buffer = string_inspector::parse_input(&args);
    let size = terminal_size();

    if let Some((Width(w), _)) = size {
        string_inspector::run_with_line_wrapping(&buffer, w);
    } else {
        eprintln!("Unable to get terminal size");
    }
}