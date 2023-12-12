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

fn number_of_arrangements(row_str: &str, config: Vec<usize>) -> u32 {
    // Remove leading working springs (.), we don't care about those
    let row = row_str.trim_start_matches(".");

    let chars = row.chars().collect_vec();

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
        let newpart = num_disabled_springs + 1;
        let new_row = String::from_iter(&chars[newpart..]);

        let newconf = config.clone()[1..].to_vec();

        return number_of_arrangements(&new_row, newconf);
    }
    // The row starts with an unknown spring (?)
    else {
        // Replace the first spot it with a working spring, and recursively check the rest:
        let row_with_working = format!("{}{}",".", String::from_iter(&chars[1..]));
        //let row_with_working = String::from_iter(&chars[1..]);
        let num_arrangements_when_working =
            number_of_arrangements(&row_with_working, config.clone());

        let row_with_disabled = format!("{}{}", "#", String::from_iter(&chars[1..]));
        let num_arrangements_when_disabled =
            number_of_arrangements(&row_with_disabled, config.clone());

        return num_arrangements_when_working + num_arrangements_when_disabled;
    }
}

fn determine_number_of_arrangements(line: &str) -> u32 {
    let (row, config_str) = line.split_once(" ").unwrap();

    let config = config_str
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect_vec();

    number_of_arrangements(row, config)
}

fn part_1(input: &str) -> u32 {
    let num_arrangements = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| determine_number_of_arrangements(line))
        .collect_vec();

    num_arrangements.into_iter().sum()
}

fn part_2(input: &str) -> u32 {
    0
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
        
        assert_eq!(determine_number_of_arrangements("???.### 1,1,3"), 1);

        assert_eq!(determine_number_of_arrangements(".??..??...?##. 1,1,3"), 4);

        assert_eq!(
            determine_number_of_arrangements("?#?#?#?#?#?#?#? 1,3,1,6"),
            1
        );

        assert_eq!(determine_number_of_arrangements("????.#...#... 4,1,1"), 1);
        assert_eq!(
            determine_number_of_arrangements("????.######..#####. 1,6,5"),
            4
        );
        
        
        assert_eq!(determine_number_of_arrangements("?###???????? 3,2,1"), 10);
    }
    
        #[test]
        fn test_part_1() {
            assert_eq!(part_1(SAMPLE_DATA), 21);
        }

        /*
    
        #[test]
        fn test_part_2() {
            assert_eq!(part_2(SAMPLE_DATA), 0);
        }
    */
}
