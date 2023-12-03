use std::collections::HashSet;
use structopt::lazy_static::lazy_static;
use regex::Regex;
use crate::common::load_from;

struct Code {
    code: u32,
    positions: Vec<Coord>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize
}

impl Code {
    fn is_valid_in(&self, valid_coords: &HashSet<Coord>) -> bool {
        self.positions.iter().any(|coord| valid_coords.contains(coord))
    }
}

impl Coord {
    fn surrounding(&self) -> Vec<Coord> {
        let row_range = self.row.checked_sub(1).unwrap_or(self.row)..=(self.row + 1);
        let col_range = self.col.checked_sub(1).unwrap_or(self.col)..=(self.col + 1);

        let mut vec: Vec<Coord> = Vec::new();
        for row in row_range {
            for col in col_range.clone() { // avoids moving the original
                vec.push(Coord { row, col });
            }
        }
        vec
    }
}

pub fn run_day() {
    let data = load_from("day03.txt");
    let mut codes: Vec<Code> = Vec::new();
    let mut symbol_locations: HashSet<Coord> = HashSet::new();
    for (idx, line) in data.lines().enumerate() {
        let (line_codes, line_symbols) = read_line(idx, line);
        codes.extend(line_codes);
        symbol_locations.extend(line_symbols);
    }
    println!("Part 1: {}", day03a(codes, symbol_locations));
}

lazy_static! {
    static ref SYMBOL_REGEX: Regex = Regex::new(r"[^a-zA-Z0-9\.]").unwrap();
    static ref DIGIT_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

fn day03a(codes: Vec<Code>, symbols: HashSet<Coord>) -> u32 {
    codes.iter().filter(|code| code.is_valid_in(&symbols)).map(|code| code.code).sum()
}

fn read_line(line_no: usize, line: &str) -> (Vec<Code>, HashSet<Coord>) {
    (
        read_numbers(line_no, line),
        read_symbols(line_no, line)
    )
}

fn read_symbols(line_no: usize, line: &str) -> HashSet<Coord> {
    SYMBOL_REGEX
        .find_iter(line)
        .into_iter()
        .map(|col| Coord { row: line_no, col: col.start() })
        .flat_map(|coord| coord.surrounding())
        .collect()
}

fn read_numbers(line_no: usize, line: &str) -> Vec<Code> {
    DIGIT_REGEX
        .find_iter(line)
        .map(|m| {
            let positions: Vec<Coord> = m.range().into_iter().map(|r| Coord { row: line_no, col: r }).collect();
            Code {
                code: str::parse::<u32>(m.as_str()).unwrap(),
                positions
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use crate::day03::{Coord, read_symbols};

    #[test]
    fn test_read_line() {

    }


    #[test]
    fn test_symbols_none() {
        let sample = ".....";
        let coords = read_symbols(1, sample);
        let expected: HashSet<Coord> = HashSet::new();

        assert_eq!(coords, expected)
    }

    #[test]
    fn test_symbols_one() {
        let sample = "..#..";
        let coords = read_symbols(1, sample);
        let expected: HashSet<Coord> = vec![
            Coord { row: 0,  col: 1 },
            Coord { row: 0,  col: 2 },
            Coord { row: 0,  col: 3 },
            Coord { row: 1,  col: 1 },
            Coord { row: 1,  col: 2 },
            Coord { row: 1,  col: 3 },
            Coord { row: 2, col:  1 },
            Coord { row: 2, col:  2 },
            Coord { row: 2, col:  3 }
        ].into_iter().collect();

        assert_eq!(coords, expected)
    }

    #[test]
    fn test_symbols_corner() {
        let sample = "#....";
        let coords = read_symbols(0, sample);
        let expected: HashSet<Coord> = vec![
            Coord { row: 0,  col: 1 },
            Coord { row: 0,  col: 0 },
            Coord { row: 1,  col: 1 },
            Coord { row: 1,  col: 0 }
        ].into_iter().collect();

        assert_eq!(coords, expected)
    }

    #[test]
    fn test_symbols_top() {
        let sample = ".#....";
        let coords = read_symbols(0, sample);
        let expected: HashSet<Coord> = vec![
            Coord { row: 0,  col: 1 },
            Coord { row: 0,  col: 0 },
            Coord { row: 0,  col: 2 },
            Coord { row: 1,  col: 1 },
            Coord { row: 1,  col: 0 },
            Coord { row: 1,  col: 2 }
        ].into_iter().collect();

        assert_eq!(coords, expected)
    }

    #[test]
    fn test_symbols_left() {
        let sample = "#....";
        let coords = read_symbols(1, sample);
        let expected: HashSet<Coord> = vec![
            Coord { row: 0,  col: 1 },
            Coord { row: 0,  col: 0 },
            Coord { row: 1,  col: 1 },
            Coord { row: 1,  col: 0 },
            Coord { row: 2,  col: 1 },
            Coord { row: 2,  col: 0 }
        ].into_iter().collect();

        assert_eq!(coords, expected)
    }

}