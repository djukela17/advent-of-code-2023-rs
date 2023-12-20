use std::env;

use advent_of_code_2023_rs::puzzles;

fn main() {
    let mut args = env::args();
    if env::args().len() < 3 {
        println!("specify day and part to run");

        return;
    }

    args.next().unwrap();

    let day = args.next().unwrap();
    let day = if day.len() < 2 {
        format!("0{}", day)
    } else {
        day
    };
    let part = args.next().unwrap();
    let filename = if let Some(_) = args.next() {
        format!("inputs/day_{}.txt", day)
    } else {
        format!("inputs/example.day_{}.txt", day)
    };

    match day.as_str() {
        "05" => match part.as_str() {
            "one" => {
                let input = std::fs::read_to_string(filename).unwrap();
                let result = puzzles::day_05::part_one(input);
                println!("day {} part {} = {}", day, part, result)
            }
            "two" => {
                let input = std::fs::read_to_string(filename).unwrap();
                let result = puzzles::day_05::part_two(input);
                println!("day {} part {} = {}", day, part, result)
            }
            _ => {
                println!("no such part");

                return;
            }
        },
        _ => {
            println!("no solution for selected day");

            return;
        }
    }
}
