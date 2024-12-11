use aoc_tools::read_file_to_string;

fn shit_numbers(numbers: Vec<String>) -> Vec<String> {
    let mut new_arr = Vec::new();
    let mut index = 0;
    while index < numbers.len() {
        let tmp_num = numbers[index].clone();
        if tmp_num == "0".to_string() {
            new_arr.push("1".to_string());
        } else if tmp_num.len() % 2 == 0 {
            let split = tmp_num.split_at(tmp_num.len() / 2);
            let parsed_1 = split.0.parse::<usize>().unwrap();
            new_arr.push(parsed_1.to_string());
            let parsed_2 = split.1.parse::<usize>().unwrap();
            new_arr.push(parsed_2.to_string());
        } else {
            let parsed = tmp_num.parse::<usize>().unwrap();
            let final_num = parsed * 2024;
            new_arr.push(final_num.to_string());
        }

        index += 1;
    }

    new_arr
}

fn step_01() -> std::io::Result<()> {
    let content = read_file_to_string()?;

    let mut numbers = content
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let runs = 25;
    for _index in 0..runs {
        numbers = shit_numbers(numbers);
        // println!("{} - {:?}", _index + 1, numbers);
    }

    println!("Part 01 : {}", numbers.len());

    Ok(())
}

fn main() {
    if let Err(e) = step_01() {
        println!("Failed to read input: {}", e);
    }
}
