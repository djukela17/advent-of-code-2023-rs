pub fn part_one(input: String) -> i32 {
    let mut schematic: Vec<Vec<char>> = Vec::new();

    input.lines().enumerate().for_each(|(i, line)| {
        schematic.push(Vec::new());

        line.chars().for_each(|c| {
            schematic[i].push(c);
        })
    });

    let mut sum = 0;

    for (i, row) in schematic.iter().enumerate() {
        let mut num = 0;
        let mut p = 1;
        let mut is_adjacent = false;

        for j in (0..row.len()).rev() {
            let c = schematic[i][j];

            if !c.is_digit(10) {
                if num > 0 {
                    // this was the last digit found -> check all NW, W and SW sides for symbols
                    if j > 0 {
                        if i > 0 {
                            // can check NW
                            let c = schematic[i - 1][j];
                            if !c.is_digit(10) && c != '.' {
                                is_adjacent = true;
                            }
                        }

                        // due to the check above, can always check E side
                        // check W
                        if !row[j].is_digit(10) && row[j] != '.' {
                            // it's a symbol
                            is_adjacent = true;
                        }

                        // check SW
                        if i < schematic.len() - 1 {
                            // cah check SE
                            let c = schematic[i + 1][j];
                            if !c.is_digit(10) && c != '.' {
                                is_adjacent = true;
                            }
                        }
                    }

                    if is_adjacent {
                        sum += num;
                    }
                }

                num = 0;
                p = 1;
                is_adjacent = false;
            } else {
                // if this is the first number
                if num == 0 && j < row.len() - 1 {
                    let j = j + 1;
                    // check all E eligible E-side coordinates
                    // if this is the first digit found, check all NE, E and SE for symbols

                    // check NE
                    if i > 0 {
                        // can check NE
                        let c = schematic[i - 1][j];
                        if !c.is_digit(10) && c != '.' {
                            is_adjacent = true;
                        }
                    }

                    // due to the check above, can always check E side
                    // check E
                    if !row[j].is_digit(10) && row[j] != '.' {
                        // it's a symbol
                        is_adjacent = true;
                    }

                    // check SE
                    if i < schematic.len() - 1 {
                        // cah check SE
                        let c = schematic[i + 1][j];
                        if !c.is_digit(10) && c != '.' {
                            is_adjacent = true;
                        }
                    }
                }

                // check N and S of the current coordinates for symbols

                // check N
                if i > 0 {
                    let c = schematic[i - 1][j];
                    if !c.is_digit(10) && c != '.' {
                        is_adjacent = true;
                    }
                }

                // check S
                if i < schematic.len() - 1 {
                    let c = schematic[i + 1][j];
                    if !c.is_digit(10) && c != '.' {
                        is_adjacent = true;
                    }
                }

                num += c.to_digit(10).unwrap() * p;
                p *= 10;
            }
        }

        // have to check for num after each row, if the number is starting from the 1st char in the
        // row
        if num > 0 {
            // at this point, every adjacent slot was already checked
            if is_adjacent {
                sum += num;
            }
        }
    }

    sum as i32
}

fn part_two(input: String) -> i32 {
    let schematic = Schematic::new(input);

    schematic.calculate_gear_ratio()
}

struct Schematic {
    board: Vec<Vec<char>>,
}

impl Schematic {
    fn new(input: String) -> Schematic {
        Schematic {
            board: input
                .lines()
                .filter(|line| line.len() > 0)
                .map(|line| -> Vec<char> { line.chars().collect() })
                .collect(),
        }
    }

