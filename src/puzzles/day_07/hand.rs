use std::collections::HashMap;
use std::sync::mpsc::TrySendError::Full;

use crate::puzzles::day_07::card::{Card::*, *};
use HandType::*;

#[derive(Debug)]
pub struct PlayedHand {
    pub hand: Hand,
    pub bid: i64,
}

impl PlayedHand {
    pub fn from(input: &str) -> PlayedHand {
        let mut parts = input.split(" ");

        PlayedHand {
            hand: Hand::from(parts.next().unwrap()),
            bid: parts.next().unwrap().parse().unwrap(),
        }
    }

    pub fn from_using_joker_rule(input: &str) -> PlayedHand {
        let mut parts = input.split(" ");

        PlayedHand {
            hand: Hand::from_using_using_joker_rule(parts.next().unwrap()),
            bid: parts.next().unwrap().parse().unwrap(),
        }
    }

    pub fn get_hand_type(&self) -> HandType {
        self.hand.get_type()
    }
}

#[derive(Debug)]
pub struct Hand {
    pub cards: [Card; 5],
}

impl Hand {
    pub fn from(card_symbols: &str) -> Hand {
        let mut cards = [Card::Ace; 5];

        for (i, c) in card_symbols.chars().enumerate() {
            cards[i] = match c {
                'A' => Ace,
                'K' => King,
                'Q' => Queen,
                'J' => Jack,
                'T' => Ten,
                '9' => Nine,
                '8' => Eight,
                '7' => Seven,
                '6' => Six,
                '5' => Five,
                '4' => Four,
                '3' => Three,
                '2' => Two,
                _ => panic!("invalid card symbol"),
            }
        }

        Hand { cards }
    }

    pub fn from_using_using_joker_rule(card_symbols: &str) -> Hand {
        let mut cards = [Card::Ace; 5];

        for (i, c) in card_symbols.chars().enumerate() {
            cards[i] = match c {
                'A' => Ace,
                'K' => King,
                'Q' => Queen,
                'J' => Joker,
                'T' => Ten,
                '9' => Nine,
                '8' => Eight,
                '7' => Seven,
                '6' => Six,
                '5' => Five,
                '4' => Four,
                '3' => Three,
                '2' => Two,
                _ => panic!("invalid card symbol"),
            }
        }

        Hand { cards }
    }

    pub fn get_type(&self) -> HandType {
        let mut cards: HashMap<&Card, u8> = HashMap::new();

        let mut jokers = 0;

        for c in self.cards.iter() {
            let entry = cards.entry(c).or_default();

            if c == &Joker {
                jokers += 1;
            }

            *entry += 1;
        }

        let mut five_of_a_kinds: u8 = 0;
        let mut four_of_a_kinds: u8 = 0;
        let mut three_of_a_kinds: u8 = 0;
        let mut pairs: u8 = 0;

        for count in cards.values() {
            match count {
                5 => five_of_a_kinds += 1,
                4 => four_of_a_kinds += 1,
                3 => three_of_a_kinds += 1,
                2 => pairs += 1,
                _ => {}
            }
        }

        if five_of_a_kinds == 1 {
            FiveOfAKind
        } else if four_of_a_kinds == 1 {
            if jokers > 0 {
                FiveOfAKind
            } else {
                FourOfAKind
            }
        } else if three_of_a_kinds == 1 && pairs == 1 {
            if jokers > 0 {
                FiveOfAKind
            } else {
                FullHouse
            }
        } else if three_of_a_kinds == 1 {
            if jokers > 0 {
                FourOfAKind
            } else {
                ThreeOfAKind
            }
        } else if pairs == 2 {
            if jokers == 1 {
                FullHouse
            } else if jokers == 2 {
                FourOfAKind
            } else {
                TwoPairs
            }
        } else if pairs == 1 {
            if jokers > 0 {
                ThreeOfAKind
            } else {
                OnePair
            }
        } else {
            if jokers == 1 {
                OnePair
            } else {
                HighCard
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn all() -> [HandType; 7] {
        [
            HighCard,
            OnePair,
            TwoPairs,
            ThreeOfAKind,
            FullHouse,
            FourOfAKind,
            FiveOfAKind,
        ]
    }
}
