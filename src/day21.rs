use std::collections::HashSet;
use tailcall::tailcall;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day21.txt");
    let grid = parse_data(data.as_str());
    println!("Part 1: {}", day21a(&grid))
}

fn day21a(grid: &Grid) -> usize {
    walk_from_start(&grid, 64).len()
}

fn walk_from_start(grid: &Grid, steps_to_take: u64) -> HashSet<Coord> {

    #[tailcall]
    fn take_step(grid: &Grid, from: HashSet<Coord>, counter: u64, max: u64) -> HashSet<Coord> {
        let next_steps: HashSet<Coord> = from.iter().flat_map(|x| grid.next(x)).collect::<HashSet<Coord>>();
        if counter < max {
            take_step(grid, next_steps, counter + 1, max)
        } else {
            next_steps
        }
    }

    take_step(grid, HashSet::from([grid.start.clone()]), 1, steps_to_take)
}

fn parse_data(data: &str) -> Grid {
    let mut start = Coord { row: 0, column: 0 };
    let mut rocks = HashSet::<Coord>::new();

    for (row, line) in data.lines().enumerate() {
        for (column, character) in line.chars().enumerate() {
            match character {
                '.' => { /* do nothing */ }
                '#' => { rocks.insert(Coord { row, column} ); }
                'S' => { start = Coord { row, column }; }
                a => { panic!("{} is not a valid character", a) }
            }
        }
    }

    Grid { start, rocks }
}

#[derive(Debug, PartialEq)]
struct Grid {
    start: Coord,
    rocks: HashSet<Coord>
}

impl Grid {
    fn next(&self, coord: &Coord) -> HashSet<Coord> {
        HashSet::from([
            Coord { row: coord.row + 1, column: coord.column },
            Coord { row: coord.row - 1, column: coord.column },
            Coord { row: coord.row, column: coord.column + 1 },
            Coord { row: coord.row, column: coord.column - 1 },
        ]).difference(&self.rocks).map(|x| x.clone()).collect()
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Coord {
    row: usize,
    column: usize
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::ops::Deref;
    use indoc::indoc;
    use structopt::lazy_static::lazy_static;
    use crate::day21::{Coord, Grid, parse_data, walk_from_start};

    const TEST_DATA: &str = indoc! {
        "...........
         .....###.#.
         .###.##..#.
         ..#.#...#..
         ....#.#....
         .##..S####.
         .##..#...#.
         .......##..
         .##.#.####.
         .##..##.##.
         ..........."
    };

    lazy_static! {
        static ref PARSED_DATA: Grid = Grid {
            start: Coord { row: 5, column: 5 },
            rocks: HashSet::from([
                Coord { row: 1, column: 5 },
                Coord { row: 1, column: 6 },
                Coord { row: 1, column: 7 },
                Coord { row: 1, column: 9 },
                Coord { row: 2, column: 1 },
                Coord { row: 2, column: 2 },
                Coord { row: 2, column: 3 },
                Coord { row: 2, column: 5 },
                Coord { row: 2, column: 6 },
                Coord { row: 2, column: 9 },
                Coord { row: 3, column: 2 },
                Coord { row: 3, column: 4 },
                Coord { row: 3, column: 8 },
                Coord { row: 4, column: 4 },
                Coord { row: 4, column: 6 },
                Coord { row: 5, column: 1 },
                Coord { row: 5, column: 2 },
                Coord { row: 5, column: 6 },
                Coord { row: 5, column: 7 },
                Coord { row: 5, column: 8 },
                Coord { row: 5, column: 9 },
                Coord { row: 6, column: 1 },
                Coord { row: 6, column: 2 },
                Coord { row: 6, column: 5 },
                Coord { row: 6, column: 9 },
                Coord { row: 7, column: 7 },
                Coord { row: 7, column: 8 },
                Coord { row: 8, column: 1 },
                Coord { row: 8, column: 2 },
                Coord { row: 8, column: 4 },
                Coord { row: 8, column: 6 },
                Coord { row: 8, column: 7 },
                Coord { row: 8, column: 8 },
                Coord { row: 8, column: 9 },
                Coord { row: 9, column: 1 },
                Coord { row: 9, column: 2 },
                Coord { row: 9, column: 5 },
                Coord { row: 9, column: 6 },
                Coord { row: 9, column: 8 },
                Coord { row: 9, column: 9 }
            ])
        };
    }

    #[test]
    fn test_parse_data() {
        assert_eq!(parse_data(TEST_DATA), *PARSED_DATA.deref())
    }

    #[test]
    fn test_walk_from_start_1() {
        assert_eq!(walk_from_start(PARSED_DATA.deref(), 1), HashSet::from(
            [
                Coord { row: 5, column: 4 },
                Coord { row: 4, column: 5 },
            ]
        ))
    }

    #[test]
    fn test_walk_from_start_2() {
        assert_eq!(walk_from_start(PARSED_DATA.deref(), 2), HashSet::from(
            [
                Coord { row: 3, column: 5 },
                Coord { row: 5, column: 3 },
                Coord { row: 5, column: 5 },
                Coord { row: 6, column: 4 },
            ]
        ))
    }

    #[test]
    fn test_walk_from_start_3() {
        assert_eq!(walk_from_start(PARSED_DATA.deref(), 3), HashSet::from(
            [
                Coord { row: 3, column: 6 },
                Coord { row: 4, column: 3 },
                Coord { row: 4, column: 5 },
                Coord { row: 5, column: 4 },
                Coord { row: 6, column: 3 },
                Coord { row: 7, column: 4 },
            ]
        ))
    }

    #[test]
    fn test_walk_from_start_6() {
        assert_eq!(walk_from_start(PARSED_DATA.deref(), 6), HashSet::from(
            [
                Coord { row: 2, column: 8 },
                Coord { row: 3, column: 1 },
                Coord { row: 3, column: 3 },
                Coord { row: 3, column: 5 },
                Coord { row: 3, column: 7 },
                Coord { row: 4, column: 0 },
                Coord { row: 4, column: 2 },
                Coord { row: 4, column: 8 },
                Coord { row: 5, column: 3 },
                Coord { row: 5, column: 5 },
                Coord { row: 6, column: 4 },
                Coord { row: 6, column: 6 },
                Coord { row: 7, column: 1 },
                Coord { row: 7, column: 3 },
                Coord { row: 7, column: 5 },
                Coord { row: 9, column: 3 },
            ]
        ))
    }

    #[test]
    fn test_day21a() {
        assert_eq!(walk_from_start(PARSED_DATA.deref(), 6).len(), 16)
    }

}