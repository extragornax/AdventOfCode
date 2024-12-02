use std::fs::File;
use std::io::{BufReader, Read};

fn file_to_vec() -> std::io::Result<(Vec<i64>, Vec<i64>)> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let (left, right): (Vec<i64>, Vec<i64>) = contents
        .split("\n")
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .map(|line| {
            let res = line
                .split(" ")
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (res[0], res[1])
        })
        .collect();

    Ok((left, right))
}

fn part_02() -> std::io::Result<()> {
    let (left, right): (Vec<i64>, Vec<i64>) = file_to_vec()?;

    let total: i64 = left.iter().map(|l| {
        let count_right = right.iter().filter(|r| r == &l).count() as i64;
        l * count_right
    }).sum();

    println!("Part 02: {}", total);

    Ok(())
}

fn part_01() -> std::io::Result<()> {
    let (mut left, mut right): (Vec<i64>, Vec<i64>) = file_to_vec()?;

    left.sort();
    right.sort();

    let total = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| {
            (a - b).abs()
        })
        .sum::<i64>();

    println!("Part 01: {}", total);

    Ok(())
}

fn main() {
    if let Err(e) = part_01() {
        println!("Failed to read input: {}", e);
    }
    if let Err(e) = part_02() {
        println!("Failed to read input: {}", e);
    }
}
