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
}
