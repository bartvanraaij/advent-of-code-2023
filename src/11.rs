use itertools::Itertools;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/11");
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

fn manhattan_distance((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> usize {
    ((ax as i64 - bx as i64).abs() + (ay as i64 - by as i64).abs())
        .try_into()
        .unwrap()
}

fn sum_shortest_paths(input: &str, expansion_multiplier: usize) -> usize {
    let mut galaxies = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            return line
                .chars()
                .enumerate()
                .filter_map(move |(x, char)| match char {
                    '#' => Some((x, y)),
                    _ => None,
                });
        })
        .collect::<Vec<(usize, usize)>>();

    let all_x = (&galaxies)
        .into_iter()
        .map(|(x, _y)| *x)
        .sorted()
        .dedup()
        .collect_vec();
    let all_y = (&galaxies)
        .into_iter()
        .map(|(_x, y)| *y)
        .sorted()
        .dedup()
        .collect_vec();

    let width = (&all_x).into_iter().max().unwrap() + 1;
    let height = (&all_y).into_iter().max().unwrap() + 1;

    let empty_x = (0..width)
        .into_iter()
        .filter(|x| !all_x.contains(x))
        .collect_vec();

    let empty_y = (0..height)
        .into_iter()
        .filter(|y| !all_y.contains(y))
        .collect_vec();

    // Move all galaxies
    for (x, y) in galaxies.iter_mut() {
        let num_shift_right = empty_x.iter().filter(|&nx| *nx < *x).count();
        let num_shift_down = empty_y.iter().filter(|&ny| *ny < *y).count();
        *x += num_shift_right * (expansion_multiplier - 1);
        *y += num_shift_down * (expansion_multiplier - 1);
    }

    let mut distances = 0;
    for combination in (&galaxies).into_iter().combinations_with_replacement(2) {
        distances += manhattan_distance(*combination[0], *combination[1]);
    }

    distances as usize
}

fn part_1(input: &str) -> usize {
    sum_shortest_paths(input, 2)
}

fn part_2(input: &str) -> usize {
    sum_shortest_paths(input, 1_000_000)
}

#[cfg(test)]
mod tests_11 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

    #[test]
    fn test_expansion() {
        assert_eq!(sum_shortest_paths(SAMPLE_DATA, 2), 374);
        assert_eq!(sum_shortest_paths(SAMPLE_DATA, 10), 1030);
        assert_eq!(sum_shortest_paths(SAMPLE_DATA, 100), 8410);
    }
}
