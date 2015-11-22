use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

type IOResult<T> = Result<T, io::Error>;

fn rot13(c: char) -> char {
    let base = match c {
        'a'...'z' => 'a' as u8,
        'A'...'Z' => 'A' as u8,
        _ => return c,
    };

    let rotated = ((c as u8) - base + 13) % 26;
    (rotated + base) as char
}

fn rotate_stdin() -> IOResult<String> {
    let mut buffer = String::new();
    try!(io::stdin().read_to_string(&mut buffer));
    Ok(buffer.chars().map(rot13).collect::<String>())
}

fn rotate_file(filename: String) -> IOResult<String> {
    let mut file = try!(File::open(filename));
    let mut contents: Vec<u8> = Vec::new();
    try!(file.read_to_end(&mut contents));

    let encoded_str = String::from_utf8(contents)
                          .unwrap()
                          .chars()
                          .map(rot13)
                          .collect::<String>();
    Ok(encoded_str)
}

fn main() {
    let rotated = env::args()
                      .nth(1)
                      .map_or_else(|| rotate_stdin(), |filename| rotate_file(filename));

    match rotated {
        Ok(text) => println!("{}", text),
        Err(err) => println!("Error: {}", err),
    }
}
