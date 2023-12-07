use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    Joker,
    Two,
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
    Ace,
}

impl Card {
    fn new(c: char) -> Self {
        use Card::*;
        match c {
            'J' => Joker,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => panic!("This shouldn't happen"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    kind: HandType,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.kind.cmp(&other.kind);
        if cmp != Ordering::Equal {
            return cmp;
        }

        for i in 0..5 {
            let cmp = self.cards[i].cmp(&other.cards[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.kind == other.kind {
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    return false;
                }
            }
        }
        false
    }
}

impl Eq for Hand {}

impl FromStr for Hand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let mut c = parts.next().unwrap().chars().map(|c| Card::new(c));
        let cards = [
            c.next().ok_or("Failed to unwrap")?,
            c.next().unwrap(),
            c.next().unwrap(),
            c.next().unwrap(),
            c.next().unwrap(),
        ];
        let bid = parts.next().unwrap().trim().parse::<usize>().unwrap();

        let mut histogram = [0; 13];
        cards.iter().for_each(|c| histogram[*c as usize] += 1);
        histogram.sort();
        let kind = match histogram[12] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if histogram[11] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if histogram[11] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            }
            _ => HandType::HighCard,
        };
        Ok(Hand { cards, bid, kind })
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut hands: Vec<_> = input
        .split("\n")
        .filter_map(|h| Hand::from_str(h).ok())
        .collect();
    hands.sort();

    let result: usize = hands.iter().enumerate().map(|(i, x)| (i + 1) * x.bid).sum();
    println!("{:?}", result);
}
