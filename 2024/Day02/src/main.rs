use aoc_tools::read_file_to_string;

fn vec_is_sorted(vec: Vec<i64>) -> bool {
    vec.iter().zip(vec.iter().skip(1)).all(|(a, b)| {
        let abs = (a - b).abs();
        a <= b && (abs >= 1 && abs <= 3)
    })
}

fn file_to_vec() -> std::io::Result<Vec<Vec<i64>>> {
    let contents = read_file_to_string()?;

    let data: Vec<Vec<i64>> = contents
        .split("\n")
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .map(|line| {
            line.split(" ")
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(data)
}

fn part_02(data: &Vec<Vec<i64>>) -> std::io::Result<()> {
    let total_safe: i64 = data
        .iter()
        .enumerate()
        .map(|(_index, line)| {
            let is_order_up = vec_is_sorted(line.clone());
            let mut tmp = line.clone();
            tmp.reverse();
            let is_order_down = vec_is_sorted(tmp);

            if is_order_up || is_order_down {
                return 1;
            }

            // Unsafe, so test each versions on current vec
            for i_arr in 0..line.len() {
                let mut tmp_line = line.clone();
                tmp_line.remove(i_arr);

                let is_order_up = vec_is_sorted(tmp_line.clone());
                let mut tmp = tmp_line.clone();
                tmp.reverse();
                let is_order_down = vec_is_sorted(tmp);

                if is_order_up || is_order_down {
                    return 1;
                }
            }

            0
        })
        .sum();
    println!("Part 02: {}", total_safe);
    Ok(())
}

fn part_01(data: &Vec<Vec<i64>>) -> std::io::Result<()> {
    let total_safe: i64 = data
        .iter()
        .map(|line| {
            let is_order_up = vec_is_sorted(line.clone());
            let mut tmp = line.clone();
            tmp.reverse();
            let is_order_down = vec_is_sorted(tmp);
            match is_order_down || is_order_up {
                true => 1,
                _ => 0
            }
        })
        .sum();
    println!("Part 01: {}", total_safe);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let data = file_to_vec()?;
    if let Err(e) = part_01(&data) {
        println!("Part 01 error: {}", e);
    }
    if let Err(e) = part_02(&data) {
        println!("Part 02 error: {}", e);
    }

    Ok(())
}
