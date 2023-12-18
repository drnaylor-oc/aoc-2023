use regex::Regex;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day18.txt");
    let edges = parse_instructions(data.as_str());
    println!("Part 1: {}", day_18a(&edges));
    println!("Part 2: {}", day_18b(&edges));
}


fn day_18a(edges: &Vec<Edge>) -> i64 {
    // shoelace to get the area (which will underestimate), pick's to get the internal points,
    // then add it all together to get the total number of points, which is the area/volume.
    // use absolute due to the fact that the sign indicated direction of verticies.
    let shoelace_area: i64 = edges.iter().map(|x| x.det().clone()).sum::<i64>().abs() / 2i64;
    let perimeter: i64 = edges.iter().map(|x| (x.first.row - x.last.row + x.first.column - x.last.column).abs()).sum();
    let picks_internal_points: i64 = shoelace_area + 1 - (perimeter / 2);
    picks_internal_points + perimeter
}

fn day_18b(edges: &Vec<Edge>) -> i64 {
    // shoelace to get the area (which will underestimate), pick's to get the internal points,
    // then add it all together to get the total number of points, which is the area/volume.
    // use absolute due to the fact that the sign indicated direction of verticies.
    let shoelace_area: i64 = edges.iter().map(|x| x.det_hex().clone()).sum::<i64>().abs() / 2i64;
    let perimeter: i64 = edges.iter().map(|x| (x.hex_first.row - x.hex_last.row + x.hex_first.column - x.hex_last.column).abs()).sum();
    let picks_internal_points: i64 = shoelace_area + 1 - (perimeter / 2);
    picks_internal_points + perimeter
}

fn parse_instructions(data: &str) -> Vec<Edge> {
    let mut row = 0i64;
    let mut column = 0i64;

    let mut row_hex = 0i64;
    let mut column_hex = 0i64;

    let mut edges: Vec<Edge> = Vec::new();

    let pattern = Regex::new(r"^([UDLR]) (\d+) \(#([0-9a-f]{5})([0-3])\)$").unwrap();
    for line in data.lines() {
        let caps = pattern.captures(line).unwrap();

        // simple
        let direction = caps.get(1).unwrap();
        let steps = str::parse::<i64>(caps.get(2).unwrap().as_str()).unwrap();
        let last = match direction.as_str() {
            "U" => Coord { row: row - steps, column },
            "D" => Coord { row: row + steps, column },
            "R" => Coord { row, column: column + steps },
            "L" => Coord { row, column: column - steps  },
            a => panic!("Unknown direction: {}", a)
        };

        // hex
        let hex_steps = i64::from_str_radix(caps.get(3).unwrap().as_str(), 16).unwrap();
        let hex_direction = caps.get(4).unwrap();
        let hex_last = match hex_direction.as_str() {
            "0" => Coord { row: row_hex, column: column_hex + hex_steps },
            "1" => Coord { row: row_hex + hex_steps, column: column_hex },
            "2" => Coord { row: row_hex, column: column_hex - hex_steps  },
            "3" => Coord { row: row_hex - hex_steps, column: column_hex },
            a => panic!("Unknown direction: {}", a)
        };

        let first = Coord { row, column };
        let hex_first = Coord { row: row_hex, column: column_hex };
        row = last.row;
        column = last.column;
        row_hex = hex_last.row;
        column_hex = hex_last.column;
        edges.push(Edge {
            first,
            last,
            hex_first,
            hex_last
        });
    }

    edges
}

#[derive(PartialEq, Debug)]
struct Edge {
    first: Coord,
    last: Coord,
    hex_first: Coord,
    hex_last: Coord
}

impl Edge {
    fn det(&self) -> i64 {
        (self.first.row * self.last.column) - (self.first.column * self.last.row)
    }

    fn det_hex(&self) -> i64 {
        (self.hex_first.row * self.hex_last.column) - (self.hex_first.column * self.hex_last.row)
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
    use indoc::indoc;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day18::{Coord, day_18a, day_18b, Edge, parse_instructions};

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
            Edge { first: Coord { row: 0, column: 0 }, last: Coord { row: 0, column: 6 }, hex_first: Coord { row: 0, column: 0}, hex_last: Coord { row: 0, column: 461937} }, // R 6 -- R 461937
            Edge { first: Coord { row: 0, column: 6 }, last: Coord { row: 5, column: 6 }, hex_first: Coord { row: 0, column: 461937}, hex_last: Coord { row: 56407, column: 461937} }, // D 5 -- D 56407
            Edge { first: Coord { row: 5, column: 6 }, last: Coord { row: 5, column: 4 }, hex_first: Coord { row: 56407, column: 461937}, hex_last: Coord { row: 56407, column: 818608} }, // L 2 -- R 356671
            Edge { first: Coord { row: 5, column: 4 }, last: Coord { row: 7, column: 4 }, hex_first: Coord { row: 56407, column: 818608 }, hex_last: Coord { row: 919647, column: 818608} }, // D 2 -- D 863240
            Edge { first: Coord { row: 7, column: 4 }, last: Coord { row: 7, column: 6 }, hex_first: Coord { row: 919647, column: 818608}, hex_last: Coord { row: 919647, column: 1186328} }, // R 2 -- R 367720
            Edge { first: Coord { row: 7, column: 6 }, last: Coord { row: 9, column: 6 }, hex_first: Coord { row: 919647, column: 1186328}, hex_last: Coord { row: 1186328, column: 1186328} }, // D 2 -- D 266681
            Edge { first: Coord { row: 9, column: 6 }, last: Coord { row: 9, column: 1 }, hex_first: Coord { row: 1186328, column: 1186328}, hex_last: Coord { row: 1186328, column: 609066} }, // L 5 -- L 577262
            Edge { first: Coord { row: 9, column: 1 }, last: Coord { row: 7, column: 1 }, hex_first: Coord { row: 1186328, column: 609066}, hex_last: Coord { row: 356353, column: 609066} }, // U 2 -- U 829975
            Edge { first: Coord { row: 7, column: 1 }, last: Coord { row: 7, column: 0 }, hex_first: Coord { row: 356353, column: 609066}, hex_last: Coord { row: 356353, column: 497056} }, // L 1 -- L 112010
            Edge { first: Coord { row: 7, column: 0 }, last: Coord { row: 5, column: 0 }, hex_first: Coord { row: 356353, column: 497056}, hex_last: Coord { row: 1186328, column: 497056} }, // U 2 -- D 829975
            Edge { first: Coord { row: 5, column: 0 }, last: Coord { row: 5, column: 2 }, hex_first: Coord { row: 1186328, column: 497056}, hex_last: Coord { row: 1186328, column: 5411} }, // R 2 -- L 491645
            Edge { first: Coord { row: 5, column: 2 }, last: Coord { row: 2, column: 2 }, hex_first: Coord { row: 1186328, column: 5411}, hex_last: Coord { row: 500254, column: 5411} }, // U 3 -- U 686074
            Edge { first: Coord { row: 2, column: 2 }, last: Coord { row: 2, column: 0 }, hex_first: Coord { row: 500254, column: 5411}, hex_last: Coord { row: 500254, column: 0} }, // L 2 -- L 5411
            Edge { first: Coord { row: 2, column: 0 }, last: Coord { row: 0, column: 0 }, hex_first: Coord { row: 500254, column: 0}, hex_last: Coord { row: 0, column: 0} } // U 2 -- U 500254
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

    #[test]
    fn test_day18b() {
        assert_eq!(day_18b(PARSED_DATA.deref()), 952408144115);
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