use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use num::abs;
use tailcall::tailcall;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day10.txt");
    let nodes = generate_node_map(data.as_str());
    let path = get_nodes_in_path(&nodes);
    println!("Part 1: {}", run_day10a(&path));
    println!("Part 2: {}", run_day10b(&path, &nodes));
}

fn run_day10a(nodes: &Vec<(usize, usize)>) -> u64 {
    nodes.len() as u64 / 2
}

fn run_day10b(nodes: &Vec<(usize, usize)>, node_map: &HashMap<(usize, usize), Node>) -> u64 {
    // shoelace then pick's
    let corners: Vec<(usize, usize)> = nodes.iter().filter(|x| node_map.get(x).unwrap().is_angle).map(|x| x.clone()).collect();
    let corners_1: Vec<(usize, usize)> = corners.iter().skip(1).chain(corners.iter().next()).map(|x| x.clone()).collect();

    // shoelace to get the area
    let area = abs::<i64>(corners
        .iter()
        .zip(corners_1.iter())
        .map(|((x1, y1), (x2, y2))| (x1 * y2) as i64 - (x2 * y1) as i64)
        .sum::<i64>() / 2) as u64;

    // picks' theorem
    let b = nodes.len() as u64;
    area + 1 - b/2
}

fn get_nodes_in_path(nodes: &HashMap<(usize, usize), Node>) -> Vec<(usize, usize)> {
    let start_node = nodes.values().find(|x| x.is_start).unwrap();
    let mut path_nodes: Vec<(usize, usize)> = Vec::new();

    #[tailcall]
    fn iterate(node: &Node, prev: (usize, usize), start: (usize, usize), path_nodes: &mut Vec<(usize, usize)>, nodes: &HashMap<(usize, usize), Node>) {
        let next = node.connections.iter().filter(|x| **x != prev).next().unwrap().clone();
        path_nodes.push(next.clone());
        if start != next {
            iterate(nodes.get(&next).unwrap(), node.coord(), start, path_nodes, nodes);
        }
    }

    iterate(start_node, start_node.connections.iter().next().unwrap().clone(), start_node.coord(), &mut path_nodes, nodes);
    path_nodes
}

fn generate_node_map(data: &str) -> HashMap<(usize, usize), Node> {
    let mut nodes: HashMap<(usize, usize), Node> = HashMap::new();
    let mut start_nodes: Vec<(usize, usize)> = Vec::new();
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(node) = create_node(x, y, c) {
                if node.is_start {
                    start_nodes.push((x, y));
                }
                nodes.insert((x, y), node);
            }
        }
    }

    // now, we go back and add the start connections
    for (x1, y1) in start_nodes {
        let mut v: Vec<(usize, usize)> = vec![
            (x1+1, y1),
            (x1, y1+1)
        ];
        if let Some(x2) = x1.checked_sub(1) {
            v.push((x2, y1));
        }
        if let Some(y2) = y1.checked_sub(1) {
            v.push((x1, y2));
        }

        let connections: HashSet<(usize, usize)> = v.iter().flat_map(|x| {
            nodes.get(x)
        }).filter_map(|r| {
            if r.connections.contains(&(x1, y1)) {
                Some((r.x, r.y))
            } else {
                None
            }
        }).collect();
        let mut iter = connections.iter();
        let (first_x, first_y) = iter.next().unwrap();
        let (second_x, second_y) = iter.next().unwrap();
        let is_angle = first_x != second_x && first_y != second_y;
        nodes.insert((x1, y1), Node { x: x1, y: y1, connections, is_start: true, is_angle });
    }
    nodes
}

