//! Functions for parsing command line input and displaying output.
use colored::*;
use std::io;
use std::io::Read;
use std::os::unix::ffi::OsStringExt;
use encoding::types::EncodingRef;
use clap::{Arg, App};
use crate::decoding::DecodedString;

const LABEL_SIZE: u16 = 7; // "bytes: / chars:" labels

pub fn parse_command_line() -> (Vec<EncodingRef>, Vec<u8>) {
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

    (encodings, buffer)
}

pub fn display_decoding(decoding: &DecodedString, max_line_width: usize) {
    println!("[{}]", decoding.encoding.name());

    let chunks = decoding.wrap_lines(max_line_width - LABEL_SIZE as usize);
    let mut first = true;

    for chunk in chunks.iter() {
        if first {
            first = false;
        } else {
            println!("");
        }

        print!("bytes: ");
        println!("{}", chunk.format_bytes());

        print!("chars: ");
        println!("{}", chunk.format_characters());
    }

    println!("");
    println!("{}", highlight_non_ascii(&decoding.to_string()));
}

pub fn display_decodings(decodings: &Vec<DecodedString>, max_line_width: usize) {
    let mut first = true;
    for decoded_string in decodings.iter() {
        if first {
            first = false;
        } else {
            println!("");
        }
        display_decoding(&decoded_string, max_line_width);
    }
}

fn highlight_non_ascii(input: &str) -> String {
    let mut output = String::new();

    for character in input.chars() {
        if character.is_ascii() {
            output.push_str(&character.to_string().green().to_string());
        } else {
            output.push_str(&character.to_string().red().to_string());
        }
    }

    output
}