use std::fs::File;
use std::io::{BufReader, Error, Read};

#[derive(Debug)]
struct Converter {
    dest_start: i64,
    src_start: i64,
    size: i64,
}

fn convert_into_next_type(seed: i64, converters: &Vec<Converter>) -> i64 {
    let mut result: i64 = seed;
    for converter in converters {
        if seed >= converter.src_start && seed < converter.src_start + converter.size {
            result = converter.dest_start + (seed - converter.src_start);
            break;
        }
    }
    result
}

fn get_seed_part_01() -> Result<Vec<i64>, std::io::Error> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let split = contents.split("\n");
    let mut seed: Vec<i64> = Vec::new();

    let mut index: usize = 0;
    while index < split.clone().count() {
        let line: &str = split.clone().nth(index).unwrap();

        if line.len() == 0 {
            index += 1;
            continue;
        }

        if line.starts_with("seeds") {
            let _split = line.split(" ");
            let seeds = _split.into_iter().map(|x| x.parse::<i64>().unwrap_or(-1)).collect::<Vec<i64>>();
            seeds.into_iter().filter(|x| *x != -1).for_each(|x| seed.push(x));
            index += 1;
            continue;
        }

        index += 1;
    }

    return Ok(seed);
}

fn get_seed_part_02() -> Result<Vec<(i64, i64)>, std::io::Error> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let split = contents.split("\n");
    let mut seed: Vec<(i64, i64)> = Vec::new();

    let mut index: usize = 0;
    while index < split.clone().count() {
        let line: &str = split.clone().nth(index).unwrap();

        if line.len() == 0 {
            index += 1;
            continue;
        }

        if line.starts_with("seeds") {
            let _split = line.split(" ");
            let seeds = _split.into_iter().map(|x| x.parse::<i64>().unwrap_or(-1)).collect::<Vec<i64>>();
            println!("SEEDS {:?}", seeds);
            let seeds: Vec<i64> = seeds.into_iter().filter(|x| *x != -1).collect();
            println!("CLEAN SEEDS {:?}", seeds);
            let mut iseed = 0;
            while iseed < seeds.len() {
                println!("iseed: {}", iseed);
                seed.push((seeds[iseed], seeds[iseed + 1]));
                iseed += 2;
                println!("iseed: {}", iseed);
            }
            break;
        }

        index += 1;
    }

    return Ok(seed);
}

