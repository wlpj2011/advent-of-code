use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 7 of 2023 Advent of Code
#[derive(Parser, Debug)]
struct Args {
    /// File to run solution code on
    #[arg()]
    file: String,

    #[clap(flatten)]
    group: Group,
}

#[derive(Parser, Debug)]
#[group(required = true)]
struct Group {
    /// Run solution to part a of day 7.
    #[arg(short)]
    a: bool,

    /// Run solution to part b of day 7.
    #[arg(short)]
    b: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    None,
}

impl Card {
    fn enum_index(&self) -> u64 {
        match *self {
            Card::Ace => 12,
            Card::King => 11,
            Card::Queen => 10,
            Card::Jack => 9,
            Card::Ten => 8,
            Card::Nine => 7,
            Card::Eight => 6,
            Card::Seven => 5,
            Card::Six => 4,
            Card::Five => 3,
            Card::Four => 2,
            Card::Three => 1,
            Card::Two => 0,
            Card::None => 0,
        }
    }

    fn from_char(card: char) -> Card {
        if card == 'A' {
            Card::Ace
        } else if card == 'K' {
            Card::King
        } else if card == 'Q' {
            Card::Queen
        } else if card == 'J' {
            Card::Jack
        } else if card == 'T' {
            Card::Ten
        } else if card == '9' {
            Card::Nine
        } else if card == '8' {
            Card::Eight
        } else if card == '7' {
            Card::Seven
        } else if card == '6' {
            Card::Six
        } else if card == '5' {
            Card::Five
        } else if card == '4' {
            Card::Four
        } else if card == '3' {
            Card::Three
        } else if card == '2' {
            Card::Two
        } else {
            Card::None
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.enum_index().cmp(&other.enum_index())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn enum_index(&self) -> u64 {
        match *self {
            HandType::FiveOfKind => 6,
            HandType::FourOfKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.enum_index().cmp(&other.enum_index())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    hand: [Card; 5],
    bet: u64,
    hand_type: HandType,
}

impl Hand {
    fn from_str(string: &str) -> Result<Hand> {
        let parts: Vec<_> = string.split_ascii_whitespace().collect();
        let mut hand: [Card; 5] = [Card::Two; 5];
        let hand_str = parts[0];
        let bet: u64 = parts[1].parse()?;
        for (place, card) in hand_str.chars().enumerate() {
            hand[place] = Card::from_char(card);
        }

        let mut hand_type = HandType::HighCard;

        let hand_clone = hand.clone();
        let mut map: HashMap<Card, u64> = HashMap::new();
        for item in hand_clone {
            *map.entry(item).or_insert(0) += 1;
        }

        let (_cards, frequencies): (Vec<Card>, Vec<u64>) = map.iter().unzip();
        if frequencies.len() == 1 {
            hand_type = HandType::FiveOfKind;
        } else if frequencies.len() == 2 {
            if frequencies.contains(&4) {
                hand_type = HandType::FourOfKind;
            } else if frequencies.contains(&3) {
                hand_type = HandType::FullHouse
            }
        } else if frequencies.len() == 3 {
            if frequencies.contains(&3) {
                hand_type = HandType::ThreeOfKind
            } else if frequencies.contains(&2) {
                hand_type = HandType::TwoPair
            }
        } else if frequencies.len() == 4 {
            hand_type = HandType::OnePair
        }

        Ok(Hand {
            hand,
            bet,
            hand_type,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (place, card) in self.hand.iter().enumerate() {
                if *card == other.hand[place] {
                    continue;
                } else {
                    return card.cmp(&other.hand[place]);
                }
            }
            return std::cmp::Ordering::Equal;
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut hands: Vec<Hand> = Vec::new();
    while reader.read_line(&mut line)? != 0 {
        hands.push(Hand::from_str(&line)?);
        line.clear();
    }
    hands.sort();
    for (rank, hand) in hands.iter().enumerate() {
        result += hand.bet * ((rank as u64) + 1);
    }
    Ok(result)
}

fn solution_b(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {
        line.clear();
    }

    Ok(result)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.group.a {
        let file = File::open(args.file.clone())?;
        let result = solution_a(file)?;

        println!("The total winnings of your hands are {result}.");
    }
    if args.group.b {
        let file = File::open(args.file.clone())?;
        let result = solution_b(file)?;

        println!("The total winnings of your hands are {result}.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};

    #[test]
    fn test_solution_a() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-07.txt")?)?, 6440);
        Ok(())
    }

    #[test]
    fn test_solution_b() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-07.txt")?)?, 0);
        Ok(())
    }

    #[test]
    fn test_hand_from_str_one_pair() -> Result<()> {
        let hand = Hand {
            hand: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
            bet: 765,
            hand_type: HandType::OnePair,
        };
        assert_eq!(Hand::from_str("32T3K 765")?, hand);
        Ok(())
    }

    #[test]
    fn test_hand_from_str_two_pair_1() -> Result<()> {
        let hand = Hand {
            hand: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
            bet: 28,
            hand_type: HandType::TwoPair,
        };
        assert_eq!(Hand::from_str("KK677 28")?, hand);
        Ok(())
    }

    #[test]
    fn test_hand_from_str_two_pair_2() -> Result<()> {
        let hand = Hand {
            hand: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
            bet: 220,
            hand_type: HandType::TwoPair,
        };
        assert_eq!(Hand::from_str("KTJJT 220")?, hand);
        Ok(())
    }

    #[test]
    fn test_hand_from_str_three_of_kind_1() -> Result<()> {
        let hand = Hand {
            hand: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
            bet: 684,
            hand_type: HandType::ThreeOfKind,
        };
        assert_eq!(Hand::from_str("T55J5 684")?, hand);
        Ok(())
    }

    #[test]
    fn test_hand_from_str_three_of_kind_2() -> Result<()> {
        let hand = Hand {
            hand: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
            bet: 483,
            hand_type: HandType::ThreeOfKind,
        };
        assert_eq!(Hand::from_str("QQQJA 483")?, hand);
        Ok(())
    }

    #[test]
    fn test_hand_compare_1() -> Result<()> {
        let hand1 = Hand {
            hand: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
            bet: 483,
            hand_type: HandType::ThreeOfKind,
        };
        let hand2 = Hand {
            hand: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
            bet: 684,
            hand_type: HandType::ThreeOfKind,
        };
        assert!(hand1 > hand2);
        Ok(())
    }

    #[test]
    fn test_hand_compare_2() -> Result<()> {
        let hand1 = Hand {
            hand: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
            bet: 483,
            hand_type: HandType::ThreeOfKind,
        };
        let hand2 = Hand {
            hand: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
            bet: 220,
            hand_type: HandType::TwoPair,
        };
        assert!(hand1 > hand2);
        Ok(())
    }

    #[test]
    fn test_hand_compare_3() -> Result<()> {
        let hand1 = Hand {
            hand: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
            bet: 28,
            hand_type: HandType::TwoPair,
        };
        let hand2 = Hand {
            hand: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
            bet: 220,
            hand_type: HandType::TwoPair,
        };
        assert!(hand1 > hand2);
        Ok(())
    }
}
