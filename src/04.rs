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
            let lists = l
                .split("|")
                .map(|p| {
                    p.trim()
                        .split(" ")
                        .flat_map(|s| s.parse::<u32>())
                        .collect::<HashSet<u32>>()
                })
                .collect::<Vec<_>>();
            let winning_numbers = lists.get(0).unwrap();
            let our_numbers = lists.get(1).unwrap();

            let intersect = winning_numbers.intersection(&our_numbers).collect::<Vec<_>>();

            let num_wins = intersect.len();

            match num_wins {
                0 => 0,
                1 => 1,
                _ => 2_u32.pow((num_wins as u32)-1)
            }
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    0
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
        assert_eq!(part_2(SAMPLE_DATA), 0);
    }
}
