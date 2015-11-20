use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

fn rot13(c: char) -> char {
    let base = match c {
        'a'...'z' => 'a' as u8,
        'A'...'Z' => 'A' as u8,
        _ => return c,
    };

    let rotated = ((c as u8) - base + 13) % 26;
    (rotated + base) as char
}

fn rotate_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();
    try!(io::stdin().read_to_string(&mut buffer));
    Ok(buffer.chars().map(rot13).collect::<String>())
}

fn rotate_file(filename: String) -> Result<String, io::Error> {
    let mut file = try!(File::open(filename));
    let mut contents: Vec<u8> = Vec::new();
    let read_bytes = try!(file.read_to_end(&mut contents));
    println!("Read {} bytes", read_bytes);

    let encoded_str = String::from_utf8(contents).unwrap().chars().map(rot13).collect::<String>();
    Ok(encoded_str)
}

fn main() {
    if let Some(input_fn) = env::args().nth(1) {
        let rotated : String = match &input_fn[..] {
            "-" => rotate_stdin().unwrap(),
            _ => rotate_file(input_fn).ok().unwrap()
        };
        println!("{}", rotated); 
    }
}
