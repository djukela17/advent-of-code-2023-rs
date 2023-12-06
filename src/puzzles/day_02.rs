pub fn part_one(input: String) -> i32 {
    println!("Day 02 - part one");

    let mut games = vec![];
    input.trim().lines().for_each(|line| {
        games.push(Game::parse(line));
    });

    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    games
        .iter()
        .filter(|game| {
            return game.max_red_cubes <= red_cubes
                && game.max_green_cubes <= green_cubes
                && game.max_blue_cubes <= blue_cubes;
        })
        .map(|game| game.id)
        .sum()
}

#[derive(Debug)]
struct Game {
    id: i32,
    max_red_cubes: i32,
    max_green_cubes: i32,
    max_blue_cubes: i32,
}

impl Game {
    fn parse(line: &str) -> Game {
        const RED: &str = "red";
        const GREEN: &str = "green";
        const BLUE: &str = "blue";

        // get the game id
        let parts: Vec<&str> = line.split(":").collect();

        let game_parts: Vec<&str> = parts[0].split(" ").collect();

        let id: i32 = game_parts[1].parse().unwrap();
        let mut max_red_cubes = 0;
        let mut max_green_cubes = 0;
        let mut max_blue_cubes = 0;

        parts[1].split(";").for_each(|line| {
            line.split(",").for_each(|l| {
                let l = l.trim();

                let parts: Vec<&str> = l.split(" ").collect();

                let n: i32 = parts[0].trim().parse().unwrap();
                let color: &str = parts[1].trim();

                match color {
                    RED => {
                        if n > max_red_cubes {
                            max_red_cubes = n;
                        }
                    }
                    GREEN => {
                        if n > max_green_cubes {
                            max_green_cubes = n
                        }
                    }
                    BLUE => {
                        if n > max_blue_cubes {
                            max_blue_cubes = n
                        }
                    }
                    _ => {
                        panic!("What color is this?!?!");
                    }
                }
            });
        });

        Game {
            id,
            max_red_cubes,
            max_green_cubes,
            max_blue_cubes,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example() {
        let input =
            std::fs::read_to_string(String::from("inputs/example.day_02_part_one.txt")).unwrap();

        let expected = 8;
        let result = part_one(input);

        assert_eq!(expected, result);
    }
}
