use std::collections::HashMap;

pub fn part_one(input: String) -> i32 {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| -> i32 {
            let card = Card::parse(line);

            card.calculate_points()
        })
        .sum()
}

pub fn part_two(input: String) -> i32 {
    let cards: Vec<Card> = input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| -> Card { Card::parse(line) })
        .collect();

    let mut instances = HashMap::new();

    cards.iter().enumerate().for_each(|(i, card)| {
        let card_copies = if let Some(c) = instances.get(&i) {
            *c
        } else {
            0
        };

        // add the current card instance
        if let Some(c) = instances.get(&i) {
            instances.insert(i, c + 1);
        } else {
            instances.insert(i, 1);
        }

        let matching_numbers = card.count_matching_numbers();

        for n in 1..matching_numbers + 1 {
            if let Some(c) = instances.get(&(i + n as usize)) {
                instances.insert(i + n as usize, c + 1 + card_copies);
            } else {
                instances.insert(i + n as usize, 1 + card_copies);
            }
        }
    });

    instances.values().sum()
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<i32>,
    scratched_numbers: Vec<i32>,
}

impl Card {
    pub fn parse(line: &str) -> Card {
        let nums: Vec<&str> = line.split(":").skip(1).collect();

        let mut iter = nums[0].split("|");

        Card {
            winning_numbers: iter
                .next()
                .unwrap()
                .split(" ")
                .filter(|num| num.trim().len() > 0)
                .map(|num| -> i32 { num.trim().parse().unwrap() })
                .collect(),
            scratched_numbers: iter
                .next()
                .unwrap()
                .split(" ")
                .filter(|num| num.trim().len() > 0)
                .map(|num| -> i32 { num.trim().parse().unwrap() })
                .collect(),
        }
    }

    pub fn count_matching_numbers(&self) -> i32 {
        let mut matching_numbers = 0;

        for num in self.scratched_numbers.iter().as_ref() {
            if self.winning_numbers.contains(num) {
                matching_numbers += 1;
            }
        }

        matching_numbers
    }

    pub fn calculate_points(&self) -> i32 {
        let mut pts = 0;

        for num in self.scratched_numbers.iter().as_ref() {
            if self.winning_numbers.contains(num) {
                match pts {
                    0 => pts += 1,
                    _ => pts *= 2,
                }
            }
        }

        pts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = std::fs::read_to_string("inputs/example.day_04.txt").unwrap();
        let expected = 13;
        let result = part_one(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn real_input() {
        let input = std::fs::read_to_string("inputs/day_04.txt").unwrap();
        let expected = 0; // this will fail
        let result = part_one(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn part_two_example_input() {
        let input = std::fs::read_to_string("inputs/example.day_04.txt").unwrap();

        let expected = 30;
        let result = part_two(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn part_two_real_input() {
        let input = std::fs::read_to_string("inputs/day_04.txt").unwrap();

        let expected = 0;
        let result = part_two(input);

        assert_eq!(expected, result);
    }
}
