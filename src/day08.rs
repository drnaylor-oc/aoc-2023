use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use num::integer::lcm;
use once_cell::sync::Lazy;
use regex::Regex;
use tailcall::tailcall;
use crate::common::load_from;
use crate::day08::Direction::*;

static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap());

pub fn run_day() {
    let data = load_from("day08.txt");
    let (directions, nodes) = parse_data(data.as_str());
    println!("Part 1: {}", day08a(&directions, &nodes));
    println!("Part 2: {}", day08b(&directions, &nodes));
}

fn day08a(directions: &Vec<Direction>, nodes: &HashMap<String, Node>) -> u64 {
    walk_nodes(directions, nodes)
}

fn day08b(directions: &Vec<Direction>, nodes: &HashMap<String, Node>) -> u64 {
    walk_nodes_simultaneously(directions, nodes)
}

#[tailcall]
fn walk(directions: &Vec<Direction>, nodes: &HashMap<String, Node>, current: &Node, end_pattern: Regex, count: u64) -> u64 {
    let direction: &Direction = directions.get(count as usize % directions.len()).unwrap();
    let new = match *direction {
        Left => &current.left,
        Right => &current.right
    };
    if end_pattern.is_match(new) {
        count + 1
    } else {
        walk(directions, nodes, nodes.get(new).unwrap(), end_pattern, count+1)
    }
}

fn walk_nodes(directions: &Vec<Direction>, nodes: &HashMap<String, Node>) -> u64 {
    walk(directions, nodes, nodes.get(&String::from("AAA")).unwrap(), Regex::new("ZZZ").unwrap(), 0)
}


fn walk_nodes_simultaneously(directions: &Vec<Direction>, nodes: &HashMap<String, Node>) -> u64 {
    // It turns out that the steps from start to finish loop, see example data.
    // So, we need to get the number of steps in a loop for each input.
    // Each loop ENDS with a -Z
    let loop_sizes: Vec<u64> = nodes.iter().filter_map(|(label, node)| {
        if label.ends_with("A") {
            Some(walk(directions, nodes, node, Regex::new("[A-Z0-9]{2}Z").unwrap(), 0))
        } else {
            None
        }
    }).collect();

    // get the lowest common multiple of these limits
    loop_sizes.iter().map(|x| x.clone()).reduce(|x, y| lcm(x, y)).unwrap()
}

fn parse_data(data: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    let mut lines = data.lines();
    let directions = parse_directions(lines.next().unwrap());

    // ignore the next line
    lines.next();

    let mut map: HashMap<String, Node, RandomState> = HashMap::new();
    while let Some(line) = lines.next() {
        if let Some(captures) = REGEX.captures(line) {
            match captures.extract() {
                (_, [current, left, right]) => map.insert(String::from(current), Node { left: String::from(left), right: String::from(right) })
            };
        }
    }

    (directions, map)
}

fn parse_directions(line: &str) -> Vec<Direction> {
    line.chars().map(|c| {
        match c {
            'L' => Left,
            'R' => Right,
            _ => panic!("Not a direction")
        }
    }).collect()
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right
}

#[derive(PartialEq, Debug)]
struct Node {
    left: String,
    right: String
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::RandomState;
    use std::collections::HashMap;
    use std::ops::Deref;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day08::{Direction, parse_data, parse_directions, Node, day08a, day08b};
    use crate::day08::Direction::*;

    const TEST_DATA_1: &str = "RL\n\
                               \n\
                               AAA = (BBB, CCC)\n\
                               BBB = (DDD, EEE)\n\
                               CCC = (ZZZ, GGG)\n\
                               DDD = (DDD, DDD)\n\
                               EEE = (EEE, EEE)\n\
                               GGG = (GGG, GGG)\n\
                               ZZZ = (ZZZ, ZZZ)";

    const TEST_DATA_2: &str = "LLR\n\
                               \n\
                               AAA = (BBB, BBB)\n\
                               BBB = (AAA, ZZZ)\n\
                               ZZZ = (ZZZ, ZZZ)";

