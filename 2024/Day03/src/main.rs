use regex::Regex;
use aoc_tools::read_file_to_string;

fn step_02() -> std::io::Result<()> {
    let contents = read_file_to_string()?;

    // Regex patterns for `mul`, `do`, and `don't` instructions
    let re_instruction = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut mul_enabled = true;
    let mut sum = 0;
    let mut start_idx = 0;

    while let Some(mat) = re_instruction.find(&contents[start_idx..]) {
        let match_str = mat.as_str();

        if match_str.starts_with("don't") {
            mul_enabled = false;
        } else if match_str.starts_with("do") {
            mul_enabled = true;
        } else {
            if mul_enabled {
                let extracted = &match_str[4..match_str.len() - 1];
                // println!("extracted {extracted}");

                let split = extracted.split(",").collect::<Vec<&str>>();

                let x: i64 = split[0].parse().unwrap();
                let y: i64 = split[1].parse().unwrap();
                sum += x * y;
            }
        }
        start_idx = start_idx + mat.start() + 1;
    }

    println!("Step 02: {sum}");

    Ok(())
}

fn step_01() -> std::io::Result<()> {
    let contents = read_file_to_string()?;

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

    if let Err(e) = step_02() {
        println!("Error: {}", e);
    }
}
