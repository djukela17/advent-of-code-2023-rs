use std::cmp::Ordering;
use Card::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Card {
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
    Joker,
}

impl Card {
    pub fn strength(&self) -> u8 {
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
            Joker => 0,
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
