use crate::common::load_from;

pub fn run_day() {
    let string = load_from("day04.txt");
    let cards = parse_lines(string.as_str());
    println!("Part 1: {}", day04a(&cards));
    println!("Part 2: {}", day04b(&cards));
}

fn parse_lines(string: &str) -> Vec<Card> {
    string.lines().map(|x| parse_line(x)).collect()
}

fn parse_line(line: &str) -> Card {
    let (card, numbers) = line.split_once(":").unwrap();
    let (wins, potentials) = numbers.split_once("|").unwrap();
    let card_no_string = card.split_whitespace().last().unwrap();
    Card {
        index: str::parse::<u32>(card_no_string).unwrap(),
        winning_numbers: wins.trim().split_whitespace().map(|x| str::parse::<u32>(x).unwrap()).collect(),
        card_numbers: potentials.trim().split_whitespace().map(|x| str::parse::<u32>(x).unwrap()).collect()
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    index: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>
}

impl Card {
    fn wins(&self) -> u32 {
        self.card_numbers.iter().filter(|number| self.winning_numbers.contains(*number)).collect::<Vec<_>>().len() as u32
    }

    fn points(&self) -> u32 {
        let wins = self.wins();
        if wins == 0 {
            0u32
        } else {
            2u32.pow(wins - 1)
        }
    }
}

fn day04a(input: &Vec<Card>) -> u32 {
    input.iter().map(|x| x.points()).sum()
}

fn day04b(input: &Vec<Card>) -> u32 {
    get_card_distribution(input).iter().sum()
}

fn get_card_distribution(input: &Vec<Card>) -> Vec<u32> {
    let mut card_numbers: Vec<u32> = vec![1u32; input.len()];
    for (idx, card) in input.iter().enumerate() {
        let no_of_cards = card_numbers[idx];
        let wins = card.wins();
        if wins > 0 {
            for m in 1..=wins {
                let x = card_numbers[idx + (m as usize)];
                card_numbers[idx + (m as usize)] = x + no_of_cards;
            }
        }
    }
    card_numbers
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use proptest::proptest;
    use proptest::collection::{hash_set, vec as prop_vec};
    use crate::day04::{Card, day04a, day04b, get_card_distribution, parse_line, parse_lines};

    const TEST_DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                             Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                             Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                             Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                             Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                             Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_day_04a() {
        let cards = parse_lines(TEST_DATA);
        assert_eq!(day04a(&cards), 13);
    }

    #[test]
    fn test_day_04b() {
        let cards = parse_lines(TEST_DATA);
        assert_eq!(day04b(&cards), 30);
    }

    #[test]
    fn test_get_card_distribution() {
        let cards = parse_lines(TEST_DATA);
        let expected: Vec<u32> = vec![
            1,
            2,
            4,
            8,
            14,
            1
        ];
        assert_eq!(get_card_distribution(&cards), expected);
    }

    #[test]
    fn get_cards_from_test_data() {
        let expected = vec![
            Card {
                index: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                card_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            },
            Card {
                index: 2,
                winning_numbers: vec![13, 32, 20, 16, 61],
                card_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19]
            },
            Card {
                index: 3,
                winning_numbers: vec![ 1, 21, 53, 59, 44],
                card_numbers: vec![69, 82, 63, 72, 16, 21, 14,  1]
            },
            Card {
                index: 4,
                winning_numbers: vec![41, 92, 73, 84, 69],
                card_numbers: vec![59, 84, 76, 51, 58,  5, 54, 83]
            },
            Card {
                index: 5,
                winning_numbers: vec![87, 83, 26, 28, 32],
                card_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36]
            },
            Card {
                index: 6,
                winning_numbers: vec![31, 18, 13, 56, 72],
                card_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11]
            }
        ];
        assert_eq!(parse_lines(TEST_DATA), expected);
    }

    fn join_numbers(numbers: &Vec<u32>) -> String {
        numbers.iter().map(|x| format!("{:>2}", x)).collect::<Vec<String>>().join(" ")
    }

    proptest! {
        #[test]
        fn test_parse_line(card in 1..256u32, winning in prop_vec(1..=99u32, 5), cards in prop_vec(1..=99u32, 8)) {
            let line = format!("Card {}: {} | {}", card, join_numbers(&winning), join_numbers(&cards));
            let expected = Card {
                index: card,
                winning_numbers: winning,
                card_numbers: cards
            };
            assert_eq!(parse_line(line.as_str()), expected);
        }
    }

    proptest! {
        #[test]
        fn test_card_points(card in 1..256u32, winning in hash_set(1..=99u32, 5), cards in hash_set(1..=99u32, 8)) {
            let card = Card {
                index: card,
                winning_numbers: winning.iter().map(|x| x.clone()).collect(),
                card_numbers: cards.iter().map(|x| x.clone()).collect()
            };

            let wins: HashSet<_> = winning.intersection(&cards).collect();
            let result = match wins.len() {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 4,
                4 => 8,
                5 => 16,
                x @ _ => panic!("There should be no more than 5 matches, but {} were found", x)
            };
            assert_eq!(card.points(), result);
        }
    }

    proptest! {
        #[test]
        fn test_card_wins(card in 1..256u32, winning in hash_set(1..=99u32, 5), cards in hash_set(1..=99u32, 8)) {
            let card = Card {
                index: card,
                winning_numbers: winning.iter().map(|x| x.clone()).collect(),
                card_numbers: cards.iter().map(|x| x.clone()).collect()
            };

            let wins: HashSet<_> = winning.intersection(&cards).collect();
            assert_eq!(card.wins(), wins.len() as u32);
        }
    }

}