fn main() {
    const INPUT_DATA_1: [Race; 4] = [
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
    ];

    let result_part_1 = part_1(Vec::from(INPUT_DATA_1));
    println!("{:?}", result_part_1);

    const INPUT_DATA_2: Race = Race {
        time: 34908986.0,
        distance: 204171312101780.0,
    };

    let result_part_2 = part_2(INPUT_DATA_2);
    println!("{:?}", result_part_2);
}

struct Race {
    time: f64,
    distance: f64,
}

fn calc_num_record_breaking_ways(time: f64, distance: f64) -> u32 {
    let lower_bound = (time - (time.powi(2) - (distance * 4.0)).sqrt()) / 2.0;
    let upper_bound = (time + (time.powi(2) - (distance * 4.0)).sqrt()) / 2.0;

    let x = upper_bound.ceil() - lower_bound.floor() - 1.0;

    x as u32
}

fn part_1(input: Vec<Race>) -> u32 {
    input
        .into_iter()
        .map(|race| calc_num_record_breaking_ways(race.time, race.distance))
        .product::<u32>()
}

fn part_2(input: Race) -> u32 {
    calc_num_record_breaking_ways(input.time, input.distance)
}

#[cfg(test)]
mod tests_06 {
    use super::*;

    const SAMPLE_DATA: [Race; 3] = [
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
    ];

    const SAMPLE_DATA_2: Race = Race {
        time: 71530.0,
        distance: 940200.0,
    };

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(Vec::from(SAMPLE_DATA)), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA_2), 71503);
    }
}
