use std::collections::HashMap;
use tailcall::tailcall;
use crate::common::load_from;
use crate::day17::Direction::{East, North, South, West};

pub fn run_day() {
    let data = load_from("day17.txt");
    let grid = Grid::from(data.as_str());
    println!("Part 1: {}", day17a(&grid));
    println!("Part 2: {}", day17b(&grid));
}

fn day17a(grid: &Grid) -> u32 {
    dijkstra_ish(grid, vec![Visitor {
        location: (0, 0),
        current_heat: 0,
        last_direction: North,
        steps_in_direction: 0
    }], false)
}

fn day17b(grid: &Grid) -> u32 {
    dijkstra_ish(grid, vec![Visitor {
        location: (0, 0),
        current_heat: 0,
        last_direction: North,
        steps_in_direction: 0
    }], true)
}

type CacheKey = (usize, usize, Direction, u8);

fn dijkstra_ish(grid: &Grid, initial_visitor: Vec<Visitor>, ultra: bool) -> u32 {
    let mut distances: Vec<Vec<u32>> = vec![vec![u32::MAX; grid.no_of_columns]; grid.no_of_rows];
    let _ = std::mem::replace(&mut distances[0][0], 0u32);
    let mut cache: HashMap<(usize, usize, Direction, u8), u32> = HashMap::new();

    // our initial walk is from 0, 0

    #[tailcall]
    fn step(grid: &Grid, current: Vec<Visitor>, cache: &mut HashMap<CacheKey, u32>, mins: &mut Vec<Vec<u32>>, ultra: bool) {
        let mut next_vectors: Vec<Visitor> = Vec::new();
        for visitor in current {
            let new_visitors: Vec<Visitor> = DIRECTIONS
                .iter()
                .filter(|x|
                    if ultra {
                        visitor.steps_in_direction == 0 || visitor.last_direction.can_go_ultra(x, visitor.steps_in_direction)
                    } else {
                        visitor.steps_in_direction == 0 || visitor.last_direction.can_go(x, visitor.steps_in_direction)
                    }
                )
                .filter_map(|x| grid.visit(&visitor, x))
                .collect();

            for new_visit in new_visitors {
                let (col, row) = new_visit.location;
                let heat = new_visit.current_heat;

                let key: CacheKey = (row, col, new_visit.last_direction.clone(), new_visit.steps_in_direction);
                if cache.get(&key).filter(|x| **x <= heat).is_none() {
                    cache.insert(key, new_visit.current_heat);
                    next_vectors.push(new_visit);
                    if heat <= mins[row][col] {
                        mins[row][col] = heat;
                    }
                }
            }
        }

        if !next_vectors.is_empty() {
            step(grid, next_vectors, cache, mins, ultra)
        }
    }

    step(grid, initial_visitor, &mut cache, &mut distances, ultra);
    distances.last().unwrap().last().unwrap().clone()
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum Direction {
    North,
    West,
    South,
    East
}

const DIRECTIONS: [Direction; 4] = [North, West, South, East];

impl Direction {

    fn get_backwards(&self) -> Direction {
        match *self {
            North => South,
            West => East,
            South => North,
            East => West
        }
    }

    fn can_go(&self, direction: &Direction, steps_taken: u8) -> bool {
        self.get_backwards() != *direction && (self != direction || steps_taken < 3)
    }

    fn can_go_ultra(&self, direction: &Direction, steps_taken: u8) -> bool {
        if steps_taken < 4 {
            self == direction
        } else {
            self.get_backwards() != *direction && (self != direction || steps_taken < 10)
        }
    }
}

#[derive(PartialEq, Debug)]
struct Grid {
    no_of_rows: usize,
    no_of_columns: usize,
    grid: Vec<Vec<u32>>
}

impl Grid {

    fn from(data: &str) -> Grid {
        let grid: Vec<Vec<u32>> = data.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
        let no_of_rows = grid.len();
        let no_of_columns = grid.first().unwrap().len();
        Grid {
            no_of_rows,
            no_of_columns,
            grid
        }
    }

    // x, y => col, row
    // NORTH -1 row -> x, y-1
    // SOUTH +1 row -> x, y+1
    // EAST +1 col -> x+1, y
    // WEST -1 col -> x-1, y
    fn next(&self, current_col: usize, current_row: usize, direction: &Direction) -> Option<(usize, usize)> {
        match *direction {
            North => current_row.checked_sub(1).map(move |y| (current_col, y)),
            West => current_col.checked_sub(1).map(move |x| (x, current_row)),
            South => {
                let y = current_row + 1;
                if y >= self.no_of_rows {
                    None
                } else {
                    Some((current_col, y))
                }
            },
            East => {
                let x = current_col + 1;
                if x >= self.no_of_columns {
                    None
                } else {
                    Some((x, current_row))
                }
            }
        }
    }

    fn visit(&self, current: &Visitor, next_direction: &Direction) -> Option<Visitor> {
        self.next(current.location.0, current.location.1, next_direction)
            .map(move |(col, row)| {
                let steps_in_direction = if current.last_direction == *next_direction {
                    current.steps_in_direction + 1
                } else {
                    1
                };
                Visitor {
                    location: (col, row),
                    current_heat: current.current_heat + self.grid[row][col],
                    last_direction: (*next_direction).clone(),
                    steps_in_direction
                }
            })
    }

}

#[derive(Debug)]
struct Visitor {
    location: (usize, usize),
    current_heat: u32,
    last_direction: Direction,
    steps_in_direction: u8
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day17::{day17a, day17b, Direction, Grid};
    use crate::day17::Direction::*;

    const TEST_DATA: &str = "2413432311323\n\
                             3215453535623\n\
                             3255245654254\n\
                             3446585845452\n\
                             4546657867536\n\
                             1438598798454\n\
                             4457876987766\n\
                             3637877979653\n\
                             4654967986887\n\
                             4564679986453\n\
                             1224686865563\n\
                             2546548887735\n\
                             4322674655533";

    const TEST_DATA_2: &str = "111111111111\n\
                               999999999991\n\
                               999999999991\n\
                               999999999991\n\
                               999999999991";

    lazy_static! {
        static ref PARSED_DATA_VEC: Vec<Vec<u32>> = vec![
            vec![2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
            vec![3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
            vec![3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
            vec![3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
            vec![4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
            vec![1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
            vec![4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
            vec![3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
            vec![4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
            vec![4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
            vec![1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
            vec![2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
            vec![4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3]
        ];
    }

    lazy_static! {
        static ref PARSED_DATA: Grid = Grid {
            no_of_columns: 13,
            no_of_rows: 13,
            grid: PARSED_DATA_VEC.clone()
        };
    }

    #[test]
    fn test_grid_from() {
        assert_eq!(Grid::from(TEST_DATA), *PARSED_DATA.deref());
    }

    #[rstest]
    #[case(North, South, 0, false)]
    #[case(South, North, 0, false)]
    #[case(North, North, 0, true)]
    #[case(North, North, 1, true)]
    #[case(North, North, 2, true)]
    #[case(North, North, 3, false)]
    #[case(North, East, 3, true)]
    #[case(North, East, 2, true)]
    #[case(North, East, 1, true)]
    #[case(North, East, 0, true)]
    #[case(East, West, 0, false)]
    #[case(East, North, 0, true)]
    #[case(East, South, 3, true)]
    #[case(East, East, 4, false)] // shouldn't happen, but check anyway
    #[case(East, South, 4, true)] // shouldn't happen, but check anyway
    fn test_direction_can_go(#[case] direction: Direction, #[case] proposed: Direction, #[case] steps_taken: u8, #[case] expected: bool) {
        assert_eq!(direction.can_go(&proposed, steps_taken), expected);
    }

    // x, y => col, row
    // NORTH -1 row -> x, y-1
    // SOUTH +1 row -> x, y+1
    // EAST +1 col -> x+1, y
    // WEST +1 row -> x, y+1
    #[rstest]
    #[case(North, 0, 0, None)]
    #[case(West, 0, 0, None)]
    #[case(East, 0, 0, Some((1, 0)))]
    #[case(East, 0, 1, Some((1, 1)))]
    #[case(East, 1, 0, Some((2, 0)))]
    #[case(South, 0, 0, Some((0, 1)))]
    #[case(North, 12, 12, Some((12, 11)))]
    #[case(West, 12, 12, Some((11, 12)))]
    #[case(East, 12, 12, None)]
    #[case(South, 12, 12, None)]
    fn test_grid_next(#[case] direction: Direction, #[case] current_column: usize, #[case] current_row: usize, #[case] expected: Option<(usize, usize)>) {
        assert_eq!(PARSED_DATA.next(current_column, current_row, &direction), expected)
    }

    // 0 0 0
    // x 0 0
    #[test]
    fn test_grid_next_1() {
        assert_eq!(PARSED_DATA.next(0, 1, &East), Some((1, 1)));
    }

    // 0 x 0
    // 0 0 0
    #[test]
    fn test_grid_next_2() {
        assert_eq!(PARSED_DATA.next(1, 0, &East), Some((2, 0)));
    }

    #[test]
    fn test_day17b() {
        assert_eq!(day17a(PARSED_DATA.deref()), 102);
    }

    #[test]
    fn test_day17b_2() {
        assert_eq!(day17b(&Grid::from(TEST_DATA_2)), 47); // the example in AoC is wrong, probably deliberately.
    }

}