use itertools::Itertools;
use std::collections::HashMap;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/03");
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

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct XY(u32, u32);

impl XY {
    fn right(self) -> XY {
        XY(self.0 + 1, self.1)
    }
    fn left(self) -> XY {
        XY(self.0 - 1, self.1)
    }
    fn top(self) -> XY {
        XY(self.0, self.1 - 1)
    }
    fn bottom(self) -> XY {
        XY(self.0, self.1 + 1)
    }
    fn top_left(self) -> XY {
        XY(self.0 - 1, self.1 - 1)
    }
    fn top_right(self) -> XY {
        XY(self.0 + 1, self.1 - 1)
    }
    fn bottom_left(self) -> XY {
        XY(self.0 - 1, self.1 + 1)
    }
    fn bottom_right(self) -> XY {
        XY(self.0 + 1, self.1 + 1)
    }
    fn surround(self) -> Vec<XY> {
        let mut vec = Vec::from([
            self.right(),
            self.bottom_right(),
            self.bottom(),
        ]);
        if self.0 > 0 {
            vec.push(self.left());
            vec.push(self.bottom_left());
        }
        if self.1 > 0 {
            vec.push(self.top());
            vec.push(self.top_right());
        }
        if self.0 > 0 && self.1 > 0 {
            vec.push(self.top_left());
        }
        vec
    }
}

#[derive(Debug, Clone)]
struct Symbol {
    char: String,
    pos: XY,
}

#[derive(Debug)]
struct Number {
    number: u32,
    pos: XY,
}

impl Number {
    fn positions(&self) -> Vec<XY> {
        let mut positions: Vec<XY> = Vec::new();
        positions.push(self.pos);
        if self.number >= 10 {
            let right = self.pos.right();
            positions.push(right);
            if self.number >= 100 {
                positions.push(right.right());
            }
        }

        positions
    }

    fn surrounding_positions(&self) -> Vec<XY> {
        self.positions()
            .iter()
            .flat_map(|pos| pos.surround())
            .collect::<Vec<XY>>()
    }
}

struct Schematic {
    symbols: HashMap<XY, Symbol>,
    numbers: HashMap<XY, Number>,
}

fn parse_input(input: &str) -> Schematic {
    let symbols = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            return line
                .chars()
                .enumerate()
                .filter(|(_, char)| !char.is_numeric() && &char.to_string() != ".")
                .map(move |(x, char)| {
                    let xy = XY(x.try_into().unwrap(), y.try_into().unwrap());
                    (
                        xy,
                        Symbol {
                            char: char.to_string(),
                            pos: xy,
                        },
                    )
                });
        })
        .collect::<HashMap<_, _>>();

    let numbers = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, char)| {
                if char.is_numeric() {
                    if x > 0 {
                        let prev_char = line.as_bytes()[x - 1] as char;
                        if prev_char.is_numeric() {
                            return None;
                        }
                    }

                    let mut number = char.to_string();
                    let next_char = line.as_bytes()[x + 1] as char;
                    if next_char.is_numeric() {
                        number = number + &next_char.to_string();
                        let next_next_char = line.as_bytes()[x + 2] as char;

                        if next_next_char.is_numeric() {
                            number = number + &next_next_char.to_string();
                        }
                    }

                    let xy = XY(x.try_into().unwrap(), y.try_into().unwrap());
                    return Some((
                        xy,
                        Number {
                            number: number.parse::<u32>().unwrap(),
                            pos: xy,
                        },
                    ));
                }

                return None;
            })
        })
        .collect::<HashMap<_, _>>();

    return Schematic { symbols, numbers };

    /*for (y, line) in lines.iter().enumerate() {
        dbg!(y);
        dbg!(line);
        for
    }*/
}

fn part_1(input: &str) -> u32 {
    let schematic = parse_input(input);
    
    schematic.numbers
        .values()
        .filter_map(|num| {
            let is_clear = num
                .surrounding_positions()
                .into_iter()
                .any(|pos| schematic.symbols.contains_key(&pos));

            if is_clear {
                Some(num.number)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests_03 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 4361);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 467835);
    }
}
