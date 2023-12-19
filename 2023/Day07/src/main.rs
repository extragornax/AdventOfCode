use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardCategory {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Clone)]
struct CardLine {
    pub cards: Vec<char>,
    pub score: u32,
}

#[derive(Debug, Clone)]
struct OrderCard {
    pub cards: CardLine,
    pub category: CardCategory,
}

// Define your char-based strength calculation function
fn char_strength(c: char) -> u32 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => todo!(),
    }
}


fn are_all_diff_test(_data: Vec<char>) -> bool {
    let mut _t = _data.clone();

    _t.dedup();

    _t.len() == _data.len()
}

fn categorise_cards_group(_data: Vec<char>) -> CardCategory {
    let mut map: HashMap<char, i64> = HashMap::new();

    _data.clone()
        .into_iter().for_each(|x| {
        if map.contains_key(&x) {
            let val = map.get(&x).unwrap();
            map.insert(x, val + 1);
        } else {
            map.insert(x, 1);
        }
    });

    for x in map.keys() {
        let val = map.get(x).unwrap();
        if *val == 5 {
            return CardCategory::FiveOfAKind;
        } else if *val == 4 {
            return CardCategory::FourOfAKind;
        } else if *val == 3 {
            // If 3 cards are the same, check if the last 2 are different
            let remain = _data.clone().into_iter().filter(|y| y != x).collect::<Vec<char>>();
            if remain[0] == remain[1] {
                return CardCategory::FullHouse;
            } else {
                return CardCategory::ThreeOfAKind;
            }
        } else if *val == 2 {
            // return CardCategory::OnePair;
            // If 2 cards are the same, check if the last 3 are different
            let remain = _data.clone().into_iter().filter(|y| y != x).collect::<Vec<char>>();
            if remain[0] == remain[1]
                || remain[0] == remain[2]
                || remain[1] == remain[2] {
                return CardCategory::TwoPairs;
            } else if are_all_diff_test(remain) {
                return CardCategory::OnePair;
            }
        } else if *val == 1 {
            if are_all_diff_test(_data.clone()) {
                return CardCategory::HighCard;
            }
        }
    };

    CardCategory::HighCard
}

fn part_01() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let split = contents.split("\n");

    let mut all_cards: Vec<CardLine> = Vec::new();

    for line in split {
        if line.len() == 0 {
            continue;
        }

        let s = line.split_whitespace();
        let cards: Vec<char> = s.clone().nth(0).unwrap().chars().collect();
        let score: u32 = s.clone().nth(1).unwrap().parse().unwrap();
        all_cards.push(CardLine { cards, score });
    }
    let mut categorized: HashMap<CardCategory, Vec<CardLine>> = HashMap::new();

    all_cards.into_iter().for_each(|x| {
        let cat = categorise_cards_group(x.clone().cards);
        if categorized.contains_key(&cat) {
            let mut val = categorized.get(&cat).unwrap().clone();
            val.push(x.clone());
            categorized.insert(cat.clone(), val);
        } else {
            categorized.insert(cat.clone(), vec![x.clone()]);
        }
    });

    let mut all_cards_sorted: Vec<CardLine> = Vec::new();

    let mut cat_keys: Vec<CardCategory> = categorized.keys().into_iter().map(|x| x.clone()).collect();
    cat_keys.sort_by(|a, b| {
        let _a = a.clone() as i64;
        let _b = b.clone() as i64;
        _a.cmp(&_b)
    });


    for key in cat_keys {
        let mut val = categorized.get(&key).unwrap().clone();
        val.sort_by(|a, b| {
            let index = a.cards.len() - 1;
            for i in 0..index {
                let _a = char_strength(a.cards[i]);
                let _b = char_strength(b.cards[i]);
                if _a < _b {
                    return std::cmp::Ordering::Less;
                } else if _a > _b {
                    return std::cmp::Ordering::Greater;
                }
            }
            std::cmp::Ordering::Equal
        });
        all_cards_sorted.append(&mut val);
    }

    let total_score = all_cards_sorted.iter().enumerate().map(|(rank, x)| {
        println!("Rank: {:?}, Score: {:?}, Cards: {:?}", rank + 1, x.score, x.cards.iter().collect::<String>());
        (rank + 1) as u32 * x.score
    }).sum::<u32>();

    println!("Total Score: {:?}", total_score);

    let test_max = 252719368;
    println!("too high ? {}", total_score > test_max);

    Ok(())
}

fn part_02() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let split = contents.split("\n");

    for line in split {}

    Ok(())
}


fn main() {
    if let Ok(_) = part_01() {}
    if let Ok(_) = part_02() {}
}
