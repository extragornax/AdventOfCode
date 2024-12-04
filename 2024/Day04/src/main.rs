use aoc_tools::read_file_to_string;

fn is_valid_double_mas(
    row: i64,
    col: i64,
    row_len: i64,
    col_len: i64,
    grid: &Vec<Vec<char>>,
) -> bool {
    if row + 1 >= row_len {
        return false;
    }
    if col + 1 >= col_len {
        return false;
    }

    let (up_l_x, up_l_y) = (row - 1, col - 1);
    let (down_r_x, down_r_y) = (row + 1, col + 1);

    let (up_r_x, up_r_y) = (row - 1, col + 1);
    let (down_l_x, down_l_y) = (row + 1, col - 1);

    let mut found = false;
    if (0 <= up_l_x && up_l_x < row_len - 1)
        && (0 <= up_r_x && up_r_x < row_len - 1)
        && (0 <= down_l_y && down_l_y < col_len - 1)
        && (0 <= up_l_y && up_l_y < col_len - 1)
    {
        if (grid[up_l_x as usize][up_l_y as usize] == 'M'
            && grid[down_r_x as usize][down_r_y as usize] == 'S')
            || (grid[up_l_x as usize][up_l_y as usize] == 'S'
                && grid[down_r_x as usize][down_r_y as usize] == 'M')
        {
            if found {
                return true;
            }
            found = true;
        }

        if (grid[up_r_x as usize][up_r_y as usize] == 'M'
            && grid[down_l_x as usize][down_l_y as usize] == 'S')
            || (grid[up_r_x as usize][up_r_y as usize] == 'S'
                && grid[down_l_x as usize][down_l_y as usize] == 'M')
        {
            if found {
                return true;
            }
        }
    }
    false
}

fn step_02() -> std::io::Result<()> {
    let content = read_file_to_string()?;
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let mut count = 0;

    // Iterate over every cell in the grid
    for row in 0..rows {
        for col in 0..cols {
            if grid[row as usize][col as usize] == 'A'
                && is_valid_double_mas(row, col, rows, cols, &grid) {
                count += 1;
            }
        }
        // }
    }

    println!("Step 02: {}", count);
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
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
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
