use regex::Regex;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/02");
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
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

struct RGB {
    r: u32,
    g: u32,
    b: u32,
}

fn parse_games(input: &str) -> Vec<Game> {
    let game_id_regex = Regex::new(r"Game (?<game_id>\d+): ").unwrap();
    let cube_counts_regex = Regex::new(r"(?<amount>\d+) (?<colour>(red|green|blue))").unwrap();

    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let caps = game_id_regex.captures(line).unwrap();
            let game_id = caps
                .name("game_id")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();

            let mut rgb = RGB { r: 0, g: 0, b:0 };
            line.split(";")
                .map(|line| line.trim())
                .flat_map(|line| {
                    line.split(", ").filter(|l| !l.is_empty()).map(|part| {
                        let caps = cube_counts_regex.captures(part).unwrap();
                        let amount = caps
                            .name("amount")
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap();

                        let colour = caps.name("colour").unwrap().as_str();

                        (colour, amount)
                    })
                })
                .for_each(|(colour, amount)| {
                    if colour == "red" && amount > rgb.r {
                        rgb.r = amount
                    };
                    if colour == "green" && amount > rgb.g {
                        rgb.g = amount
                    };
                    if colour == "blue" && amount > rgb.b {
                        rgb.b = amount
                    };
                });

            Game {
                id: game_id,
                red: rgb.r,
                green: rgb.g,
                blue: rgb.b,
            }
        })
        .collect()
}

fn part_1(input: &str) -> u32 {
    parse_games(input)
        .iter()
        .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .map(|game| game.id)
        .sum()
}

fn part_2(input: &str) -> u32 {
    parse_games(input)
        .iter()
        .map(|game| game.red * game.green * game.blue)
        .sum()
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
