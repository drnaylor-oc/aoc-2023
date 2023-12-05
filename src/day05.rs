use std::collections::VecDeque;
use std::ops::Range;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day05.txt");
    let parsed_data: ParsedData = parse_lines(data.as_str());
    println!("{}", day05a(&parsed_data));
    println!("{}", day05b(&parsed_data));
}

fn parse_lines(str: &str) -> ParsedData {
    let lines = str.lines();

    // seeds = line 1
    let mut seeds: Vec<u64> = vec![];

    let mut mappings: VecDeque<Vec<Mapping>> = VecDeque::new();

    for line in lines {
        if line.starts_with("seeds: ") {
            seeds = line.split_once(":").unwrap().1.trim().split_whitespace().map(|x| str::parse::<u64>(x).unwrap()).collect();
        } else if line.contains("map:") {
            mappings.push_back(vec![]);
        } else if !line.is_empty() {
            let [final_no, initial_no, range] = line.split_whitespace().map(|x| str::parse::<u64>(x).unwrap()).collect::<Vec<u64>>()[0..3] else { panic!("should not happen") };
            let m = mappings.back_mut().unwrap();
            m.push(Mapping { initial_range: initial_no..(initial_no + range), final_start: final_no });
        }
    }

    // to ensure the borrow checker doesn't complain, we pop the vector from the enclosing vector, moving
    // the values to here.
    ParsedData {
        seeds,
        seed_to_soil: mappings.pop_front().unwrap(),
        soil_to_fertilizer: mappings.pop_front().unwrap(),
        fertilizer_to_water: mappings.pop_front().unwrap(),
        water_to_light: mappings.pop_front().unwrap(),
        light_to_temperature: mappings.pop_front().unwrap(),
        temperature_to_humidity: mappings.pop_front().unwrap(),
        humidity_to_location: mappings.pop_front().unwrap()
    }
}

fn walk_data(seed: &u64, parsed_data: &ParsedData) -> u64 {
    let soil = get_mapping(&seed, &parsed_data.seed_to_soil);
    let fertilizer = get_mapping(&soil, &parsed_data.soil_to_fertilizer);
    let water = get_mapping(&fertilizer, &parsed_data.fertilizer_to_water);
    let light  = get_mapping(&water, &parsed_data.water_to_light);
    let temperature = get_mapping(&light, &parsed_data.light_to_temperature);
    let humidity = get_mapping(&temperature, &parsed_data.temperature_to_humidity);
    let location = get_mapping(&humidity, &parsed_data.humidity_to_location);
    location
}

fn get_mapping(value: &u64, mappings: &Vec<Mapping>) -> u64 {
    for mapping in mappings {
        if let Some(m) = mapping.map_if_valid(value) {
            return m; // early return intended
        }
    }
    value.clone()
}

fn day05a(parsed_data: &ParsedData) -> u64 {
    parsed_data.seeds.iter().map(|seed| walk_data(seed, parsed_data)).min().unwrap()
}

fn day05b(parsed_data: &ParsedData) -> u64 {
    0
}

#[derive(PartialEq, Debug)]
struct ParsedData {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>
}

#[derive(PartialEq, Debug)]
struct Mapping {
    initial_range: Range<u64>,
    final_start: u64
}

