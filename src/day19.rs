use std::cmp::max;
use std::collections::HashMap;
use regex::Regex;
use tailcall::tailcall;
use crate::common::load_from;
use crate::day19::Result::*;
use crate::day19::Check::*;
use crate::day19::Category::*;

pub fn run_day() {
    let data = load_from("day19.txt");
    let (rules, parts) = parse_data(data.as_str());
    println!("Part 1: {}", day19a(&parts, &rules));
    println!("Part 2: {}", day19b(&rules));
}

fn day19a(parts: &Vec<Part>, rules: &HashMap<String, Vec<Check>>) -> u64 {
    parts.iter().filter(|part| run_workflow(part, rules, "in")).map(|x| x.sum()).sum()
}

fn day19b(rules: &HashMap<String, Vec<Check>>) -> u64 {
    run_ranges(vec![(String::from("in"), PartRange::init())], rules, 0)
}

#[tailcall]
fn run_workflow(part: &Part, rules: &HashMap<String, Vec<Check>>, current_rule: &str) -> bool {
    let rule = rules.get(current_rule).unwrap();

    let result = rule.iter().filter_map(|check| {
        match check {
            LessThan(category, value, result) => {
                if part.get(&category) < *value {
                    Some(result)
                } else {
                    None
                }
            }
            GreaterThan(category, value, result) => {
                if part.get(&category) > *value {
                    Some(result)
                } else {
                    None
                }
            }
            Always(result) => Some(result)
        }
    }).next().unwrap();

    if let Workflow(wf) = result {
        run_workflow(part, rules, wf.as_str())
    } else if *result == Accept {
        true
    } else {
        false
    }

}

#[tailcall]
fn run_ranges(part_range: Vec<(String, PartRange)>, rules: &HashMap<String, Vec<Check>>, acc: u64) -> u64 {
    let mut next_ranges: Vec<(String, PartRange)> = Vec::new();
    let mut add: u64 = acc;
    for (next_workflow, range) in part_range {
        let checks = rules.get(next_workflow.as_str()).unwrap();
        let mut left = range.clone();
        for check in checks {
            let (new_workflow, cont) = left.split(check);
            if let Some(result) = new_workflow {
                // send this down the next step
                match check.result() {
                    Workflow(flow) => {
                        next_ranges.push((flow.clone(), result))
                    },
                    Accept => add += result.total(),
                    Reject => ()
                }
            }

            if let Some(next) = cont {
                left = next
            } else {
                // nothing more to do!
                break;
            }
        }
    }

    if next_ranges.is_empty() {
        add
    } else {
        run_ranges(next_ranges, rules, add)
    }
}

fn parse_data(data: &str) -> (HashMap<String, Vec<Check>>, Vec<Part>) {
    let rule_regex = Regex::new(r"([a-z]+)\{(.+)}").unwrap();

    let mut lines = data.lines();
    let mut rules: HashMap<String, Vec<Check>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let captures = rule_regex.captures(line).unwrap();
        let key = String::from(captures.get(1).unwrap().as_str());
        let rule_string = captures.get(2).unwrap().as_str();
        let checks: Vec<Check> = rule_string.split(",").map(|x| Check::parse(x)).collect();
        rules.insert(key, checks);
    }

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        parts.push(Part::parse(line));
    }

    (rules, parts)
}

#[derive(PartialEq, Debug)]
enum Category {
    X,
    M,
    A,
    S
}

impl Category {
    fn from(string: &str) -> Category {
        match string {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            x => panic!("{} is not a category", x)
        }
    }
}

#[derive(PartialEq, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64
}

impl Part {

    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }

    fn get(&self, category: &Category) -> u64 {
        match category {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.s
        }
    }

    fn parse(string: &str) -> Part {
        let mut result = Part { x: 0, m: 0, a: 0, s: 0 };
        let parts = string[1..(string.len()-1)].split(",");
        for part in parts {
            let (key, string_value) = part.split_once("=").unwrap();
            let value = string_value.parse::<u64>().unwrap();
            match key {
               "x" => result.x = value,
               "m" => result.m = value,
               "a" => result.a = value,
               "s" => result.s = value,
                l => panic!("{} is not a attribute", l)
            };
        }

        result
    }
}

#[derive(PartialEq, Debug)]
enum Check {
    LessThan(Category, u64, Result),
    GreaterThan(Category, u64, Result),
    Always(Result)
}

impl Check {
    fn result(&self) -> Result {
        match self {
            LessThan(_, _, r) => r,
            GreaterThan(_, _, r) => r,
            Always(r) => r
        }.clone()
    }

