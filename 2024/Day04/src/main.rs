use std::io::{BufRead};
use aoc_tools::read_file_to_string;

fn is_valid_arm(rows: usize, cols: usize, x: usize, y: usize, dx: isize, dy: isize, grid: &Vec<Vec<char>>) -> bool {
    let nx = x as isize + dx;
    let ny = y as isize + dy;
    let fx = x as isize + 2 * dx;
    let fy = y as isize + 2 * dy;

    if nx >= 0 && ny >= 0 && fx >= 0 && fy >= 0 &&
        (nx as usize) < rows && (ny as usize) < cols &&
        (fx as usize) < rows && (fy as usize) < cols
    {
        // "M.A.S" or "S.A.M"
        let middle = grid[nx as usize][ny as usize];
        let far = grid[fx as usize][fy as usize];
        (grid[x][y] == 'M' && middle == 'A' && far == 'S') ||
            (grid[x][y] == 'S' && middle == 'A' && far == 'M')
    } else {
        false
    }
}

fn step_02() -> std::io::Result<()> {
    let content = read_file_to_string()?; // Read the input file

    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Iterate over the grid to find X-MAS patterns
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'A' {
                // Define diagonal directions for the arms
                let deltas = [
                    (-1, -1), // Top-left
                    (-1, 1),  // Top-right
                    (1, -1),  // Bottom-left
                    (1, 1),   // Bottom-right
                ];

                for k in 0..4 {
                    // Each "X" requires two distinct diagonals
                    let (dx1, dy1) = deltas[k];
                    let (dx2, dy2) = deltas[(k + 2) % 4]; // Opposite diagonal

                    // Check both diagonals to form an "X"
                    if is_valid_arm(rows, cols, i, j, dx1, dy1, &grid) &&
                        is_valid_arm(rows, cols, i, j, dx2, dy2, &grid)
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    // Each X-MAS pattern is counted twice (once for each diagonal pairing)
    println!("Part 02: {}", count / 2);

    Ok(())
}

fn step_01() -> std::io::Result<()> {
    let contents = read_file_to_string()?;
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let word = "XMAS";
    let word_len = word.len();
    let mut count = 0;

    let directions = [
        (0, 1),  // Right
        (0, -1), // Left
        (1, 0),  // Down
        (-1, 0), // Up
        (1, 1),  // Down-Right
        (1, -1), // Down-Left
        (-1, 1), // Up-Right
        (-1, -1), // Up-Left
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    let is_in_bounds = |x: isize, y: isize| -> bool {
        x >= 0 && y >= 0 && (x as usize) < rows && (y as usize) < cols
    };

    let search_in_direction = |x: usize, y: usize, dx: isize, dy: isize| -> bool {
        (0..word_len).all(|i| {
            let nx = x as isize + i as isize * dx;
            let ny = y as isize + i as isize * dy;
            is_in_bounds(nx, ny) && grid[nx as usize][ny as usize] == word.chars().nth(i).unwrap()
        })
    };

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if search_in_direction(i, j, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    println!("Part 01: {}", count);

    Ok(())
}


fn main() {
    if let Err(e) = step_01() {
        println!("Error: {}", e);
    }
    if let Err(e) = step_02() {
        println!("Error: {}", e);
    }
}
