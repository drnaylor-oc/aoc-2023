use itertools::Itertools;
use crate::common::load_from;
use crate::day12::Entry::{Damaged, Operational, Unknown};

pub fn run_day() {
    let data = load_from("day12.txt");
    let rows = parse_lines(data.as_str());
    println!("Part 1: {}", day12a(&rows));
}

fn day12a(rows: &Vec<Row>) -> u64 {
    rows.iter().map(find_combinations).sum()
}

fn find_combinations(row: &Row) -> u64 {
    let unknown_damaged = row.sum_unknown_damaged();
    let unknown_indexes = row.get_unknown_indexes();
    // this will give us each combination to loop over.
    let combinations: Vec<Vec<&usize>> = unknown_indexes.iter().combinations(unknown_damaged).collect();
    combinations.iter().filter(|x| row.guess_damaged(*x)).count() as u64
}

fn parse_lines(string: &str) -> Vec<Row> {
    string.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Row {
    let mut split = line.split_whitespace();
    let entries: Vec<Entry> = split.next().unwrap().chars().map(|c| {
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            a@_ => panic!("Unexpected char: {}", a)
        }
    }).collect();
    let contiguous: Vec<u64> = split.next().unwrap().split(",").map(|e| str::parse::<u64>(e).unwrap()).collect();
    Row { entries, contiguous }
}

#[derive(PartialEq, Debug)]
struct Row {
    entries: Vec<Entry>,
    contiguous: Vec<u64>
}

impl Row {
    fn get_unknown_indexes(&self) -> Vec<usize> {
        self.entries.iter().enumerate().filter_map(|(idx, entry)| {
            match entry {
                Unknown => Some(idx),
                _ => None
            }
        }).collect()
    }

    fn sum_known_damaged(&self) -> usize {
        self.entries.iter().filter(|x| **x == Damaged).count()
    }

    fn sum_total_damaged(&self) -> usize {
        self.contiguous.iter().map(|x| x.clone() as usize).sum()
    }

    fn sum_unknown_damaged(&self) -> usize {
        self.sum_total_damaged() - self.sum_known_damaged()
    }

    fn guess_damaged(&self, guess: &Vec<&usize>) -> bool {
        let result: Vec<u64> = self.entries.iter().enumerate().map(|(idx, x)| {
            match x {
                Unknown => {
                    if (*guess).contains(&&idx) {
                        Damaged
                    } else {
                        Operational
                    }
                },
                x@_ => x.clone()
            }
        }).dedup_with_count().filter_map(|(count, entry)| {
            if entry == Damaged {
                Some(count as u64)
            } else {
                None
            }
        }).collect();
        result == self.contiguous
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Entry {
    Operational,
    Damaged,
    Unknown
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day12::{Row, parse_line, parse_lines, day12a, find_combinations};
    use crate::day12::Entry::*;

    const TEST_DATA_1: &str = "???.### 1,1,3\n\
                               .??..??...?##. 1,1,3\n\
                               ?#?#?#?#?#?#?#? 1,3,1,6\n\
                               ????.#...#... 4,1,1\n\
                               ????.######..#####. 1,6,5\n\
                               ?###???????? 3,2,1";

    lazy_static! {
        static ref PARSED_DATA_1: Vec<Row> = vec![
            Row {
                entries: vec![ Unknown, Unknown, Unknown, Operational, Damaged, Damaged, Damaged ],
                contiguous: vec![1,1,3]
            },
            Row {
                entries: vec![ Operational, Unknown, Unknown, Operational, Operational, Unknown, Unknown, Operational, Operational, Operational, Unknown, Damaged, Damaged, Operational ],
                contiguous: vec![1,1,3]
            },
            Row {
                entries: vec![ Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown ],
                contiguous: vec![1,3,1,6]
            },
            Row {
                entries: vec![ Unknown, Unknown, Unknown, Unknown, Operational, Damaged, Operational, Operational, Operational, Damaged, Operational, Operational, Operational ],
                contiguous: vec![4,1,1]
            },
            Row {
                entries: vec![ Unknown, Unknown, Unknown, Unknown, Operational, Damaged, Damaged, Damaged, Damaged, Damaged, Damaged, Operational, Operational, Damaged, Damaged, Damaged, Damaged, Damaged, Operational ],
                contiguous: vec![1,6,5]
            },
            Row {
                entries: vec![ Unknown, Damaged, Damaged, Damaged, Unknown, Unknown, Unknown, Unknown, Unknown, Unknown, Unknown, Unknown ],
                contiguous: vec![3,2,1]
            },
        ];
    }

    #[test]
    fn test_day12a() {
        assert_eq!(day12a(PARSED_DATA_1.deref()), 21);
    }

    #[rstest]
    #[case(0, vec![0,1,2])]
    #[case(1, vec![1,2,5,6,10])]
    #[case(2, vec![0,2,4,6,8,10,12,14])]
    #[case(3, vec![0,1,2,3])]
    #[case(4, vec![0,1,2,3])]
    #[case(5, vec![0,4,5,6,7,8,9,10,11])]
    fn test_get_unknown_indexes(#[case] idx: usize, #[case] expected_indexes: Vec<usize>) {
        assert_eq!(PARSED_DATA_1.deref().get(idx).unwrap().get_unknown_indexes(), expected_indexes);
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 4)]
    #[case(2, 1)]
    #[case(3, 1)]
    #[case(4, 4)]
    #[case(5, 10)]
    fn test_find_combinations(#[case] idx: usize, #[case] expected: u64) {
        assert_eq!(find_combinations(PARSED_DATA_1.deref().get(idx).unwrap()), expected);
    }

    #[test]
    fn test_parse_line() {
        for (idx, line) in TEST_DATA_1.lines().enumerate() {
            assert_eq!(parse_line(line), *PARSED_DATA_1.deref().get(idx).unwrap());
        }
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(parse_lines(TEST_DATA_1), *PARSED_DATA_1.deref());
    }

}