use std::str::Lines;
use std::iter::Peekable;
use itertools::Itertools;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day13.txt");
    let maps = parse_lines(data.as_str());
    println!()
}

fn day13a(maps: &Vec<GroundMap>) -> u64 {
    0
}

fn parse_lines(test_data: &str) -> Vec<GroundMap> {
    let mut lines = test_data.lines().peekable();
    let mut maps: Vec<GroundMap> = Vec::new();
    while let Some(x) = lines.peek() {
        // Ignore the blank lines
        if !x.is_empty() {
            maps.push(parse_map(&mut lines));
        } else {
            lines.next(); // forces the next line to iterate.
        }
    }
    maps
}

fn parse_map(data: &mut Peekable<Lines>) -> GroundMap {
    let mut rows: Vec<Vec<Ground>> = Vec::new();
    while let Some(x) = data.next_if(|x| !(*x).is_empty()) {
        rows.push(parse_line(x));
    }
    let no_of_columns = rows.first().unwrap().len();
    let no_of_rows = rows.len();
    GroundMap {
        rows,
        no_of_columns,
        no_of_rows,
        is_transposed: false
    }
}

fn parse_line(line: &str) -> Vec<Ground> {
    line.chars().map(|x| {
        match x {
            '#' => Ground::Rock,
            '.' => Ground::Ash,
            _ => panic!("Unexpected character")
        }
    }).collect()
}

#[derive(PartialEq, Debug, Clone)]
enum Ground {
    Ash,
    Rock
}

#[derive(PartialEq, Debug, Clone)]
struct GroundMap {
    rows: Vec<Vec<Ground>>,
    no_of_rows: usize,
    no_of_columns: usize,
    is_transposed: bool
}

impl GroundMap {
    fn transpose(&self) -> GroundMap {
        let mut rows: Vec<Vec<Ground>> = Vec::new();
        for col in 0..self.no_of_columns {
            let mut new_row: Vec<Ground> = Vec::new();
            for row in 0..self.no_of_rows {
                unsafe { // We know this will all be here.
                    new_row.push(self.rows.get_unchecked(row).get_unchecked(col).clone());
                }
            }
            rows.push(new_row);
        }
        GroundMap { rows, no_of_rows: self.no_of_columns, no_of_columns: self.no_of_rows, is_transposed: !self.is_transposed }
    }

    fn find_reflection(&self) -> Option<u64> {
        let rows_1 = self.rows.iter().take(self.no_of_rows - 1);
        let rows_2 = self.rows.iter().skip(1);
        let potential_reflections: Vec<usize> = rows_1
            .zip(rows_2)
            .enumerate()
            .filter(|(idx, (left, right))| {
                if *left.eq(right) {
                    Some(idx + 1) // +1 means that
                } else {
                    None
                }
            })
            .collect();
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use structopt::lazy_static::lazy_static;
    use crate::day13::{GroundMap, parse_lines};
    use crate::day13::Ground::*;

    const TEST_DATA: &str = "#.##..##.\n\
                             ..#.##.#.\n\
                             ##......#\n\
                             ##......#\n\
                             ..#.##.#.\n\
                             ..##..##.\n\
                             #.#.##.#.\n\
                             \n\
                             #...##..#\n\
                             #....#..#\n\
                             ..##..###\n\
                             #####.##.\n\
                             #####.##.\n\
                             ..##..###\n\
                             #....#..#";

    lazy_static! {
        static ref PARSED_DATA: Vec<GroundMap> = vec![
            GroundMap {
                rows: vec![
                    vec![Rock, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Ash],
                    vec![Ash, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash],
                    vec![Rock, Rock, Ash, Ash, Ash, Ash, Ash, Ash, Rock],
                    vec![Rock, Rock, Ash, Ash, Ash, Ash, Ash, Ash, Rock],
                    vec![Ash, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash],
                    vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Ash],
                    vec![Rock, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash]
                ],
                no_of_rows: 7,
                no_of_columns: 9,
                is_transposed: false
            },
            GroundMap {
                rows: vec![
                    vec![Rock, Ash, Ash, Ash, Rock, Rock, Ash, Ash, Rock],
                    vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash, Ash, Rock],
                    vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Rock],
                    vec![Rock, Rock, Rock, Rock, Rock, Ash, Rock, Rock, Ash],
                    vec![Rock, Rock, Rock, Rock, Rock, Ash, Rock, Rock, Ash],
                    vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Rock],
                    vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash, Ash, Rock],
                ],
                no_of_rows: 7,
                no_of_columns: 9,
                is_transposed: false
            },
        ];
    }

    lazy_static! {
        static ref TRANSPOSED_PARSED_DATA_1: GroundMap =
            GroundMap {
                rows: vec![
                    vec![Rock, Ash, Rock, Rock, Ash, Ash, Rock],
                    vec![Ash, Ash, Rock, Rock, Ash, Ash, Ash],
                    vec![Rock, Rock, Ash, Ash, Rock, Rock, Rock],
                    vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash],
                    vec![Ash, Rock, Ash, Ash, Rock, Ash, Rock],
                    vec![Ash, Rock, Ash, Ash, Rock, Ash, Rock],
                    vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash],
                    vec![Rock, Rock, Ash, Ash, Rock, Rock, Rock],
                    vec![Ash, Ash, Rock, Rock, Ash, Ash, Ash]
                ],
                no_of_rows: 9,
                no_of_columns: 7,
                is_transposed: true
            };
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(parse_lines(TEST_DATA), *PARSED_DATA.deref());
    }

    #[test]
    fn test_transpose() {
        assert_eq!(PARSED_DATA.get(0).unwrap().transpose(), *TRANSPOSED_PARSED_DATA_1.deref());
    }

}