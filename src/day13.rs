use std::str::Lines;
use std::iter::Peekable;
use itertools::Itertools;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day13.txt");
    let maps = parse_lines(data.as_str());
    println!("Part 1: {}", day13a(&maps));
    println!("Part 2: {}", day13b(&maps));
}

fn day13a(maps: &Vec<GroundMap>) -> u64 {
    maps.iter().map(|x| {
        x.find_reflection().unwrap_or_else(|| x.transpose().find_reflection().unwrap())
    }).sum()
}

fn day13b(maps: &Vec<GroundMap>) -> u64 {
    maps.iter().map(find_and_fix_smudge).sum()
}

fn find_and_fix_smudge(map: &GroundMap) -> u64 {
    map.fix_smudge().unwrap_or_else(|| map.transpose().fix_smudge().unwrap())
}

fn parse_lines(test_data: &str) -> Vec<GroundMap> {
    let mut lines = test_data.lines().peekable();
    let mut maps: Vec<GroundMap> = Vec::new();
    while let Some(x) = lines.peek() {
        // Ignore the blank lines
        if !x.is_empty() {
            maps.push(parse_map(&mut lines));
        } else {
            lines.next(); // forces the next line to iterate.
        }
    }
    maps
}

fn parse_map(data: &mut Peekable<Lines>) -> GroundMap {
    let mut rows: Vec<Vec<Ground>> = Vec::new();
    while let Some(x) = data.next_if(|x| !(*x).is_empty()) {
        rows.push(parse_line(x));
    }
    let no_of_columns = rows.first().unwrap().len();
    let no_of_rows = rows.len();
    GroundMap {
        rows,
        no_of_columns,
        no_of_rows,
        is_transposed: false
    }
}

fn parse_line(line: &str) -> Vec<Ground> {
    line.chars().map(|x| {
        match x {
            '#' => Ground::Rock,
            '.' => Ground::Ash,
            _ => panic!("Unexpected character")
        }
    }).collect()
}

#[derive(PartialEq, Debug, Clone)]
enum Ground {
    Ash,
    Rock
}


#[derive(PartialEq, Debug, Clone)]
struct GroundMap {
    rows: Vec<Vec<Ground>>,
    no_of_rows: usize,
    no_of_columns: usize,
    is_transposed: bool
}

impl GroundMap {
    fn transpose(&self) -> GroundMap {
        let mut rows: Vec<Vec<Ground>> = Vec::new();
        for col in 0..self.no_of_columns {
            let mut new_row: Vec<Ground> = Vec::new();
            for row in 0..self.no_of_rows {
                unsafe { // We know this will all be here.
                    new_row.push(self.rows.get_unchecked(row).get_unchecked(col).clone());
                }
            }
            rows.push(new_row);
        }
        GroundMap { rows, no_of_rows: self.no_of_columns, no_of_columns: self.no_of_rows, is_transposed: !self.is_transposed }
    }

    fn find_reflection(&self) -> Option<u64> {
        let rows_1 = self.rows.iter().take(self.no_of_rows - 1);
        let rows_2 = self.rows.iter().skip(1);
        let potential_reflections: Vec<usize> = rows_1
            .zip(rows_2)
            .enumerate()
            .filter_map(|(idx, (left, right))| {
                if left.eq(right) {
                    Some(idx + 1) // +1 means that
                } else {
                    None
                }
            })
            .collect();

        // if reflection is between zero and one, we get 1, so we need to do
        // idx * 2 with a reverse iterator.
        potential_reflections.iter().filter(|x| self.check_reflection_around(**x, None)).next().map(|x| if self.is_transposed {
            x.clone() as u64
        } else {
            (*x as u64) * 100
        })
    }

    fn check_reflection_around(&self, reflection_line: usize, ignore: Option<(usize, usize)>) -> bool {
        let (actual_reflection_line, rows_to_check, no_of_rows): (usize, Vec<Vec<Ground>>, usize) = if let Some((first, second)) = ignore {
            (
                reflection_line - 1,
                self.rows.iter().enumerate().filter(|(idx, _)| *idx != first && *idx != second).map(|x| x.1.clone()).collect(),
                self.no_of_rows - 2
            )
        } else {
            (reflection_line, self.rows.clone(), self.no_of_rows)
        };

        if actual_reflection_line == 0 || actual_reflection_line == no_of_rows {
            // we're at the edge, so it's a reflection
            true
        } else {
            let reverse_idx = no_of_rows - actual_reflection_line;
            let window = reverse_idx.min(actual_reflection_line);
            let start = if reverse_idx < reflection_line { // the slice is to the bottom
                actual_reflection_line - window
            } else {
                0
            };

            let list: Vec<&Vec<Ground>> = rows_to_check.iter().skip(start).take(window * 2).collect();
            let reverse_list: Vec<&Vec<Ground>> = rows_to_check.iter().skip(start).take(window * 2).rev().collect();
            list.eq(&reverse_list)
        }
    }

