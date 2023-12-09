pub fn part_one(input: String) -> i32 {
    let mut schematic: Vec<Vec<char>> = Vec::new();

    input.lines().enumerate().for_each(|(i, line)| {
        schematic.push(Vec::new());

        line.chars().for_each(|c| {
            schematic[i].push(c);
        })
    });

    println!("schematic.len() = {}", schematic.len());

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
                if num == 0 && j < row.len()-1 {
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input =
            std::fs::read_to_string("inputs/example.day_03_part_one.txt").unwrap();

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
}