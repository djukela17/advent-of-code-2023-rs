use crate::puzzles::day_07::game::Game;

pub struct Puzzle {}

impl Puzzle {
    pub fn new() -> Puzzle {
        Puzzle {}
    }

    pub fn run(&self, input: String) -> i64 {
        let game = Game::from_using_joker_rule(input);

        game.calculate_total_winnings()
    }
}
