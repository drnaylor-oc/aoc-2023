use std::collections::HashMap;
use crate::common::load_from;
use crate::day23::Direction::{Any, East, North, South, West};

pub fn run_day() {
    let data = load_from("day23.txt");
    let path_data = parse_data(data.as_str());
}

fn parse_data(data: &str) -> (HashMap<(usize, usize), Direction>, (usize, usize), (usize, usize)) {
    let mut set = HashMap::<(usize, usize), Direction>::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    for (row, line) in data.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            match character {
                '.' => {
                    let coord = (col, row);
                    if row == 0 {
                        start = coord;
                    }
                    end = coord;
                    set.insert((col, row), Any)
                },
                '>' => set.insert((col, row), East),
                '<' => set.insert((col, row), West),
                '^' => set.insert((col, row), North),
                'v' => set.insert((col, row), South),
                _ => None
            };
        }
    }

    (set, start, end)
}



#[derive(Hash, PartialEq, Eq, Debug)]
enum Direction {
    Any,
    North,
    South,
    East,
    West
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use indoc::indoc;
    use crate::day23::{Direction, parse_data};
    use crate::day23::Direction::*;

    const TEST_DATA: &str = indoc! {
        "#.#####################
         #.......#########...###
         #######.#########.#.###
         ###.....#.>.>.###.#.###
         ###v#####.#v#.###.#.###
         ###.>...#.#.#.....#...#
         ###v###.#.#.#########.#
         ###...#.#.#.......#...#
         #####.#.#.#######.#.###
         #.....#.#.#.......#...#
         #.#####.#.#.#########v#
         #.#...#...#...###...>.#
         #.#.#v#######v###.###v#
         #...#.>.#...>.>.#.###.#
         #####v#.#.###v#.#.###.#
         #.....#...#...#.#.#...#
         #.#########.###.#.#.###
         #...###...#...#...#.###
         ###.###.#.###v#####v###
         #...#...#.#.>.>.#.>.###
         #.###.###.#.###.#.#v###
         #.....###...###...#...#
         #####################.#"
    };

