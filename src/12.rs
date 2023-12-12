use cached::proc_macro::cached;
use cached::UnboundCache;
use itertools::Itertools;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/12");
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

fn cache_key(row: &str, config: &Vec<usize>) -> String {
    format!("{}x{}x", row, config.into_iter().join("_"))
}

#[cached(
    type = "UnboundCache<String, i64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ cache_key(row,config) }"#
)]
fn number_of_arrangements(
    row: &str,
    config: &Vec<usize>,
) -> i64 {
    let chars = row.chars().collect_vec();

    if row.starts_with(".") {
        let row_next_part = String::from_iter(&chars[1..]);
        return number_of_arrangements(&row_next_part, &config);
    }

    if chars.len() == 0 {
        if config.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if config.len() == 0 {
        // The config prescribes no more disabled springs, but we still have some, so this is not possible
        if chars.contains(&'#') {
            return 0;
        } else {
            // Only unknown or working springs remaining, possible!
            return 1;
        }
    }

    //Number of disabled springs to check for in this iteration
    let num_disabled_springs = config[0];

    // The row starts with a known damaged spring (#)
    if chars[0] == '#' {
        // The config prescribes more springs than spots are available, not possible:
        if chars.len() < num_disabled_springs {
            return 0;
        }

        // There is a working spring (.) in the remaining spots of the config length, not possible:
        if chars[..num_disabled_springs].contains(&'.') {
            return 0;
        }

        // Only one disabled spring left, but the config prescribes differently, not possible:
        if chars.len() == 1 && num_disabled_springs != 1 {
            return 0;
        }

        // Character after the end of this part is a #, that is not possible (should be . or ?)
        if chars.len() > num_disabled_springs && chars[num_disabled_springs] == '#' {
            return 0;
        }

        if chars.len() == num_disabled_springs {
            if config.len() == 1 {
                // This is the last group
                return 1;
            } else {
                return 0;
            }
        }

        // This config part looks good, move on to the next:
        let row_next_part = String::from_iter(&chars[(num_disabled_springs+1)..]);
        return number_of_arrangements(&row_next_part, &config[1..].to_vec());
    }
    // The row starts with an unknown spring (?)
    else {
        // Replace the first spot it with a working spring, and recursively check the rest:
        let row_with_working = String::from_iter(&chars[1..]);
        let num_arrangements_when_working =
            number_of_arrangements(&row_with_working, &config);

        let row_with_disabled = format!("{}{}", "#", String::from_iter(&chars[1..]));
        let num_arrangements_when_disabled =
            number_of_arrangements(&row_with_disabled, &config);

        return num_arrangements_when_working + num_arrangements_when_disabled;
    }
}

fn determine_number_of_arrangements(line: &str, unfold: usize) -> i64 {
    let (row_str_raw, config_str_raw) = line.split_once(" ").unwrap();

    let row_str_vec = vec![row_str_raw; unfold];
    let config_str_vec = vec![config_str_raw; unfold];

    let row = row_str_vec.join("&");
    let config_str = config_str_vec.join(",");
    let config = config_str
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect_vec();

    number_of_arrangements(&row, &config)
}

fn part_1(input: &str) -> i64 {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| determine_number_of_arrangements(line, 1))
        .sum()
}

fn part_2(input: &str) -> i64 {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| determine_number_of_arrangements(line, 5))
        .sum()
}

#[cfg(test)]
mod tests_12 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;

    #[test]
    fn test_num_arrangements() {
        assert_eq!(determine_number_of_arrangements("???.### 1,1,3", 1), 1);
        assert_eq!(
            determine_number_of_arrangements(".??..??...?##. 1,1,3", 1),
            4
        );
        assert_eq!(
            determine_number_of_arrangements("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            1
        );
        assert_eq!(
            determine_number_of_arrangements("????.#...#... 4,1,1", 1),
            1
        );
        assert_eq!(
            determine_number_of_arrangements("????.######..#####. 1,6,5", 1),
            4
        );
        assert_eq!(
            determine_number_of_arrangements("?###???????? 3,2,1", 1),
            10
        );
    }

    #[test]
    fn test_num_arrangements_unfolded() {
        assert_eq!(determine_number_of_arrangements("???.### 1,1,3", 5), 1);
        assert_eq!(
            determine_number_of_arrangements(".??..??...?##. 1,1,3", 5),
            16384
        );
        assert_eq!(
            determine_number_of_arrangements("?#?#?#?#?#?#?#? 1,3,1,6", 5),
            1
        );
        assert_eq!(
            determine_number_of_arrangements("????.#...#... 4,1,1", 5),
            16
        );
        assert_eq!(
            determine_number_of_arrangements("????.######..#####. 1,6,5", 5),
            2500
        );
        assert_eq!(
            determine_number_of_arrangements("?###???????? 3,2,1", 5),
            506250
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 525152);
    }
}
