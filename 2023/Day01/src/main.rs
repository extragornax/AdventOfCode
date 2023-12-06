use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn is_num(c: u8) -> bool {
    match c {
        b'0'..=b'9' => true,
        _ => false,
    }
}

fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut numbers: Vec<i64> = vec![];
    let split = contents.split("\n");

    let mut count: i64 = 0;
    for line in split {
        let mut one: Option<char> = None;
        let mut two: Option<char> = None;
        line.as_bytes().into_iter().for_each(|c| {
            if !is_num(*c) {
                return;
            }
            if one.is_none() {
                one = Some(char::from(*c));
            }
            two = Some(char::from(*c));
        });

        if one.is_some() && two.is_some() {
            let num = format!("{}{}", one.unwrap(), two.unwrap()).parse::<i64>().unwrap();
            numbers.push(num);
        }
    }
    let total = numbers.iter().sum::<i64>();
    println!("PART 1: Answer {:?}", total);
    Ok(())
}

fn replace_number_in_str(mut tochange: String) -> String {
    let numbers = vec![("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];
    for n in numbers {
        tochange = tochange.replace(n.0, n.1);
    }
    tochange
}

fn check_starts_with_number(tocheck: String) -> Option<(String, String)> {
    let numbers = vec![("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];
    for n in numbers {
        if tocheck.starts_with(n.0) {
            return Some((n.0.to_string(), n.1.to_string()));
        }
    }
    None
}

fn replace_if_starts_with(tochange: String) -> String {
    if let Some(n) = check_starts_with_number(tochange.clone()) {
        return tochange.replacen(&n.0, &n.1, 1);
    }
    return tochange;
}

/*
    55189 -> Too low
    54458 -> Too low
 */
fn part_02() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut numbers: Vec<i64> = vec![];
    let split = contents.split("\n");

    let mut count: i64 = 0;
    for line in split {
        let tmp = line.clone();
        // let line = replace_number_in_str(tmp.to_string());
        let mut one: Option<char> = None;
        let mut two: Option<char> = None;
        let line = replace_if_starts_with(line.to_string());

        let mut line = line;

        while line.len() > 0 {
            line = replace_if_starts_with(line.to_string());
            let c = line.remove(0).to_string();
            let c = c.as_bytes()[0];
            if !is_num(c) {
                continue;
            }
            if one.is_none() {
                one = Some(char::from(c));
            }
            two = Some(char::from(c));
        }

        if one.is_some() && two.is_some() {
            let numtoparse = format!("{}{}", one.unwrap(), two.unwrap());
            let num = numtoparse.clone().parse::<i64>().unwrap();
            numbers.push(num);

            // println!("from {tmp} to {line} into {num} -> {numtoparse}");
        }
    }
    let total = numbers.iter().sum::<i64>();
    println!("PART 2: Answer {:?}", total);
    Ok(())
}

fn main() -> std::io::Result<()> {
    if let Ok(_) = part_01() {}
    if let Ok(_) = part_02() {}
    Ok(())
}
