use array_tool::vec::Intersect;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn check_if_no_dup(_data: Vec<char>, _len: usize) -> bool {
    if _data.len() != _len {
        false
    } else {
        let t = _data.clone().intersect(_data.clone());
        if t.len() != _data.len() {
            false
        } else {
            true
        }
    }
}

fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut cur: Vec<char> = Vec::new();
    for (i, letter) in contents.chars().enumerate() {
        if cur.len() < 4 {
            cur.push(letter);
            if cur.len() == 4 {
                if check_if_no_dup(cur.clone(), 4) {
                    println!(
                        "Part 01: index {} {:?}",
                        i + 1,
                        cur.iter().collect::<String>()
                    );
                    return Ok(());
                }
            }
        } else {
            cur.remove(0);
            cur.push(letter);
            if check_if_no_dup(cur.clone(), 4) {
                println!(
                    "Part 01: index {} {:?}",
                    i + 1,
                    cur.iter().collect::<String>()
                );
                return Ok(());
            }
        }
    }

    println!("Part 01: nothing found");

    Ok(())
}

fn part_02() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut cur: Vec<char> = Vec::new();
    for (i, letter) in contents.chars().enumerate() {
        if cur.len() < 14 {
            cur.push(letter);
            if cur.len() == 14 {
                if check_if_no_dup(cur.clone(), 14) {
                    println!(
                        "Part 02: index {} {:?}",
                        i + 1,
                        cur.iter().collect::<String>()
                    );
                    return Ok(());
                }
            }
        } else {
            cur.remove(0);
            cur.push(letter);
            if check_if_no_dup(cur.clone(), 14) {
                println!(
                    "Part 02: index {} {:?}",
                    i + 1,
                    cur.iter().collect::<String>()
                );
                return Ok(());
            }
        }
    }

    println!("Part 02: nothing found");

    Ok(())
}

fn main() {
    if let Ok(_) = part_01() {};
    if let Ok(_) = part_02() {};
}
