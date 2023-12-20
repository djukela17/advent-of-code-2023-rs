use std::str::Split;

pub fn part_one(input: String) -> i64 {
    let mut segments = input.split("\n\n");

    let seed_id_segment = segments.next().unwrap();

    let seed_ids: Vec<i64> = seed_id_segment
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|num| num.trim().parse().unwrap())
        .collect();

    let chain = Chain::new(segments);

    let mut closest: Option<(i64, i64)> = None;

    for seed_id in seed_ids.iter() {
        match closest {
            None => closest = Some((*seed_id, chain.dest_for(*seed_id))),
            Some(c) => {
                let location = chain.dest_for(*seed_id);

                if location < c.1 {
                    closest = Some((*seed_id, location));
                }
            }
        }
    }

    closest.unwrap().1
}

pub fn part_two(input: String) -> i64 {
    let mut segments = input.split("\n\n");

    let seed_ids = SeedRanges::new(segments.next().unwrap());

    let chain = Chain::new(segments);

    let mut closest: Option<i64> = None;

    for seed_range in seed_ids.ranges.iter() {
        for seed_id in seed_range.start..seed_range.start + seed_range.len {
            match closest {
                None => closest = Some(chain.dest_for(seed_id)),
                Some(closest_loc) => {
                    let location = chain.dest_for(seed_id);

                    if location < closest_loc {
                        closest = Some(location);
                    }
                }
            }
        }
    }

    closest.unwrap()
}

#[derive(Debug)]
struct Range {
    start: i64,
    len: i64,
}

#[derive(Debug)]
struct SeedRanges {
    ranges: Vec<Range>,
}

impl SeedRanges {
    fn new(input: &str) -> SeedRanges {
        let mut seed_id: Option<i64> = None;
        let mut len: Option<i64> = None;

        let mut ranges = vec![];

        input
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .for_each(|num| {
                if seed_id == None {
                    seed_id = Some(num.trim().parse().unwrap());

                    return;
                }

                if len == None {
                    len = Some(num.trim().parse().unwrap());
                }

                ranges.push(Range {
                    start: seed_id.unwrap(),
                    len: len.unwrap(),
                });

                seed_id = None;
                len = None;
            });

        SeedRanges { ranges }
    }
}

#[derive(Debug)]
struct Chain {
    maps: Vec<SrcToDestRangeMap>,
}

impl Chain {
    fn new(segments: Split<&str>) -> Chain {
        Chain {
            maps: segments
                .map(|segment| SrcToDestRangeMap::new(segment))
                .collect(),
        }
    }

    fn dest_for(&self, seed: i64) -> i64 {
        let mut dest = seed;

        for map in self.maps.iter() {
            dest = map.dest_for(dest);
        }

        dest
    }
}

#[derive(Debug)]
struct SrcToDestRangeMap {
    maps: Vec<SrcToDestRange>,
}

impl SrcToDestRangeMap {
    fn new(segment: &str) -> SrcToDestRangeMap {
        let mut maps = vec![];

        segment
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .lines()
            .for_each(|line| {
                let mut nums = line.trim().split_whitespace();

                let dest = nums.next().unwrap().parse().unwrap();
                let src = nums.next().unwrap().parse().unwrap();
                let range = nums.next().unwrap().parse().unwrap();

                maps.push(SrcToDestRange::new(src, dest, range))
            });

        SrcToDestRangeMap { maps }
    }

    fn dest_for(&self, src: i64) -> i64 {
        let mut is_mapped = false;
        let mut idx = 0;

        for (i, map) in self.maps.iter().enumerate() {
            if map.has_dest_for_src(src) {
                is_mapped = true;
                idx = i;

                break;
            }
        }

        if !is_mapped {
            return src;
        }

        self.maps[idx].dest_for(src)
    }
}

#[derive(Debug)]
struct SrcToDestRange {
    src: i64,
    dest: i64,
    range: i64,
}

impl SrcToDestRange {
    fn new(src: i64, dest: i64, range: i64) -> SrcToDestRange {
        SrcToDestRange { src, dest, range }
    }

    fn has_dest_for_src(&self, src: i64) -> bool {
        src >= self.src && src < self.src + self.range
    }

    // dest_for will now work correctly if src is not mapped here
    fn dest_for(&self, src: i64) -> i64 {
        if !self.has_dest_for_src(src) {
            panic!("no dest mapped for src");
        }

        let offset = src - self.src;

        self.dest + offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        let input = std::fs::read_to_string("inputs/example.day_05.txt").unwrap();

        let expected = 35;
        let result = part_one(input);

        assert_eq!(expected, result);
    }
}
