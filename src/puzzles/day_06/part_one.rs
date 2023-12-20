pub struct Puzzle {}

impl Puzzle {
    pub fn new() -> Puzzle {
        Puzzle {}
    }
    pub fn run(&self, input: String) -> i64 {
        let mut lines = input.lines();

        let times: Vec<i32> = lines
            .next()
            .unwrap()
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|part| part.len() > 0)
            .map(|num| num.parse().unwrap())
            .collect();

        let distances: Vec<i32> = lines
            .next()
            .unwrap()
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|part| part.len() > 0)
            .map(|num| num.parse().unwrap())
            .collect();

        let races: Vec<Race> = times
            .iter()
            .zip(distances)
            .map(|(time, distance)| {
                return Race {
                    time: *time,
                    distance,
                };
            })
            .collect();

        println!("{:?}", races);

        races.iter().fold(1, |acc, race| {
            acc * race.count_ways_to_win()
        }) as i64
    }
}

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

impl Race {
    fn count_ways_to_win(&self) -> u32 {
        let mut ways_to_win = 0;

        for charge_time  in 1..self.time {
            let distance_traveled = (self.time - charge_time) * charge_time;
            if distance_traveled > self.distance {
                ways_to_win += 1;
            }
        }

        ways_to_win
    }
}