    const TEST_DATA_3: &str = "LR\n\
                               \n\
                               11A = (11B, XXX)\n\
                               11B = (XXX, 11Z)\n\
                               11Z = (11B, XXX)\n\
                               22A = (22B, XXX)\n\
                               22B = (22C, 22C)\n\
                               22C = (22Z, 22Z)\n\
                               22Z = (22B, 22B)\n\
                               XXX = (XXX, XXX)";

    lazy_static! {
        static ref DIRECTION_1: Vec<Direction> = vec![Right, Left];
    }
    lazy_static! {
        static ref DIRECTION_2: Vec<Direction> = vec![Left, Left, Right];
    }
    lazy_static! {
        static ref DIRECTION_3: Vec<Direction> = vec![Left, Right];
    }
    lazy_static! {
        static ref NODES_1: HashMap<String, Node, RandomState> = HashMap::from([
            (String::from("AAA"), Node { left: String::from("BBB"), right: String::from("CCC") }),
            (String::from("BBB"), Node { left: String::from("DDD"), right: String::from("EEE") }),
            (String::from("CCC"), Node { left: String::from("ZZZ"), right: String::from("GGG") }),
            (String::from("DDD"), Node { left: String::from("DDD"), right: String::from("DDD") }),
            (String::from("EEE"), Node { left: String::from("EEE"), right: String::from("EEE") }),
            (String::from("GGG"), Node { left: String::from("GGG"), right: String::from("GGG") }),
            (String::from("ZZZ"), Node { left: String::from("ZZZ"), right: String::from("ZZZ") }),
        ]);
    }
    lazy_static! {
        static ref NODES_2: HashMap<String, Node, RandomState> = HashMap::from([
            (String::from("AAA"), Node { left: String::from("BBB"), right: String::from("BBB") }),
            (String::from("BBB"), Node { left: String::from("AAA"), right: String::from("ZZZ") }),
            (String::from("ZZZ"), Node { left: String::from("ZZZ"), right: String::from("ZZZ") }),
        ]);
    }
    lazy_static! {
        static ref NODES_3: HashMap<String, Node, RandomState> = HashMap::from([
            (String::from("11A"), Node { left: String::from("11B"), right: String::from("XXX") }),
            (String::from("11B"), Node { left: String::from("XXX"), right: String::from("11Z") }),
            (String::from("11Z"), Node { left: String::from("11B"), right: String::from("XXX") }),
            (String::from("22A"), Node { left: String::from("22B"), right: String::from("XXX") }),
            (String::from("22B"), Node { left: String::from("22C"), right: String::from("22C") }),
            (String::from("22C"), Node { left: String::from("22Z"), right: String::from("22Z") }),
            (String::from("22Z"), Node { left: String::from("22B"), right: String::from("22B") }),
            (String::from("XXX"), Node { left: String::from("XXX"), right: String::from("XXX") }),
        ]);
    }

    #[rstest]
    #[case("LRLRLR", vec![Left, Right, Left, Right, Left, Right])]
    #[case("LLRR", vec![Left, Left, Right, Right])]
    #[case("RRLRLLR", vec![Right, Right, Left, Right, Left, Left, Right])]
    fn test_parse_direction(#[case] string: &str, #[case] expected: Vec<Direction>) {
        assert_eq!(parse_directions(string), expected);
    }

    #[test]
    fn test_parse_data_1() {
        let (directions, nodes) = parse_data(TEST_DATA_1);
        assert_eq!(directions, *DIRECTION_1.deref());
        assert_eq!(nodes, *NODES_1.deref());
    }

    #[test]
    fn test_parse_data_2() {
        let (directions, nodes) = parse_data(TEST_DATA_2);
        assert_eq!(directions, *DIRECTION_2.deref());
        assert_eq!(nodes, *NODES_2.deref());
    }

    #[test]
    fn test_parse_data_3() {
        let (directions, nodes) = parse_data(TEST_DATA_3);
        assert_eq!(directions, *DIRECTION_3.deref());
        assert_eq!(nodes, *NODES_3.deref());
    }

    #[test]
    fn test_day08a() {
        assert_eq!(day08a(DIRECTION_1.deref(), NODES_1.deref()), 2);
        assert_eq!(day08a(DIRECTION_2.deref(), NODES_2.deref()), 6);
    }

    #[test]
    fn test_day08b() {
        assert_eq!(day08b(DIRECTION_3.deref(), NODES_3.deref()), 6);
    }

}