use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day06.txt");

    println!("Part 1: {}", day06a(data.as_str()));
    println!("Part 2: {}", day06b(data.as_str()));
}

fn day06a(data: &str) -> u64 {
    parse_data_part_a(data)
        .iter()
        .map(determine_records)
        .product()
}

fn day06b(data: &str) -> u64 {
    let record = parse_data_part_b(data);
    determine_records(&record)
}

fn determine_records(record: &TimeDistanceRecords) -> u64 {
    // find the lowest and the highest value
    let first = (0..record.time).filter(|t| t * (record.time - t) > record.distance).next().unwrap();
    let last = (0..record.time).rev().filter(|t| t * (record.time - t) > record.distance).next().unwrap();
    last - first + 1 // need this to be inclusive
}

fn parse_data_part_a(data: &str) -> Vec<TimeDistanceRecords> {
    let mut lines = data.lines();
    let times = parse_line_part_a(lines.next().expect("First line was missing"));
    let distance = parse_line_part_a(lines.next().expect("Second line was missing"));

    times.iter().zip(distance).map(|x| TimeDistanceRecords { time: x.0.clone(), distance: x.1.clone() }).collect()
}

fn parse_data_part_b(data: &str) -> TimeDistanceRecords {
    let mut lines = data.lines();
    let time = parse_line_part_b(lines.next().expect("First line was missing"));
    let distance = parse_line_part_b(lines.next().expect("Second line was missing"));

    TimeDistanceRecords { time, distance }
}

fn parse_line_part_a(data: &str) -> Vec<u64> {
    data.split_whitespace().skip(1).map(|x| str::parse::<u64>(x).unwrap()).collect()
}

fn parse_line_part_b(data: &str) -> u64 {
    let string_no = data.split_whitespace().skip(1).collect::<Vec<&str>>().join("");
    str::parse::<u64>(string_no.as_str()).unwrap()
}


#[derive(Debug, PartialEq)]
struct TimeDistanceRecords {
    time: u64,
    distance: u64
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use once_cell::sync::Lazy;
    use rstest::rstest;
    use crate::day06::{day06a, day06b, determine_records, parse_data_part_a, parse_data_part_b, TimeDistanceRecords};

    const TEST_DATA: &str = "Time:      7  15   30\n\
                             Distance:  9  40  200";

    static PARSED_DATA_PART_A: Lazy<Vec<TimeDistanceRecords>> = Lazy::new(|| vec![
        TimeDistanceRecords {
            time: 7,
            distance: 9
        },
        TimeDistanceRecords {
            time: 15,
            distance: 40
        },
        TimeDistanceRecords {
            time: 30,
            distance: 200
        }
    ]);

    static PARSED_DATA_PART_B: Lazy<TimeDistanceRecords> = Lazy::new(||
        TimeDistanceRecords {
            time: 71530,
            distance: 940200
        });

    #[test]
    fn test_parse_data_part_a() {
        assert_eq!(parse_data_part_a(TEST_DATA), *PARSED_DATA_PART_A.deref());
    }

    #[test]
    fn test_parse_data_part_b() {
        assert_eq!(parse_data_part_b(TEST_DATA), *PARSED_DATA_PART_B.deref());
    }

    #[test]
    fn test_day06a_aoc_soln() {
        assert_eq!(day06a(TEST_DATA), 288);
    }

    #[test]
    fn test_day06b_aoc_soln() {
        assert_eq!(day06b(TEST_DATA), 71503);
    }

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    #[case(71530, 940200, 71503)]
    fn test_determine_records(#[case] time: u64, #[case] distance: u64, #[case] expected: u64) {
        let data = TimeDistanceRecords { time, distance };
        assert_eq!(determine_records(&data), expected);
    }

}