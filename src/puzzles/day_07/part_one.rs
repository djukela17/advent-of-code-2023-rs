use crate::puzzles::day_07::card::{Card::*, *};
use crate::puzzles::day_07::game::Game;
use crate::puzzles::day_07::hand::{HandType::*, *};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        let input = std::fs::read_to_string("../../../inputs/example.day_07.txt").unwrap();

        assert_eq!(6440, Puzzle {}.run(input));
    }
}
