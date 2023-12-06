use std::fs::File;
use std::io::{BufReader, Read};

fn is_number(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

fn retrieve_number_from_pos(x: i64, y: i64, all: &Vec<Vec<char>>) -> (Vec<Vec<char>>, i64) {
    let mut all = all.clone();
    if x < 0 || y < 0 {
        return (all, 0);
    }
    let up_down = x;
    let mut side_to_side = y;
    loop {
        if up_down < 0 || side_to_side < 0 {
            break;
        }
        let c = all[up_down as usize][side_to_side as usize];
        if is_number(c) {
            side_to_side -= 1;
            continue;
        } else {
            break;
        }
    }
    side_to_side += 1;
    let mut number_vec = vec![];
    if side_to_side < 0 || side_to_side >= all[up_down as usize].len() as i64 {
        return (all, 0);
    }
    println!("Updown {:?} Sidetoside {:?} -> len {}", up_down, side_to_side, all[up_down as usize].len() as i64);
    while (side_to_side < all[up_down as usize].len() as i64) && is_number(all[up_down as usize][side_to_side as usize]) {
        number_vec.push(all[up_down as usize][side_to_side as usize]);
        all[up_down as usize][side_to_side as usize] = '.';
        side_to_side += 1;
    }
    (all, number_vec.clone().iter().collect::<String>().parse::<i64>().unwrap_or(0))
}

/*
    539219 Too low (dedup)
    540212 Good
    911882 Too high (not dedup)
 */
fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let split = contents.split("\n");
    let mut all: Vec<Vec<char>> = vec![];
    for s in split {
        let mut tmp: Vec<char> = vec![];
        if s.len() == 0 {
            continue;
        }
        for c in s.chars() {
            tmp.push(c);
        }
        all.push(tmp);
    }

    #[derive(Debug)]
    struct SymbolsPos {
        symbol: char,
        x: i64,
        y: i64,
    }

    let mut symbols: Vec<SymbolsPos> = vec![];
    for (i, line) in all.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '.' && !is_number(*c) {
                symbols.push(SymbolsPos {
                    symbol: *c,
                    x: i as i64,
                    y: j as i64,
                });
            }
        }
    }

    println!("Symbols {:?}", symbols);

    #[derive(Debug)]
    struct SymbolsNums {
        symbol: char,
        numbers: Vec<i64>,
    }
    let mut all_numbers_for_symbol: Vec<SymbolsNums> = vec![];

    for s in &symbols {
        let x = s.x;
        let y = s.y;
        let mut symbol_data = SymbolsNums {
            symbol: s.symbol,
            numbers: vec![],
        };
        if x > 0 && y > 0 && is_number(all[(x - 1) as usize][(y - 1) as usize]) {
            let _t = retrieve_number_from_pos(x - 1, y - 1, &all);
            all = _t.0;
            symbol_data.numbers.push(_t.1);
        }

        if x > 0 && is_number(all[(x - 1) as usize][(y) as usize]) {
            let _t = retrieve_number_from_pos(x - 1, y, &all);
            all = _t.0;
            symbol_data.numbers.push(_t.1);
        }

        if x > 0 && y < all[x as usize].len() as i64 && is_number(all[(x - 1) as usize][(y + 1) as usize]) {
            let _t = retrieve_number_from_pos(x - 1, y + 1, &all);
            all = _t.0;
            symbol_data.numbers.push(_t.1);
        }

        if y > 0 && is_number(all[(x) as usize][(y - 1) as usize]) {
            let _t = retrieve_number_from_pos(x, y - 1, &all);
            all = _t.0;
            symbol_data.numbers.push(_t.1);
        }

        if (y + 1) < all[x as usize].len() as i64 && is_number(all[(x) as usize][(y + 1) as usize]) {
            let _t = retrieve_number_from_pos(x, y + 1, &all);
            all = _t.0;
            symbol_data.numbers.push(_t.1);
        }

        if (x + 1) < all.len() as i64 && y > 0 && is_number(all[(x + 1) as usize][(y - 1) as usize]) {
            let _t = retrieve_number_from_pos(x + 1, y - 1, &all);
            all = _t.0;
            symbol_data.numbers.push(_t.1);
        }

        if (x + 1) < all.len() as i64 && is_number(all[(x + 1) as usize][(y) as usize]) {
            let _t = retrieve_number_from_pos(x + 1, y, &all);
            all = _t.0;
            symbol_data.numbers.push(_t.1);
        }

        if (x + 1) < all.len() as i64 && y + 1 < all[x as usize].len() as i64 && is_number(all[(x + 1) as usize][(y + 1) as usize]) {
            let _t = retrieve_number_from_pos(x + 1, y + 1, &all);
            all = _t.0;
            symbol_data.numbers.push(_t.1);
        }

        println!("Symbol data {:?}", symbol_data);

        // symbol_data.numbers.dedup();

        all_numbers_for_symbol.push(symbol_data);
    }
    // println!("All numbers {:?}", all_numbers_for_symbol);

    let total = all_numbers_for_symbol.iter().map(|s|
        s.numbers.iter().sum::<i64>()
    ).sum::<i64>();

    println!("Part 01: {:?}", total);
    Ok(())
}


fn main() {
    if let Ok(_) = part_01() {}
}
