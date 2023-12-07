use itertools::Itertools;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/05");
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
struct SrcDestSet {
    src: usize,
    dest: usize,
    len: usize,
}

impl SrcDestSet {
    fn new(src: usize, dest: usize, len: usize) -> Self {
        Self { src, dest, len }
    }

    fn map(&self, inp: usize) -> usize {
        if inp >= self.src && inp <= self.len {
            self.dest + inp
        } else {
            inp
        }
    }
}

#[derive(Debug)]
struct SrcDestMap {
    sets: Vec<SrcDestSet>,
}

impl SrcDestMap {
    fn new(sets: Vec<SrcDestSet>) -> Self {
        Self { sets }
    }

    fn map(&self, inp: usize) -> usize {
        dbg!(inp);
        for set in &self.sets {
            if inp >= set.src && inp <= (set.src+set.len) {
                return set.dest + ( inp - set.src)
            }
        }

        return inp;
    }

}

struct Almanac {}

fn parse_seeds(input: &str) -> Vec<u32> {
    return input[7..]
        .split(" ")
        .flat_map(|c| c.parse::<u32>())
        .collect_vec();
}

fn parse_src_dest_maps(input: &str) -> SrcDestMap {
    SrcDestMap::new(
        input
            .split("\n")
            .skip(1)
            .filter(|l| !l.is_empty())
            .map(|l| {
                let (dest, src, len) = l
                    .split(" ")
                    .flat_map(|c| c.parse::<usize>())
                    .collect_tuple()
                    .unwrap();

                SrcDestSet { src, dest, len }
            })
            .collect::<Vec<SrcDestSet>>(),
    )
}

fn part_1(input: &str) -> u32 {
    
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = parse_seeds(input_parts[0]);

    let (
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = input_parts[1..]
        .into_iter()
        .map(|s| parse_src_dest_maps(s))
        .tuples()
        .exactly_one()
        .unwrap();

    dbg!(&seeds);

    dbg!(&seed_to_soil);
    dbg!(&fertilizer_to_water);

    let location_numbers = seeds.into_iter().map(|seed| {

        dbg!(&seed);
        
        let soil = seed_to_soil.map(seed.try_into().unwrap());
        dbg!(soil);
        let fert = soil_to_fertilizer.map(soil);
        dbg!(fert);
        let water = fertilizer_to_water.map(fert);
        dbg!(water);
        let light = water_to_light.map(water);
        dbg!(light);
        let temp = light_to_temperature.map(light);
        dbg!(temp);
        let hum = temperature_to_humidity.map(temp);
        dbg!(hum);
        let loc = humidity_to_location.map(hum);


        dbg!(&loc);
        
        loc as u32
    }).collect::<Vec<u32>>();


    dbg!(&location_numbers);

    //dbg!(&s2s);

    location_numbers.into_iter().min().unwrap()

}


fn part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests_05 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 0);
    }
}
