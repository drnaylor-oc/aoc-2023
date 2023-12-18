use itertools::Itertools;
use regex::Regex;
use crate::common::load_from;

type RGB = (u8, u8, u8);

pub fn run_day() {
    let data = load_from("day18.txt");
    let edges = parse_instructions(data.as_str());
    println!("Part 1: {}", day_18a(&edges));
}


fn day_18a(edges: &Vec<Edge>) -> i64 {
    // shoelace to get the area (which will underestimate), pick's to get the internal points,
    // then add it all together to get the total number of points, which is the area/volume.
    // use absolute due to the fact that the sign indicated direction of verticies.
    let shoelace_area: i64 = edges.iter().map(|x| x.det().clone()).sum::<i64>().abs() / 2i64;
    let perimeter = i64::from(edges.iter().map(|x| x.first.row.abs_diff(x.last.row) + x.first.column.abs_diff(x.last.column)).map(|x| x as u32).sum::<u32>());
    let picks_internal_points: i64 = shoelace_area + 1 - (perimeter / 2);
    picks_internal_points + perimeter
}


fn parse_instructions(data: &str) -> Vec<Edge> {
    let mut row = 0i64;
    let mut column = 0i64;

    let mut edges: Vec<Edge> = Vec::new();

    let pattern = Regex::new(r"^([UDLR]) (\d+) \(#([0-9a-f]{6})\)$").unwrap();
    for line in data.lines() {
        let caps = pattern.captures(line).unwrap();
        let direction = caps.get(1).unwrap();
        let steps = str::parse::<i64>(caps.get(2).unwrap().as_str()).unwrap();
        let colour = parse_hex(caps.get(3).unwrap().as_str());
        let last = match direction.as_str() {
            "U" => Coord { row: row - steps, column },
            "D" => Coord { row: row + steps, column },
            "R" => Coord { row, column: column + steps },
            "L" => Coord { row, column: column - steps  },
            a => panic!("Unknown direction: {}", a)
        };
        let first = Coord { row, column };
        row = last.row;
        column = last.column;
        edges.push(Edge {
            first,
            last,
            colour
        });
    }

    edges
}

fn parse_hex(hex: &str) -> (u8, u8, u8) {
    (0..6)
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect_tuple()
        .unwrap()
}

#[derive(PartialEq, Debug)]
struct Edge {
    first: Coord,
    last: Coord,
    colour:  RGB
}

impl Edge {
    fn det(&self) -> i64 {
        (self.first.row * self.last.column) - (self.first.column * self.last.row)
    }
}

#[derive(PartialEq, Hash, Debug)]
struct Coord {
    row: i64,
    column: i64
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::fmt::Write as _;
    use indoc::indoc;
    use proptest::{prop_compose, proptest};
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day18::{Coord, day_18a, Edge, parse_hex, parse_instructions};

    const TEST_DATA: &str = indoc! {
                            "R 6 (#70c710)
                             D 5 (#0dc571)
                             L 2 (#5713f0)
                             D 2 (#d2c081)
                             R 2 (#59c680)
                             D 2 (#411b91)
                             L 5 (#8ceee2)
                             U 2 (#caa173)
                             L 1 (#1b58a2)
                             U 2 (#caa171)
                             R 2 (#7807d2)
                             U 3 (#a77fa3)
                             L 2 (#015232)
                             U 2 (#7a21e3)" };

    lazy_static! {
        static ref PARSED_DATA: Vec<Edge> = vec![
            Edge { first: Coord { row: 0, column: 0 }, last: Coord { row: 0, column: 6 }, colour: (112, 199, 16) }, // R 6
            Edge { first: Coord { row: 0, column: 6 }, last: Coord { row: 5, column: 6 }, colour: (13, 197, 113) }, // D 5
            Edge { first: Coord { row: 5, column: 6 }, last: Coord { row: 5, column: 4 }, colour: (87, 19, 240) }, // L 2
            Edge { first: Coord { row: 5, column: 4 }, last: Coord { row: 7, column: 4 }, colour: (210, 192, 129) }, // D 2
            Edge { first: Coord { row: 7, column: 4 }, last: Coord { row: 7, column: 6 }, colour: (89, 198, 128) }, // R 2
            Edge { first: Coord { row: 7, column: 6 }, last: Coord { row: 9, column: 6 }, colour: (65, 27, 145) }, // D 2
            Edge { first: Coord { row: 9, column: 6 }, last: Coord { row: 9, column: 1 }, colour: (140, 238, 226) }, // L 5
            Edge { first: Coord { row: 9, column: 1 }, last: Coord { row: 7, column: 1 }, colour: (202, 161, 115) }, // U 2
            Edge { first: Coord { row: 7, column: 1 }, last: Coord { row: 7, column: 0 }, colour: (27, 88, 162) }, // L 1
            Edge { first: Coord { row: 7, column: 0 }, last: Coord { row: 5, column: 0 }, colour: (202, 161, 113) }, // U 2
            Edge { first: Coord { row: 5, column: 0 }, last: Coord { row: 5, column: 2 }, colour: (120, 7, 210) }, // R 2
            Edge { first: Coord { row: 5, column: 2 }, last: Coord { row: 2, column: 2 }, colour: (167, 127, 163) }, // U 3
            Edge { first: Coord { row: 2, column: 2 }, last: Coord { row: 2, column: 0 }, colour: (1, 82, 50) }, // L 2
            Edge { first: Coord { row: 2, column: 0 }, last: Coord { row: 0, column: 0 }, colour: (122, 33, 227) } // U 2
        ];
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(parse_instructions(TEST_DATA), *PARSED_DATA.deref())
    }

    #[test]
    fn test_day18a() {
        assert_eq!(day_18a(PARSED_DATA.deref()), 62);
    }

    prop_compose! {
        fn hex_strategy()(r in 0..255u8, g in 0..255u8, b in 0..255u8) -> (String, (u8, u8, u8)) {
            let mut hex_str = String::with_capacity(6);
            write!(&mut hex_str, "{:02x}", r).unwrap();
            write!(&mut hex_str, "{:02x}", g).unwrap();
            write!(&mut hex_str, "{:02x}", b).unwrap();
            (hex_str, (r, g, b))
        }
    }

    proptest! {
        #[test]
        fn test_parse_hex(hex in hex_strategy()) {
            assert_eq!(parse_hex(hex.0.as_str()), hex.1);
        }
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, -30)]
    #[case(2, -10)]
    #[case(3, -8)]
    #[case(4, 14)]
    #[case(5, -12)]
    #[case(6, -45)]
    #[case(7, 2)]
    #[case(8, -7)]
    #[case(9, 0)]
    #[case(10, 10)]
    #[case(11, 6)]
    #[case(12, -4)]
    #[case(13, 0)]
    fn test_edge_det(#[case] idx: usize, #[case] expected: i64) {
        assert_eq!(PARSED_DATA.get(idx).unwrap().det(), expected);
    }

}