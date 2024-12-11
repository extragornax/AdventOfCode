use aoc_tools::read_file_to_string;
use std::collections::HashMap;

fn count_until_end(num: usize, steps_left: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if steps_left == 0 {
        return 1;
    }

    if let Some(&cached) = memo.get(&(num, steps_left)) {
        return cached;
    }

    let result = if num == 0 {
        count_until_end(1, steps_left - 1, memo)
    } else {
        let num_digits = (num as f64).log10().floor() as usize + 1;
        if num_digits % 2 == 0 {
            let divisor = 10_usize.pow((num_digits / 2) as u32);
            let left = num / divisor;
            let right = num % divisor;
            count_until_end(left, steps_left - 1, memo) + count_until_end(right, steps_left - 1, memo)
        } else {
            count_until_end(num * 2024, steps_left - 1, memo)
        }
    };

    memo.insert((num, steps_left), result);
    result
}

fn run_iters(number_steps: usize) -> std::io::Result<usize> {
    let content = read_file_to_string()?;

    let initial_numbers = content
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut memo = HashMap::new();
    let mut total_count = 0;

    for num in initial_numbers {
        total_count += count_until_end(num, number_steps, &mut memo);
    }

    Ok(total_count)
}

fn steps() -> std::io::Result<()> {
    println!("Part 01 : {}", run_iters(25)?);
    println!("Part 02 : {}", run_iters(75)?);

    Ok(())
}

fn main() {
    if let Err(e) = steps() {
        println!("Failed to read input: {}", e);
    }
}
