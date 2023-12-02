use std::collections::HashMap;
use std::str::Lines;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day02.txt");
    let games = parse_lines(data.lines());
    println!("Part 1: {}", day02a(&games));
    println!("Part 2: {}", day02b(&games));
}

fn day02a(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .filter(|game| game.supports(12, 13, 14))
        .map(|game| game.index)
        .sum()
}

fn day02b(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .map(|x| x.power())
        .sum()
}

#[derive(Debug, PartialEq)]
struct Game {
    index: u32,
    sets: Vec<Set>
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Set {
    red: u8,
    green: u8,
    blue: u8
}

fn parse_lines(lines: Lines) -> Vec<Game> {
    lines.map(parse_line).collect()
}

fn parse_line(line: &str) -> Game {
    let split_line = line.split_once(":").unwrap();
    Game {
        index: parse_game_index(split_line.0),
        sets: parse_sets(split_line.1)
    }
}

static BALL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) (red|green|blue)").unwrap());

fn parse_game_index(game: &str) -> u32 {
    str::parse::<u32>(game.replace("Game ", "").as_str()).unwrap()
}

fn parse_sets(sets: &str) -> Vec<Set> {
    sets.split(";").map(|x| parse_set(x.trim())).collect()
}

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

impl Game {
    fn supports(&self, red: u8, green: u8, blue: u8) -> bool {
        self.sets.iter().all(|s| {
            s.red <= red && s.green <= green && s.blue <= blue
        })
    }

    fn power(&self) -> u32 {
        let red_max: u32 = self.sets.iter().map(|x| x.red).max().unwrap_or(0).into();
        let green_max: u32 = self.sets.iter().map(|x| x.green).max().unwrap_or(0).into();
        let blue_max: u32 = self.sets.iter().map(|x| x.blue).max().unwrap_or(0).into();

        red_max * green_max * blue_max
    }

}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use proptest::collection::{vec as prop_vec};
    use proptest::{prop_assert_eq, prop_compose, proptest};
    use crate::day02::{parse_set, parse_game_index, Set, Game, parse_sets, parse_line, parse_lines, day02a, day02b};

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

    proptest! {
        #[test]
        fn test_parse_game_index(input in 1..=100u32) {
            let a = format!("Game {}", input);
            prop_assert_eq!(parse_game_index(a.as_str()), input);
        }
    }

    proptest! {
        #[test]
        fn test_parse_sets(input in prop_vec(set_strategy(), 1..=5)) {
            let string_to_parse: String = input.iter().map(|x| x.get_string()).collect::<Vec<String>>().join("; ");
            let mut sets = parse_sets(string_to_parse.as_str());
            let mut sorted_input = input; // taking ownership here as we won't use the original after this
            // sorting is done on the vecs themselves, so return unit.
            sets.sort();
            sorted_input.sort();
            prop_assert_eq!(sets, sorted_input);
        }
    }

    proptest! {
        #[test]
        fn test_parse_game_line(sets in prop_vec(set_strategy(), 1..=5), index in 1..=100u32) {
            let string_to_parse: String = format!("Game {index}: {}", sets.iter().map(|x| x.get_string()).collect::<Vec<String>>().join("; "));
            let game = parse_line(string_to_parse.as_str());
            prop_assert_eq!(Game { index, sets }, game);
        }
    }

    proptest! {
        #[test]
        fn test_game_supports(sets in prop_vec(set_strategy(), 1..=5), red_max in 0..=50u8, green_max in 0..=50u8, blue_max in 0..=50u8) {
            let game = Game { index: 1, sets }; // sets is moved here.
            let expected = game.sets.iter().map(|x| x.red).max().unwrap_or(0u8) <= red_max
                && game.sets.iter().map(|x| x.green).max().unwrap_or(0u8) <= green_max
                && game.sets.iter().map(|x| x.blue).max().unwrap_or(0u8) <= blue_max;
            assert_eq!(game.supports(red_max, green_max, blue_max), expected);
        }
    }

    #[test]
    fn test_game_supports_true() {
        let game = Game { index: 1, sets: vec![Set { red: 1, green: 2, blue: 3}] };
        assert_eq!(game.supports(2, 3, 4), true);
    }

    #[test]
    fn test_game_supports_false() {
        let game = Game { index: 1, sets: vec![Set { red: 10, green: 20, blue: 30}] };
        assert_eq!(game.supports(2, 3, 4), false);
    }

    proptest! {
        #[test]
        fn test_game_power(sets in prop_vec(set_strategy(), 1..=5)) {
            let game = Game { index: 1, sets }; // sets is moved here.
            let red: u32 = game.sets.iter().map(|x| x.red).max().unwrap_or(0u8).into();
            let green: u32 = game.sets.iter().map(|x| x.green).max().unwrap_or(0u8).into();
            let blue: u32 = game.sets.iter().map(|x| x.blue).max().unwrap_or(0u8).into();
            assert_eq!(game.power(), red * green * blue);
        }
    }

    static EXAMPLE_DATA: Lazy<Vec<Game>> = Lazy::new(|| vec![
        Game { index: 1, sets: vec![Set { red: 4, blue: 3, green: 0}, Set { red: 1, blue: 6, green: 2}, Set { red: 0, blue: 0, green: 2}]},
        Game { index: 2, sets: vec![Set { red: 0, blue: 1, green: 2}, Set { red: 1, blue: 4, green: 3}, Set { red: 0, blue: 1, green: 1}]},
        Game { index: 3, sets: vec![Set { red: 20, blue: 6, green: 8}, Set { red: 4, blue: 5, green: 13}, Set { red: 1, blue: 0, green: 5}]},
        Game { index: 4, sets: vec![Set { red: 3, blue: 6, green: 1}, Set { red: 6, blue: 0, green: 3}, Set { red: 14, blue: 15, green: 3}]},
        Game { index: 5, sets: vec![Set { red: 6, blue: 1, green: 3}, Set { red: 1, blue: 2, green: 2}]}
    ]);

    #[test]
    fn test_parsing_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(parse_lines(input.lines()), *Lazy::force(&EXAMPLE_DATA))
    }

    #[test]
    fn test_part_a() {
        assert_eq!(day02a(&EXAMPLE_DATA), 8);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(day02b(&EXAMPLE_DATA), 2286);
    }
}
