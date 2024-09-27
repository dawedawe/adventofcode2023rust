use std::{
    collections::{self},
    fs,
};

const INPUT: &str = "day07input.txt";

// #[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Debug)]
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
}

impl Card {
    fn part1_value(&self) -> i32 {
        match self {
            Card::_2 => 0,
            Card::_3 => 1,
            Card::_4 => 2,
            Card::_5 => 3,
            Card::_6 => 4,
            Card::_7 => 5,
            Card::_8 => 6,
            Card::_9 => 7,
            Card::T => 8,
            Card::J => 9,
            Card::Q => 10,
            Card::K => 11,
            Card::A => 12,
        }
    }

    fn cmp_part1(&self, other: &Self) -> std::cmp::Ordering {
        self.part1_value().cmp(&other.part1_value())
    }

    fn part2_value(&self) -> i32 {
        match self {
            Card::J => 0,
            Card::_2 => 1,
            Card::_3 => 2,
            Card::_4 => 3,
            Card::_5 => 4,
            Card::_6 => 5,
            Card::_7 => 6,
            Card::_8 => 7,
            Card::_9 => 8,
            Card::T => 9,
            Card::Q => 10,
            Card::K => 11,
            Card::A => 12,
        }
    }

    fn cmp_part2(&self, other: &Self) -> std::cmp::Ordering {
        self.part2_value().cmp(&other.part2_value())
    }

    fn from(char: &char) -> Card {
        match char {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::_9,
            '8' => Card::_8,
            '7' => Card::_7,
            '6' => Card::_6,
            '5' => Card::_5,
            '4' => Card::_4,
            '3' => Card::_3,
            '2' => Card::_2,
            _ => panic!("unknown char"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
enum CardType {
    FiveOfAKind = 6,  // where all five cards have the same label: AAAAA
    FourOfAKind = 5, // where four cards have the same label and one card has a different label: AA8AA
    FullHouse = 4, // where three cards have the same label, and the remaining two cards share a different label: 23332
    ThreeOfAKind = 3, // three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    TwoPair = 2, // where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    OnePair = 1, // where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    HighCard = 0, // where all cards' labels are distinct: 23456
}

impl CardType {
    fn get_type(mut cards: Vec<Card>) -> CardType {
        assert_eq!(5, cards.len());
        cards.sort_by(|a, b| a.cmp_part1(b));
        let h: collections::HashSet<Card> = collections::HashSet::from_iter(cards.clone());
        let diff_labels = h.len();
        if diff_labels == 1 {
            return CardType::FiveOfAKind;
        }

        if diff_labels == 2 {
            // fourofakind or fullhouse
            if cards[0] != cards[1] || cards[3] != cards[4] {
                return CardType::FourOfAKind;
            }
            if cards[1] != cards[2] || cards[2] != cards[3] {
                return CardType::FullHouse;
            }
            panic!("fourofakind or fullhouse");
        }

        if diff_labels == 3 {
            // threeofakind or two pair
            if cards[0] == cards[1] && cards[1] == cards[2]
                || cards[1] == cards[2] && cards[2] == cards[3]
                || cards[2] == cards[3] && cards[3] == cards[4]
            {
                return CardType::ThreeOfAKind;
            }
            if cards[0] != cards[1] || cards[2] != cards[3] || cards[4] != cards[3] {
                return CardType::TwoPair;
            }
            panic!("threeofakind or two pair");
        }

        if diff_labels == 4 {
            return CardType::OnePair;
        }

        if diff_labels == 5 {
            return CardType::HighCard;
        }

        panic!("bad state");
    }

    fn turn_jokers_into(mut cards: Vec<Card>, non_joker: &Card) -> Vec<Card> {
        for card in cards.iter_mut().take(4) {
            if *card == Card::J {
                *card = non_joker.clone();
            }
        }
        cards
    }

    fn jokerize(mut cards: Vec<Card>) -> CardType {
        assert_eq!(5, cards.len());

        let jokers = cards.iter().filter(|c| **c == Card::J).count();
        if jokers == 0 || jokers == 5 {
            return CardType::get_type(cards);
        }

        cards.sort_by(|a, b| a.cmp_part2(b));
        let non_jokers = cards
            .clone()
            .into_iter()
            .filter(|c| *c != Card::J)
            .collect::<Vec<Card>>();
        let biggest_non_joker = non_jokers
            .chunk_by(|a, b| a == b)
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap();
        let non_joker = &biggest_non_joker[0];
        let cards = CardType::turn_jokers_into(cards, non_joker);
        CardType::get_type(cards)
    }
}

struct Hand {
    cards: Vec<Card>,
    card_type: CardType,
}

impl Hand {
    fn new(cards: Vec<Card>, part2: bool) -> Self {
        let card_type = if part2 {
            CardType::jokerize(cards.clone())
        } else {
            CardType::get_type(cards.clone())
        };
        Hand { cards, card_type }
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.card_type > other.card_type {
            std::cmp::Ordering::Greater
        } else if self.card_type < other.card_type {
            std::cmp::Ordering::Less
        } else {
            let first_differ = self
                .cards
                .iter()
                .zip(other.cards.clone())
                .find(|(a, b)| **a != *b);
            match first_differ {
                Some((a, b)) => a.cmp_part1(&b),
                None => std::cmp::Ordering::Equal,
            }
        }
    }

    fn cmp_part2(&self, other: &Self) -> std::cmp::Ordering {
        if self.card_type > other.card_type {
            std::cmp::Ordering::Greater
        } else if self.card_type < other.card_type {
            std::cmp::Ordering::Less
        } else {
            let first_differ = self
                .cards
                .iter()
                .zip(other.cards.clone())
                .find(|(a, b)| **a != *b);
            match first_differ {
                Some((a, b)) => a.cmp_part2(&b),
                None => std::cmp::Ordering::Equal,
            }
        }
    }
}

fn parse_line(line: &str, part2: bool) -> (Hand, i32) {
    let parts = line.split_whitespace().collect::<Vec<&str>>();

    let cards = parts[0]
        .bytes()
        .map(|c| {
            let ch = c as char;
            Card::from(&ch)
        })
        .collect();
    let hand = Hand::new(cards, part2);
    let bid = parts[1].parse::<i32>().unwrap();
    (hand, bid)
}

pub fn part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();
    let mut hand_and_bids = lines
        .into_iter()
        .map(|line| parse_line(line, false))
        .collect::<Vec<(Hand, i32)>>();
    hand_and_bids.sort_by(|a, b| a.0.cmp(&b.0));

    let sum: i32 = (0..hand_and_bids.len())
        .map(|idx| {
            let rank = (idx + 1) as i32;
            rank * hand_and_bids[idx].1
        })
        .sum();

    println!("{}", sum);
}

pub fn part2() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();
    let mut hand_and_bids = lines
        .into_iter()
        .map(|line| parse_line(line, true))
        .collect::<Vec<(Hand, i32)>>();
    hand_and_bids.sort_by(|a, b| a.0.cmp_part2(&b.0));

    let sum: i32 = (0..hand_and_bids.len())
        .map(|idx| {
            let rank = (idx + 1) as i32;
            rank * hand_and_bids[idx].1
        })
        .sum();

    println!("{}", sum);
}
