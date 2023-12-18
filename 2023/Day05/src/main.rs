use std::fs::File;
use std::io::{BufReader, Read};

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

fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let split = contents.split("\n");
    let mut seed: Vec<i64> = Vec::new();
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

        if line.starts_with("seeds") {
            let _split = line.split(" ");
            let seeds = _split.into_iter().map(|x| x.parse::<i64>().unwrap_or(-1)).collect::<Vec<i64>>();
            seeds.into_iter().filter(|x| *x != -1).for_each(|x| seed.push(x));
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

    // println!("converted_seeds-> {:?}", converted_seeds);
    // println!("seed-> {:?}", seed);
    // println!("seed_to_soil-> {:?}", seed_to_soil);
    // println!("soil_to_fertilizer-> {:?}", soil_to_fertilizer);
    // println!("fertilizer_to_water-> {:?}", fertilizer_to_water);
    // println!("water_to_light-> {:?}", water_to_light);
    // println!("light_to_temperature-> {:?}", light_to_temperature);
    // println!("temperature_to_humidity-> {:?}", temperature_to_humidity);

    let result = match converted_seeds.clone().first() {
        Some(x) => x.clone(),
        None => -1,
    };

    println!("Part 01: {}", result);

    Ok(())
}

fn main() {
    if let Ok(_) = part_01() {}
}