fn handle_retrieve_data() -> Result<(Vec<Converter>, Vec<Converter>, Vec<Converter>, Vec<Converter>, Vec<Converter>, Vec<Converter>, Vec<Converter>), std::io::Error> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let split = contents.split("\n");
    let mut seed_to_soil: Vec<Converter> = Vec::new();
    let mut soil_to_fertilizer: Vec<Converter> = Vec::new();
    let mut fertilizer_to_water: Vec<Converter> = Vec::new();
    let mut water_to_light: Vec<Converter> = Vec::new();
    let mut light_to_temperature: Vec<Converter> = Vec::new();
    let mut temperature_to_humidity: Vec<Converter> = Vec::new();
    let mut humidity_to_location: Vec<Converter> = Vec::new();

    let mut index: usize = 0;
    while index < split.clone().count() {
        let line: &str = split.clone().nth(index).unwrap();

        if line.len() == 0 {
            index += 1;
            continue;
        }

        if line.starts_with("seed-to-soil map:") {
            index += 1;
            while split.clone().nth(index).unwrap().len() > 0 {
                let line = split.clone().nth(index).unwrap();
                let _split = line.split(" ");
                seed_to_soil.push(Converter {
                    dest_start: _split.clone().nth(0).unwrap().parse::<i64>().unwrap_or(-1),
                    src_start: _split.clone().nth(1).unwrap().parse::<i64>().unwrap_or(-1),
                    size: _split.clone().nth(2).unwrap().parse::<i64>().unwrap_or(-1),
                });
                index += 1;
            }
        }

        if line.starts_with("soil-to-fertilizer map:") {
            index += 1;
            while split.clone().nth(index).unwrap().len() > 0 {
                let line = split.clone().nth(index).unwrap();
                let _split = line.split(" ");
                soil_to_fertilizer.push(Converter {
                    dest_start: _split.clone().nth(0).unwrap().parse::<i64>().unwrap_or(-1),
                    src_start: _split.clone().nth(1).unwrap().parse::<i64>().unwrap_or(-1),
                    size: _split.clone().nth(2).unwrap().parse::<i64>().unwrap_or(-1),
                });
                index += 1;
            }
        }

        if line.starts_with("fertilizer-to-water map:") {
            index += 1;
            while split.clone().nth(index).unwrap().len() > 0 {
                let line = split.clone().nth(index).unwrap();
                let _split = line.split(" ");
                fertilizer_to_water.push(Converter {
                    dest_start: _split.clone().nth(0).unwrap().parse::<i64>().unwrap_or(-1),
                    src_start: _split.clone().nth(1).unwrap().parse::<i64>().unwrap_or(-1),
                    size: _split.clone().nth(2).unwrap().parse::<i64>().unwrap_or(-1),
                });
                index += 1;
            }
        }

        if line.starts_with("water-to-light map:") {
            index += 1;
            while split.clone().nth(index).unwrap().len() > 0 {
                let line = split.clone().nth(index).unwrap();
                let _split = line.split(" ");
                water_to_light.push(Converter {
                    dest_start: _split.clone().nth(0).unwrap().parse::<i64>().unwrap_or(-1),
                    src_start: _split.clone().nth(1).unwrap().parse::<i64>().unwrap_or(-1),
                    size: _split.clone().nth(2).unwrap().parse::<i64>().unwrap_or(-1),
                });
                index += 1;
            }
        }

        if line.starts_with("light-to-temperature map:") {
            index += 1;
            while split.clone().nth(index).unwrap().len() > 0 {
                let line = split.clone().nth(index).unwrap();
                let _split = line.split(" ");
                light_to_temperature.push(Converter {
                    dest_start: _split.clone().nth(0).unwrap().parse::<i64>().unwrap_or(-1),
                    src_start: _split.clone().nth(1).unwrap().parse::<i64>().unwrap_or(-1),
                    size: _split.clone().nth(2).unwrap().parse::<i64>().unwrap_or(-1),
                });
                index += 1;
            }
        }

        if line.starts_with("temperature-to-humidity map:") {
            index += 1;
            while split.clone().nth(index).unwrap().len() > 0 {
                let line = split.clone().nth(index).unwrap();
                let _split = line.split(" ");
                temperature_to_humidity.push(Converter {
                    dest_start: _split.clone().nth(0).unwrap().parse::<i64>().unwrap_or(-1),
                    src_start: _split.clone().nth(1).unwrap().parse::<i64>().unwrap_or(-1),
                    size: _split.clone().nth(2).unwrap().parse::<i64>().unwrap_or(-1),
                });
                index += 1;
            }
        }

        if line.starts_with("humidity-to-location map:") {
            index += 1;
            while split.clone().nth(index).unwrap().len() > 0 {
                let line = split.clone().nth(index).unwrap();
                let _split = line.split(" ");
                humidity_to_location.push(Converter {
                    dest_start: _split.clone().nth(0).unwrap().parse::<i64>().unwrap_or(-1),
                    src_start: _split.clone().nth(1).unwrap().parse::<i64>().unwrap_or(-1),
                    size: _split.clone().nth(2).unwrap().parse::<i64>().unwrap_or(-1),
                });
                index += 1;
            }
        }

        index += 1;
    }
    return Ok((seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location));
}

