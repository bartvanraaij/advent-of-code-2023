use itertools::Itertools;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/09");
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

fn extrapolate_sequence(sequence: Vec<i64>) -> (i64,i64) {
    // If all numbers are 0, we are done!
    if (&sequence).into_iter().all(|num| *num == 0) {
        return (0,0);
    }

    let last_number = (&sequence).last().unwrap();
    let first_number = (&sequence).first().unwrap();

    let next_sequence = (&sequence)
        .into_iter()
        .tuple_windows() // Takes the current and next value, puts them in a tuple
        .map(|(cur, next)| next - cur) // Substract the current value from the next
        .collect_vec();

    let (next_forward_extrapolation,next_backward_extrapolation) = extrapolate_sequence(next_sequence);

    (*last_number + next_forward_extrapolation, *first_number - next_backward_extrapolation)
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(" ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn part_1(input: &str) -> i64 {
    let sequences = parse_input(input);
    let extrapolations: Vec<i64> = sequences
        .into_iter()
        .map(|s| extrapolate_sequence(s).0)
        .collect();

    extrapolations.into_iter().sum()
}

fn part_2(input: &str) -> i64 {
    let sequences = parse_input(input);
    let extrapolations: Vec<i64> = sequences
        .into_iter()
        .map(|s| extrapolate_sequence(s).1)
        .collect();

    extrapolations.into_iter().sum()
}

#[cfg(test)]
mod tests_00 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 114);
    }

    #[test]
    fn test_single() {
        assert_eq!(part_1("0 3 6 9 12 15"), 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 2);
    }
}
