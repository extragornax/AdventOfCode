use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut elves: Vec<i64> = vec![];
    let split = contents.split("\n");

    let mut count: i64 = 0;
    for line in split {
        if line.len() == 0 {
            elves.push(count);
            count = 0;
            continue;
        } else {
            count += line.parse::<i64>().unwrap();
        }
    }

    elves.sort();
    elves.reverse();
    println!("Answer {:?}", elves[0]);

    Ok(())
}
