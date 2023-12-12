use std::collections::HashMap;
use std::iter::once;
use crate::common::load_from;
use crate::day12::Entry::{Damaged, Operational, Unknown};

pub fn run_day() {
    let data = load_from("day12.txt");
    let rows = parse_lines(data.as_str());
    println!("Part 1: {}", day12a(&rows));
    println!("Part 2: {}", day12b(&rows));
}

fn day12a(rows:  &Vec<Row>) -> u64 {
    rows.iter().map(find_memoized_combinations).sum()
}

fn day12b(rows: &Vec<Row>) -> u64 {
    rows.iter().map(|x| x.unfold()).map(|x| find_memoized_combinations(&x)).sum()
}

fn find_memoized_combinations(row: &Row) -> u64 {
    let mut combinations_for: HashMap<(Vec<Entry>, Option<u64>, Vec<u64>), u64> = HashMap::new();

    fn check_next(current_row: Vec<Entry>, current_damaged_left: Option<u64>, upcoming_combination: Vec<u64>, combinations_for: &mut HashMap<(Vec<Entry>, Option<u64>, Vec<u64>), u64>) -> u64 {
        fn get_next_entry(current_row: &Vec<Entry>) -> Vec<Entry> {
            current_row.iter().skip(1).map(|x| x.clone()).collect::<Vec<Entry>>()
        }

        let key = (current_row.clone(), current_damaged_left.clone(), upcoming_combination.clone());

        if current_row.is_empty() {
            if current_damaged_left.unwrap_or(0) == 0 && upcoming_combination.is_empty() {
                1 // this worked
            } else {
                0 // still stuff to go
            }
        } else if combinations_for.contains_key(&key) {
            combinations_for.get(&key).unwrap().clone()
        } else {
            let val = match current_row.first().unwrap() {
               Operational => {
                   if current_damaged_left.unwrap_or(0) > 0 {
                       // If we have gotten here, that means we've not completed the
                       // row of damaged springs. We therefore return nothing.
                       0
                   } else {
                       // In this case, go to the next spring. Everything is still valid.
                       check_next(get_next_entry(&current_row), None, upcoming_combination, combinations_for)
                   }
               },
               Damaged => {
                   match current_damaged_left {
                       None => {
                           // We're starting a new combination if we have one.
                           if (&upcoming_combination).is_empty() {
                               // We don't, so this will produce no permutations from here.
                               0
                           } else {
                               // We start a new combination, as we have a damaged spring
                               // We pop the first entry off the entry and combination vecs, reduce the number of springs
                               // we need by one, then continue.
                               check_next(
                                   get_next_entry(&current_row),
                                   Some(upcoming_combination.first().unwrap() - 1),
                                   upcoming_combination.iter().skip(1).map(|x| x.clone()).collect::<Vec<u64>>(),
                                   combinations_for
                               )
                           }
                       },
                       Some(0) => 0, // This won't fit -- we are not expecting another damaged spring but we found one.
                       Some(x) => {
                           // We are still expecting a damaged spring, so go to the next entry expecting one less.
                           check_next(
                               get_next_entry(&current_row),
                               Some(x - 1),
                               upcoming_combination.clone(),
                               combinations_for
                           )
                       }
                   }
               },
               Unknown => {
                   // In this case, we replace the unknown in two ways -- with a Damaged and an Operational --
                   // then resend it through this stack.
                   check_next(
                       once(Damaged).chain(current_row.iter().skip(1).map(|x| x.clone())).collect::<Vec<Entry>>(),
                       current_damaged_left.clone(),
                       upcoming_combination.clone(),
                       combinations_for
                   ) + check_next(
                       once(Operational).chain(current_row.iter().skip(1).map(|x| x.clone())).collect::<Vec<Entry>>(),
                       current_damaged_left.clone(),
                       upcoming_combination.clone(),
                       combinations_for
                   )
               }
           };
            combinations_for.insert(key, val);
            val
       }
    }

    check_next(row.entries.clone(), None, row.contiguous.clone(), &mut combinations_for)
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

#[derive(PartialEq, Debug, Hash)]
struct Row {
    entries: Vec<Entry>,
    contiguous: Vec<u64>
}

impl Row {
    fn unfold(&self) -> Row {
        let entries = self.entries.iter().map(|x| x.clone()).chain(once(Unknown)).cycle().take(self.entries.len() * 5 + 4).collect();
        let contiguous: Vec<u64> = self.contiguous.iter().cycle().take(self.contiguous.len() * 5).map(|x| x.clone()).collect();
        Row { entries, contiguous }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
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
    use crate::day12::{Row, parse_line, parse_lines, day12a, find_memoized_combinations, day12b};
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

    #[test]
    fn test_day12b() {
        assert_eq!(day12b(PARSED_DATA_1.deref()), 525152);
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
    fn test_find_memoized_combinations(#[case] idx: usize, #[case] expected: u64) {
        assert_eq!(find_memoized_combinations(PARSED_DATA_1.deref().get(idx).unwrap()), expected);
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