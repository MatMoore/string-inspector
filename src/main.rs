use terminal_size::{Width, terminal_size};

use string_inspector::DecodedString;
use std::borrow::Cow;

extern crate clap;

fn main() {
    let (encodings, buffer) = string_inspector::cli::parse_command_line();

    let size = terminal_size().map(|(Width(w), _)| w);
    if size.is_none() {
        eprintln!("Unable to determine terminal size: wrapping output at 80 characters and disabling colors.");
        colored::control::set_override(false);
    }

    let size = size.unwrap_or(80) as usize;

    let results: Vec<Result<DecodedString, Cow<'static, str>>> = encodings.into_iter().map(|encoding| DecodedString::decode(&buffer, encoding)).collect();
    if results.iter().any(|result| result.is_err()) {
        panic!("Unable to interpret input. This is a bug.");
    }

    let decodings = results.into_iter().map(|result| result.unwrap()).collect();

    string_inspector::cli::display_decodings(&decodings, size)
}