    fn fix_smudge(&self) -> Option<u64> {
        fn check_for_reflection(s: &GroundMap, row_idx_1: usize) -> Option<u64> {
            ((row_idx_1 + 1)..s.no_of_rows).step_by(2).filter_map(|row_idx_2| {  // by 2 as the reflection must have 0, 2, 4 between
                let candidate: Vec<_> = s.rows.get(row_idx_1).unwrap().iter()
                    .zip_eq(s.rows.get(row_idx_2).unwrap().iter())
                    .enumerate()
                    .filter_map(|(idx, (first, second))| {
                        if first.eq(second) {
                            None
                        } else {
                            Some(idx)
                        }
                    })
                    .collect();
                if candidate.len() == 1 {
                    // reflection line
                    // The +1 is due to the fact that the lines are always an odd number apart.
                    // If 0 and 1 are the removed lines, the line is at 1
                    // If 0 and 3 are the removed lines, the line is at 2
                    // If 0 and 5 are the removed lines, the line is at 3
                    let original_reflection_line = row_idx_1 + (row_idx_2 - row_idx_1 + 1) / 2;
                    if s.check_reflection_around(original_reflection_line, Some((row_idx_1, row_idx_2))) {
                        Some(original_reflection_line as u64)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }).next()

        }

        (0..self.no_of_rows)
            .filter_map(|idx| check_for_reflection(self, idx))
            .next()
            .map(|x| {
                if self.is_transposed {
                    x
                } else {
                    x * 100
                }
            })
    }
}

// fn print_line(vec: &Vec<Ground>) -> String {
//     vec.iter().map(|x| match x {
//         Ash => ".",
//         Rock => "#"
//     }).join("")
// }

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day13::{GroundMap, parse_lines, day13a, day13b, find_and_fix_smudge};
    use crate::day13::Ground::*;

    const TEST_DATA: &str = "#.##..##.\n\
                             ..#.##.#.\n\
                             ##......#\n\
                             ##......#\n\
                             ..#.##.#.\n\
                             ..##..##.\n\
                             #.#.##.#.\n\
                             \n\
                             #...##..#\n\
                             #....#..#\n\
                             ..##..###\n\
                             #####.##.\n\
                             #####.##.\n\
                             ..##..###\n\
                             #....#..#";

    lazy_static! {
        static ref PARSED_DATA: Vec<GroundMap> = vec![
            GroundMap {
                rows: vec![
                    vec![Rock, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Ash],
                    vec![Ash, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash],
                    vec![Rock, Rock, Ash, Ash, Ash, Ash, Ash, Ash, Rock],
                    vec![Rock, Rock, Ash, Ash, Ash, Ash, Ash, Ash, Rock],
                    vec![Ash, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash],
                    vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Ash],
                    vec![Rock, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash]
                ],
                no_of_rows: 7,
                no_of_columns: 9,
                is_transposed: false
            },
            GroundMap {
                rows: vec![
                    vec![Rock, Ash, Ash, Ash, Rock, Rock, Ash, Ash, Rock],
                    vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash, Ash, Rock],
                    vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Rock],
                    vec![Rock, Rock, Rock, Rock, Rock, Ash, Rock, Rock, Ash],
                    vec![Rock, Rock, Rock, Rock, Rock, Ash, Rock, Rock, Ash],
                    vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Rock],
                    vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash, Ash, Rock],
                ],
                no_of_rows: 7,
                no_of_columns: 9,
                is_transposed: false
            },
        ];
    }

    lazy_static! {
        static ref TRANSPOSED_PARSED_DATA_1: GroundMap =
            GroundMap {
                rows: vec![
                    vec![Rock, Ash, Rock, Rock, Ash, Ash, Rock],
                    vec![Ash, Ash, Rock, Rock, Ash, Ash, Ash],
                    vec![Rock, Rock, Ash, Ash, Rock, Rock, Rock],
                    vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash],
                    vec![Ash, Rock, Ash, Ash, Rock, Ash, Rock],
                    vec![Ash, Rock, Ash, Ash, Rock, Ash, Rock],
                    vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash],
                    vec![Rock, Rock, Ash, Ash, Rock, Rock, Rock],
                    vec![Ash, Ash, Rock, Rock, Ash, Ash, Ash]
                ],
                no_of_rows: 9,
                no_of_columns: 7,
                is_transposed: true
            };
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(parse_lines(TEST_DATA), *PARSED_DATA.deref());
    }

    #[test]
    fn test_transpose() {
        assert_eq!(PARSED_DATA.get(0).unwrap().transpose(), *TRANSPOSED_PARSED_DATA_1.deref());
    }

    #[rstest]
    #[case(0, false, None)]
    #[case(0, true, Some(5))]
    #[case(1, false, Some(400))]
    #[case(1, true, None)]
    fn find_reflections(#[case] idx: usize, #[case] transpose: bool, #[case] expected: Option<u64>) {
        let map = PARSED_DATA.deref().get(idx).unwrap();
        let sut = if transpose {
            map.transpose()
        } else {
            map.clone()
        };
        assert_eq!(sut.find_reflection(), expected);
    }

    #[rstest]
    #[case(0, false, Some(300))]
    #[case(0, true, None)]
    #[case(1, false, Some(100))]
    #[case(1, true, None)]
    fn test_fix_smudge(#[case] idx: usize, #[case] transpose: bool, #[case] expected: Option<u64>) {
        let map = PARSED_DATA.deref().get(idx).unwrap();
        let sut = if transpose {
            map.transpose()
        } else {
            map.clone()
        };
        assert_eq!(sut.fix_smudge(), expected);
    }

    #[test]
    fn test_find_and_fix_smudge_1() {
        let gm = &PARSED_DATA.deref().get(0).unwrap().transpose();
        let sut = GroundMap {
            rows: gm.rows.clone(),
            no_of_rows: gm.no_of_rows,
            no_of_columns: gm.no_of_columns,
            is_transposed: false
        };
        assert_eq!(find_and_fix_smudge(&sut), 3);
    }

    #[test]
    fn test_day13a() {
        assert_eq!(day13a(PARSED_DATA.deref()), 405);
    }

    #[test]
    fn test_day13b() {
        assert_eq!(day13b(PARSED_DATA.deref()), 400);
    }

}