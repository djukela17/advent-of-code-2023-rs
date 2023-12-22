use crate::puzzles::day_07::hand::{HandType, PlayedHand};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    played_hands: Vec<PlayedHand>,
}

impl Game {
    pub fn from(input: String) -> Game {
        Game {
            played_hands: input
                .lines()
                .filter(|line| line.len() > 0)
                .map(|line| PlayedHand::from(line))
                .collect(),
        }
    }

    pub fn from_using_joker_rule(input: String) -> Game {
        Game {
            played_hands: input
                .lines()
                .filter(|line| line.len() > 0)
                .map(|line| PlayedHand::from_using_joker_rule(line))
                .collect(),
        }
    }

    pub fn calculate_total_winnings(&self) -> i64 {
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
