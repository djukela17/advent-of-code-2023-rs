use advent_of_code_2023_rs::puzzles;

fn main() {
    let input = std::fs::read_to_string("inputs/day_05.txt").unwrap();

    let result = puzzles::day_05::part_one(input);

    println!("day 05 part one = {}", result);
}
