use std::env::Args;
use std::{env, fs};

fn read_input_file(args: Args) -> String {
    let args_strings = args.collect::<Vec<String>>();
    let default_input_filename = &String::from("input.txt");
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

fn part_1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|digits| digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0))
        .sum()
}

fn get_all_line_numbers(line: &str) -> Vec<u32> {
    let mut numbers = Vec::new();

    for i in 0..line.len() {
        let char_at_index = line.chars().nth(i).unwrap();
        match char_at_index.to_digit(10) {
            Some(n) => numbers.push(n),
            None => {
                if line[i..].starts_with("one") {
                    numbers.push(1);
                } else if line[i..].starts_with("two") {
                    numbers.push(2);
                } else if line[i..].starts_with("three") {
                    numbers.push(3);
                } else if line[i..].starts_with("four") {
                    numbers.push(4);
                } else if line[i..].starts_with("five") {
                    numbers.push(5);
                } else if line[i..].starts_with("six") {
                    numbers.push(6);
                } else if line[i..].starts_with("seven") {
                    numbers.push(7);
                } else if line[i..].starts_with("eight") {
                    numbers.push(8);
                } else if line[i..].starts_with("nine") {
                    numbers.push(9);
                }
            }
        }
    }

    numbers
}

fn part_2(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line| get_all_line_numbers(line))
        .map(|digits| digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_SAMPLE_DATA: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const PART_2_SAMPLE_DATA: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(PART_1_SAMPLE_DATA), 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(PART_2_SAMPLE_DATA), 281);
    }
}
