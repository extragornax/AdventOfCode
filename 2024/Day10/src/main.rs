use aoc_tools::read_file_to_string;
use std::collections::{HashMap, HashSet};

fn file_content() -> std::io::Result<Vec<Vec<u32>>> {
    let content = read_file_to_string()?;

    let mut final_data = Vec::<Vec<u32>>::new();
    for l in content.lines() {
        let chars = l
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u32)
            .collect::<Vec<u32>>();
        final_data.push(chars);
    }

    Ok(final_data)
}

fn find_neighbors(x: usize, y: usize, map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x + 1 < map.len() {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y + 1 < map[0].len() {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn bfs_trailhead_score(x: usize, y: usize, map: &Vec<Vec<u32>>) -> usize {
    let mut visited = HashSet::new();
    let mut queue = vec![(x, y, 0)]; // (x, y, current_height)
    let mut reachable_nines = HashSet::new();

    while let Some((cx, cy, current_height)) = queue.pop() {
        if visited.contains(&(cx, cy)) {
            continue;
        }
        visited.insert((cx, cy));

        let height = map[cx][cy];
        if height != current_height {
            continue;
        }

        if height == 9 {
            reachable_nines.insert((cx, cy));
        }

        for (nx, ny) in find_neighbors(cx, cy, map) {
            if !visited.contains(&(nx, ny)) && map[nx][ny] == height + 1 {
                queue.push((nx, ny, height + 1));
            }
        }
    }

    reachable_nines.len()
}

fn calculate_trailhead_scores(map: &Vec<Vec<u32>>) -> usize {
    let mut total_score = 0;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                total_score += bfs_trailhead_score(x, y, map);
            }
        }
    }

    total_score
}

fn count_distinct_trails(
    x: usize,
    y: usize,
    map: &Vec<Vec<u32>>,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&cached) = memo.get(&(x, y)) {
        return cached;
    }

    let current_height = map[x][y];
    if current_height == 9 {
        return 1;
    }

    let mut total_trails = 0;
    for (nx, ny) in find_neighbors(x, y, map) {
        if map[nx][ny] == current_height + 1 {
            total_trails += count_distinct_trails(nx, ny, map, memo);
        }
    }

    memo.insert((x, y), total_trails);
    total_trails
}

fn calculate_trailhead_ratings(map: &Vec<Vec<u32>>) -> usize {
    let mut total_rating = 0;
    let mut memo = HashMap::new();

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                total_rating += count_distinct_trails(x, y, map, &mut memo);
            }
        }
    }

    total_rating
}

fn step_02() -> std::io::Result<()> {
    let map = file_content()?;

    let total_rating = calculate_trailhead_ratings(&map);
    println!("Step 02: {}", total_rating);
    Ok(())
}

fn step_01() -> std::io::Result<()> {
    let map = file_content()?;
    let total_score = calculate_trailhead_scores(&map);
    println!("Step 01: {}", total_score);
    Ok(())
}

fn main() {
    if let Err(e) = step_01() {
        println!("Failed to read input: {}", e);
    }

    if let Err(e) = step_02() {
        println!("Failed to read input: {}", e);
    }
}
