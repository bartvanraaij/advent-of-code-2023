use itertools::Itertools;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/10");
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

fn part_1(input: &str) -> usize {
    //input.split("\n").filter(|l| !l.is_empty()).map(|line| {});


    0
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests_10 {
    use super::*;

    const SAMPLE_DATA_1: &str = r#"
.....
.S-7.
.|.|.
.L-J.
.....
"#;

  const SAMPLE_DATA_2: &str = r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#;

    const SAMPLE_DATA_3: &str = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    const SAMPLE_DATA_4: &str = r#"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"#;
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA_1), 4);
        assert_eq!(part_1(SAMPLE_DATA_2), 4);
        assert_eq!(part_1(SAMPLE_DATA_3), 8);
        assert_eq!(part_1(SAMPLE_DATA_4), 8);
    }

    #[test]
    fn test_part_2() {
        //assert_eq!(part_2(SAMPLE_DATA), 0);
    }
}
