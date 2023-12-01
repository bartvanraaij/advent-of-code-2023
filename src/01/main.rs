use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = &String::from("input.txt");
    let input_filepath: &str = args.get(1).unwrap_or(default);

    let input = fs::read_to_string(input_filepath).expect("input file should be readable");

    let result_part_1 = part_1(&input);
    println!("{:?}", result_part_1);

    let result_part_2 = part_2(&input);
    println!("{:?}", result_part_2);
}

fn part_1(input: &str) -> u32 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut total_sum = 0;

    for line in lines {
        let numbers = line
            .chars()
            .filter_map(|char| if char.is_digit(10) { Some(char) } else { None })
            .collect::<Vec<char>>();

        let first_digit = numbers.first().unwrap_or(&'0');
        let last_digit = numbers.last().unwrap_or(&'0');

        let calibration_value = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap_or(0);

        total_sum += calibration_value;
    }

    total_sum
}

fn get_all_line_numbers(line: &str) -> Vec<u32> {
    let mut numbers = Vec::new();

    for i in 0..line.len() {
        if line[i..].starts_with("one") { numbers.push(1); }
        if line[i..].starts_with("two") { numbers.push(2); }
        if line[i..].starts_with("three") { numbers.push(3); }
        if line[i..].starts_with("four") { numbers.push(4); }
        if line[i..].starts_with("five") { numbers.push(5); }
        if line[i..].starts_with("six") { numbers.push(6); }
        if line[i..].starts_with("seven") { numbers.push(7); }
        if line[i..].starts_with("eight") { numbers.push(8); }
        if line[i..].starts_with("nine") { numbers.push(9); }
        
        let char_at_index = line.chars().nth(i).unwrap();
        if char_at_index.is_digit(10) { 
            numbers.push(char_at_index.to_digit(10).unwrap_or(0));
        }
    
    }

    numbers
}

fn part_2(input: &str) -> u32 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut total_sum = 0;

    for line in lines {
        let numbers = get_all_line_numbers(line);
        let first_digit = numbers.first().unwrap_or(&0);
        let last_digit = numbers.last().unwrap_or(&0);

        let calibration_value = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap_or(0);

        total_sum += calibration_value;
    }

    total_sum
}
