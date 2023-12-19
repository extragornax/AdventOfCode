fn part_01() -> std::io::Result<()> {
    // Actual vec
    let races: Vec<(i32, i32)> = vec![(58, 434), (81, 1041), (96, 2219), (76, 1218)];
    // Example
    // let races: Vec<(i32, i32)> = vec![(7, 9), (15, 40), (30, 200)];

    let res = races.iter().map(|(time, dist_win)| {
        let mut count_beat = 0;
        for speed in 1..*time {
            let remain = *time - speed;
            let dist = speed * remain;
            if dist > *dist_win {
                count_beat += 1;
            }
        }
        return count_beat;
    }).collect::<Vec<i32>>();

    println!("Part 01: {:?}", res.into_iter().reduce(|mut a, b| {
        a *= b;
        a
    }).unwrap());

    Ok(())
}

fn part_02() -> std::io::Result<()> {
    // Actual vec
    let races: Vec<(i64, i64)> = vec![(58819676, 434104122191218)];
    // Example
    // let races: Vec<(i64, i64)> = vec![(71530, 940200)];

    let res = races.iter().map(|(time, dist_win)| {
        let mut count_beat = 0;
        for speed in 1..*time {
            let remain = *time - speed;
            let dist = speed * remain;
            if dist > *dist_win {
                count_beat += 1;
            }
        }
        return count_beat;
    }).collect::<Vec<i32>>();

    println!("Part 02: {:?}", res.into_iter().reduce(|mut a, b| {
        a *= b;
        a
    }).unwrap());

    Ok(())
}

fn main() {
    if let Ok(_) = part_01() {}
    if let Ok(_) = part_02() {}
}
