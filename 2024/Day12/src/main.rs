use std::collections::{HashSet, VecDeque};
use aoc_tools::read_file_to_string;

fn explore_region_1(
    grid: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    char_type: char,
    visited: &mut Vec<Vec<bool>>,
    directions: &[(i64, i64)],
) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut area = 0;
    let mut perimeter = 0;

    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));
    visited[start_x][start_y] = true;

    while let Some((x, y)) = queue.pop_front() {
        area += 1;
        for &(dx, dy) in directions {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;

            if nx >= 0 && nx < rows as i64 && ny >= 0 && ny < cols as i64 {
                let nx = nx as usize;
                let ny = ny as usize;

                let (nx, ny) = (nx, ny);

                if grid[nx][ny] == char_type && !visited[nx][ny] {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                } else if grid[nx][ny] != char_type {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (area, perimeter)
}

fn explore_region_2(
    grid: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    char_type: char,
    visited: &mut Vec<Vec<bool>>,
    directions: &[(i64, i64)],
) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut area = 0;
    let mut edges = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));
    visited[start_x][start_y] = true;

    while let Some((x, y)) = queue.pop_front() {
        area += 1;

        for &(dx, dy) in directions {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;

            if nx >= 0 && nx < rows as i64 && ny >= 0 && ny < cols as i64 {
                let (nx, ny) = (nx as usize, ny as usize);

                if grid[nx][ny] == char_type {
                    if !visited[nx][ny] {
                        visited[nx][ny] = true;
                        queue.push_back((nx, ny));
                    }
                } else {
                    // Add edge only once for the region
                    edges.insert(((x.min(nx), y.min(ny)), (x.max(nx), y.max(ny))));
                }
            } else {
                // Edge to the outside boundary
                edges.insert(((x, y), ((x as i64 + dx) as usize, (y as i64 + dy) as usize)));
            }
        }
    }

    let sides = edges.len();
    (area, sides)
}

fn calculate_total_price(grid: Vec<Vec<char>>, is_step_one: bool) -> u64 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut total_price = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for x in 0..rows {
        for y in 0..cols {
            if !visited[x][y] {
                let char_type = grid[x][y];
                let (area, perimeter) = match is_step_one {
                    true => explore_region_1(&grid, x, y, char_type, &mut visited, &directions),
                    false => explore_region_2(&grid, x, y, char_type, &mut visited, &directions),
                };
                total_price += (area * perimeter) as u64;
            }
        }
    }

    total_price
}


fn get_grid() -> std::io::Result<Vec<Vec<char>>> {
    let content = read_file_to_string()?;
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    Ok(grid)
}

fn part_02() -> std::io::Result<()> {
    let grid = get_grid()?;

    let price =  calculate_total_price(grid, false);
    println!("Step 02: {}", price);
    Ok(())
}

fn part_01() -> std::io::Result<()> {
    let grid = get_grid()?;

    let price =  calculate_total_price(grid, true);
    println!("Step 01: {}", price);

    Ok(())
}

fn main() {
    if let Err(e) = part_01() {
        println!("Part 1 error: {}", e);
    }
    if let Err(e) = part_02() {
        println!("Part 2 error: {}", e);
    }
}
