use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Lines;
use itertools::Itertools;
use tailcall::tailcall;
use crate::common::load_from;
use crate::day14::RockType::{Cube, Rounded};

pub fn run_day() {
    let data = load_from("day14.txt");
    let dish: Dish = Dish::parse(data.as_str());
    println!("Part 1: {}", day14a(&dish));
    println!("Part 2: {}", day14b(&dish));
}

fn day14a(dish: &Dish) -> u64 {
    dish.tilt_north().calculate_load_north()
}

fn day14b(dish: &Dish) -> u64 {
    dish.cycle(1_000_000_000).calculate_load_north()
}

#[derive(PartialEq, Debug)]
struct Dish {

    rock_locations: HashMap<(usize, usize), RockType>,
    no_of_rows: usize,
    no_of_columns: usize

}

impl Dish {

    fn parse(data: &str) -> Dish {
        let mut lines: Peekable<Lines> = data.lines().peekable();
        let no_of_columns = lines.peek().unwrap().len();
        let no_of_rows = lines.count();

        let rock_locations: HashMap<(usize, usize), RockType> = data.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                match c {
                    'O' => Some(((x.clone(), y.clone()), Rounded)),
                    '#' => Some(((x.clone(), y.clone()), Cube)),
                    _ => None
                }
            })
        }).collect();

        Dish {
            rock_locations,
            no_of_rows,
            no_of_columns
        }
    }

    fn cycle(&self, max_count: u64) -> Dish {
        let mut cache: HashMap<Vec<(usize, usize)>, u64> = HashMap::new();

        #[tailcall]
        fn cycles(current_position: HashMap<(usize, usize), RockType>, cols: usize, rows: usize, count: u64, max_count: u64, cache: &mut HashMap<Vec<(usize, usize)>, u64>) -> HashMap<(usize, usize), RockType> {
            let cycled = perform_cycle(&current_position, cols, rows);
            let cache_value: Vec<(usize, usize)> = cycled.iter().filter_map(|((x, y), rock)| match rock {
                Cube => None,
                Rounded => Some((x.clone(), y.clone()))
            }).sorted().collect();
            let next_count = if let Some(original_index) = cache.insert(cache_value, count) {
                // we have a cycle
                let cycle_length = count - original_index;
                let left_to_go = (max_count - count) % cycle_length;
                // clear the cache to avoid problems down the line
                cache.clear();
                max_count - left_to_go
            } else {
                count + 1
            };

            if next_count == max_count {
                cycled
            } else {
                cycles(cycled, cols, rows, next_count, max_count, cache)
            }
        }

        Dish {
            rock_locations: cycles(self.rock_locations.clone(), self.no_of_columns, self.no_of_rows, 1, max_count, &mut cache),
            no_of_rows: self.no_of_rows,
            no_of_columns: self.no_of_columns
        }

    }

    fn tilt_north(&self) -> Dish {
        Dish {
            rock_locations: tilt(&self.rock_locations, self.no_of_columns),
            no_of_rows: self.no_of_rows,
            no_of_columns: self.no_of_columns
        }
    }

    fn calculate_load_north(&self) -> u64 {
        self.rock_locations.iter().filter_map(|((_, row), rock)| {
            match rock {
                Rounded => Some((self.no_of_columns - row) as u64),
                Cube => None
            }
        }).sum()
    }
}

fn tilt(original_locations: &HashMap<(usize, usize), RockType>, cols: usize) -> HashMap<(usize, usize), RockType> {
    let mut rock_locations: HashMap<(usize, usize), RockType> = HashMap::new();
    // work on a per column basis:
    for col in 0..cols {
        // create the column
        let vec: Vec<(usize, RockType)> = original_locations.iter().filter_map(|((x, y), rock_type)| {
            if *x == col {
                Some((y.clone(), rock_type.clone()))
            } else {
                None
            }
        })
            .sorted_by_cached_key(|(idx, _)| idx.clone())
            .collect();

        let mut next_idx = 0;
        for (idx, rock_type) in vec {
            match rock_type {
                Cube => {
                    rock_locations.insert((col, idx), rock_type);
                    next_idx = idx + 1;
                },
                Rounded => {
                    rock_locations.insert((col, next_idx), rock_type);
                    next_idx = next_idx + 1;
                }
            }
        }
    }

    rock_locations
}

