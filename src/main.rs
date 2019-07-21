use terminal_size::{Width, terminal_size};
use std::io;
use std::io::Read;
use std::os::unix::ffi::OsStringExt;
use encoding::types::EncodingRef;
use string_inspector::DecodedString;
use std::borrow::Cow;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("string-inspector")
                          .version("0.0.1")
                          .about("Inspects unicode strings")
                          .arg(Arg::with_name("text")
                              .index(1)
                              .multiple(true))
                          .arg(Arg::with_name("encoding")
                               .short("e")
                               .long("encoding")
                               .value_name("ENCODING")
                               .multiple(true)
                               .number_of_values(1)
                               .possible_values(&["utf8", "latin1"])
                               .default_value("utf8")
                               .help("Encoding to include in the output")
                               .takes_value(true))

                          .get_matches();

    let encodings: Vec<EncodingRef> = matches.values_of("encoding").unwrap()
        .map(|e| encoding::label::encoding_from_whatwg_label(e).unwrap())
        .collect();

    let text = matches.values_of_os("text");

    let buffer = match text {
        Some(args) => {
            let mut arg_bytes: Vec<Vec<u8>> = Vec::new();
            for arg in args {
                arg_bytes.push(arg.to_owned().into_vec());
            }

            arg_bytes.join(&0x20)
        }
        None => {
            eprintln!("No arguments passed to program: reading text from standard input...");
            let mut result: Vec<u8> = Vec::new();
            io::stdin().read_to_end(&mut result).expect("Unable to read from stdin");
            result
        }
    };

    let size = terminal_size().map(|(Width(w), _)| w);
    if size.is_none() {
        eprintln!("Unable to determine terminal size: wrapping output at 80 characters and disabling colors.");
        colored::control::set_override(false);
    }

    let size = size.unwrap_or(80) as usize;

    let results: Vec<Result<DecodedString, Cow<'static, str>>> = encodings.into_iter().map(|encoding| string_inspector::DecodedString::decode(&buffer, encoding)).collect();
    if results.iter().any(|result| result.is_err()) {
        panic!("Unable to interpret input. This is a bug.");
    }

    let mut first = true;
    for result in results.into_iter() {
        if first {
            first = false;
        } else {
            println!("");
        }
        let decoded_string = result.unwrap();
        string_inspector::display_decoding(&decoded_string, size);
    }
}