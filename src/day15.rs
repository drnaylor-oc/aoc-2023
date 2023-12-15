use std::collections::HashMap;
use itertools::Itertools;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day15.txt");
    let hashes = parse_hashes(data.as_str());
    println!("Part 1: {}", day15a(&hashes));
    println!("Part 2: {}", day15b(data.as_str()));
}

fn day15a(hashes: &Vec<u64>) -> u64 {
    hashes.iter().sum()
}

fn day15b(line: &str) -> u64 {
    const NEW_VEC: Vec<(&str, u64)> = Vec::new();
    let mut boxes: [Vec<(&str, u64)>; 256] = [NEW_VEC; 256];

    for entry in line.split(",") {
        let (label, box_no, operation) = process_label_and_operation(entry);
        let vec = boxes.get_mut(box_no as usize).unwrap();
        let existing_idx = vec.iter().find_position(|(l, _)| *l == label);
        match operation {
            Operation::Remove => {
                if let Some((idx, _)) = existing_idx {
                    vec.remove(idx);
                }
            }
            Operation::Add(focal_length) => {
                if let Some((idx, _)) = existing_idx {
                    vec.remove(idx);
                    vec.insert(idx, (label, focal_length));
                } else {
                    vec.push((label, focal_length));
                }
            }
        }
    }

    boxes.iter()
        .enumerate()
        .map(|(i, l)| {
            (i as u64 + 1) * l.iter().enumerate().map(|(idx, (_, focal_length))| ((idx as u64) + 1) * focal_length).sum::<u64>()
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum Operation {
    Remove,
    Add(u64)
}

fn process_label_and_operation<'a>(label: &'a str) -> (&'a str, u64, Operation) {
    let (label, operation): (&'a str, Operation) = if label.ends_with("-") {
        (label.trim_end_matches("-"), Operation::Remove)
    } else {
        label.split_once("=").map(|(l, v)| (l, Operation::Add(str::parse::<u64>(v).unwrap()))).unwrap()
    };

    (label, parse_hash(label), operation)
}

fn parse_hashes<'a>(line: &'a str) -> Vec<u64> {
    let mut cache = HashMap::<&'a str, u64>::new();
    let mut hashes: Vec<u64> = Vec::new();
    for entry in line.split(",") {
        hashes.push(cache.entry(entry).or_insert_with(|| parse_hash(entry)).clone());
    }

    hashes
}

fn parse_hash(entry: &str) -> u64 {
    // ASCII is first 7 bits of UTF-8
    entry.chars().map(u64::from).fold(0, |acc, next| ((acc + next) * 17) % 256)
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day15::{day15a, day15b, Operation, parse_hash, parse_hashes, process_label_and_operation};

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    lazy_static! {
        static ref PARSED_HASHES: Vec<u64> = vec![
            30,
            253,
            97,
            47,
            14,
            180,
            9,
            197,
            48,
            214,
            231
        ];
    }

    #[test]
    fn test_parse_hashes() {
        assert_eq!(parse_hashes(TEST_INPUT), *PARSED_HASHES.deref());
    }

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_parse_hash(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(parse_hash(input), expected);
    }

    #[test]
    fn test_day15a() {
        assert_eq!(day15a(&parse_hashes(TEST_INPUT)), 1320);
    }

    #[rstest]
    #[case("rn=1", "rn", 0, Operation::Add(1))]
    #[case("cm-", "cm", 0, Operation::Remove)]
    #[case("qp=3", "qp", 1, Operation::Add(3))]
    #[case("cm=2", "cm", 0, Operation::Add(2))]
    #[case("qp-", "qp", 1, Operation::Remove)]
    #[case("pc=4", "pc", 3, Operation::Add(4))]
    #[case("ot=9", "ot", 3, Operation::Add(9))]
    #[case("ab=5", "ab", 3, Operation::Add(5))]
    #[case("pc-", "pc", 3, Operation::Remove)]
    #[case("pc=6", "pc", 3, Operation::Add(6))]
    #[case("ot=7", "ot", 3, Operation::Add(7))]
    fn test_process_label_and_operation(#[case] input: &str, #[case] label: &str, #[case] box_no: u64, #[case] operation: Operation) {
        assert_eq!(process_label_and_operation(input), (label, box_no, operation));
    }

    #[test]
    fn test_day15b() {
        assert_eq!(day15b(TEST_INPUT), 145);
    }

}