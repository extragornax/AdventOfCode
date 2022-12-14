use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Movements {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(stacks_str: &str, mut stacks: Vec<Vec<char>>) -> std::io::Result<Vec<Vec<char>>> {
    for line in stacks_str.lines().rev() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let second = chunk.nth(1).unwrap();
            if second.is_alphabetic() {
                stacks[idx].push(second);
            }
        }
    }

    Ok(stacks)
}

fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let (left, instructions_str) = contents.split_once("\n\n").unwrap();
    let (stacks_str, platforms) = left.rsplit_once('\n').unwrap();
    let num_stacks: usize = platforms
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    stacks = parse_stacks(stacks_str, stacks)?;

    let splitted_instructions = instructions_str.split("\n");

    let instructions: Vec<Movements> = splitted_instructions
        .into_iter()
        .map(|x| {
            let mut splitted = x.split(" ");
            let from: usize;
            let to: usize;
            let amount: usize;

            splitted.next().unwrap();
            amount = splitted.next().unwrap().parse().unwrap();
            splitted.next().unwrap();
            from = splitted.next().unwrap().parse().unwrap();
            splitted.next().unwrap();
            to = splitted.next().unwrap().parse().unwrap();

            Movements {
                amount: amount,
                from: from - 1,
                to: to - 1,
            }
        })
        .collect();

    for instru in instructions {
        for _ in 0..instru.amount {
            let t = stacks[instru.from].pop().unwrap();
            stacks[instru.to].push(t);
        }
    }

    let all_chars: String = stacks
        .into_iter()
        .map(|x| x.clone().pop().unwrap())
        .collect::<Vec<char>>()
        .iter()
        .collect::<String>();

    println!("Part 01: {:?}", all_chars);

    Ok(())
}

fn part_02() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let (left, instructions_str) = contents.split_once("\n\n").unwrap();
    let (stacks_str, platforms) = left.rsplit_once('\n').unwrap();
    let num_stacks: usize = platforms
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    stacks = parse_stacks(stacks_str, stacks)?;

    let splitted_instructions = instructions_str.split("\n");

    let instructions: Vec<Movements> = splitted_instructions
        .into_iter()
        .map(|x| {
            let mut splitted = x.split(" ");
            let from: usize;
            let to: usize;
            let amount: usize;

            splitted.next().unwrap();
            amount = splitted.next().unwrap().parse().unwrap();
            splitted.next().unwrap();
            from = splitted.next().unwrap().parse().unwrap();
            splitted.next().unwrap();
            to = splitted.next().unwrap().parse().unwrap();

            Movements {
                amount: amount,
                from: from - 1,
                to: to - 1,
            }
        })
        .collect();

    for instru in instructions {
        let mut tmp: Vec<char> = vec![];
        for _ in 0..instru.amount {
            tmp.push(stacks[instru.from].pop().unwrap());
        }
        tmp.reverse();
        for t in tmp {
            stacks[instru.to].push(t);
        }
    }

    let all_chars: String = stacks
        .into_iter()
        .map(|x| x.clone().pop().unwrap())
        .collect::<Vec<char>>()
        .iter()
        .collect::<String>();

    println!("Part 02: {:?}", all_chars);

    Ok(())
}

fn main() -> std::io::Result<()> {
    if let Ok(_) = part_01() {}
    if let Ok(_) = part_02() {}
    Ok(())
}