    fn get_parsed() -> HashMap<(usize, usize), Direction> {
            HashMap::from([
                ((1, 0), Any),
                ((1, 1), Any),
                ((2, 1), Any),
                ((3, 1), Any),
                ((4, 1), Any),
                ((5, 1), Any),
                ((6, 1), Any),
                ((7, 1), Any),
                ((17, 1), Any),
                ((18, 1), Any),
                ((19, 1), Any),
                ((7, 2), Any),
                ((17, 2), Any),
                ((19, 2), Any),
                ((3, 3), Any),
                ((4, 3), Any),
                ((5, 3), Any),
                ((6, 3), Any),
                ((7, 3), Any),
                ((9, 3), Any),
                ((10, 3), East),
                ((11, 3), Any),
                ((12, 3), East),
                ((13, 3), Any),
                ((17, 3), Any),
                ((19, 3), Any),
                ((3, 4), South),
                ((9, 4), Any),
                ((11, 4), South),
                ((13, 4), Any),
                ((17, 4), Any),
                ((19, 4), Any),
                ((3, 5), Any),
                ((4, 5), East),
                ((5, 5), Any),
                ((6, 5), Any),
                ((7, 5), Any),
                ((9, 5), Any),
                ((11, 5), Any),
                ((13, 5), Any),
                ((14, 5), Any),
                ((15, 5), Any),
                ((16, 5), Any),
                ((17, 5), Any),
                ((19, 5), Any),
                ((20, 5), Any),
                ((21, 5), Any),
                ((3, 6), South),
                ((7, 6), Any),
                ((9, 6), Any),
                ((11, 6), Any),
                ((21, 6), Any),
                ((3, 7), Any),
                ((4, 7), Any),
                ((5, 7), Any),
                ((7, 7), Any),
                ((9, 7), Any),
                ((11, 7), Any),
                ((12, 7), Any),
                ((13, 7), Any),
                ((14, 7), Any),
                ((15, 7), Any),
                ((16, 7), Any),
                ((17, 7), Any),
                ((19, 7), Any),
                ((20, 7), Any),
                ((21, 7), Any),
                ((5, 8), Any),
                ((7, 8), Any),
                ((9, 8), Any),
                ((17, 8), Any),
                ((19, 8), Any),
                ((1, 9), Any),
                ((2, 9), Any),
                ((3, 9), Any),
                ((4, 9), Any),
                ((5, 9), Any),
                ((7, 9), Any),
                ((9, 9), Any),
                ((11, 9), Any),
                ((12, 9), Any),
                ((13, 9), Any),
                ((14, 9), Any),
                ((15, 9), Any),
                ((16, 9), Any),
                ((17, 9), Any),
                ((19, 9), Any),
                ((20, 9), Any),
                ((21, 9), Any),
                ((1, 10), Any),
                ((7, 10), Any),
                ((9, 10), Any),
                ((11, 10), Any),
                ((21, 10), South),
                ((1, 11), Any),
                ((3, 11), Any),
                ((4, 11), Any),
                ((5, 11), Any),
                ((7, 11), Any),
                ((8, 11), Any),
                ((9, 11), Any),
                ((11, 11), Any),
                ((12, 11), Any),
                ((13, 11), Any),
                ((17, 11), Any),
                ((18, 11), Any),
                ((19, 11), Any),
                ((20, 11), East),
                ((21, 11), Any),
                ((1, 12), Any),
                ((3, 12), Any),
                ((5, 12), South),
                ((13, 12), South),
                ((17, 12), Any),
                ((21, 12), South),
                ((1, 13), Any),
                ((2, 13), Any),
                ((3, 13), Any),
                ((5, 13), Any),
                ((6, 13), East),
                ((7, 13), Any),
                ((9, 13), Any),
                ((10, 13), Any),
                ((11, 13), Any),
                ((12, 13), East),
                ((13, 13), Any),
                ((14, 13), East),
                ((15, 13), Any),
                ((17, 13), Any),
                ((21, 13), Any),
                ((5, 14), South),
                ((7, 14), Any),
                ((9, 14), Any),
                ((13, 14), South),
                ((15, 14), Any),
                ((17, 14), Any),
                ((21, 14), Any),
                ((1, 15), Any),
                ((2, 15), Any),
                ((3, 15), Any),
                ((4, 15), Any),
                ((5, 15), Any),
                ((7, 15), Any),
                ((8, 15), Any),
                ((9, 15), Any),
                ((11, 15), Any),
                ((12, 15), Any),
                ((13, 15), Any),
                ((15, 15), Any),
                ((17, 15), Any),
                ((19, 15), Any),
                ((20, 15), Any),
                ((21, 15), Any),
                ((1, 16), Any),
                ((11, 16), Any),
                ((15, 16), Any),
                ((17, 16), Any),
                ((19, 16), Any),
                ((1, 17), Any),
                ((2, 17), Any),
                ((3, 17), Any),
                ((7, 17), Any),
                ((8, 17), Any),
                ((9, 17), Any),
                ((11, 17), Any),
                ((12, 17), Any),
                ((13, 17), Any),
                ((15, 17), Any),
                ((16, 17), Any),
                ((17, 17), Any),
                ((19, 17), Any),
                ((3, 18), Any),
                ((7, 18), Any),
                ((9, 18), Any),
                ((13, 18), South),
                ((19, 18), South),
                ((1, 19), Any),
                ((2, 19), Any),
                ((3, 19), Any),
                ((5, 19), Any),
                ((6, 19), Any),
                ((7, 19), Any),
                ((9, 19), Any),
                ((11, 19), Any),
                ((12, 19), East),
                ((13, 19), Any),
                ((14, 19), East),
                ((15, 19), Any),
                ((17, 19), Any),
                ((18, 19), East),
                ((19, 19), Any),
                ((1, 20), Any),
                ((5, 20), Any),
                ((9, 20), Any),
                ((11, 20), Any),
                ((15, 20), Any),
                ((17, 20), Any),
                ((19, 20), South),
                ((1, 21), Any),
                ((2, 21), Any),
                ((3, 21), Any),
                ((4, 21), Any),
                ((5, 21), Any),
                ((9, 21), Any),
                ((10, 21), Any),
                ((11, 21), Any),
                ((15, 21), Any),
                ((16, 21), Any),
                ((17, 21), Any),
                ((19, 21), Any),
                ((20, 21), Any),
                ((21, 21), Any),
                ((21, 22), Any),
            ])
        }

    #[test]
    fn test_parse_data() {
        assert_eq!(parse_data(TEST_DATA), (get_parsed(), (1, 0), (21, 22)));
    }

}