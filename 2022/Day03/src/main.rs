use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Rugsack {
    pub com_one: String,
    pub com_two: String,
}

fn letter_to_pts(_letter: char) -> i64 {
    match _letter {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}

fn find_common_three_string(one: String, two: String, three: String) -> i64 {
    let mut this_sack: Option<char> = None;

    for c in one.chars() {
        if this_sack.is_some() {
            break;
        }
        for d in two.chars() {
            if this_sack.is_some() {
                break;
            }
            for e in three.chars() {
                if c == d && d == e {
                    this_sack = Some(c);
                    break;
                }
            }
        }
    }
    match this_sack {
        Some(e) => letter_to_pts(e),
        None => 0,
    }
}

fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let split = contents.split("\n");
    let mut sacks: Vec<Rugsack> = vec![];

    split.for_each(|line| {
        let com_one = &line[0..line.len() / 2];
        let com_two = &line[line.len() / 2..line.len()];
        let sack = Rugsack {
            com_one: com_one.to_string(),
            com_two: com_two.to_string(),
        };
        sacks.push(sack);
    });

    let mut match_letters: Vec<char> = vec![];

    for sack in sacks {
        let mut this_sack: Option<char> = None;

        for c in sack.com_one.chars() {
            if this_sack.is_some() {
                break;
            }
            for d in sack.com_two.chars() {
                if c == d {
                    this_sack = Some(c);
                    break;
                }
            }
        }
        match this_sack {
            Some(e) => match_letters.push(e),
            None => continue,
        }
    }

    let sum = match_letters
        .into_iter()
        .map(|x| letter_to_pts(x))
        .sum::<i64>();

    println!("Part 01: {} ", sum);

    Ok(())
}

#[derive(Clone)]
struct Grouped {
    pub one: Option<String>,
    pub two: Option<String>,
    pub three: Option<String>,
}

fn part_02() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let split = contents.split("\n");
    let mut groups: Vec<Grouped> = vec![];

    let mut current: Grouped = Grouped {
        one: None,
        two: None,
        three: None,
    };
    for line in split {
        if current.one.is_none() {
            current.one = Some(line.to_string());
        } else if current.two.is_none() {
            current.two = Some(line.to_string());
        } else if current.three.is_none() {
            current.three = Some(line.to_string());
        }

        if current.one.is_some() && current.two.is_some() && current.three.is_some() {
            groups.push(current.clone());
            current = Grouped {
                one: None,
                two: None,
                three: None,
            };
        }
    }

    let total = groups
        .into_iter()
        .map(|x| find_common_three_string(x.one.unwrap(), x.two.unwrap(), x.three.unwrap()))
        .sum::<i64>();

    println!("Part 02: {}", total);

    Ok(())
}

fn main() -> std::io::Result<()> {
    if let Ok(_) = part_01() {}
    if let Ok(_) = part_02() {}
    Ok(())
}
