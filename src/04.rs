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
    0
}

fn part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests_04 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 0);
    }
}
