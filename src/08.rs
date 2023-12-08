use std::collections::HashMap;
use std::{env, fs};
use num::integer::lcm;

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/08");
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

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn parse_nodes(input: &str) -> HashMap<String, Node> {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .skip(1)
        .map(|l| {
            let name = (&l[0..3]).to_string();
            let left = &l[7..10];
            let right = &l[12..15];

            let node = Node {
                name: name.clone(),
                left: left.to_string(),
                right: right.to_string(),
            };
            (name, node)
        })
        .collect::<HashMap<_, _>>()
}

fn parse_instructions(input: &str) -> &[u8] {
    let instr_str = input.lines().filter(|l| !l.is_empty()).nth(0).unwrap();
    instr_str.as_bytes()
}

fn part_1(input: &str) -> u32 {
    let instrs = parse_instructions(input);
    let nodes = parse_nodes(input);

    let mut direction_i = 0;
    let mut steps = 0;
    let mut curr_node = nodes.get("AAA").unwrap();

    while curr_node.name != "ZZZ" {
        let direction = instrs[direction_i] as char;

        let next_node_name: String = match direction {
            'L' => curr_node.left.clone(),
            _ => curr_node.right.clone(),
        };

        curr_node = nodes.get(&next_node_name).unwrap();

        direction_i += 1;
        if direction_i >= instrs.len() {
            direction_i = 0;
        }

        steps += 1;
    }

    steps
}

fn part_2(input: &str) -> u64 {
    let instrs = parse_instructions(input);
    let nodes = parse_nodes(input);

    let all_starting_nodes = nodes
        .values()
        .into_iter()
        .filter(|n| n.name.ends_with("A"))
        .collect::<Vec<&Node>>();

    let mut steps: Vec<u64> = Vec::new();

    for starting_node in all_starting_nodes {
        let mut curr_node = starting_node;
        let mut direction_i = 0;
        let mut step = 0;

        while !curr_node.name.ends_with("Z") {
            let direction = instrs[direction_i] as char;

            let next_node_name: String = match direction {
                'L' => curr_node.left.clone(),
                _ => curr_node.right.clone(),
            };

            let next_node = nodes.get(&next_node_name).unwrap();

            curr_node = next_node;

            direction_i += 1;
            if direction_i >= instrs.len() {
                direction_i = 0;
            }

            step += 1;
        }

        steps.push(step);
        
    }

    steps.into_iter().reduce(|acc, curr| {
        lcm(acc,curr)

    }).unwrap()
}

#[cfg(test)]
mod tests_08 {
    use super::*;

    const SAMPLE_DATA_1: &str = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;
    const SAMPLE_DATA_2: &str = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    const SAMPLE_DATA_3: &str = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA_1), 2);
        assert_eq!(part_1(SAMPLE_DATA_2), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA_3), 6);
    }
}
