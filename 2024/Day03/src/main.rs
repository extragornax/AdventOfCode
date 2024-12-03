use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

fn step_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // Regex pattern to match valid `mul(X,Y)` instructions.
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let sum: i64 = re
        .captures_iter(contents.as_str())
        .map(|cap| {
            let x: i64 = cap[1].parse().unwrap();
            let y: i64 = cap[2].parse().unwrap();
            x * y
        })
        .sum();

    println!("Step 01: {sum}");

    Ok(())
}

fn main() {
    if let Err(e) = step_01() {
        println!("Error: {}", e);
    }
}