fn part_01() -> std::io::Result<()> {
    let seed = get_seed_part_01()?;

    println!("all_seeds: {:?}", seed.len());

    let (seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location) = handle_retrieve_data()?;

    let mut converted_seeds = seed.clone().into_iter()
        .map(|x| convert_into_next_type(x, &seed_to_soil))
        .map(|x| convert_into_next_type(x, &soil_to_fertilizer))
        .map(|x| convert_into_next_type(x, &fertilizer_to_water))
        .map(|x| convert_into_next_type(x, &water_to_light))
        .map(|x| convert_into_next_type(x, &light_to_temperature))
        .map(|x| convert_into_next_type(x, &temperature_to_humidity))
        .map(|x| convert_into_next_type(x, &humidity_to_location))
        .collect::<Vec<i64>>();

    converted_seeds.sort();

    let result = match converted_seeds.clone().first() {
        Some(x) => x.clone(),
        None => -1,
    };

    println!("Part 01: {}", result);

    Ok(())
}

// fn part_02() -> std::io::Result<()> {
//     let seed: Vec<(i64, i64)> = get_seed_part_02()?;
//
//     println!("Calculating list of seeds");
//     let (seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location) = handle_retrieve_data()?;
//
//     let total_to_calc: i64 = seed.clone().into_iter().map(|x| x.1).sum::<i64>();
//     let mut total_tested: i64 = 0;
//
//     let mut smalles_value = -1;
//     for (index, s) in seed.iter().enumerate() {
//         println!("Calculating seed {}%", (index / seed.len()) * 100);
//         let mut i = s.1;
//         while i > 0 {
//             println!("testing {} -> {}%", s.0 + i, (total_tested / total_to_calc) * 100);
//             total_tested += 1;
//             let all_seeds = vec![s.0 + i];
//
//             let mut converted_seeds = all_seeds.clone().into_iter()
//                 .map(|x| convert_into_next_type(x, &seed_to_soil))
//                 .map(|x| convert_into_next_type(x, &soil_to_fertilizer))
//                 .map(|x| convert_into_next_type(x, &fertilizer_to_water))
//                 .map(|x| convert_into_next_type(x, &water_to_light))
//                 .map(|x| convert_into_next_type(x, &light_to_temperature))
//                 .map(|x| convert_into_next_type(x, &temperature_to_humidity))
//                 .map(|x| convert_into_next_type(x, &humidity_to_location))
//                 .collect::<Vec<i64>>();
//             let min = converted_seeds.into_iter().min();
//             match min {
//                 Some(x) => {
//                     if smalles_value == -1 || x < smalles_value {
//                         smalles_value = x;
//                     }
//                 }
//                 None => {}
//             }
//             i -= 1;
//         }
//     }
//
//     println!("Part 02: {}", smalles_value);
//
//     Ok(())
// }


// 24261545 -> Takes 42mn to calculate
fn part_02() -> std::io::Result<()> {
    let seed: Vec<(i64, i64)> = get_seed_part_02()?;

    println!("Calculating list of seeds");
    let (seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location) = handle_retrieve_data()?;


    let all_values: Vec<i64> = seed.iter().enumerate().map(|(index, (seed_value, seed_size))| {
        println!("Calculating seed {}%", (index * 100) / seed.len());
        let range = 0..*seed_size;
        let all_vals: Vec<i64> = range.into_iter().map(|y|
            {
                let converted_seeds: Vec<i64> = vec![seed_value + y]
                    .into_iter()
                    .map(|x| convert_into_next_type(x, &seed_to_soil))
                    .map(|x| convert_into_next_type(x, &soil_to_fertilizer))
                    .map(|x| convert_into_next_type(x, &fertilizer_to_water))
                    .map(|x| convert_into_next_type(x, &water_to_light))
                    .map(|x| convert_into_next_type(x, &light_to_temperature))
                    .map(|x| convert_into_next_type(x, &temperature_to_humidity))
                    .map(|x| convert_into_next_type(x, &humidity_to_location))
                    .collect();
                return converted_seeds[0];
            }).collect();
        let min = all_vals.into_iter().min();
        match min {
            Some(x) => {
                return x;
            }
            None => {
                return -1;
            }
        }
    }).collect();

    let smalles_value = all_values.into_iter().filter(|ee| *ee != -1).min();

    println!("Part 02: {}", smalles_value.unwrap_or(-1));

    Ok(())
}

fn main() {
    if let Ok(_) = part_01() {}
    if let Ok(_) = part_02() {}
}
