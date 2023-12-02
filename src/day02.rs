use std::collections::HashMap;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::common::load_from;

pub fn day02() {
    let data = load_from("day02.txt");
    println!("{}", day02a(data.as_str()));
    println!("{}", day02b(data.as_str()));
}

fn day02a(input: &str) -> String {
    String::from("no")
}

fn day02b(input: &str) -> String {
    String::from("no")
}

#[derive(Debug, PartialEq)]
struct Game {
    index: u8,
    sets: Vec<Set>
}

#[derive(Debug, PartialEq)]
struct Set {
    red: u8,
    green: u8,
    blue: u8
}

fn parse_line(line: &str) -> Game {
    // Line looks like
    // Game n: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game {
        index: 1,
        sets: vec![]
    }
}

static BALL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) (red|green|blue)").unwrap());

fn parse_set(set: &str) -> Set {
    let split_set_string = set.split(",").map(|s| s.trim());
    let mut map: HashMap<String, u8> = HashMap::new();

    for entry in split_set_string {
        let captures = BALL_REGEX.captures(entry).unwrap();
        // Gets the groups, assumes they are there, adds the colour to the map and the number.
        map.insert(captures.get(2).unwrap().as_str().to_string(), str::parse::<u8>(captures.get(1).unwrap().as_str()).unwrap());
    }

    Set {
        red: map.get("red").map(|x| x.clone()).unwrap_or(0),
        green: map.get("green").map(|x| x.clone()).unwrap_or(0),
        blue: map.get("blue").map(|x| x.clone()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use proptest::{prop_assert_eq, prop_compose, proptest};
    use crate::day02::{parse_set, Set};

    impl Set {
        fn get_string(&self) -> String {
            let mut output: Vec<String> = Vec::new();
            if self.red > 0 {
                output.push(format!("{} red", self.red));
            }
            if self.green > 0 {
                output.push(format!("{} green", self.green));
            }
            if self.blue > 0 {
                output.push(format!("{} blue", self.blue));
            }

            output.join(", ")
        }
    }

    prop_compose! {
        fn set_strategy()(red in 0..50u8, green in 0..50u8, blue in 0..50u8) -> Set {
            Set {
                red,
                green,
                blue
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_set(input in set_strategy()) {
            let string_to_parse = input.get_string();
            prop_assert_eq!(parse_set(string_to_parse.as_str()), input);
        }
    }

}
