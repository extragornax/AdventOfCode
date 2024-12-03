use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_file_to_string() -> Result<String, std::io::Error> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}