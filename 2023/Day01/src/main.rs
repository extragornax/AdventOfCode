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

// fn part_02() -> std::io::Result<()> {
//     let file = File::open("input.txt")?;
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents)?;
//
//     let mut elves: Vec<i64> = vec![];
//     let split = contents.split("\n");
//
//     let mut count: i64 = 0;
//     for line in split {
//         if line.len() == 0 {
//             elves.push(count);
//             count = 0;
//             continue;
//         } else {
//             count += line.parse::<i64>().unwrap();
//         }
//     }
//
//     elves.sort();
//     elves.reverse();
//     let mut answer: i64 = 0;
//     for i in 0..3 {
//         answer += elves[i];
//     }
//
//     println!("PART 2: Answer {:?}", answer);
//     Ok(())
// }

fn main() -> std::io::Result<()> {
    if let Ok(_) = part_01() {}
    // if let Ok(_) = part_02() {}
    Ok(())
}