fn create_node(x: usize, y: usize, c: char) -> Option<Node> {
    // '|' is a vertical pipe connecting north and south.
    // '-' is a horizontal pipe connecting east and west.
    // 'L' is a 90-degree bend connecting north and east.
    // 'J' is a 90-degree bend connecting north and west.
    // '7' is a 90-degree bend connecting south and west.
    // 'F' is a 90-degree bend connecting south and east.
    match c {
        '|' => y.checked_sub(1).map(|y1| Node { x, y, connections:  HashSet::from([(x, y1), (x, y+1)]), is_start: false, is_angle: false}),
        '-' => x.checked_sub(1).map(|x1| Node { x, y, connections:  HashSet::from([(x+1, y), (x1, y)]), is_start: false, is_angle: false}),
        'L' => y.checked_sub(1).map(|y1| Node { x, y, connections:  HashSet::from([(x+1, y), (x, y1)]), is_start: false, is_angle: true}),
        'J' => {
            y.checked_sub(1)
                .map(|y1| x.checked_sub(1)
                    .map(|x1| Node { x, y, connections:  HashSet::from([(x1, y), (x, y1)]), is_start: false, is_angle: true})).flatten()
        },
        '7' => x.checked_sub(1).map(|x1| Node { x, y, connections:  HashSet::from([(x, y+1), (x1, y)]), is_start: false, is_angle: true}),
        'F' => Some(Node { x, y, connections:  HashSet::from([(x, y+1), (x+1, y)]), is_start: false, is_angle: true}),
        'S'  => Some(Node { x, y, connections: HashSet::new(), is_start: true, is_angle: false}), // no coords means start
        _ => None
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Node {
    x: usize,
    y: usize,
    connections: HashSet<(usize, usize)>,
    is_start: bool,
    is_angle: bool
}

impl Node {
    fn coord(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.is_start.hash(state);
        for i in &self.connections {
            i.hash(state);
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashSet, HashMap};
    use std::ops::Deref;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day10::{run_day10a, run_day10b, Node, generate_node_map, get_nodes_in_path};

    const TEST_DATA_1: &str = ".....\n\
                               .F-7.\n\
                               .|.|.\n\
                               .L-J.\n\
                               .....";

    const TEST_DATA_2: &str = ".....\n\
                               .S-7.\n\
                               .|.|.\n\
                               .L-J.\n\
                               .....";

    const TEST_DATA_3: &str = "7-F7-\n\
                               .FJ|7\n\
                               SJLL7\n\
                               |F--J\n\
                               LJ.LJ";

    const TEST_DATA_4: &str = "...........\n\
                               .S-------7.\n\
                               .|F-----7|.\n\
                               .||.....||.\n\
                               .||.....||.\n\
                               .|L-7.F-J|.\n\
                               .|..|.|..|.\n\
                               .L--J.L--J.\n\
                               ...........";

    const TEST_DATA_5: &str = ".F----7F7F7F7F-7....\n\
                               .|F--7||||||||FJ....\n\
                               .||.FJ||||||||L7....\n\
                               FJL7L7LJLJ||LJ.L-7..\n\
                               L--J.L7...LJS7F-7L7.\n\
                               ....F-J..F7FJ|L7L7L7\n\
                               ....L7.F7||L7|.L7L7|\n\
                               .....|FJLJ|FJ|F7|.LJ\n\
                               ....FJL-7.||.||||...\n\
                               ....L---J.LJ.LJLJ...";

    const TEST_DATA_6: &str = "FF7FSF7F7F7F7F7F---7\n\
                               L|LJ||||||||||||F--J\n\
                               FL-7LJLJ||||||LJL-77\n\
                               F--JF--7||LJLJ7F7FJ-\n\
                               L---JF-JLJ.||-FJLJJ7\n\
                               |F|F-JF---7F7-L7L|7|\n\
                               |FFJF7L7F-JF7|JL---7\n\
                               7-L-JL7||F7|L7F-7F7|\n\
                               L.L7LFJ|||||FJL7||LJ\n\
                               L7JLJL-JLJLJL--JLJ.L";

    lazy_static! {
        static ref NODES_1: HashMap<(usize, usize), Node> = HashMap::from([
            ((1,1), Node { x: 1, y: 1, connections: HashSet::from([(2, 1), (1, 2)]), is_start: false, is_angle: true}),
            ((2,1), Node { x: 2, y: 1, connections: HashSet::from([(1, 1), (3, 1)]), is_start: false, is_angle: false}),
            ((3,1), Node { x: 3, y: 1, connections: HashSet::from([(2, 1), (3, 2)]), is_start: false, is_angle: true}),
            ((3,2), Node { x: 3, y: 2, connections: HashSet::from([(3, 1), (3, 3)]), is_start: false, is_angle: false}),
            ((3,3), Node { x: 3, y: 3, connections: HashSet::from([(2, 3), (3, 2)]), is_start: false, is_angle: true}),
            ((2,3), Node { x: 2, y: 3, connections: HashSet::from([(1, 3), (3, 3)]), is_start: false, is_angle: false}),
            ((1,3), Node { x: 1, y: 3, connections: HashSet::from([(1, 2), (2, 3)]), is_start: false, is_angle: true}),
            ((1,2), Node { x: 1, y: 2, connections: HashSet::from([(1, 1), (1, 3)]), is_start: false, is_angle: false}),
        ]);
    }

    lazy_static! {
        static ref NODES_2: HashMap<(usize, usize), Node> = HashMap::from([
            ((1,1), Node { x: 1, y: 1, connections: HashSet::from([(2, 1), (1, 2)]), is_start: true, is_angle: true}),
            ((2,1), Node { x: 2, y: 1, connections: HashSet::from([(1, 1), (3, 1)]), is_start: false, is_angle: false}),
            ((3,1), Node { x: 3, y: 1, connections: HashSet::from([(2, 1), (3, 2)]), is_start: false, is_angle: true}),
            ((3,2), Node { x: 3, y: 2, connections: HashSet::from([(3, 1), (3, 3)]), is_start: false, is_angle: false}),
            ((3,3), Node { x: 3, y: 3, connections: HashSet::from([(2, 3), (3, 2)]), is_start: false, is_angle: true}),
            ((2,3), Node { x: 2, y: 3, connections: HashSet::from([(1, 3), (3, 3)]), is_start: false, is_angle: false}),
            ((1,3), Node { x: 1, y: 3, connections: HashSet::from([(1, 2), (2, 3)]), is_start: false, is_angle: true}),
            ((1,2), Node { x: 1, y: 2, connections: HashSet::from([(1, 1), (1, 3)]), is_start: false, is_angle: false}),
        ]);
    }

    #[test]
    fn test_create_nodes_1() {
        assert_eq!(generate_node_map(TEST_DATA_1), *NODES_1.deref())
    }

    #[test]
    fn test_create_nodes_2() {
        assert_eq!(generate_node_map(TEST_DATA_2), *NODES_2.deref())
    }

    #[test]
    fn test_part_1() {
        let node_map = generate_node_map(TEST_DATA_3);
        let path = get_nodes_in_path(&node_map);
        assert_eq!(run_day10a(&path), 8);
    }

    #[rstest]
    #[case(TEST_DATA_2, 1)]
    #[case(TEST_DATA_3, 1)]
    #[case(TEST_DATA_4, 4)]
    #[case(TEST_DATA_5, 8)]
    #[case(TEST_DATA_6, 10)]
    fn test_part_2(#[case] test: &str, #[case] expected: u64) {
        let node_map = generate_node_map(test);
        let path = get_nodes_in_path(&node_map);
        assert_eq!(run_day10b(&path, &node_map), expected);
    }

}