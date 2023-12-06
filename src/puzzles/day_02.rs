pub fn part_one(input: String) -> i32 {
    println!("Day 02 - part one");

    println!("Working with input: {}", input);

    42
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string(String::from("inputs/example.day_02_part_one.txt")).unwrap();

        let expected = 42;
        let result = part_one(input);

        assert_eq!(expected, result);
    }
}