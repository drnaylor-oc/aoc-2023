use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use tailcall::tailcall;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day10.txt");
    let nodes = generate_node_map(data.as_str());
    println!("Part 1: {}", run_day10a(&nodes));
}

fn run_day10a(nodes: &HashMap<(usize, usize), Node>) -> u64 {
    let start_node = nodes.values().find(|x| x.is_start).unwrap();

    #[tailcall]
    fn iterate(node: &Node, prev: (usize, usize), start: (usize, usize), count: u64, nodes: &HashMap<(usize, usize), Node>) -> u64 {
        let next = node.connections.iter().filter(|x| **x != prev).next().unwrap().clone();
        if start == next {
            count + 1 // one more step is needed
        } else {
            iterate(nodes.get(&next).unwrap(), node.coord(), start, count + 1, nodes)
        }
    }

    iterate(start_node, start_node.connections.iter().next().unwrap().clone(), start_node.coord(), 0, nodes) / 2
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

        let connections = v.iter().flat_map(|x| {
            nodes.get(x)
        }).filter_map(|r| {
            if r.connections.contains(&(x1, y1)) {
                Some((r.x, r.y))
            } else {
                None
            }
        }).collect();
        nodes.insert((x1, y1), Node { x: x1, y: y1, connections, is_start: true });
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
        '|' => y.checked_sub(1).map(|y1| Node { x, y, connections:  HashSet::from([(x, y1), (x, y+1)]), is_start: false}),
        '-' => x.checked_sub(1).map(|x1| Node { x, y, connections:  HashSet::from([(x+1, y), (x1, y)]), is_start: false}),
        'L' => y.checked_sub(1).map(|y1| Node { x, y, connections:  HashSet::from([(x+1, y), (x, y1)]), is_start: false}),
        'J' => {
            y.checked_sub(1)
                .map(|y1| x.checked_sub(1)
                    .map(|x1| Node { x, y, connections:  HashSet::from([(x1, y), (x, y1)]), is_start: false})).flatten()
        },
        '7' => x.checked_sub(1).map(|x1| Node { x, y, connections:  HashSet::from([(x, y+1), (x1, y)]), is_start: false}),
        'F' => Some(Node { x, y, connections:  HashSet::from([(x, y+1), (x+1, y)]), is_start: false}),
        'S'  => Some(Node { x, y, connections: HashSet::new(), is_start: true}), // no coords means start
        _ => None
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Node {
    x: usize,
    y: usize,
    connections: HashSet<(usize, usize)>,
    is_start: bool
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
    use structopt::lazy_static::lazy_static;
    use crate::day10::{run_day10a, Node, generate_node_map};

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

    lazy_static! {
        static ref NODES_1: HashMap<(usize, usize), Node> = HashMap::from([
            ((1,1), Node { x: 1, y: 1, connections: HashSet::from([(2, 1), (1, 2)]), is_start: false}),
            ((2,1), Node { x: 2, y: 1, connections: HashSet::from([(1, 1), (3, 1)]), is_start: false}),
            ((3,1), Node { x: 3, y: 1, connections: HashSet::from([(2, 1), (3, 2)]), is_start: false}),
            ((3,2), Node { x: 3, y: 2, connections: HashSet::from([(3, 1), (3, 3)]), is_start: false}),
            ((3,3), Node { x: 3, y: 3, connections: HashSet::from([(2, 3), (3, 2)]), is_start: false}),
            ((2,3), Node { x: 2, y: 3, connections: HashSet::from([(1, 3), (3, 3)]), is_start: false}),
            ((1,3), Node { x: 1, y: 3, connections: HashSet::from([(1, 2), (2, 3)]), is_start: false}),
            ((1,2), Node { x: 1, y: 2, connections: HashSet::from([(1, 1), (1, 3)]), is_start: false}),
        ]);
    }

    lazy_static! {
        static ref NODES_2: HashMap<(usize, usize), Node> = HashMap::from([
            ((1,1), Node { x: 1, y: 1, connections: HashSet::from([(2, 1), (1, 2)]), is_start: true}),
            ((2,1), Node { x: 2, y: 1, connections: HashSet::from([(1, 1), (3, 1)]), is_start: false}),
            ((3,1), Node { x: 3, y: 1, connections: HashSet::from([(2, 1), (3, 2)]), is_start: false}),
            ((3,2), Node { x: 3, y: 2, connections: HashSet::from([(3, 1), (3, 3)]), is_start: false}),
            ((3,3), Node { x: 3, y: 3, connections: HashSet::from([(2, 3), (3, 2)]), is_start: false}),
            ((2,3), Node { x: 2, y: 3, connections: HashSet::from([(1, 3), (3, 3)]), is_start: false}),
            ((1,3), Node { x: 1, y: 3, connections: HashSet::from([(1, 2), (2, 3)]), is_start: false}),
            ((1,2), Node { x: 1, y: 2, connections: HashSet::from([(1, 1), (1, 3)]), is_start: false}),
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
        assert_eq!(run_day10a(&node_map), 8);
    }
}