fn rotate(x: usize, y: usize, cols_after_rotation: usize) -> (usize, usize) {
    // rotating 90deg.
    // x => y, y => max_cols - x - 1
    // (0, 0) -> (9, 0)
    // (1, 1) -> (8, 1)
    (cols_after_rotation - y - 1, x)
}

fn perform_cycle(current_position: &HashMap<(usize, usize), RockType>, cols: usize, rows: usize) -> HashMap<(usize, usize), RockType> {
    let north = tilt(current_position, cols);
    let rotate_north_to_west = north.iter().map(|((x, y), rock)| (rotate(*x, *y, rows), rock.clone())).collect();
    let west = tilt(&rotate_north_to_west, rows);
    let rotate_west_to_south = west.iter().map(|((x, y), rock)| (rotate(*x, *y, cols), rock.clone())).collect();
    let south = tilt(&rotate_west_to_south, cols);
    let rotate_south_to_east = south.iter().map(|((x, y), rock)| (rotate(*x, *y, rows), rock.clone())).collect();
    let east = tilt(&rotate_south_to_east, rows);
    east.iter().map(|((x, y), rock)| (rotate(*x, *y, cols), rock.clone())).collect() // back to north
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
enum RockType {
    Cube,
    Rounded
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::Deref;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day14::{day14a, day14b, Dish, perform_cycle, rotate};
    use crate::day14::RockType::*;

    const TEST_DATA: &str = "O....#....\n\
                             O.OO#....#\n\
                             .....##...\n\
                             OO.#O....O\n\
                             .O.....O#.\n\
                             O.#..O.#.#\n\
                             ..O..#O..O\n\
                             .......O..\n\
                             #....###..\n\
                             #OO..#....";

    const ONE_CYCLE: &str = ".....#....\n\
                            ....#...O#\n\
                            ...OO##...\n\
                            .OO#......\n\
                            .....OOO#.\n\
                            .O#...O#.#\n\
                            ....O#....\n\
                            ......OOOO\n\
                            #...O###..\n\
                            #..OO#....";

    const TWO_CYCLE: &str = ".....#....\n\
                             ....#...O#\n\
                             .....##...\n\
                             ..O#......\n\
                             .....OOO#.\n\
                             .O#...O#.#\n\
                             ....O#...O\n\
                             .......OOO\n\
                             #..OO###..\n\
                             #.OOO#...O";

    const THREE_CYCLE: &str = ".....#....\n\
                               ....#...O#\n\
                               .....##...\n\
                               ..O#......\n\
                               .....OOO#.\n\
                               .O#...O#.#\n\
                               ....O#...O\n\
                               .......OOO\n\
                               #...O###.O\n\
                               #.OOO#...O";

    const TILTED_NORTH_TEST_DATA: &str = "OOOO.#.O..\n\
                                          OO..#....#\n\
                                          OO..O##..O\n\
                                          O..#.OO...\n\
                                          ........#.\n\
                                          ..#....#.#\n\
                                          ..O..#.O.O\n\
                                          ..O.......\n\
                                          #....###..\n\
                                          #....#....";

    lazy_static! {
        static ref PARSED_DISH: Dish = Dish {
            rock_locations: HashMap::from([
                ((0, 0), Rounded),
                ((5, 0), Cube),
                ((0, 1), Rounded),
                ((2, 1), Rounded),
                ((3, 1), Rounded),
                ((4, 1), Cube),
                ((9, 1), Cube),
                ((5, 2), Cube),
                ((6, 2), Cube),
                ((0, 3), Rounded),
                ((1, 3), Rounded),
                ((3, 3), Cube),
                ((4, 3), Rounded),
                ((9, 3), Rounded),
                ((1, 4), Rounded),
                ((7, 4), Rounded),
                ((8, 4), Cube),
                ((0, 5), Rounded),
                ((2, 5), Cube),
                ((5, 5), Rounded),
                ((7, 5), Cube),
                ((9, 5), Cube),
                ((2, 6), Rounded),
                ((5, 6), Cube),
                ((6, 6), Rounded),
                ((9, 6), Rounded),
                ((7, 7), Rounded),
                ((0, 8), Cube),
                ((5, 8), Cube),
                ((6, 8), Cube),
                ((7, 8), Cube),
                ((0, 9), Cube),
                ((1, 9), Rounded),
                ((2, 9), Rounded),
                ((5, 9), Cube),
            ]),
            no_of_rows: 10,
            no_of_columns: 10
        };
    }


    lazy_static! {
        static ref TILTED_NORTH_PARSED_DISH: Dish = Dish {
            rock_locations: HashMap::from([
                ((0, 0), Rounded),
                ((1, 0), Rounded),
                ((2, 0), Rounded),
                ((3, 0), Rounded),
                ((5, 0), Cube),
                ((7, 0), Rounded),
                ((0, 1), Rounded),
                ((1, 1), Rounded),
                ((4, 1), Cube),
                ((9, 1), Cube),
                ((0, 2), Rounded),
                ((1, 2), Rounded),
                ((4, 2), Rounded),
                ((5, 2), Cube),
                ((6, 2), Cube),
                ((9, 2), Rounded),
                ((0, 3), Rounded),
                ((3, 3), Cube),
                ((5, 3), Rounded),
                ((6, 3), Rounded),
                ((8, 4), Cube),
                ((2, 5), Cube),
                ((7, 5), Cube),
                ((9, 5), Cube),
                ((2, 6), Rounded),
                ((5, 6), Cube),
                ((7, 6), Rounded),
                ((9, 6), Rounded),
                ((2, 7), Rounded),
                ((0, 8), Cube),
                ((5, 8), Cube),
                ((6, 8), Cube),
                ((7, 8), Cube),
                ((0, 9), Cube),
                ((5, 9), Cube),
            ]),
            no_of_rows: 10,
            no_of_columns: 10
        };
    }

    #[test]
    fn test_parse_data() {
        assert_eq!(Dish::parse(TEST_DATA), *PARSED_DISH.deref());
    }

    #[test]
    fn test_parse_data_2() {
        assert_eq!(Dish::parse(TILTED_NORTH_TEST_DATA), *TILTED_NORTH_PARSED_DISH.deref());
    }

    #[test]
    fn test_tilt_north() {
        let tilted = PARSED_DISH.deref().tilt_north();
        assert_eq!(tilted, *TILTED_NORTH_PARSED_DISH.deref());
    }

    #[test]
    fn test_calculate_load_north() {
        assert_eq!(TILTED_NORTH_PARSED_DISH.calculate_load_north(), 136);
    }

    #[test]
    fn test_cycle() {
        let original = &PARSED_DISH.rock_locations;
        let first = perform_cycle(original, 10, 10);
        let second = perform_cycle(&first, 10, 10);
        let third = perform_cycle(&second, 10, 10);
        assert_eq!(first, Dish::parse(ONE_CYCLE).rock_locations);
        assert_eq!(second, Dish::parse(TWO_CYCLE).rock_locations);
        assert_eq!(third, Dish::parse(THREE_CYCLE).rock_locations);
    }

    // 3 x 4 -> 4 x 3 clockwise 90 deg
    //
    // a b x x      x c a
    // c x x x  ->  x x b
    // x x x x      x x x
    //              x x x
    #[rstest]
    #[case(0, 0, 2, 0)]
    #[case(1, 0, 2, 1)]
    #[case(0, 1, 1, 0)]
    fn test_rotation_single(#[case] x_start: usize, #[case] y_start: usize, #[case] x_end: usize, #[case] y_end: usize) {
        assert_eq!(rotate(x_start, y_start, 3), (x_end, y_end));
    }

    #[test]
    fn test_day14a() {
        assert_eq!(day14a(PARSED_DISH.deref()), 136);
    }

    #[test]
    fn test_day14b() {
        assert_eq!(day14b(PARSED_DISH.deref()), 64);
    }

}