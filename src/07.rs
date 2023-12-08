use im::Vector;
use itertools::Itertools;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    env, fs,
};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/07");
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

#[derive(Debug, Clone)]
struct Card {
    char: String,
    value: u32,
    value_j: u32,
}

const CHARS: [&str; 12] = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2"];

impl Card {
    fn new(char: &str) -> Self {
        let value = match char {
            "A" => 13,
            "K" => 12,
            "Q" => 11,
            "J" => 10,
            "T" => 9,
            "9" => 8,
            "8" => 7,
            "7" => 6,
            "6" => 5,
            "5" => 4,
            "4" => 3,
            "3" => 2,
            "2" => 1,
            _ => 0,
        };
        let value_j = match char {
            "A" => 13,
            "K" => 12,
            "Q" => 11,
            "T" => 10,
            "9" => 9,
            "8" => 8,
            "7" => 7,
            "6" => 6,
            "5" => 5,
            "4" => 4,
            "3" => 3,
            "2" => 2,
            "J" => 1,
            _ => 0,
        };

        Self {
            char: char.to_string(),
            value,
            value_j,
        }
    }

    fn is_joker(&self) -> bool {
        self.char.eq("J")
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    hand_type_j: HandType,
    bid_amount: u32,
}

impl Hand {
    fn new(cards: Vec<Card>, bid_amount: u32) -> Self {
        let hand_type = Hand::get_type(&cards);
        let hand_type_j = Hand::get_type_j(&cards);
        Self {
            cards,
            bid_amount,
            hand_type,
            hand_type_j,
        }
    }

    fn get_type(cards: &Vec<Card>) -> HandType {
        let mut cards_grouped: Vec<(u32, Vec<&Card>)> = Vec::new();
        for (key, group) in &(&cards)
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.value, &a.value))
            .group_by(|c| c.value)
        {
            cards_grouped.push((key, group.collect()));
        }

        cards_grouped.sort_by(|a, b| Ord::cmp(&b.1.len(), &a.1.len()));

        let num_groups = cards_grouped.len() as u32;

        if num_groups == 1 {
            return HandType::FiveOfAKind;
        }
        if num_groups == 2 {
            if cards_grouped.first().unwrap().1.len() as u32 == 4 {
                return HandType::FourOfAKind;
            }
            return HandType::FullHouse;
        }
        if num_groups == 3 {
            if cards_grouped.first().unwrap().1.len() as u32 == 3 {
                return HandType::ThreeOfAKind;
            }
            return HandType::TwoPair;
        }

        if num_groups == 4 {
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }

    fn get_type_x(cards: &Vector<Card>) -> HandType {
        let mut cards_grouped: Vec<(u32, Vec<&Card>)> = Vec::new();
        for (key, group) in &(&cards)
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.value, &a.value))
            .group_by(|c| c.value)
        {
            cards_grouped.push((key, group.collect()));
        }

        cards_grouped.sort_by(|a, b| Ord::cmp(&b.1.len(), &a.1.len()));

        let num_groups = cards_grouped.len() as u32;

        if num_groups == 1 {
            return HandType::FiveOfAKind;
        }
        if num_groups == 2 {
            if cards_grouped.first().unwrap().1.len() as u32 == 4 {
                return HandType::FourOfAKind;
            }
            return HandType::FullHouse;
        }
        if num_groups == 3 {
            if cards_grouped.first().unwrap().1.len() as u32 == 3 {
                return HandType::ThreeOfAKind;
            }
            return HandType::TwoPair;
        }

        if num_groups == 4 {
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }

    fn get_type_j(cards: &Vec<Card>) -> HandType {
        if !cards.into_iter().any(|c| c.is_joker()) {
            return Hand::get_type(cards);
        }

        // Determine the highest possible value of J
        let mut highest_score = 0;
        let mut highest_type: HandType = HandType::HighCard;

        //let cards_n = Vector::from(cards);

        // Jokers
        let num_jokers = cards
            .into_iter()
            .filter(|j| j.is_joker())
            .collect_vec()
            .len();
        let joker_combinations = CHARS
            .into_iter()
            .combinations_with_replacement(num_jokers)
            .collect_vec();
        let mut joker_indexes: Vec<usize> = Vec::new();

        for (pos, card) in cards.iter().enumerate() {
            if card.is_joker() {
                joker_indexes.push(pos);
            }
        }

        for joker_comb in joker_combinations {
            let mut new_cards = Vector::from(cards);

            for (n, new_char) in joker_comb.iter().enumerate() {
                let joker_pos = joker_indexes[n];

                let new_card = Card::new(new_char);
                new_cards = new_cards.update(joker_pos, new_card);
            }

            let new_type = Hand::get_type_x(&new_cards);
            let new_score = hand_type_strength(&new_type);

            if new_score > highest_score {
                highest_score = new_score;
                highest_type = new_type;
            }
        }

        highest_type
    }

    fn strength(&self) -> u32 {
        let ht = &(*self).hand_type;
        hand_type_strength(ht)
    }

    fn strength_j(&self) -> u32 {
        let ht = &(*self).hand_type_j;
        hand_type_strength(ht)
    }
}

fn hand_type_strength(hand_type: &HandType) -> u32 {
    match hand_type {
        HandType::HighCard => 1,
        HandType::OnePair => 2,
        HandType::TwoPair => 3,
        HandType::ThreeOfAKind => 4,
        HandType::FullHouse => 5,
        HandType::FourOfAKind => 6,
        HandType::FiveOfAKind => 7,
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn part_1(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (cards_str, bid_amount_str) = l.split(" ").tuples().exactly_one().unwrap();

            let cards = cards_str
                .chars()
                .map(|c| Card::new(&c.to_string()))
                .collect_vec();

            let bid_amount = bid_amount_str.parse::<u32>().unwrap();

            let hand = Hand::new(cards, bid_amount);

            hand
        })
        .sorted_by(|a, b| match Ord::cmp(&a.strength(), &b.strength()) {
            Equal => {
                for n in 0..=4 {
                    let cmp = Ord::cmp(&a.cards[n].value, &b.cards[n].value);
                    if cmp == Less {
                        return Less;
                    }
                    if cmp == Greater {
                        return Greater;
                    }
                }

                Equal
            }
            x => x,
        })
        .enumerate()
        .map(|(i, hand)| {
            let rank = (i as u32) + 1;
            let winning = rank * hand.bid_amount;

            winning
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (cards_str, bid_amount_str) = l.split(" ").tuples().exactly_one().unwrap();

            let cards = cards_str
                .chars()
                .map(|c| Card::new(&c.to_string()))
                .collect_vec();

            let bid_amount = bid_amount_str.parse::<u32>().unwrap();

            let hand = Hand::new(cards, bid_amount);

            hand
        })
        .sorted_by(|a, b| match Ord::cmp(&a.strength_j(), &b.strength_j()) {
            Equal => {
                for n in 0..=4 {
                    let cmp = Ord::cmp(&a.cards[n].value_j, &b.cards[n].value_j);
                    if cmp == Less {
                        return Less;
                    }
                    if cmp == Greater {
                        return Greater;
                    }
                }

                Equal
            }
            x => x,
        })
        .enumerate()
        .map(|(i, hand)| {
            let rank = (i as u32) + 1;
            let winning = rank * hand.bid_amount;

            winning
        })
        .sum()
}

#[cfg(test)]
mod tests_07 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 5905);
    }
}
