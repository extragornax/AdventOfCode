use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Pair {
    pub play: String,
    pub reply: String,
}

const WIN: i64 = 6;
const EQUAL: i64 = 3;
const LOSE: i64 = 0;

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSORS: &str = "C";

const REP_ROCK: &str = "X";
const REP_PAPER: &str = "Y";
const REP_SCISSORS: &str = "Z";

fn get_points(line: Pair) -> i64 {
    let pts = match line.play.as_str() {
        ROCK => match line.reply.as_str() {
            REP_ROCK => EQUAL,
            REP_PAPER => WIN,
            REP_SCISSORS => LOSE,
            _ => 0,
        },
        PAPER => match line.reply.as_str() {
            REP_ROCK => LOSE,
            REP_PAPER => EQUAL,
            REP_SCISSORS => WIN,
            _ => 0,
        },
        SCISSORS => match line.reply.as_str() {
            REP_ROCK => WIN,
            REP_PAPER => LOSE,
            REP_SCISSORS => EQUAL,
            _ => 0,
        },
        _ => 0,
    };

    pts + match line.reply.as_str() {
        REP_ROCK => 1,
        REP_PAPER => 2,
        REP_SCISSORS => 3,
        _ => 0,
    }
}

fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut strat: Vec<Pair> = vec![];
    let split = contents.split("\n");
    for line in split {
        if line.len() == 0 {
            continue;
        } else {
            let mut split = line.split(" ");
            let play = split.next().unwrap();
            let reply = split.next().unwrap();
            strat.push(Pair {
                play: play.to_string(),
                reply: reply.to_string(),
            });
        }
    }

    let pts = strat.into_iter().map(|x| get_points(x)).sum::<i64>();

    println!("{:?}", pts);

    // println!("PART 1: Answer {:?}", elves[0]);
    Ok(())
}

fn part_02() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut strat: Vec<i64> = vec![];
    let split = contents.split("\n");

    // println!("PART 2: Answer {:?}", answer);
    Ok(())
}

fn main() -> std::io::Result<()> {
    if let Ok(_) = part_01() {}
    // if let Ok(_) = part_02() {}
    Ok(())
}
