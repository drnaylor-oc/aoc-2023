use std::collections::HashSet;
use structopt::lazy_static::lazy_static;
use regex::Regex;
use crate::common::load_from;

#[derive(Debug, PartialEq)]
struct Code {
    code: u32,
    positions: Vec<Coord>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Symbol {
    is_gear: bool,
    coord: Coord
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
    let (codes, symbol_locations, symbol_coords) = load_data(data.as_str());
    println!("Part 1: {}", day03a(&codes, &symbol_coords));
    println!("Part 2: {}", day03b(&codes, &symbol_locations));
}

fn load_data(input: &str) -> (Vec<Code>, HashSet<Symbol>, HashSet<Coord>) {
    let mut codes: Vec<Code> = Vec::new();
    let mut symbol_locations: HashSet<Symbol> = HashSet::new();
    for (idx, line) in input.lines().enumerate() {
        let (line_codes, line_symbols) = read_line(idx, line);
        codes.extend(line_codes);
        symbol_locations.extend(line_symbols);
    }
    let symbol_coords: HashSet<Coord> = get_surrounding_coords_from_symbols(&symbol_locations);
    (codes, symbol_locations, symbol_coords)
}

lazy_static! {
    static ref SYMBOL_REGEX: Regex = Regex::new(r"[^a-zA-Z0-9\.]").unwrap();
    static ref DIGIT_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

fn day03a(codes: &Vec<Code>, symbols: &HashSet<Coord>) -> u32 {
    codes.iter().filter(|code| code.is_valid_in(&symbols)).map(|code| code.code).sum()
}

fn day03b(code: &Vec<Code>, symbol: &HashSet<Symbol>) -> u32 {
    symbol
        .iter()
        .filter(|s| s.is_gear)
        .map(|gear| select_codes_from_gear(gear, &code))
        .sum()
}

fn select_codes_from_gear(symbol: &Symbol, codes: &Vec<Code>) -> u32 {
    let coords = symbol.coord.surrounding();
    let mut items: Vec<u32> = Vec::new();
    for code in codes {
        if coords.iter().any(|c| code.positions.contains(c)) {
            items.push(code.code)
        }
    }

    // If we found two entries next to the *, it's a gear, else it's not
    if items.len() != 2 {
        0
    } else {
        items[0] * items[1]
    }
}

fn read_line(line_no: usize, line: &str) -> (Vec<Code>, HashSet<Symbol>) {
    (
        read_numbers(line_no, line),
        read_symbols(line_no, line)
    )
}

fn read_symbols(line_no: usize, line: &str) -> HashSet<Symbol> {
    SYMBOL_REGEX
        .find_iter(line)
        .into_iter()
        .map(|col| Symbol { is_gear: col.as_str() == "*", coord: Coord { row: line_no, col: col.start() } })
        .collect()
}

fn get_surrounding_coords_from_symbols(symbols: &HashSet<Symbol>) -> HashSet<Coord> {
    symbols.iter().flat_map(|symbol| symbol.coord.surrounding()).collect()
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
    use proptest::{prop_assert_eq, prop_compose, proptest};
    use proptest::bool::{ANY as ANY_BOOL};
    use proptest::strategy::Just;
    use proptest::collection::{vec as prop_vec};
    use crate::day03::{Code, Coord, day03a, day03b, load_data, read_numbers, read_symbols, Symbol};

    #[test]
    fn test_symbols_none() {
        let sample = ".....";
        let coords = read_symbols(1, sample);
        let expected: HashSet<Symbol> = HashSet::new();

        assert_eq!(coords, expected);
    }

    #[test]
    fn test_symbols_one() {
        let sample = "#....";
        let coords = read_symbols(1, sample);
        let mut expected: HashSet<Symbol> = HashSet::new();
        expected.insert(Symbol {
            is_gear: false,
            coord: Coord { row: 1, col: 0 }
        });

        assert_eq!(coords, expected);
    }

    #[test]
    fn test_symbols_one_gear() {
        let sample = "..*..";
        let coords = read_symbols(2, sample);
        let mut expected: HashSet<Symbol> = HashSet::new();
        expected.insert(Symbol {
            is_gear: true,
            coord: Coord { row: 2, col: 2 }
        });

        assert_eq!(coords, expected);
    }

    prop_compose! {
        fn symbol_strategy(row: u8)(is_gear in ANY_BOOL, col in ..50usize) -> Symbol {
            Symbol {
                is_gear,
                coord: Coord {
                    row: row as usize,
                    col
                }
            }
        }
    }

    // Only the last paramter list will be considered
    prop_compose! {
        fn list_of_symbols()(row in ..50u8)
                            (symbols in prop_vec(symbol_strategy(row), 1..=10), row in Just(row)) -> (u8, HashSet<Symbol>) {
            let mut s = symbols;
            // we need to dedup by column, note that the is_gear might be different so we have to do it manually
            // rather than by just using a set.
            s.sort_by_key(|x| x.coord.col);
            s.dedup_by_key(|x1| x1.coord.col);
            (row, HashSet::from_iter(s))
        }
    }

    proptest! {
        #[test]
        fn test_symbols_property(row_and_symbols in list_of_symbols()) {
            let (row, symbols) = row_and_symbols;
            let mut sample = ["."; 50];
            for i in &symbols {
                sample[i.coord.col] = if i.is_gear {
                    "*"
                } else {
                    "#"
                };
            }
            let result_string = sample.join("");
            println!("{}", result_string);

            let result = read_symbols(row as usize, result_string.as_str());
            prop_assert_eq!(symbols, result);
        }
    }

    #[test]
    fn test_coord_surroundings_normal() {
        let expected = vec![
            Coord { row: 0, col: 0 },
            Coord { row: 0, col: 1 },
            Coord { row: 0, col: 2 },
            Coord { row: 1, col: 0 },
            Coord { row: 1, col: 1 },
            Coord { row: 1, col: 2 },
            Coord { row: 2, col: 0 },
            Coord { row: 2, col: 1 },
            Coord { row: 2, col: 2 }
        ];
        assert_eq!(Coord { row: 1, col: 1 }.surrounding(), expected);
    }

    #[test]
    fn test_coord_surroundings_top() {
        let expected = vec![
            Coord { row: 0, col: 0 },
            Coord { row: 0, col: 1 },
            Coord { row: 0, col: 2 },
            Coord { row: 1, col: 0 },
            Coord { row: 1, col: 1 },
            Coord { row: 1, col: 2 }
        ];
        assert_eq!(Coord { row: 0, col: 1 }.surrounding(), expected);
    }

    #[test]
    fn test_coord_surroundings_left() {
        let expected = vec![
            Coord { row: 0, col: 0 },
            Coord { row: 0, col: 1 },
            Coord { row: 1, col: 0 },
            Coord { row: 1, col: 1 },
            Coord { row: 2, col: 0 },
            Coord { row: 2, col: 1 }
        ];
        assert_eq!(Coord { row: 1, col: 0 }.surrounding(), expected);
    }

    #[test]
    fn test_coord_surroundings_top_left() {
        let expected = vec![
            Coord { row: 0, col: 0 },
            Coord { row: 0, col: 1 },
            Coord { row: 1, col: 0 },
            Coord { row: 1, col: 1 }
        ];
        assert_eq!(Coord { row: 0, col: 0 }.surrounding(), expected);
    }

    #[test]
    fn test_read_numbers() {
        let line = "..123..*32..$.45..";
        let expected: Vec<Code> = vec![
            Code { code: 123, positions: vec![Coord { row: 0, col: 2 }, Coord { row: 0, col: 3 }, Coord { row: 0, col: 4 }] },
            Code { code: 32, positions: vec![Coord { row: 0, col: 8 }, Coord { row: 0, col: 9 }]},
            Code { code: 45, positions: vec![Coord { row: 0, col: 14 }, Coord { row: 0, col: 15 }] },
        ];
        assert_eq!(read_numbers(0, line), expected);
    }

    const INPUT_EXAMPLE: &str = "467..114..\n\
                                ...*......\n\
                                ..35..633.\n\
                                ......#...\n\
                                617*......\n\
                                .....+.58.\n\
                                ..592.....\n\
                                ......755.\n\
                                ...$.*....\n\
                                .664.598..";

    #[test]
    fn test_day03a() {
        let (codes, _, symbol_coords) = load_data(INPUT_EXAMPLE);
        assert_eq!(day03a(&codes, &symbol_coords), 4361);
    }

    #[test]
    fn test_day03b() {
        let (codes, symbol_locations, _) = load_data(INPUT_EXAMPLE);
        assert_eq!(day03b(&codes, &symbol_locations), 467835);
    }

}