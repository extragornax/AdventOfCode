use std::fs::File;
use std::io::{BufReader, Read};

fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let split = contents.split("\n");

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut vec_err_index: Vec<i64> = vec![];
    let mut all_index: Vec<i64> = vec![];
    for line in split {
        let game_split = line.split(":");
        let game_num = game_split.clone().nth(0).unwrap_or("");
        if game_num == "" {
            continue;
        }

        let game_num = game_num.split(" ").nth(1).unwrap().parse::<i64>().unwrap();
        all_index.push(game_num);
        let games = game_split.clone().nth(1).unwrap().split(";");
        for game in games {
            let game_vec = game.split(", ");
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for g in game_vec.clone() {
                let g = g.trim();
                let g_split = g.split(" ");
                match g_split.clone().nth(1) {
                    Some("red") => {
                        red = g_split.clone().nth(0).unwrap().parse::<i64>().unwrap();
                    }
                    Some("green") => {
                        green = g_split.clone().nth(0).unwrap().parse::<i64>().unwrap();
                    }
                    Some("blue") => {
                        blue = g_split.clone().nth(0).unwrap().parse::<i64>().unwrap();
                    }
                    _ => {}
                }
            }
            if red > max_red || green > max_green || blue > max_blue {
                if vec_err_index.contains(&game_num) {
                    continue;
                }
                vec_err_index.push(game_num);
            }
        }
    }

    println!("Part 01: {:?}", all_index.iter().sum::<i64>() - vec_err_index.iter().sum::<i64>());

    Ok(())
}

fn part_02() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let split = contents.split("\n");

    let mut totals: Vec<i64> = vec![];

    for line in split {
        let game_split = line.split(":");
        let game_num = game_split.clone().nth(0).unwrap_or("");
        if game_num == "" {
            continue;
        }

        let games = game_split.clone().nth(1).unwrap().split(";");

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for game in games {
            let game_vec = game.split(", ");
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for g in game_vec.clone() {
                let g = g.trim();
                let g_split = g.split(" ");
                match g_split.clone().nth(1) {
                    Some("red") => {
                        red = g_split.clone().nth(0).unwrap().parse::<i64>().unwrap();
                    }
                    Some("green") => {
                        green = g_split.clone().nth(0).unwrap().parse::<i64>().unwrap();
                    }
                    Some("blue") => {
                        blue = g_split.clone().nth(0).unwrap().parse::<i64>().unwrap();
                    }
                    _ => {}
                }
            }
            if red > min_red {
                min_red = red;
            }
            if green > min_green {
                min_green = green;
            }
            if blue > min_blue {
                min_blue = blue;
            }
        }
        totals.push(min_red * min_green * min_blue);
    }

    println!("Part 02: {:?}", totals.iter().sum::<i64>());

    Ok(())
}

fn main() {
    if let Ok(_) = part_01() {}
    if let Ok(_) = part_02() {}
}
