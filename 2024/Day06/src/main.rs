use std::sync::mpsc;
use std::thread;
use aoc_tools::read_file_to_string;
use std::collections::HashSet;

const DIRECTIONS: [char; 4] = ['^', '>', 'v', '<'];

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum DirectionEn {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    print!("{}[2J", 27 as char); // Clear terminal
    for y in grid {
        for x in y {
            print!("{}", x);
        }
        println!();
    }
    println!();
}

fn count_in_grid(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == 'X')
        .count()
}

fn update_move(
    mut grid: Vec<Vec<char>>,
    (y_pos, x_pos): (i64, i64),
    direction: DirectionEn,
) -> (Vec<Vec<char>>, (i64, i64), DirectionEn, bool) {
    let (next_y, next_x) = match direction {
        DirectionEn::Up => (y_pos - 1, x_pos),
        DirectionEn::Right => (y_pos, x_pos + 1),
        DirectionEn::Down => (y_pos + 1, x_pos),
        DirectionEn::Left => (y_pos, x_pos - 1),
    };

    // Check if the guard is about to leave the grid
    if next_y < 0 || next_x < 0 || next_y >= grid.len() as i64 || next_x >= grid[0].len() as i64 {
        grid[y_pos as usize][x_pos as usize] = 'X'; // Mark current position
        return (grid, (y_pos, x_pos), direction, true);
    }

    // Mark current position as visited
    grid[y_pos as usize][x_pos as usize] = 'X';

    // Check if the guard needs to turn
    if grid[next_y as usize][next_x as usize] == '#' {
        let new_direction = match direction {
            DirectionEn::Up => DirectionEn::Right,
            DirectionEn::Right => DirectionEn::Down,
            DirectionEn::Down => DirectionEn::Left,
            DirectionEn::Left => DirectionEn::Up,
        };
        grid[y_pos as usize][x_pos as usize] = DIRECTIONS[new_direction.clone() as usize];
        return (grid, (y_pos, x_pos), new_direction, false);
    }

    // Move guard forward
    grid[next_y as usize][next_x as usize] = DIRECTIONS[direction.clone() as usize];
    (grid, (next_y, next_x), direction, false)
}

fn part_01() -> std::io::Result<()> {
    let content = read_file_to_string()?;
    let mut grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    // Find the initial position and direction of the guard
    let ((start_y, start_x), start_direction) = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &c)| {
                DIRECTIONS.contains(&c).then(|| ((y as i64, x as i64), match c {
                    '^' => DirectionEn::Up,
                    '>' => DirectionEn::Right,
                    'v' => DirectionEn::Down,
                    '<' => DirectionEn::Left,
                    _ => panic!("Unknown direction: {}", c),
                }))
            })
        })
        .expect("Guard's initial position not found");

    let mut position = (start_y, start_x);
    let mut direction = start_direction;

    loop {
        let (next_grid, next_position, next_direction, guard_left) =
            update_move(grid.clone(), position, direction);
        grid = next_grid;
        position = next_position;
        direction = next_direction;

        if guard_left {
            println!("Part 01: {}", count_in_grid(&grid));
            return Ok(());
        }
    }
}

fn find_cycle(mut grid: Vec<Vec<char>>, start_position: (i64, i64), start_direction: DirectionEn) -> bool {
    let mut visited_states = HashSet::new();
    let mut position = start_position;
    let mut direction = start_direction;
    let mut step_count = 0;

    loop {
        if !visited_states.insert((position, direction.clone())) {
            return true; // Cycle detected
        }

        let (next_grid, next_position, next_direction, guard_left) =
            update_move(grid.clone(), position, direction);
        grid = next_grid;
        position = next_position;
        direction = next_direction;

        if guard_left || step_count > 10_000 {
            break; // Guard leaves or too many steps
        }
        step_count += 1;
    }

    false
}


fn part_02() -> std::io::Result<()> {
    let content = read_file_to_string()?;
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    // Find the initial position and direction of the guard
    let ((start_y, start_x), start_direction) = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &c)| {
                DIRECTIONS.contains(&c).then(|| ((y as i64, x as i64), match c {
                    '^' => DirectionEn::Up,
                    '>' => DirectionEn::Right,
                    'v' => DirectionEn::Down,
                    '<' => DirectionEn::Left,
                    _ => panic!("Unknown direction: {}", c),
                }))
            })
        })
        .expect("Guard's initial position not found");

    // Split the grid into four sections
    let height = grid.len();
    let width = grid[0].len();
    let quarter_height = height / 12;

    let (tx, rx) = mpsc::channel();

    for i in 0..12 {
        let tx = tx.clone();
        let subgrid_start = i * quarter_height;
        let subgrid_end = if i == 11 { height } else { (i + 1) * quarter_height };
        let grid_clone = grid.clone();
        let start_position = (start_y, start_x);
        let start_direction = start_direction.clone();

        thread::spawn(move || {
            let mut possible_positions = 0;

            for y in subgrid_start..subgrid_end {
                for x in 0..width {
                    // Skip walls and the starting position
                    if grid_clone[y][x] != '.' || (y as i64 == start_position.0 && x as i64 == start_position.1) {
                        continue;
                    }

                    // Place obstruction
                    let mut grid_with_obstruction = grid_clone.clone();
                    grid_with_obstruction[y][x] = '#';

                    // Check for a cycle
                    if find_cycle(grid_with_obstruction, start_position, start_direction.clone()) {
                        possible_positions += 1;
                    }
                }
            }

            tx.send(possible_positions).expect("Failed to send result");
        });
    }

    drop(tx); // Close the sending side to avoid deadlock

    let total_possible_positions: usize = rx.iter().sum();
    println!("Part 02: {}", total_possible_positions);
    Ok(())
}

fn main() {
    if let Err(e) = part_01() {
        println!("Error in Part 01: {}", e);
    }
    if let Err(e) = part_02() {
        println!("Error in Part 02: {}", e);
    }
}
