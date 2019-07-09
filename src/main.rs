//use std::io;
//use std::io::Read;

fn main() {
    //let mut buffer = String::new();
    //io::stdin().read_to_string(&mut buffer).expect("Unable to read from stdin");
    //println!("{}", buffer);
    let buffer = "hello world, here is £1";
    let mut byte_buffer = [0; 4];

    for character in buffer.chars() {
        let char_size = character.encode_utf8(&mut byte_buffer).len();
        print!("{:width$}", character, width = char_size * 3);
    }
    println!("");

    for byte in buffer.bytes() {
        print!("{:x} ", byte)
    }
    println!("")
}
