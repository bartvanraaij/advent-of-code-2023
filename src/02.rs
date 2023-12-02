use regex::Regex;
use std::cell::Cell;
use std::env::Args;
use std::{env, fs};

fn read_input_file(args: Args) -> String {
    let args_strings = args.collect::<Vec<String>>();
    let default_input_filename = &String::from("input/02");
    let input_filepath: &str = args_strings.get(1).unwrap_or(default_input_filename);
    fs::read_to_string(input_filepath).expect("input file should be readable")
}

fn main() {
    let input = read_input_file(env::args());
    let result_part_1 = part_1(&input);
    println!("{:?}", result_part_1);

    let result_part_2 = part_2(&input);
    println!("{:?}", result_part_2);
}

#[derive(Debug)]
struct Game {
    id: u32,
    red: Cell<u32>,
    green: Cell<u32>,
    blue: Cell<u32>,
}

impl Game {
    fn new(id: u32) -> Game {
        Game {
            id,
            red: Into::into(0),
            green: Into::into(0),
            blue: Into::into(0),
        }
    }

    fn set_max(&self, amount: u32, colour: &str) {
        if colour == "red" {
            if self.red.get() < amount {
                self.red.set(amount);
            }
        }
        if colour == "green" {
            if self.green.get() < amount {
                self.green.set(amount);
            }
        }
        if colour == "blue" {
            if self.blue.get() < amount {
                self.blue.set(amount);
            }
        }
    }

    fn pow(&self) -> u32 {
        self.red.get() * self.green.get() * self.blue.get()
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let re = Regex::new(r"Game (?<game_id>\d+): ").unwrap();
            let caps = re.captures(line).unwrap();
            let game_id = caps
                .name("game_id")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            let game_str = re.replace(line, "");

            let game = Game::new(game_id);
            game_str
                .split(";")
                .map(|line| line.trim())
                .flat_map(|line| {
                    let parts = line.split(", ").filter(|l| !l.is_empty()).map(|part| {
                        let re = Regex::new(r"(?<amount>\d+) (?<colour>(red|green|blue))").unwrap();
                        let caps = re.captures(part).unwrap();
                        let amount = caps
                            .name("amount")
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap();

                        let colour = caps.name("colour").unwrap().as_str();

                        (colour, amount)
                    });

                    parts
                })
                .for_each(|(colour, amount)| {
                    game.set_max(amount, colour);
                });

            game
        })
        .collect()
}

fn part_1(input: &str) -> u32 {
    parse_games(input)
        .iter()
        .filter(|game| game.red.get() <= 12 && game.green.get() <= 13 && game.blue.get() <= 14)
        .map(|game| game.id)
        .sum()
}

fn part_2(input: &str) -> u32 {
    parse_games(input).iter().map(|game| game.pow()).sum()
}

#[cfg(test)]
mod tests_02 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 2286);
    }
}
