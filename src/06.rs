fn input_data() -> Vec<Race> {
    Vec::from([
        Race {
            time: 34.0,
            distance: 204.0,
        },
        Race {
            time: 90.0,
            distance: 1713.0,
        },
        Race {
            time: 89.0,
            distance: 1210.0,
        },
        Race {
            time: 86.0,
            distance: 1780.0,
        },
    ])
}

fn main() {
    let result_part_1 = part_1(input_data());
    println!("{:?}", result_part_1);

    let result_part_2 = part_2(input_data());
    println!("{:?}", result_part_2);
}

struct Race {
    time: f32,
    distance: f32,
}

impl Race {
    fn num_record_breaking_ways(&self) -> u32 {
        let lower_bound = (self.time - (self.time.powi(2) - (self.distance * 4.0)).sqrt()) / 2.0;
        let upper_bound = (self.time + (self.time.powi(2) - (self.distance * 4.0)).sqrt()) / 2.0;

        let x = upper_bound.ceil() - lower_bound.floor() - 1.0;


        x as u32
    }
}

fn part_1(input: Vec<Race>) -> u32 {
    input
        .into_iter()
        .map(|race| race.num_record_breaking_ways())
        .product::<u32>()
}

fn part_2(input: Vec<Race>) -> u32 {
    0
}

#[cfg(test)]
mod tests_06 {
    use super::*;

    fn sample_data() -> Vec<Race> {
        Vec::from([
            Race {
                time: 7.0,
                distance: 9.0,
            },
            Race {
                time: 15.0,
                distance: 40.0,
            },
            Race {
                time: 30.0,
                distance: 200.0,
            },
        ])
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(sample_data()), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(sample_data()), 0);
    }
}
