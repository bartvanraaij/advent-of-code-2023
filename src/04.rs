use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/04");
    let input_filepath: &str = args.get(1).unwrap_or(default_input_filename);
    fs::read_to_string(input_filepath).expect("input file should be readable")
}

fn main() {
    let input = read_input_file(env::args().collect());
    let result_part_1 = part_1(&input);
    println!("{:?}", result_part_1);

    let result_part_2 = part_2(&input);
    println!("{:?}", result_part_2);
}

fn part_1(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split(":").nth(1).unwrap())
        .map(|l| {
            l.split("|")
                .map(|p| {
                    p.trim()
                        .split(" ")
                        .flat_map(|s| s.parse::<u32>())
                        .collect::<HashSet<u32>>()
                })
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|c| {
                    let intersect = &c[0].intersection(&c[1]).collect::<Vec<_>>();
                    let num_wins = intersect.len();

                    match num_wins {
                        0 => 0,
                        _ => 2_u32.pow((num_wins as u32) - 1),
                    }
                })
                .exactly_one()
                .unwrap()
        })
        .sum()
}

#[derive(Debug)]
struct Card {
    number: u32,
    winning: HashSet<u32>,
    ours: HashSet<u32>,
}

impl Card {
    fn new(number: u32, winning: &HashSet<u32>, ours: &HashSet<u32>) -> Card {
        Card {
            number,
            winning: winning.clone(),
            ours: ours.clone(),
        }
    }

    fn score(&self) -> u32 {
        self.winning
            .intersection(&self.ours)
            .collect::<Vec<_>>()
            .len() as u32
    }
}

fn part_2(input: &str) -> u32 {
    let cards = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, l)| {
            let numbers_str = l.split(":").nth(1).unwrap();
            let numbers_vecs = numbers_str
                .split("|")
                .map(|p| {
                    p.trim()
                        .split(" ")
                        .flat_map(|s| s.parse::<u32>())
                        .collect::<HashSet<u32>>()
                })
                .collect::<Vec<_>>();

            return (
                (i as u32 + 1),
                Card::new(
                    i as u32 + 1,
                    numbers_vecs.get(0).unwrap(),
                    numbers_vecs.get(1).unwrap(),
                ),
            );
        })
        .collect::<HashMap<u32, Card>>();

    let mut multipliers = cards
        .iter()
        .map(|(i, _)| (*i, 1 as u32))
        .collect::<HashMap<_, _>>();

    for card_num in 1..cards.len() {
        let card = cards.get(&(card_num as u32)).unwrap();
        let this_card_mp = *multipliers.get(&(card_num as u32)).unwrap();

        for win in 1..=(card.score()) {
            let card_to_up_num = &card.number + win;
            multipliers
                .entry(card_to_up_num)
                .and_modify(|n| *n += this_card_mp);
        }
    }

    let s: u32 = multipliers.values().sum::<u32>();

    s
}

#[cfg(test)]
mod tests_04 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 30);
    }
}
