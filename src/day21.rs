use std::cmp::max;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use num::integer::gcd;
use tailcall::tailcall;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day21.txt");
    let grid = parse_data(data.as_str());
    println!("Part 1: {}", day21a(&grid));
    println!("Part 2: {}", day21b(&grid));
}

fn day21a(grid: &Grid) -> usize {
    walk_from_start(&grid, 64).len()
}

fn day21b(grid: &Grid) -> u64 {
    walk_on_infinite(&grid, 26501365)
}

fn walk_on_infinite(grid: &Grid, steps_to_take: u64) -> u64 {
    let length = grid.rows as u64;
    let enlarged_grid = grid.expand_twice();

    // find stable pattern (flip flop) in original square
    // once found, count, then find length and 2 length later, that's our three points
    // determine quadratic from there.

    #[tailcall]
    fn take_step_until_flip_flop(grid: &Grid, previous: HashSet<Coord>, from: HashSet<Coord>, counter: u64, max: u64) -> (HashSet<Coord>, u64) {
        let next_steps: HashSet<Coord> = from.iter().flat_map(|x| grid.next(x)).collect::<HashSet<Coord>>();
        let next_in_grid: HashSet<Coord> = next_steps
            .iter()
            .filter(|x| x.row < (grid.rows as isize) && x.row >= 0 && x.column < (grid.columns as isize) && x.column >= 0)
            .map(|x| x.clone())
            .collect();
        if next_in_grid == previous || counter == max {
            // this step is our result, return state and count
            (next_steps, counter)
        } else {
            let current: HashSet<Coord> = from.iter()
                .filter(|x| x.row < (grid.rows as isize) && x.row >= 0 && x.column < (grid.columns as isize) && x.column >= 0)
                .map(|x| x.clone())
                .collect();
            take_step_until_flip_flop(grid, current, next_steps, counter + 1, max)
        }
    }

    let initial: HashSet<Coord> = HashSet::from([grid.start.clone()]);
    let (state, first_index) = take_step_until_flip_flop(&enlarged_grid, HashSet::new(), initial, 1, steps_to_take);

    if first_index == steps_to_take {
        state.len() as u64
    } else {
        let first_value = state.len() as u64;
        let target_index_1 = first_index + length;
        let target_index_2 = first_index + 2 * length;

        let second_target = take_step(&enlarged_grid, state, first_index + 1, target_index_1);
        let second_value = second_target.len() as u64;
        let third_target = take_step(&enlarged_grid, second_target, first_index + 1, target_index_2);
        let third_value = third_target.len() as u64;

        // equations are
        // a x^2 + b x + c = y(x)
        // y0 = c
        // y1 = a + b + c
        // y2 = 4a + 2b + c
        //
        // So, (y1 - y0) = a + b
        // and (y2 = 2a + 2(y1 - y0) + y0
        //
        // This means that a = (y2 - 2y1 - y0) / 2
        // so (y1 - y0) - a = b

        // We assume that (target - first index) % length = 0
        let loop_remainder = (steps_to_take - first_index) % length;
        if loop_remainder != 0 {
            panic!("loop {loop_remainder}")
        }

        let target = loop_remainder / (steps_to_take - first_index);

        let a = ((third_value - 2 * second_value - first_value) / 2);
        let b = second_value - first_value - a;

        a * target * target + b * target + first_value
    }
}

#[tailcall]
fn take_step(grid: &Grid, from: HashSet<Coord>, counter: u64, max: u64) -> HashSet<Coord> {
    let next_steps: HashSet<Coord> = from.iter().flat_map(|x| grid.next(x)).collect::<HashSet<Coord>>();
    if counter < max {
        take_step(grid, next_steps, counter + 1, max)
    } else {
        next_steps
    }
}

fn walk_from_start(grid: &Grid, steps_to_take: u64) -> HashSet<Coord> {
    take_step(grid, HashSet::from([grid.start.clone()]), 1, steps_to_take)
}

fn parse_data(data: &str) -> Grid {
    let mut start = Coord { row: 0, column: 0 };
    let mut rocks = HashSet::<Coord>::new();

    let mut rows: usize = 0;
    let mut columns: usize = 0;
    for (row, line) in data.lines().enumerate() {
        for (column, character) in line.chars().enumerate() {
            match character {
                '.' => { /* do nothing */ }
                '#' => { rocks.insert(Coord { row: row as isize, column: column as isize} ); }
                'S' => { start = Coord { row: row as isize, column: column as isize }; }
                a => { panic!("{} is not a valid character", a) }
            }
            columns = max(columns, column);
        }
        rows = max(rows, row);
    }

    Grid { start, rocks, columns, rows }
}

#[derive(Debug, PartialEq)]
struct Grid {
    start: Coord,
    rocks: HashSet<Coord>,
    rows: usize,
    columns: usize
}

impl Grid {
    fn expand_twice(&self) -> Grid {
        let mut rock_on = HashSet::new();
        for r in -2..=2isize {
            for c in -2..=2isize {
                rock_on.extend(self.rocks.iter().map(|x| Coord { row: x.row + r * self.rows as isize, column: x.column + c * self.columns as isize }));
            }
        }

        Grid {
            start: self.start,
            rocks: rock_on,
            rows: self.rows,
            columns: self.columns
        }
    }

    fn next(&self, coord: &Coord) -> HashSet<Coord> {
        HashSet::from([
            Coord { row: coord.row + 1, column: coord.column },
            Coord { row: coord.row - 1, column: coord.column },
            Coord { row: coord.row, column: coord.column + 1 },
            Coord { row: coord.row, column: coord.column - 1 },
        ]).difference(&self.rocks).map(|x| x.clone()).collect()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Coord {
    row: isize,
    column: isize
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::ops::Deref;
    use indoc::indoc;
    use structopt::lazy_static::lazy_static;
    use crate::day21::{Coord, Grid, parse_data, walk_from_start, walk_on_infinite};

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
            rows: 10,
            columns: 10,
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

    #[test]
    fn test_day21b() {
        assert_eq!(walk_on_infinite(PARSED_DATA.deref(), 6), 16);
        assert_eq!(walk_on_infinite(PARSED_DATA.deref(), 10), 50);
        assert_eq!(walk_on_infinite(PARSED_DATA.deref(), 50), 1594);
        assert_eq!(walk_on_infinite(PARSED_DATA.deref(), 100), 6536);
        assert_eq!(walk_on_infinite(PARSED_DATA.deref(), 500), 167004);
        assert_eq!(walk_on_infinite(PARSED_DATA.deref(), 1000), 668697);
        assert_eq!(walk_on_infinite(PARSED_DATA.deref(), 5000), 16733044);
    }

}