use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use crate::common::load_from;
use crate::day07::Card::*;

pub fn run_day() {
    let data = load_from("day07.txt");
    let mut hands = parse_all_hands(data.as_str());
    rank_hands(&mut hands);
    println!("Part 1: {}", day07a(&hands));
    println!("Part 2: {}", day07b(&hands));
}

fn day07a(data: &Vec<Hand>) -> u64 {
    data.iter().enumerate().map(|(idx, hand)| (idx as u64 + 1) * hand.bid).sum()
}

fn day07b(data: &Vec<Hand>) -> u64 {
    let mut new_hands: Vec<Hand> = data.iter().map(|x| {
        let cards: Vec<Card> =  x.cards.iter().map(|x| {
            if *x == Jack {
                Joker
            } else {
                x.clone()
            }
        }).collect();
        let bid = x.bid.clone();
        create_hand_from_cards_and_bid(cards, bid)
    }).collect();
    rank_hands(&mut new_hands);
    new_hands.iter().enumerate().map(|(idx, hand)| (idx as u64 + 1) * hand.bid).sum()
}

fn rank_hands(hands: &mut Vec<Hand>) {
    hands.sort()
}

fn parse_all_hands(lines: &str) -> Vec<Hand> {
    lines.lines().map(parse_hand).collect()
}

fn parse_hand(line: &str) -> Hand {
    let inputs: Vec<&str> = line.split_whitespace().collect();
    let cards: Vec<Card> = inputs[0].chars().map(|c| string_to_card(&c)).collect();
    let bid: u64 = str::parse::<u64>(inputs[1]).unwrap();
    create_hand_from_cards_and_bid(cards, bid)
}

fn create_hand_from_cards_and_bid(cards: Vec<Card>, bid: u64) -> Hand {
    let hand_type: HandType = determine_hand(&cards);
    Hand {
        cards,
        hand_type,
        bid
    }
}

fn determine_hand(cards: &Vec<Card>) -> HandType {
    let mut x = HashMap::<&Card, u8, RandomState>::new();
    for c in cards {
        if !x.contains_key(c) {
            x.insert(c, 0u8);
        }

        *x.get_mut(c).unwrap() += 1u8;
    }

    // We can now remove the joker from the pack, if it's there, and get its value
    let val_joker_inc: u8 = x.remove(&Joker).map(|x| x.clone()).unwrap_or(0);

    // This gets the pairs etc
    let mut results: Vec<u8> = x.values().map(|x| x.clone()).collect();
    results.sort();
    results.reverse();
    // We want to add the jokers to the highest number, as that will always give the best result:
    //
    // For one joker, for example:
    //
    // HighCard -> OnePair
    // TwoPair -> FullHouse
    // ThreeOfAKind -> FourOfAKind
    //
    // There is an edge case if you have all jokers, that's just your five of a kind, folks!
    if results.is_empty() {
        results.push(val_joker_inc);
    } else {
        results[0] += val_joker_inc;
    }

    // numbers are sorted ascending, but we actually want to know what the biggest was.
    let first_entry = results.first().unwrap();
    match *first_entry {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if results.len() == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        },
        2 => {
            if results.len() == 3 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        },
        1 => HandType::HighCard,
        _ => panic!("Should not happen")
    }
}

