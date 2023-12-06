use std::str::Split;

#[cfg(test)]
mod tests {
    use crate::daily_input;
    use super::*;

    #[test]
    fn parses_races() {
        let races = Races::from("Time:      7  15   30\nDistance:  9  40  200");

        assert_eq!(races.races, vec![Race { time: 7, distance: 9 }, Race { time: 15, distance: 40 }, Race { time: 30, distance: 200 }]);
    }

    #[test]
    fn count_winning_ways() {
        let race = Race { time: 7, distance: 9 };

        assert_eq!(race.winning_ways(), 4);
    }

    #[test]
    fn solves_example_part1() {
        let races = Races::from("Time:      7  15   30\nDistance:  9  40  200");

        assert_eq!(races.product_of_winning_ways(), 288);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(6);
        let races = Races::from(&input);

        assert_eq!(races.product_of_winning_ways(), 281600);
    }

    #[test]
    fn parses_single_race() {
        let race = Race::from("Time:      7  15   30\nDistance:  9  40  200");

        assert_eq!(race, Race { time: 71530, distance: 940200 });
    }

    #[test]
    fn solves_example_part2() {
        let race = Race::from("Time:      7  15   30\nDistance:  9  40  200");

        assert_eq!(race.winning_ways(), 71503);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(6);
        let race = Race::from(&input);

        assert_eq!(race.winning_ways(), 33875953);
    }
}

struct Races {
    races: Vec<Race>,
}

#[derive(PartialEq, Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Races {
    pub fn from(input: &str) -> Races {
        let mut lines = input.split('\n');
        let times: Vec<u64> = Self::parse_line(&mut lines);
        let distances: Vec<u64> = Self::parse_line(&mut lines);

        let mut races = vec![];
        for (i, time) in times.into_iter().enumerate() {
            races.push(Race { time, distance: distances[i] });
        }

        Races { races }
    }

    fn parse_line(lines: &mut Split<char>) -> Vec<u64> {
        lines.next().unwrap().split_whitespace().skip(1).map(|n| n.parse().unwrap()).collect()
    }

    pub fn product_of_winning_ways(&self) -> u32 {
        self.races.iter().map(|r| r.winning_ways()).product()
    }
}

impl Race {
    pub fn from(input: &str) -> Race {
        let mut lines = input.split('\n');
        let time = Self::parse_line(&mut lines);
        let distance = Self::parse_line(&mut lines);

        Race { time, distance }
    }

    fn parse_line(lines: &mut Split<char>) -> u64 {
        lines.next().unwrap().split(':').nth(1).unwrap().replace(' ', "").parse().unwrap()
    }


    pub fn winning_ways(&self) -> u32 {
        let mut count = 0;

        for hold in 0..self.time {
            let distance = (self.time - hold) * hold;
            if distance > self.distance {
                count += 1
            }
        }

        count
    }
}