impl Mapping {
    fn map_if_valid(&self, initial: &u64) -> Option<u64> {
        self.initial_range.contains(initial).then(|| self.final_start + (initial - self.initial_range.start))
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use once_cell::sync::Lazy;
    use rstest::rstest;
    use crate::day05::{day05a, get_mapping, Mapping, parse_lines, ParsedData, walk_data};

    const TEST_DATA: &str = "seeds: 79 14 55 13\n\
                            \n\
                            seed-to-soil map:\n\
                            50 98 2\n\
                            52 50 48\n\
                            \n\
                            soil-to-fertilizer map:\n\
                            0 15 37\n\
                            37 52 2\n\
                            39 0 15\n\
                            \n\
                            fertilizer-to-water map:\n\
                            49 53 8\n\
                            0 11 42\n\
                            42 0 7\n\
                            57 7 4\n\
                            \n\
                            water-to-light map:\n\
                            88 18 7\n\
                            18 25 70\n\
                            \n\
                            light-to-temperature map:\n\
                            45 77 23\n\
                            81 45 19\n\
                            68 64 13\n\
                            \n\
                            temperature-to-humidity map:\n\
                            0 69 1\n\
                            1 0 69\n\
                            \n\
                            humidity-to-location map:\n\
                            60 56 37\n\
                            56 93 4\n\
                            ";

    static PARSED_DATA: Lazy<ParsedData> = Lazy::new(|| ParsedData {
        seeds: vec![79, 14, 55, 13],
        seed_to_soil: vec![
            Mapping { initial_range: 98..100, final_start: 50 },
            Mapping { initial_range: 50..98, final_start: 52 }
        ],
        soil_to_fertilizer: vec![
            Mapping { initial_range: 15..52, final_start: 0 },
            Mapping { initial_range: 52..54, final_start: 37 },
            Mapping { initial_range: 0..15, final_start: 39 }
        ],
        fertilizer_to_water: vec![
            Mapping { initial_range: 53..61, final_start: 49 },
            Mapping { initial_range: 11..53, final_start: 0 },
            Mapping { initial_range: 0..7, final_start: 42 },
            Mapping { initial_range: 7..11, final_start: 57 }
        ],
        water_to_light: vec![
            Mapping { initial_range: 18..25, final_start: 88 },
            Mapping { initial_range: 25..95, final_start: 18 }
        ],
        light_to_temperature: vec![
            Mapping { initial_range: 77..100, final_start: 45 },
            Mapping { initial_range: 45..64, final_start: 81 },
            Mapping { initial_range: 64..77, final_start: 68 }
        ],
        temperature_to_humidity: vec![ // nice
            Mapping { initial_range: 69..70, final_start: 0 },
            Mapping { initial_range: 0..69, final_start: 1 }
        ],
        humidity_to_location: vec![
            Mapping { initial_range: 56..93, final_start: 60 },
            Mapping { initial_range: 93..97, final_start: 56 }
        ]
    });

    #[test]
    fn test_day05a() {
        let data = parse_lines(TEST_DATA);
        assert_eq!(35, day05a(&data));
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(parse_lines(TEST_DATA), *PARSED_DATA.deref());
    }

    #[rstest]
    #[case(79, Some(82))]
    #[case(14, Some(17))]
    #[case(2, None)]
    #[case(90, None)]
    fn test_map_if_value(#[case] input: u64, #[case] expected: Option<u64>) {
        let mapping = Mapping {
            initial_range: 14..90,
            final_start: 17
        };

        assert_eq!(mapping.map_if_valid(&input), expected);
    }

    #[rstest]
    #[case(79, 82)]
    #[case(14, 17)]
    #[case(2, 2)]
    #[case(90, 90)]
    fn get_mapping_with_one(#[case] input: u64, #[case] expected: u64) {
        let mapping = vec![Mapping {
            initial_range: 14..90,
            final_start: 17
        }];

        assert_eq!(get_mapping(&input, &mapping), expected);
    }

    #[rstest]
    #[case(79, 82)]
    #[case(14, 23)]
    #[case(2, 11)]
    #[case(90, 90)]
    fn get_mapping_with_two(#[case] input: u64, #[case] expected: u64) {
        let mapping = vec![Mapping {
            initial_range: 70..90,
            final_start: 73
        },
        Mapping {
            initial_range: 1..15,
            final_start: 10
        }];

        assert_eq!(get_mapping(&input, &mapping), expected);
    }

    #[rstest]
    #[case(79, 82)]
    #[case(14, 43)]
    #[case(55, 86)]
    #[case(13, 35)]
    fn test_walk_data(#[case] seed: u64, #[case] expected: u64) {
        assert_eq!(walk_data(&seed, PARSED_DATA.deref()), expected);
    }

}