fn string_to_card(c: &char) -> Card {
    match c {
        '2' => Two,
        '3' => Three,
        '4' => Four,
        '5' => Five,
        '6' => Six,
        '7' => Seven,
        '8' => Eight,
        '9' => Nine,
        'T' => Ten,
        'J' => Jack,
        'Q' => Queen,
        'K' => King,
        'A' => Ace,
        _   => panic!("This should not happen")
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u64
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type).then(self.cards.cmp(&other.cards))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
#[repr(u8)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Clone)]
#[repr(u8)]
enum Card {
    Joker = 1,
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::ops::Deref;
    use once_cell::sync::Lazy;
    use rand::seq::SliceRandom;
    use rstest::rstest;
    use crate::day07::{Card, day07a, day07b, determine_hand, Hand, HandType, parse_all_hands, rank_hands};
    use crate::day07::Card::*;

    const TEST_DATA: &str = "32T3K 765\n\
                             T55J5 684\n\
                             KK677 28\n\
                             KTJJT 220\n\
                             QQQJA 483";

    static PARSED_DATA: Lazy<Vec<Hand>> = Lazy::new(|| {
        vec![
            Hand {
                cards: vec![Three, Two, Ten, Three, King],
                hand_type: HandType::OnePair,
                bid: 765
            },
            Hand {
                cards: vec![Ten, Five, Five, Jack, Five],
                hand_type: HandType::ThreeOfAKind,
                bid: 684
            },
            Hand {
                cards: vec![King, King, Six, Seven, Seven],
                hand_type: HandType::TwoPair,
                bid: 28
            },
            Hand {
                cards: vec![King, Ten, Jack, Jack, Ten],
                hand_type: HandType::TwoPair,
                bid: 220
            },
            Hand {
                cards: vec![Queen, Queen, Queen, Jack, Ace],
                hand_type: HandType::ThreeOfAKind,
                bid: 483
            }
        ]
    });

    static ORDERED_DATA: Lazy<Vec<Hand>> =
    Lazy::new(|| vec![
        Hand {
            cards: vec![Three, Two, Ten, Three, King],
            hand_type: HandType::OnePair,
            bid: 765
        },
        Hand {
            cards: vec![King, Ten, Jack, Jack, Ten],
            hand_type: HandType::TwoPair,
            bid: 220
        },
        Hand {
            cards: vec![King, King, Six, Seven, Seven],
            hand_type: HandType::TwoPair,
            bid: 28
        },
        Hand {
            cards: vec![Ten, Five, Five, Jack, Five],
            hand_type: HandType::ThreeOfAKind,
            bid: 684
        },
        Hand {
            cards: vec![Queen, Queen, Queen, Jack, Ace],
            hand_type: HandType::ThreeOfAKind,
            bid: 483
        }
    ]);

    #[test]
    fn test_day07a() {
        assert_eq!(day07a(ORDERED_DATA.deref()), 6440);
    }

    #[test]
    fn test_day07b() {
        assert_eq!(day07b(ORDERED_DATA.deref()), 5905);
    }

    #[test]
    fn test_parse_all_hands() {
        assert_eq!(parse_all_hands(TEST_DATA), *PARSED_DATA.deref())
    }

    #[test]
    fn test_ordering() {
        let mut data: Vec<Hand> = (*PARSED_DATA.deref()).iter().map(|x| x.clone()).collect();
        rank_hands(&mut data);
        assert_eq!(data, *ORDERED_DATA.deref());
    }

    #[rstest]
    #[case(HandType::OnePair, HandType::TwoPair, Ordering::Less)]
    #[case(HandType::ThreeOfAKind, HandType::TwoPair, Ordering::Greater)]
    #[case(HandType::FiveOfAKind, HandType::FourOfAKind, Ordering::Greater)]
    #[case(HandType::FullHouse, HandType::ThreeOfAKind, Ordering::Greater)]
    fn test_ordering_hand_types(#[case] left: HandType, #[case] right: HandType, #[case] expected: Ordering) {
        assert_eq!(left.cmp(&right), expected);
    }

    #[test]
    fn test_ordering_two_different_hand_types() {
        let left = Hand {
            cards: vec![King, King, Six, Seven, Seven],
            hand_type: HandType::TwoPair,
            bid: 28
        };
        let right = Hand {
            cards: vec![Ten, Five, Five, Jack, Five],
            hand_type: HandType::ThreeOfAKind,
            bid: 684
        };
        assert_eq!(left.cmp(&right), Ordering::Less)
    }

    #[rstest]
    #[case(Joker, Joker, Joker, Joker, Joker, HandType::FiveOfAKind)]
    #[case(King, King, King, King, King, HandType::FiveOfAKind)]
    #[case(King, King, King, King, Joker, HandType::FiveOfAKind)]
    #[case(King, Queen, King, King, Joker, HandType::FourOfAKind)]
    #[case(King, Queen, King, Joker, Joker, HandType::FourOfAKind)]
    #[case(King, Queen, Joker, Joker, Joker, HandType::FourOfAKind)]
    #[case(King, Queen, Queen, Joker, Joker, HandType::FourOfAKind)]
    #[case(King, King, Queen, Queen, Joker, HandType::FullHouse)]
    #[case(King, Queen, Jack, Joker, Joker, HandType::ThreeOfAKind)]
    #[case(King, Queen, Jack, King, Joker, HandType::ThreeOfAKind)]
    #[case(King, Queen, Jack, King, King, HandType::ThreeOfAKind)]
    #[case(King, Queen, Ten, Ten, Joker, HandType::ThreeOfAKind)]
    #[case(King, King, Ten, Ten, Nine, HandType::TwoPair)] // can't get a two pair with a joker!
    #[case(King, Queen, Jack, Ten, Joker, HandType::OnePair)]
    #[case(King, Queen, Jack, Ten, Ten, HandType::OnePair)]
    #[case(King, Queen, Jack, Ten, Nine, HandType::HighCard)]
    fn test_hand_types(#[case] first: Card, #[case] second: Card, #[case] third: Card, #[case] fourth: Card, #[case] fifth: Card, #[case] hand_type: HandType) {
        let mut cards =  vec![first, second, third, fourth, fifth];
        cards.shuffle(&mut rand::thread_rng());
        assert_eq!(determine_hand(&cards), hand_type);
    }

}