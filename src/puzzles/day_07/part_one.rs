use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use Card::*;
use HandType::*;

pub struct Puzzle {}

impl Puzzle {
    pub fn new() -> Puzzle {
        Puzzle {}
    }

    pub fn run(&self, input: String) -> i64 {
        let game = Game::from(input);

        game.calculate_total_winnings()
    }
}

#[derive(Debug)]
struct Game {
    played_hands: Vec<PlayedHand>,
}

impl Game {
    fn from(input: String) -> Game {
        Game {
            played_hands: input
                .lines()
                .filter(|line| line.len() > 0)
                .map(|line| PlayedHand::from(line))
                .collect(),
        }
    }

    fn calculate_total_winnings(&self) -> i64 {
        let mut played_hands_by_type: HashMap<HandType, Vec<&PlayedHand>> = HashMap::new();

        for ph in self.played_hands.iter() {
            let t = ph.get_hand_type();
            let entry = played_hands_by_type.entry(t).or_default();

            entry.push(ph);
        }

        let mut ordered_hands = HashMap::new();

        for (hand_type, mut played_hands) in played_hands_by_type {
            played_hands.sort_by(|a, b| {
                for i in 0..a.hand.cards.len() {
                    if a.hand.cards[i] < b.hand.cards[i] {
                        return Ordering::Less;
                    } else if a.hand.cards[i] > b.hand.cards[i] {
                        return Ordering::Greater;
                    }
                }

                Ordering::Equal
            });

            ordered_hands.insert(hand_type, played_hands);
        }

        // now that all the cards are sorted, start from the lowest and count total winnings
        let mut total_winnings = 0;
        let mut total_rank = 0;

        for hand_type in HandType::all() {
            if let Some(played_hands) = ordered_hands.get(&hand_type) {
                for ph in played_hands {
                    total_rank += 1;

                    total_winnings += ph.bid * total_rank;
                }
            }
        }

        total_winnings
    }
}

#[derive(Debug)]
struct PlayedHand {
    hand: Hand,
    bid: i64,
}

impl PlayedHand {
    fn from(input: &str) -> PlayedHand {
        let mut parts = input.split(" ");

        PlayedHand {
            hand: Hand::from(parts.next().unwrap()),
            bid: parts.next().unwrap().parse().unwrap(),
        }
    }

    fn get_hand_type(&self) -> HandType {
        self.hand.get_type()
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn from(card_symbols: &str) -> Hand {
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

    fn get_type(&self) -> HandType {
        let mut cards: HashMap<&Card, u8> = HashMap::new();

        for c in self.cards.iter() {
            let entry = cards.entry(c).or_default();

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
            FourOfAKind
        } else if three_of_a_kinds == 1 && pairs == 1 {
            FullHouse
        } else if three_of_a_kinds == 1 {
            ThreeOfAKind
        } else if pairs == 2 {
            TwoPairs
        } else if pairs == 1 {
            OnePair
        } else {
            HighCard
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl HandType {
    fn all() -> [HandType; 7] {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
}

impl Card {
    fn strength(&self) -> u8 {
        match self {
            Ace => 13,
            King => 12,
            Queen => 11,
            Jack => 10,
            Ten => 9,
            Nine => 8,
            Eight => 7,
            Seven => 6,
            Six => 5,
            Five => 4,
            Four => 3,
            Three => 2,
            Two => 1,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.strength() < other.strength() {
            Some(Ordering::Less)
        } else if self.strength() > other.strength() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        let input = std::fs::read_to_string("../../../inputs/example.day_07.txt").unwrap();

        assert_eq!(6440, Puzzle {}.run(input));
    }
}