    // parse_number in N or W
    fn parse_number(row: &Vec<char>, x: usize) -> Option<u32> {
        // parse from start, going in E direction
        let mut part_no: Option<u32> = None;
        let mut pow = 1;

        for i in x..row.len() {
            let c = row[i];
            if !c.is_digit(10) {
                break;
            }

            let digit = c.to_digit(10).unwrap();

            match part_no {
                None => part_no = Some(digit),
                Some(num) => part_no = Some(num * 10 + digit),
            }

            pow *= 10;
        }

        // parse any digit in the S direction
        if let Some(mut num) = part_no {
            for i in (0..x).rev() {
                let c = row[i];
                if !c.is_digit(10) {
                    break;
                }

                let digit = c.to_digit(10).unwrap();

                num += digit * pow;
                pow *= 10;
            }

            return Some(num);
        }

        part_no
    }

    fn calculate_gear_ratio(&self) -> i32 {
        let mut gear_ratio = 0;

        for (y, row) in self.board.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if c == &'*' {
                    let mut adjacent_part_numbers = vec![];

                    // W
                    if x > 0 {
                        if let Some(part_no) = Schematic::parse_number(&row, x - 1) {
                            if !adjacent_part_numbers.contains(&part_no) {
                                adjacent_part_numbers.push(part_no);
                            }
                        }
                    }

                    // N
                    if y > 0 {
                        if let Some(part_no) = Schematic::parse_number(&self.board[y - 1], x) {
                            if !adjacent_part_numbers.contains(&part_no) {
                                adjacent_part_numbers.push(part_no);
                            }
                        }
                    }

                    // NW
                    if y > 0 && x > 0 {
                        if let Some(part_no) = Schematic::parse_number(&self.board[y - 1], x - 1) {
                            if !adjacent_part_numbers.contains(&part_no) {
                                adjacent_part_numbers.push(part_no);
                            }
                        }
                    }

                    // NE
                    if y > 0 && x < row.len() - 1 {
                        if let Some(part_no) = Schematic::parse_number(&self.board[y - 1], x + 1) {
                            if !adjacent_part_numbers.contains(&part_no) {
                                adjacent_part_numbers.push(part_no);
                            }
                        }
                    }

                    // E
                    if x < row.len() - 1 {
                        if let Some(part_no) = Schematic::parse_number(&row, x + 1) {
                            if !adjacent_part_numbers.contains(&part_no) {
                                adjacent_part_numbers.push(part_no);
                            }
                        }
                    }

                    // SW
                    if y < self.board.len() - 1 && x > 0 {
                        if let Some(part_no) = Schematic::parse_number(&self.board[y + 1], x - 1) {
                            if !adjacent_part_numbers.contains(&part_no) {
                                adjacent_part_numbers.push(part_no);
                            }
                        }
                    }

                    // SE
                    if y < self.board.len() - 1 && x < row.len() - 1 {
                        if let Some(part_no) = Schematic::parse_number(&self.board[y + 1], x + 1) {
                            if !adjacent_part_numbers.contains(&part_no) {
                                adjacent_part_numbers.push(part_no);
                            }
                        }
                    }

                    // S
                    if y < self.board.len() - 1 {
                        if let Some(part_no) = Schematic::parse_number(&self.board[y + 1], x) {
                            if !adjacent_part_numbers.contains(&part_no) {
                                adjacent_part_numbers.push(part_no);
                            }
                        }
                    }

                    if adjacent_part_numbers.len() > 1 {
                        gear_ratio += adjacent_part_numbers
                            .iter()
                            .fold(1, |acc, part_no| acc * part_no);
                    }
                }
            }
        }

        gear_ratio as i32
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = std::fs::read_to_string("inputs/example.day_03_part_one.txt").unwrap();

        let expected = 4361;
        let result = part_one(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn real_input() {
        let input = std::fs::read_to_string("inputs/day_03_part_one.txt").unwrap();

        let expected = 0; // this will depend on real input
        let result = part_one(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn part_two_example_input() {
        let input = std::fs::read_to_string("inputs/example.day_03_part_one.txt").unwrap();

        let result = part_two(input);
        let expected = 467835;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_two_real_input() {
        let input = std::fs::read_to_string("inputs/day_03_part_one.txt").unwrap();

        let result = part_two(input);
        let expected = 0;

        assert_eq!(expected, result);
    }
}
