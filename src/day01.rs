use std::collections::HashMap;
use once_cell;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::common::load_from;

pub fn run_day() {
    let data = load_from("day01.txt");
    println!("Part 1 answer: {}", day01a(data.as_str()));
    println!("Part 2 answer: {}", day01b(data.as_str()));
}

fn day01a(data: &str) -> i32 {
    let split_data = split_data_lines(data);
    parse_lines(split_data)
}

fn day01b(data: &str) -> i32 {
    let replaced = number_replacement(data);
    parse_lines(replaced)
}

fn parse_lines(data: Vec<String>) -> i32 {
    let digits = map_string_to_int(data);
    digits.iter().sum::<i32>()
}

fn split_data_lines(string: &str) -> Vec<String> {
    string.lines().map(|s| s.trim().to_string()).collect()
}

fn map_string_to_int(strings: Vec<String>) -> Vec<i32> {
    strings.iter().map(|s| digit_scraper(s.as_str())).collect()
}

fn digit_scraper(string: &str) -> i32 {
    let re = Regex::new(r"\d").unwrap();

    let mut matches = re.find_iter(string);
    match matches.next() {
        Some(first_match) => {
            // We assume it's always there, we'll panic if not
            let result: String = format!("{}{}", first_match.as_str(), matches.last().unwrap_or(first_match).as_str());
            str::parse::<i32>(result.as_str()).unwrap()
        },
        None => panic!("No digits in the line {}", string)
    }

}

static NUMBERS: Lazy<HashMap<&str, &str>> = Lazy::new(|| HashMap::from([
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9")
]));

/*
 * This function takes a string, finds the first number word in there and replace it. It then
 * looks at the original string, and finds the last wordy number, IF there are no digits in the
 * string after the last wordy number. Look at the tests to see what it actually does, but quick
 * examples:
 *
 * * onetwothree becomes 1twothree3
 * * one23 becomes 123
 * * onetwo3 becomes 1two3
 * * one2three becomes 12three3
 *
 * This is necessary because of the overlap of some numbers, like so -- only a problem for
 * the last entry so that's why the number is just tacked onto the end if no other numbers are
 * at the end:
 *
 * * onetwothreeight -> 1twothreeight8
 * * onetwothreeight9 -> 1twothreeight9
 * * oneight -> 1ight8
 */
fn number_replacement(str: &str) -> Vec<String> {
    let split = split_data_lines(str);
    let regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    // regex is greedy, so this will look for the LAST entry IF it doesn't end in a number
    let last_regex = Regex::new(r"^[a-z0-9]*(one|two|three|four|five|six|seven|eight|nine)[^\d]*$").unwrap();

    let mut result_vec: Vec<String> = vec![];
    for line in split {
        // find the first entry and replace it:
        let mut first_replacement = line.clone();
        match regex.find(first_replacement.as_str()) {
            None => (),
            Some(m) => {
                first_replacement.replace_range(m.range(), NUMBERS.get(m.as_str()).unwrap());
                // now we look for the last match, if we can find any more
                // We find it on the original line, and don't do any changes if we end in a digit
                first_replacement = match last_regex.captures(line.as_str()) {
                    None => first_replacement,
                    Some(captures) => format!("{}{}", first_replacement, NUMBERS.get(captures.get(1).unwrap().as_str()).unwrap())
                };
            }
        }
        result_vec.push(first_replacement)
    }

    result_vec
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::day01::{day01a, day01b, digit_scraper, map_string_to_int, number_replacement, parse_lines, split_data_lines};

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test_digit_scraper(#[case] input_string: &str, #[case] expected: i32) {
        let result: i32 = digit_scraper(input_string);
        assert_eq!(expected, result, "Expected {}, but got {} instead", expected, result);
    }

    #[test]
    fn test_map_string_to_list() {
        let input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string()
        ];
        let output = vec![12, 38, 15, 77];
        let result = map_string_to_int(input);
        assert_eq!(output, result)
    }

    #[test]
    fn test_split_data_lines() {
        let input = "1abc2
                        pqr3stu8vwx
                        a1b2c3d4e5f
                        treb7uchet";
        let expected = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string()
        ];
        assert_eq!(expected, split_data_lines(input))
    }

    #[rstest]
    #[case("one1", "11")]
    #[case("2two1", "221")]
    #[case("three", "33")]
    #[case("four4", "44")]
    #[case("5five", "555")]
    #[case("6six6", "666")]
    #[case("6seven", "677")]
    #[case("7eight9", "789")]
    #[case("zoneight234", "z1ight234")]
    #[case("4nineeightseven2", "49eightseven2")]
    #[case("4nineightseven2", "49ightseven2")]
    #[case("manynines", "many9s9")]
    #[case("31oneight", "311ight8")]
    #[case("31fiveoneight", "315oneight8")]
    #[case("oneitthreeandnineplus3three", "1itthreeandnineplus3three3")]
    fn test_number_replacement(#[case] input: String, #[case] output: String) {
        assert_eq!(vec![output], number_replacement(input.as_str()))
    }

    #[test]
    fn test_number_replacement_with_multiple_lines() {
        let input = vec![
            "one1",
            "2two1",
            "three",
            "four4",
            "5five",
            "6six6",
            "6seven",
            "7eight9",
            "zoneight234",
            "4nineeightseven2",
            "4nineightseven2",
            "manynines",
            "oneitthreeandnineplus3three"
        ].join("\n");
        let output: Vec<String> = vec![
            "11",
            "221",
            "33", // three is converted, and is also the last entry with no digit at the end
            "44",
            "555", // "five" is seen as the first and last number, so is appended as well as replaced in line
            "666", // 6 is a digit after, so the engine leaves it alone
            "677", // as above with seven
            "789", // 9 is a digit after, so the engine leaves it alone
            "z1ight234",
            "49eightseven2",
            "49ightseven2",
            "many9s9", // as above with nine
            "1itthreeandnineplus3three3" // 3 is appended
        ].iter().map(|x| x.to_string()).collect();
        assert_eq!(output, number_replacement(input.as_str()))
    }

    #[test]
    fn test_parse_lines_case_1() {
        let input: Vec<String> = vec![
            "one1",
            "2two1",
            "3",
            "44",
            "55",
            "666",
            "67",
            "789",
            "z1ight234",
            "49eight72",
            "49ight72",
            "many9s",
            "1itthreeandnineplus33"
        ].iter().map(|x| x.to_string()).collect();
        assert_eq!(586, parse_lines(input))
    }

    #[test]
    fn test_parse_lines_case_2() {
        let input = "1abc2
                        pqr3stu8vwx
                        a1b2c3d4e5f
                        treb7uchet".lines().map(|x| x.to_string()).collect();
        assert_eq!(142, parse_lines(input))
    }

    #[test]
    fn test_day01a() {
        let input = "1abc2
                        pqr3stu8vwx
                        a1b2c3d4e5f
                        treb7uchet";
        assert_eq!(142, day01a(input))
    }

    #[test]
    fn test_day01b() {
        let input = "two1nine
                    eightwothree
                    abcone2threexyz
                    xtwone3four
                    4nineeightseven2
                    zoneight234
                    7pqrstsixteen";
        assert_eq!(281, day01b(input))
    }
}