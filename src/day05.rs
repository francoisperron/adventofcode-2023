use std::str::Split;

#[cfg(test)]
mod tests {
    use crate::{daily_example, daily_input};
    use super::*;

    #[test]
    fn parses_seeds() {
        let input = daily_example(5);
        let almanac = Almanac::from(input);

        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn parses_maps() {
        let input = daily_example(5);
        let almanac = Almanac::from(input);

        assert_eq!(almanac.maps.len(), 7);
        let expected_map = Map { ranges: vec![Range { min: 98, max: 100, diff: -48 }, Range { min: 50, max: 98, diff: 2 }] };
        assert_eq!(almanac.maps.into_iter().next().unwrap(), expected_map);
    }

    #[test]
    fn solve_example_part1() {
        let input = daily_example(5);
        let almanac = Almanac::from(input);

        assert_eq!(almanac.lowest_location(), 35);
    }

    #[test]
    fn solve_part1() {
        let input = daily_input(5);
        let almanac = Almanac::from(input);

        assert_eq!(almanac.lowest_location(), 600279879);
    }

    #[test]
    fn solves_example_part2() {
        let input = daily_example(5);
        let almanac = Almanac::from_multiple_seeds(input);

        assert_eq!(almanac.lowest_location(), 46);
    }

    //ran for 2 hours on my mbp...
    #[test]
    #[ignore]
    fn solve_part2() {
        let input = daily_input(5);
        let almanac = Almanac::from_multiple_seeds(input);

        assert_eq!(almanac.lowest_location(), 20191102);
    }
}

#[derive(PartialEq, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(PartialEq, Debug)]
struct Map {
    ranges: Vec<Range>,
}

#[derive(PartialEq, Debug)]
struct Range {
    min: i64,
    max: i64,
    diff: i64,
}

impl Almanac {
    pub fn from(input: String) -> Almanac {
        let mut lines = input.split("\n\n");
        Almanac {
            seeds: Self::parse_seeds(&mut lines),
            maps: lines.map(Map::from).collect(),
        }
    }

    pub fn from_multiple_seeds(input: String) -> Almanac {
        let mut lines = input.split("\n\n");
        let simple_seeds = Self::parse_seeds(&mut lines);
        let mut simple_seeds_iter = simple_seeds.iter();
        let mut seeds = vec![];
        while let Some(start) = simple_seeds_iter.next() {
            let end = start + simple_seeds_iter.next().unwrap();
            let seeds_to_add: Vec<u64> = (*start..end).collect();
            seeds.extend_from_slice(&seeds_to_add);
        }
        Almanac {
            seeds,
            maps: lines.map(Map::from).collect(),
        }
    }

    fn parse_seeds(lines: &mut Split<&str>) -> Vec<u64> {
        lines.next().unwrap().split(':').next_back().unwrap().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect()
    }

    pub fn lowest_location(&self) -> u64 {
        self.seeds.iter().map(|&s| self.maps.iter().fold(s, |s, m| m.apply(s) as u64)).min().unwrap()
    }
}

impl Map {
    pub fn from(line: &str) -> Map {
        let mut parts = line.split('\n');
        parts.next();
        let ranges = parts.map(|p| {
            let numbers: Vec<i64> = p.split(' ').map(|n| n.parse().unwrap()).collect();
            Range { min: numbers[1], max: numbers[1] + numbers[2], diff: numbers[0] - numbers[1] }
        }).collect();

        Map { ranges }
    }

    pub fn apply(&self, seed: u64) -> i64 {
        self.ranges.iter()
            .find(|r| r.min <= seed as i64 && r.max >= seed as i64)
            .map(|r| r.diff + seed as i64).unwrap_or(seed as i64)
    }
}