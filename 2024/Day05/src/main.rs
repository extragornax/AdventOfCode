use aoc_tools::read_file_to_string;
use std::collections::{HashMap, HashSet};

fn parse_data() -> std::io::Result<(Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let content = read_file_to_string()?;

    let split = content
        .split("\n");

    let mut rules_before: Vec<String> = Vec::new();
    let mut update_before: Vec<String> = Vec::new();

    let mut rules_mode = true;
    for line in split {
        if rules_mode && !line.is_empty() {
            rules_before.push(line.to_string());
        } else if line.is_empty() {
            rules_mode = false;
        } else {
            update_before.push(line.to_string());
        }
    }

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    rules_before.iter().for_each(|line| {
        let s = line.split("|").collect::<Vec<&str>>();
        let one = s[0].parse::<i32>().unwrap();
        let two = s[1].parse::<i32>().unwrap();

        rules.push((one, two));
    });

    update_before.iter().for_each(|line| {
        let mut vec_tmp = Vec::new();
        let s = line.split(",").collect::<Vec<&str>>();
        for _s in s {
            let parsed = _s.parse::<i32>().unwrap();
            vec_tmp.push(parsed);
        }
        updates.push(vec_tmp);
    });

    Ok((rules, updates))
}

fn part_01() -> std::io::Result<()> {
    let (rules, updates) = parse_data()?;

    // Parse rules into a dependency graph
    let mut dependencies: HashMap<i32, HashSet<i32>> = HashMap::new();
    for &(x, y) in &rules {
        dependencies.entry(y).or_default().insert(x);
    }

    let mut sum_of_middles = 0;

    'updates_for: for update in updates {
        // Create a position map for this update
        let mut positions: HashMap<i32, usize> = HashMap::new();
        for (i, &page) in update.iter().enumerate() {
            positions.insert(page, i);
        }

        // Check if the update respects all ordering rules
        for &(x, y) in &rules {
            if let (Some(&pos_x), Some(&pos_y)) = (positions.get(&x), positions.get(&y)) {
                if pos_x >= pos_y {
                    continue 'updates_for; // Rule violated; skip this update
                }
            }
        }

        // If the update is valid, add its middle page to the sum
        let middle_page = update[update.len() / 2];
        sum_of_middles += middle_page;
    }

    println!("Part_01: {}", sum_of_middles);

    Ok(())
}

fn main() {
    if let Err(e) = part_01() {
        println!("Error: {}", e);
    }
}