    fn parse(rule: &str) -> Check {
        if rule.contains(":") {
            let (first, action) = rule.split_once(":").unwrap();
            let category = Category::from(&first[0..1]);
            let bound = first[2..].parse::<u64>().unwrap();
            if &first[1..2] == ">" {
                GreaterThan(category, bound, Result::parse(action))
            } else {
                LessThan(category, bound, Result::parse(action))
            }
        } else {
            Always(Result::parse(rule))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Result {
    Workflow(String),
    Accept,
    Reject
}

impl Result {
    fn parse(string: &str) -> Result {
        match string {
            "A" => Accept,
            "R" => Reject,
            x => Workflow(String::from(x))
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct SimpleRange {
    min: u64,
    max: u64
}

impl SimpleRange {
    fn init() -> SimpleRange {
        SimpleRange { min: 1, max: 4000 }
    }

    fn range(&self) -> u64 {
        max(0, self.max + 1 - self.min)
    }

    fn split_less_than(&self, amt: u64) -> (Option<SimpleRange>, Option<SimpleRange>) {
        if self.min >= amt {
            (None, Some(self.clone()))
        } else if self.max < amt {
            (Some(self.clone()), None)
        } else {
            (Some(SimpleRange { min: self.min, max: amt - 1}), Some(SimpleRange { min: amt, max: self.max }))
        }
    }

    fn split_greater_than(&self, amt: u64) -> (Option<SimpleRange>, Option<SimpleRange>) {
        if self.min > amt {
            (Some(self.clone()), None)
        } else if self.max <= amt {
            (None, Some(self.clone()))
        } else {
            (Some(SimpleRange { min: amt + 1, max: self.max}), Some(SimpleRange { min: self.min, max: amt }))
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct PartRange {
    x: SimpleRange,
    m: SimpleRange,
    a: SimpleRange,
    s: SimpleRange
}

impl PartRange {

    fn init() -> PartRange {
        PartRange {
            x: SimpleRange::init(),
            m: SimpleRange::init(),
            a: SimpleRange::init(),
            s: SimpleRange::init()
        }
    }

    fn total(&self) -> u64 {
        self.x.range() * self.m.range() * self.s.range() * self.a.range()
    }

    fn split(&self, check: &Check) -> (Option<PartRange>, Option<PartRange>) {
        match check {
            LessThan(X, amt, _) => {
                let (first, second) = self.x.split_less_than(*amt);
                (
                    first.map(|r| PartRange { x: r, m: self.m.clone(), a: self.a.clone(), s: self.s.clone() }),
                    second.map(|r| PartRange { x: r, m: self.m.clone(), a: self.a.clone(), s: self.s.clone() })
                )
            },
            LessThan(M, amt, _) => {
                let (first, second) = self.m.split_less_than(*amt);
                (
                    first.map(|r| PartRange { x: self.x.clone(), m: r, a: self.a.clone(), s: self.s.clone() }),
                    second.map(|r| PartRange { x: self.x.clone(), m: r, a: self.a.clone(), s: self.s.clone() })
                )
            },
            LessThan(A, amt, _) => {
                let (first, second) = self.a.split_less_than(*amt);
                (
                    first.map(|r| PartRange { x: self.x.clone(), m: self.m.clone(), a: r, s: self.s.clone() }),
                    second.map(|r| PartRange { x: self.x.clone(), m: self.m.clone(), a: r, s: self.s.clone() })
                )
            },
            LessThan(S, amt, _) => {
                let (first, second) = self.s.split_less_than(*amt);
                (
                    first.map(|r| PartRange { x: self.x.clone(), m: self.m.clone(), a: self.a.clone(), s: r }),
                    second.map(|r| PartRange { x: self.x.clone(), m: self.m.clone(), a: self.a.clone(), s: r })
                )
            },
            GreaterThan(X, amt, _) => {
                let (first, second) = self.x.split_greater_than(*amt);
                (
                    first.map(|r| PartRange { x: r, m: self.m.clone(), a: self.a.clone(), s: self.s.clone() }),
                    second.map(|r| PartRange { x: r, m: self.m.clone(), a: self.a.clone(), s: self.s.clone() })
                )
            },
            GreaterThan(M, amt, _) => {
                let (first, second) = self.m.split_greater_than(*amt);
                (
                    first.map(|r| PartRange { x: self.x.clone(), m: r, a: self.a.clone(), s: self.s.clone() }),
                    second.map(|r| PartRange { x: self.x.clone(), m: r, a: self.a.clone(), s: self.s.clone() })
                )
            },
            GreaterThan(A, amt, _) => {
                let (first, second) = self.a.split_greater_than(*amt);
                (
                    first.map(|r| PartRange { x: self.x.clone(), m: self.m.clone(), a: r, s: self.s.clone() }),
                    second.map(|r| PartRange { x: self.x.clone(), m: self.m.clone(), a: r, s: self.s.clone() })
                )
            },
            GreaterThan(S, amt, _) => {
                let (first, second) = self.s.split_greater_than(*amt);
                (
                    first.map(|r| PartRange { x: self.x.clone(), m: self.m.clone(), a: self.a.clone(), s: r }),
                    second.map(|r| PartRange { x: self.x.clone(), m: self.m.clone(), a: self.a.clone(), s: r })
                )
            },
            Always(_) => (Some(self.clone()), None)
        }
    }

}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::ops::Deref;
    use indoc::indoc;
    use proptest::proptest;
    use rstest::rstest;
    use structopt::lazy_static::lazy_static;
    use crate::day19::{Part, Check, parse_data, day19a, day19b};
    use crate::day19::Check::*;
    use crate::day19::Category::*;
    use crate::day19::Result::*;

    const TEST_DATA: &str = indoc! {
        "px{a<2006:qkq,m>2090:A,rfg}
         pv{a>1716:R,A}
         lnx{m>1548:A,A}
         rfg{s<537:gd,x>2440:R,A}
         qs{s>3448:A,lnx}
         qkq{x<1416:A,crn}
         crn{x>2662:A,R}
         in{s<1351:px,qqz}
         qqz{s>2770:qs,m<1801:hdj,R}
         gd{a>3333:R,R}
         hdj{m>838:A,pv}

         {x=787,m=2655,a=1222,s=2876}
         {x=1679,m=44,a=2067,s=496}
         {x=2036,m=264,a=79,s=2244}
         {x=2461,m=1339,a=466,s=291}
         {x=2127,m=1623,a=2188,s=1013}"
    };

    lazy_static! {
        static ref PARSED_CHECK_SETS: HashMap<String, Vec<Check>> = HashMap::from([
            (String::from("px"), vec![LessThan(A, 2006, Workflow(String::from("qkq"))), GreaterThan(M, 2090, Accept), Always(Workflow(String::from("rfg")))]),
            (String::from("pv"), vec![GreaterThan(A, 1716, Reject), Always(Accept)]),
            (String::from("lnx"), vec![GreaterThan(M, 1548, Accept), Always(Accept)]),
            (String::from("rfg"), vec![LessThan(S, 537, Workflow(String::from("gd"))), GreaterThan(X, 2440, Reject), Always(Accept)]),
            (String::from("qs"), vec![GreaterThan(S, 3448, Accept), Always(Workflow(String::from("lnx")))]),
            (String::from("qkq"), vec![LessThan(X, 1416, Accept), Always(Workflow(String::from("crn")))]),
            (String::from("crn"), vec![GreaterThan(X, 2662, Accept), Always(Reject)]),
            (String::from("in"), vec![LessThan(S, 1351, Workflow(String::from("px"))), Always(Workflow(String::from("qqz")))]),
            (String::from("qqz"), vec![GreaterThan(S, 2770, Workflow(String::from("qs"))), LessThan(M, 1801, Workflow(String::from("hdj"))), Always(Reject)]),
            (String::from("gd"), vec![GreaterThan(A, 3333, Reject), Always(Reject)]),
            (String::from("hdj"), vec![GreaterThan(M, 838, Accept), Always(Workflow(String::from("pv")))]),
        ]);
    }

    lazy_static! {
        static ref PARSED_PARTS: Vec<Part> = vec![
            Part { x: 787, m: 2655, a:1222, s: 2876},
            Part { x: 1679, m: 44, a:2067, s: 496},
            Part { x: 2036, m: 264, a:79, s: 2244},
            Part { x: 2461, m: 1339, a:466, s: 291},
            Part { x: 2127, m: 1623, a:2188, s: 1013}
        ];
    }

    #[test]
    fn test_day19a() {
        assert_eq!(day19a(PARSED_PARTS.deref(), PARSED_CHECK_SETS.deref()), 19114);
    }

    #[test]
    fn test_day19b() {
        assert_eq!(day19b(PARSED_CHECK_SETS.deref()), 167409079868000);
    }

    #[test]
    fn test_parsing() {
        let (checks, parts) = parse_data(TEST_DATA);
        assert_eq!(checks, *PARSED_CHECK_SETS.deref());
        assert_eq!(parts, *PARSED_PARTS.deref());
    }

    #[rstest]
    #[case("a<2006:qkq", LessThan(A, 2006, Workflow(String::from("qkq"))))]
    #[case("m>2090:A", GreaterThan(M, 2090, Accept))]
    #[case("a>1716:R", GreaterThan(A, 1716, Reject))]
    #[case("A", Always(Accept))]
    #[case("s<537:gd", LessThan(S, 537, Workflow(String::from("gd"))))]
    #[case("lnx", Always(Workflow(String::from("lnx"))))]
    #[case("x<1416:A", LessThan(X, 1416, Accept))]
    #[case("R", Always(Reject))]
    #[case("s<1351:px", LessThan(S, 1351, Workflow(String::from("px"))))]
    #[case("s>2770:qs", GreaterThan(S, 2770, Workflow(String::from("qs"))))]
    #[case("a>3333:R", GreaterThan(A, 3333, Reject))]
    fn test_check_parse(#[case] input: &str, #[case] expected: Check) {
        assert_eq!(Check::parse(input), expected);
    }

    proptest! {
        #[test]
        fn test_parts_parse(x in 0..=9999u64, m in 0..=9999u64, a in 0..=9999u64, s in 0..=9999u64) {
            // create string
            let input = format!("{{x={x},m={m},a={a},s={s}}}");
            assert_eq!(Part::parse(input.as_str()), Part { x, m, a, s })
        }
    }

}