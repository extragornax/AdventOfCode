use std::fs::File;
use std::io::{BufReader, Read};

fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let split = contents.split("\n");
    let mut total_score = 0;
    for line in split {
        if line.len() == 0 {
            continue;
        }
        let card_num_split = line.split(":");
        let vec_infos = card_num_split.clone().nth(0).clone().unwrap().split(" ").into_iter().map(|s| s).collect::<Vec<&str>>();
        let card_num: i64 = vec_infos[vec_infos.len() - 1].parse::<i64>().unwrap();
        let card_boards = card_num_split.clone().nth(1).unwrap().split("|");

        let winning_cards = card_boards.clone().nth(0).unwrap().to_string();
        winning_cards.trim();

        let mycards_cards = card_boards.clone().nth(1).unwrap().to_string();
        mycards_cards.trim();

        let winning: Vec<i64> = winning_cards.split(" ").into_iter().map(|s| s.parse::<i64>().unwrap_or(-1)).collect::<Vec<i64>>();
        let mycards: Vec<i64> = mycards_cards.split(" ").into_iter().map(|s| s.parse::<i64>().unwrap_or(-1)).collect::<Vec<i64>>();
        let mut score = 0;

        let winning = winning.iter().filter(|c| **c != -1).collect::<Vec<&i64>>().into_iter().map(|c| *c).collect::<Vec<i64>>();

        mycards.iter().for_each(|c| {
            if winning.contains(c) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        });

        // println!("Line: {} - Score: {}", card_num, score);

        total_score += score;
    }

    println!("Total score: {}", total_score);

    Ok(())
}

fn main() {
    if let Ok(_) = part_01() {};
    // if let Ok(_) = part_02() {};
}
