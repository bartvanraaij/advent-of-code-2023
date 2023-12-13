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
    parse_input(input)
        .iter()
        .filter_map(|pattern| pattern.find_reflection(None))
        .fold(0, |acc, res| acc + res.score())
}

fn part_2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter_map(|pattern| {
            let orig_reflection = pattern.find_reflection(None);

            for y in 0..pattern.height {
                for x in 0..pattern.width {
                    let changed_pattern = pattern.with_flipped_char_at(x, y);
                    match changed_pattern.find_reflection(orig_reflection.as_ref()) {
                        Some(new_result) => return Some(new_result),
                        None => {}
                    };
                }
            }
            None
        })
        .fold(0, |acc, res| acc + res.score())
}

fn parse_input(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern_str| Pattern::new(pattern_str))
        .collect_vec()
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

#[derive(Debug)]
struct Pattern {
    pattern: String,
    width: usize,
    height: usize,
}

impl Pattern {
    fn new(pattern_str: &str) -> Self {
        let pattern = pattern_str.trim().to_string();
        let height = pattern.chars().filter(|c| c == &'\n').count() + 1;
        let width = pattern.chars().take_while(|c| c != &'\n').count();

        Self {
            pattern,
            height,
            width,
        }
    }

    fn rotated(&self) -> Self {
        let lines = self
            .pattern
            .split("\n")
            .filter(|l| !l.is_empty())
            .collect_vec();

        let mut rotated_pattern = String::new();
        for x in 0..self.width {
            for line in lines.iter() {
                rotated_pattern += &line.chars().nth(x).unwrap().to_string();
            }
            rotated_pattern += "\n";
        }

        return Pattern::new(&rotated_pattern);
    }

    fn with_flipped_char_at(&self, flip_x: usize, flip_y: usize) -> Self {
        let mut new_pattern = String::new();

        for (y, line) in self
            .pattern
            .split("\n")
            .filter(|l| !l.is_empty())
            .enumerate()
        {
            for (x, char) in line.chars().enumerate() {
                if (flip_x, flip_y) == (x, y) {
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

        return Pattern::new(&new_pattern);
    }

    fn find_reflection_position(&self, skip_position: Option<usize>) -> Option<usize> {
        let mut last_line = "";
        let lines = self
            .pattern
            .split("\n")
            .filter(|l| !l.is_empty())
            .collect_vec();

        'outer: for (y, line) in lines.iter().enumerate() {
            if skip_position == Some(y) {
                last_line = line;
                continue;
            }

            if line == &last_line {
                for i in 0..(y) {
                    if (y + i) >= lines.len() {
                        break;
                    }

                    if lines[y + i] != lines[y - i - 1] {
                        last_line = line;
                        continue 'outer;
                    }
                }
                return Some(y);
            }

            last_line = line;
        }

        return None;
    }

    fn find_reflection(&self, skip_result: Option<&ReflectionResult>) -> Option<ReflectionResult> {
        let (skip_y, skip_x) = match skip_result {
            Some(result) => match result.orientation {
                ReflectionOrientation::H => (Some(result.i), None),
                ReflectionOrientation::V => (None, Some(result.i)),
            },
            None => (None, None),
        };

        match self.find_reflection_position(skip_y) {
            Some(y) => Some(ReflectionResult::new(y, ReflectionOrientation::H)),
            None => {
                let rotated = self.rotated();
                match rotated.find_reflection_position(skip_x) {
                    Some(x) => Some(ReflectionResult::new(x, ReflectionOrientation::V)),
                    None => None,
                }
            }
        }
    }
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
