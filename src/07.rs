use itertools::Itertools;
use std::{env, fs, cmp::Ordering::{Equal, Less,Greater}};

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

#[derive(Debug)]
struct Card {
    char: String,
    value: u32,
}

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
        Self {
            char: char.to_string(),
            value,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid_amount: u32,
}

impl Hand {
    fn new(cards: Vec<Card>, bid_amount: u32) -> Self {
        let hand_type = Hand::get_type(&cards);
        Self { cards, 
            bid_amount,
            hand_type
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

    fn strength(&self) -> u32 {
        match &self.hand_type {
            HandType::HighCard => 1,
            HandType::OnePair => 2,
            HandType::TwoPair => 3,
            HandType::ThreeOfAKind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfAKind => 6,
            HandType::FiveOfAKind => 7,
        }
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

            dbg!(&hand);
            dbg!(&hand.strength());

            return hand;
        })
        .sorted_by(|a, b| match Ord::cmp(&a.strength(), &b.strength()) {
            Equal => {
                for n in 0..=4 {
                    let cmp = Ord::cmp(&a.cards[n].value, &b.cards[n].value);
                    dbg!(&a.cards[n].value);
                    dbg!(&b.cards[n].value);
                    if cmp == Less {
                        return Less;
                    }
                    if cmp == Greater {
                        return Greater;
                    }
                } 

                Equal
            },
            x => x,
        })
        .enumerate()
        .inspect(|(i, hand)| {

            let rank = (*i as u32) +1;
            dbg!(&rank);
            dbg!(&hand);
        })
        .map(|(i, hand)| {
            let rank = (i as u32) +1;
            let winning = rank * hand.bid_amount;

            winning
        })
        .sum()
        
    

}

fn part_2(input: &str) -> u32 {
    0
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

    /*
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 0);
    }
    */
}
