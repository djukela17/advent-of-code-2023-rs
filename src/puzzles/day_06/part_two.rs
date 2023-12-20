use crate::puzzles::day_06::part_one::Race;

pub struct Puzzle {}

impl Puzzle {
    pub fn new() -> Puzzle {
        Puzzle {}
    }
    pub fn run(&self, input: String) -> i64 {
        let mut lines = input.lines();

        let time: i64 = lines
            .next()
            .unwrap()
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .replace(" ", "")
            .parse()
            .unwrap();

        let distance: i64 = lines
            .next()
            .unwrap()
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .replace(" ", "")
            .parse()
            .unwrap();

        println!("times: {:?}", time);

        let race = Race::new(time, distance);

        race.count_ways_to_win()
    }
}
