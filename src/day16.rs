use std::collections::{HashMap, HashSet};
use tailcall::tailcall;
use crate::common::load_from;
use crate::day16::Direction::*;
use crate::day16::Object::{MirrorBLUR, MirrorULBR, SplitterH, SplitterV};

type Vector = ((usize, usize), Direction);

pub fn run_day() {
    let data = load_from("day16.txt");
    let board = create_board(data.as_str());
    println!("Part 1: {}", day16a(&board));
    println!("Part 2: {}", day16b(&board));
}

fn day16a(board: &Board) -> usize {
    let steps = run_steps(board, ((0, 0), Right));
    let set: HashSet<(usize, usize)> = steps.iter().map(|((x, y), _)| (x.clone(), y.clone())).collect();
    set.len()
}

fn day16b(board: &Board) -> usize {
    let mut max: usize = 0;

    fn get_steps(row: usize, col: usize, direction: Direction, board: &Board) -> usize {
        let steps = run_steps(board, ((col, row), direction));
        let set: HashSet<(usize, usize)> = steps.iter().map(|((x, y), _)| (x.clone(), y.clone())).collect();
        set.len()
    }

    for row in 0..board.rows {
        max = max.max(get_steps(row, 0, Right, board));
        max = max.max(get_steps(row, board.columns - 1, Left, board));
    }

    for col in 0..board.columns {
        max = max.max(get_steps(0, col, Down, board));
        max = max.max(get_steps(board.rows - 1, col, Up, board));
    }

    max
}

fn run_steps(board: &Board, init: Vector) -> HashSet<Vector> {
    let mut steps: HashSet<Vector> = HashSet::new();
    steps.insert(init);
    step(vec![init], board, &mut steps);
    steps
}

#[tailcall]
fn step(incoming: Vec<Vector>, board: &Board, steps: &mut HashSet<Vector>) {
    if !incoming.is_empty() {
        let mut outgoing: Vec<Vector> = Vec::new();
        for ((x1, y1), direction) in incoming {
            let object = board.objects.get(&(x1, y1));
            for d in direction.clone().strike_object_option(object) {
                if let Some(next) = d.next(x1, y1, board.columns, board.rows) {
                    if steps.insert(next.clone()) {
                        // next is the coords and incoming direction.
                        outgoing.push(next);
                    }
                }
            }
        }
        step(outgoing, board, steps);
    }
}

fn create_board(data: &str) -> Board {
    let lines: Vec<&str> = data.lines().collect();
    let rows = lines.len();
    let columns = lines.first().unwrap().len();

    let objects: HashMap<(usize, usize), Object> = lines
        .iter()
        .enumerate()
        .flat_map(|(idx, line)| {
            let y = idx;
            // the y is unique to this iterator so we move it into here.
            line.chars().enumerate().flat_map(move |(x1, c1)| {
                let x = x1;
                let c = c1;
                get_object(&c).map(|obj| ((x.clone(), y.clone()), obj))
            })
        })
        .collect();

    Board { rows, columns, objects }
}

