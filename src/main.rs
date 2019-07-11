use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let buffer = string_inspector::parse_input(&args);
    string_inspector::run(&buffer);
}