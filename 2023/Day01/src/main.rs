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
    53815 -> Incorrect
 */
// Version couting eightwo as 2 (last digit)
// fn part_02() -> std::io::Result<()> {
//     let file = File::open("input.txt")?;
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents)?;
//
//     let mut numbers: Vec<i64> = vec![];
//     let split = contents.split("\n");
//
//     for line in split {
//         if line.len() == 0 {
//             continue;
//         }
//
//         let line_bkp = line.clone();
//
//         let mut tmp_line = line.clone().to_string();
//         let mut number_lst: Vec<char> = vec![];
//
//         let line_len = tmp_line.len() - 1;
//
//         for ind in 0..line_len {
//             let _t: Option<(String, String)> = check_starts_with_number(tmp_line.clone());
//             if _t.is_some() {
//                 number_lst.push(_t.unwrap().1.chars().next().unwrap());
//                 tmp_line.remove(0);
//             } else {
//                 let pop = tmp_line.remove(0);
//                 if is_num(pop as u8) {
//                     number_lst.push(pop);
//                 }
//             }
//         }
//
//         let numtoparse = match number_lst.len() {
//             0 => { String::from("0") }
//             _ => {
//                 let one = number_lst.remove(0);
//                 let mut two = number_lst.pop();
//                 if two.is_none() {
//                     two = Some(one);
//                 }
//                 format!("{}{}", one, two.unwrap())
//             }
//         };
//
//         println!("from {:?} to {:?}", line_bkp, numtoparse);
//         let num = numtoparse.clone().parse::<i64>().unwrap();
//         numbers.push(num);
//     }
//     let total = numbers.iter().sum::<i64>();
//     let toolow = 55189;
//
//     println!("PART 2: Answer {:?} -> is high enough <{}> ({})", total, total > toolow, toolow);
//     Ok(())
// }


/*
    55189 -> Too low
    54458 -> Too low
    53815 -> Incorrect
    8448139 -> Incorrect
 */
fn part_02() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut numbers: Vec<i64> = vec![];
    let split = contents.split("\n");
    let mut sum = 0;

    for line in split {
        let mut digits = String::new();
        let mut is_digit = false;

        for c in line.chars() {
            if c.is_digit(10) {
                digits.push(c);
                is_digit = true;
            } else if c.is_alphabetic() && is_digit {
                // If the current character is alphabetic and we were parsing a digit,
                // it might be a spelled-out digit
                let digit_value = match digits.as_str() {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => 0, // Default to 0 if not a spelled-out digit
                };
                sum += digit_value;
                digits.clear();
                is_digit = false;
            }
        }

        if is_digit {
            // If the line ends with a digit, add its value to the sum
            let digit_value = match digits.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => digits.parse::<u32>().unwrap_or(0), // Parse as a number if not a spelled-out digit
            };
            sum += digit_value;
        }
    }

    println!("The sum of calibration values is: {}", sum);

    Ok(())
}

/*
// Version couting eightwo as 8 (last digit)
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

*/

fn main() -> std::io::Result<()> {
    if let Ok(_) = part_01() {}
    if let Ok(_) = part_02() {}
    Ok(())
}
