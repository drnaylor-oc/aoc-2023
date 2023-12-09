use std::collections::HashSet;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day09.txt");
    let input: Vec<Vec<i128>> = parse_lines_to_numbers(data.as_str());
    println!("Part 1: {}", day09a(&input));
    println!("Part 2: {}", day09b(&input));
}

fn day09a(data: &Vec<Vec<i128>>) -> i128 {
    let data_to_process: Vec<(usize, Function)> = data.iter().map(parse_line).collect();
    calculate(&data_to_process)
}

fn day09b(data: &Vec<Vec<i128>>) -> i128 {
    let data_to_process: Vec<(usize, Function)> = data.iter().map(|x| {
        let reversed: Vec<i128> = x.iter().map(|x| x.clone()).rev().collect(); // reverses the order to get a new set of functions.
        parse_line(&reversed)
    }).collect();
    calculate(&data_to_process)
}

fn calculate(data: &Vec<(usize, Function)>) -> i128 {
    data.iter().map(|(next_idx, func)| func.get(*next_idx)).sum()
}

fn parse_lines_to_numbers(lines: &str) -> Vec<Vec<i128>> {
    lines.lines().map(|line| line.split_whitespace().map(|x| {
        str::parse::<i128>(x).unwrap()
    }).collect()).collect()
}

fn parse_line(numbers: &Vec<i128>) -> (usize, Function) {
    let function = Function {
        initial: numbers.first().unwrap().clone(),
        then: Some(Box::new(calculate_function(&numbers)))
    };
    (numbers.len(), function)
}

fn calculate_function(numbers: &Vec<i128>) -> Function {
    let differences: Vec<i128> = numbers[..numbers.len()-1].iter().zip(numbers[1..].iter()).map(|(first, second)| second - first).collect();
    if HashSet::<&i128>::from_iter(differences.iter()).len() == 1 {
        Function {
            initial: differences[0],
            then: None
        }
    } else {
        Function {
            initial: differences[0],
            then: Some(Box::new(calculate_function(&differences)))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Function {
    initial: i128,
    then: Option<Box<Function>>
}

impl Function {
    fn get(&self, index: usize) -> i128 {
        if index == 0 {
            self.initial
        } else {
            self.then.as_ref().map(|x| {
                let right: i128 = (0..index).map(|i| x.as_ref().get(i)).sum(); // factorials
                right + self.initial
            }).unwrap_or(self.initial)
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day09::{calculate, day09a, day09b, Function, parse_line, parse_lines_to_numbers};

    const TEST_DATA: &str = "0 3 6 9 12 15\n\
                             1 3 6 10 15 21\n\
                             10 13 16 21 30 45";

    const TEST_DATA_2: &str = "16 22 27 23 4 -16 23 267 1025 2943 7407 17445 39670 88339 193607 417868 887273 1851158 3792224 7628687";

    lazy_static! {
        static ref TEST_FUNCTION_1: Function = Function {
            initial: 0,
            then: Some(Box::new(Function {
                initial: 3,
                then: None
            }))
        };
    }

    lazy_static! {
        static ref TEST_FUNCTION_2: Function = Function {
            initial: 1,
            then: Some(Box::new(Function {
                initial: 2,
                then: Some(Box::new(Function {
                    initial: 1,
                    then: None
                }))
            }))
        };
    }

    lazy_static! {
        static ref TEST_FUNCTION_3: Function = Function {
            initial: 10,
            then: Some(Box::new(Function {
                initial: 3,
                then: Some(Box::new(Function {
                    initial: 0,
                    then: Some(Box::new(Function {
                        initial: 2,
                        then: None
                    }))
                }))
            }))
        };
    }

    fn create_func(data: Vec<i128>) -> Function {
        #[tailcall::tailcall]
        fn accumulate(data_to_append: &[i128], acc: Function) -> Function {
            if data_to_append.is_empty() {
                acc
            } else {
                accumulate(&data_to_append[..data_to_append.len()-1], Function {
                    initial: data_to_append.last().unwrap().clone(),
                    then: Some(Box::new(acc))
                })
            }
        }

        accumulate(&data[..data.len()-1], Function {
            initial: data.last().unwrap().clone(),
            then: None
        })
    }

    lazy_static! {
        static ref TEST_FUNCTION_4: Function = create_func(vec![
            16,
            6,
            -1,
            -8,
            2,
            18,
            8,
            6,
            17,
            20,
            15,
            16,
            21,
            1,
            1,
            18,
            7,
            -5
        ]);
    }

    #[test]
    fn test_day09a() {
        let data: Vec<Vec<i128>> = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45]
        ];
        assert_eq!(day09a(&data), 114);
    }

    #[test]
    fn test_day09b() {
        let data: Vec<Vec<i128>> = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45]
        ];
        assert_eq!(day09b(&data), 2);
    }

    #[test]
    fn test_calculate() {
        let data: Vec<(usize, Function)> = vec![
            (6, TEST_FUNCTION_1.clone()),
            (6, TEST_FUNCTION_2.clone()),
            (6, TEST_FUNCTION_3.clone())
        ];
        assert_eq!(calculate(&data), 114);
    }

    #[test]
    fn test_parse_lines() {
        let data: Vec<(usize, Function)> = parse_lines_to_numbers(TEST_DATA).iter().map(|x| parse_line(x)).collect();
        assert_eq!(vec![
            (6, TEST_FUNCTION_1.clone()),
            (6, TEST_FUNCTION_2.clone()),
            (6, TEST_FUNCTION_3.clone())
        ], data);
    }

    #[test]
    fn test_line_from_puzzle() {
        let data = parse_lines_to_numbers(TEST_DATA_2).first().unwrap().iter().map(|x| x.clone()).collect(); // cloning
        assert_eq!(day09a(&vec![data]), 15083115);
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 3)]
    #[case(2, 6)]
    #[case(3, 9)]
    #[case(4, 12)]
    #[case(5, 15)]
    #[case(6, 18)]
    fn test_function_1_get(#[case] index: usize, #[case] expected: i128) {
        assert_eq!(TEST_FUNCTION_1.get(index), expected);
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 3)]
    #[case(2, 6)]
    #[case(3, 10)]
    #[case(4, 15)]
    #[case(5, 21)]
    #[case(6, 28)]
    fn test_function_2_get(#[case] index: usize, #[case] expected: i128) {
        assert_eq!(TEST_FUNCTION_2.get(index), expected);
    }

    #[rstest]
    #[case(0, 10)]
    #[case(1, 13)]
    #[case(2, 16)]
    #[case(3, 21)]
    #[case(4, 30)]
    #[case(5, 45)]
    #[case(6, 68)]
    fn test_function_3_get(#[case] index: usize, #[case] expected: i128) {
        assert_eq!(TEST_FUNCTION_3.get(index), expected);
    }

    #[rstest]
    #[case(0,  16)]
    #[case(1,  22)]
    #[case(2,  27)]
    #[case(3,  23)]
    #[case(4,  4)]
    #[case(5,  -16)]
    #[case(6,  23)]
    #[case(7,  267)]
    #[case(8,  1025)]
    #[case(9,  2943)]
    #[case(10, 7407)]
    #[case(11, 17445)]
    #[case(12, 39670)]
    #[case(13, 88339)]
    #[case(14, 193607)]
    #[case(15, 417868)]
    #[case(16, 887273)]
    #[case(17, 1851158)]
    #[case(18, 3792224)]
    #[case(19, 7628687)]
    #[case(20, 15083115)]
    fn test_function4_get(#[case] index: usize, #[case] expected: i128) {
        assert_eq!(TEST_FUNCTION_4.get(index), expected);
    }

}