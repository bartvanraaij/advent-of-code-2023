use itertools::Itertools;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/13");
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
    input
        .split("\n\n")
        .filter_map(|p| pattern_result(p, &None))
        .map(|r| r.score())
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(|pattern| {
            let pattern_lines = pattern.split("\n").filter(|l| !l.is_empty()).collect_vec();
            let pattern_width = pattern_lines[0].len();
            let pattern_height = pattern_lines.len();

            let orig_pattern_result = Some(pattern_result(pattern, &None).unwrap());

            for y in 0..pattern_height {
                for x in 0..pattern_width {
                    let changed_pattern = flip_char_at(pattern, (x, y));

                    match pattern_result(&*changed_pattern, &orig_pattern_result) {
                        Some(new_result) => return Some(new_result),
                        None => {}
                    };
                }
            }

            None
        })
        .map(|r| r.score())
        .sum()
}

fn find_reflection_position(pattern: &str, skip_if: usize) -> usize {
    let mut last_line = "";
    let lines = pattern.split("\n").filter(|l| !l.is_empty()).collect_vec();
    'outer: for (y, line) in lines.iter().enumerate() {
        if y == skip_if {
            last_line = line;
            continue;
        }

        if line == &last_line {
            for i in 0..(y - 1) {
                if (y + i + 1) >= lines.len() {
                    break;
                }

                if lines[y + i + 1] != lines[y - i - 2] {
                    last_line = line;
                    continue 'outer;
                }
            }
            return y;
        }

        last_line = line;
    }

    return 0;
}

enum ReflectionOrientation {
    H,
    V,
}

struct ReflectionResult {
    i: usize,
    orientation: ReflectionOrientation,
}

impl ReflectionResult {
    fn new(i: usize, orientation: ReflectionOrientation) -> Self {
        Self { i, orientation }
    }

    fn score(&self) -> usize {
        match self.orientation {
            ReflectionOrientation::H => self.i * 100,
            _ => self.i,
        }
    }
}

fn pattern_result(
    pattern: &str,
    skip_result: &Option<ReflectionResult>,
) -> Option<ReflectionResult> {
    let skip_y = match &skip_result {
        Some(s) => match s.orientation {
            ReflectionOrientation::H => s.i,
            _ => 0,
        },
        None => 0,
    };

    let y = find_reflection_position(pattern, skip_y);

    if y > 0 {
        return Some(ReflectionResult::new(y, ReflectionOrientation::H));
    }

    let skip_x = match &skip_result {
        Some(s) => match s.orientation {
            ReflectionOrientation::V => s.i,
            _ => 0,
        },
        None => 0,
    };
    let rotated = rotate_pattern(pattern);
    let x = find_reflection_position(&*rotated, skip_x);

    if x > 0 {
        return Some(ReflectionResult::new(x, ReflectionOrientation::V));
    }

    return None;
}

fn rotate_pattern(pattern: &str) -> String {
    let lines = pattern.split("\n").filter(|l| !l.is_empty()).collect_vec();
    let line_length = lines[0].len();

    let mut rotated_pattern = String::new();
    for x in 0..line_length {
        for line in lines.iter() {
            rotated_pattern += &line.chars().nth(x).unwrap().to_string();
        }
        rotated_pattern += "\n";
    }

    rotated_pattern
}

fn flip_char_at(pattern: &str, (xs, ys): (usize, usize)) -> String {
    let lines = pattern.split("\n").filter(|l| !l.is_empty()).collect_vec();

    let mut new_pattern = String::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if (xs, ys) == (x, y) {
                if char == '#' {
                    new_pattern += ".";
                } else {
                    new_pattern += "#";
                }
            } else {
                new_pattern += &char.to_string();
            }
        }
        new_pattern += "\n";
    }

    new_pattern
}

#[cfg(test)]
mod tests_13 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 405);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 400);
    }
}