fn get_object(c: &char) -> Option<Object> {
    match c {
        '/'  => Some(MirrorBLUR),
        '\\' => Some(MirrorULBR),
        '-'  => Some(SplitterH),
        '|'  => Some(SplitterV),
        _    => None
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Direction {
    Left,
    Down,
    Right,
    Up
}

impl Direction {

    fn next(&self, x: usize, y: usize, x_max: usize, y_max: usize) -> Option<Vector> {
        match *self {
            Left => x.checked_sub(1).map(|r| ((r, y), self.clone())),
            Down => {
                let next_y = y + 1;
                if next_y == y_max {
                    None
                } else {
                    Some(((x, next_y), self.clone()))
                }
            }
            Right => {
                let next_x = x + 1;
                if next_x == x_max {
                    None
                } else {
                    Some(((next_x, y), self.clone()))
                }
            },
            Up => y.checked_sub(1).map(|r| ((x, r), self.clone()))
        }
    }

    fn strike_object_option(&self, object: Option<&Object>) -> Vec<Direction> {
        object.map(|x| self.strike_object(x)).unwrap_or_else(|| vec![self.clone()])
    }

    fn strike_object(&self, object: &Object) -> Vec<Direction> {
        match *object {
            MirrorULBR => { // \
                match self {
                    Left => vec![Up],
                    Down => vec![Right],
                    Right => vec![Down],
                    Up => vec![Left]
                }
            },
            MirrorBLUR => { // /
                match self {
                    Right => vec![Up],
                    Down => vec![Left],
                    Left => vec![Down],
                    Up => vec![Right]
                }
            },
            SplitterH => { // -
                match self {
                    Down => vec![Left, Right],
                    Up => vec![Left, Right],
                    dir => vec![dir.clone()]
                }
            },
            SplitterV => { // |
                match self {
                    Left => vec![Up, Down],
                    Right => vec![Up, Down],
                    dir => vec![dir.clone()]
                }
            }
        }
    }

}

#[repr(u8)]
#[derive(PartialEq, Debug)]
enum Object {
    MirrorULBR,
    MirrorBLUR,
    SplitterH,
    SplitterV
}

#[derive(PartialEq, Debug)]
struct Board {
    rows: usize,
    columns: usize,
    objects: HashMap<(usize, usize), Object>
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::ops::Deref;
    use indoc::indoc;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day16::{Board, create_board, day16a, day16b, Direction, Object};
    use crate::day16::Object::*;
    use crate::day16::Direction::*;

    const TEST_INPUT: &str = indoc! {
        r#".|...\....
           |.-.\.....
           .....|-...
           ........|.
           ..........
           .........\
           ..../.\\..
           .-.-/..|..
           .|....-|.\
           ..//.|...."#
    };

    lazy_static! {
        static ref PARSED_INPUT: Board = Board {
            rows: 10,
            columns: 10,
            objects: HashMap::from([
                ((1, 0), SplitterV),
                ((5, 0), MirrorULBR),
                ((0, 1), SplitterV),
                ((2, 1), SplitterH),
                ((4, 1), MirrorULBR),
                ((5, 2), SplitterV),
                ((6, 2), SplitterH),
                ((8, 3), SplitterV),
                ((9, 5), MirrorULBR),
                ((4, 6), MirrorBLUR),
                ((6, 6), MirrorULBR),
                ((7, 6), MirrorULBR),
                ((1, 7), SplitterH),
                ((3, 7), SplitterH),
                ((4, 7), MirrorBLUR),
                ((7, 7), SplitterV),
                ((1, 8), SplitterV),
                ((6, 8), SplitterH),
                ((7, 8), SplitterV),
                ((9, 8), MirrorULBR),
                ((2, 9), MirrorBLUR),
                ((3, 9), MirrorBLUR),
                ((5, 9), SplitterV)
            ])
        };
    }

    #[test]
    fn test_create_board() {
        assert_eq!(create_board(TEST_INPUT), *PARSED_INPUT.deref());
    }

    #[rstest]
    #[case(Down, MirrorULBR, vec![Right])]
    #[case(Up, MirrorULBR, vec![Left])]
    #[case(Right, MirrorULBR, vec![Down])]
    #[case(Left, MirrorULBR, vec![Up])]
    #[case(Down, MirrorBLUR, vec![Left])]
    #[case(Left, MirrorBLUR, vec![Down])]
    #[case(Up, MirrorBLUR, vec![Right])]
    #[case(Right, MirrorBLUR, vec![Up])]
    #[case(Right, SplitterH, vec![Right])]
    #[case(Left, SplitterH, vec![Left])]
    #[case(Up, SplitterH, vec![Left, Right])]
    #[case(Down, SplitterH, vec![Left, Right])]
    #[case(Down, SplitterV, vec![Down])]
    #[case(Up, SplitterV, vec![Up])]
    #[case(Left, SplitterV, vec![Up, Down])]
    #[case(Right, SplitterV, vec![Up, Down])]
    fn test_direction_strike_object(#[case] input: Direction, #[case] object: Object, #[case] output: Vec<Direction>) {
        assert_eq!(input.strike_object(&object), output);
    }

    #[test]
    fn test_day16a() {
        assert_eq!(day16a(PARSED_INPUT.deref()), 46);
    }

    #[test]
    fn test_day16b() {
        assert_eq!(day16b(PARSED_INPUT.deref()), 51);
    }

}