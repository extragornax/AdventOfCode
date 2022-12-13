use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Pair {
    one_s: i64,
    one_e: i64,
    two_s: i64,
    two_e: i64,
}
fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let split = contents.split("\n");
    let mut pairs: Vec<Pair> = vec![];

    for line in split {
        let mut pair = Pair {
            one_s: 0,
            one_e: 0,
            two_s: 0,
            two_e: 0,
        };

        let mut subline = line.split(",");
        let one: &str = subline.next().unwrap();
        let mut parsed_one = one.split("-");
        pair.one_s = parsed_one.next().unwrap().parse::<i64>().unwrap();
        pair.one_e = parsed_one.next().unwrap().parse::<i64>().unwrap();
        let two: &str = subline.next().unwrap();
        let mut parsed_two = two.split("-");
        pair.two_s = parsed_two.next().unwrap().parse::<i64>().unwrap();
        pair.two_e = parsed_two.next().unwrap().parse::<i64>().unwrap();
        pairs.push(pair);
    }

    let count = pairs
        .into_iter()
        .map(|e| {
            if (e.one_s <= e.two_s && e.one_e >= e.two_e)
                || (e.one_s >= e.two_s && e.one_e <= e.two_e)
            {
                1 as i64
            } else if e.one_s >= e.two_s && e.one_e <= e.two_e {
                1 as i64
            } else {
                0 as i64
            }
        })
        .sum::<i64>();

    println!("PART 1: Answer {:?}", count);
    Ok(())
}

fn part_02() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let split = contents.split("\n");
    let mut pairs: Vec<Pair> = vec![];

    for line in split {
        let mut pair = Pair {
            one_s: 0,
            one_e: 0,
            two_s: 0,
            two_e: 0,
        };

        let mut subline = line.split(",");
        let one: &str = subline.next().unwrap();
        let mut parsed_one = one.split("-");
        pair.one_s = parsed_one.next().unwrap().parse::<i64>().unwrap();
        pair.one_e = parsed_one.next().unwrap().parse::<i64>().unwrap();
        let two: &str = subline.next().unwrap();
        let mut parsed_two = two.split("-");
        pair.two_s = parsed_two.next().unwrap().parse::<i64>().unwrap();
        pair.two_e = parsed_two.next().unwrap().parse::<i64>().unwrap();
        pairs.push(pair);
    }

    let count = pairs
        .into_iter()
        .map(|e| {
            if (e.one_s <= e.two_s && e.one_e >= e.two_e)
                || (e.one_s >= e.two_s && e.one_e <= e.two_e)
            {
                1 as i64
            } else if e.one_s >= e.two_s && e.one_e <= e.two_e {
                1 as i64
            } else if e.one_s >= e.two_s && e.one_s <= e.two_e {
                // one_s between two_s and two_e
                1 as i64
            } else if e.one_e >= e.two_s && e.one_e <= e.two_e {
                // one_e between two_s and two_e
                1 as i64
            } else if e.two_s >= e.one_s && e.two_s <= e.one_e {
                // two_s between one_s and one_e
                1 as i64
            } else if e.two_e >= e.one_s && e.two_e <= e.one_e {
                // two_e between one_s and one_e
                1 as i64
            } else {
                0 as i64
            }
        })
        .sum::<i64>();

    println!("PART 2: Answer {:?}", count);
    Ok(())
}

fn main() -> std::io::Result<()> {
    if let Ok(_) = part_01() {}
    if let Ok(_) = part_02() {}
    Ok(())
}
