use std::cmp::Ordering;
use crate::day12::Spring::{Damaged, Operational, Unknown};

#[cfg(test)]
mod tests {
    use crate::daily_input;
    use crate::day12::{HotSprings, Row};
    use crate::day12::Spring::{Damaged, Operational, Unknown};

    const EXAMPLE_INPUT: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn parses_row() {
        let row = Row::from("???.### 1,1,3");

        assert_eq!(row.springs, vec![Unknown, Unknown, Unknown, Operational, Damaged, Damaged, Damaged]);
        assert_eq!(row.records, vec![1, 1, 3]);
    }

    #[test]
    fn finds_unique_arrangement() {
        assert_eq!(Row::from("# 1").arrangements(), 1);
        assert_eq!(Row::from("? 1").arrangements(), 1);
        assert_eq!(Row::from(".# 1").arrangements(), 1);
        assert_eq!(Row::from("#. 1").arrangements(), 1);
        assert_eq!(Row::from(".? 1").arrangements(), 1);
        assert_eq!(Row::from("?. 1").arrangements(), 1);
    }

    #[test]
    fn finds_damaged_springs_groups() {
        // groups of any # or ? followed by a .
        assert!(Row::from("###. 1,1").is_a_damaged_group(0, 3));
        assert!(Row::from("???. 1,1").is_a_damaged_group(0, 3));
        assert!(Row::from("#?#. 1,1").is_a_damaged_group(0, 3));

        // groups ending at the end
        assert!(Row::from("### 1,1").is_a_damaged_group(0, 3));
        assert!(Row::from("??? 1,1").is_a_damaged_group(0, 3));

        // groups too big are excluded
        assert!(!Row::from("### 1,1").is_a_damaged_group(0, 4));
        assert!(!Row::from("??? 1,1").is_a_damaged_group(0, 4));
    }

    #[test]
    fn finds_arrangements_in_row() {
        assert_eq!(Row::from("???.### 1,1,3").arrangements(), 1);
        assert_eq!(Row::from(".??..??...?##. 1,1,3").arrangements(), 4);
        assert_eq!(Row::from("?#?#?#?#?#?#?#? 1,3,1,6").arrangements(), 1);
        assert_eq!(Row::from("????.#...#... 4,1,1").arrangements(), 1);
        assert_eq!(Row::from("????.######..#####. 1,6,5").arrangements(), 4);
        assert_eq!(Row::from("?###???????? 3,2,1").arrangements(), 10);
    }

    #[test]
    fn solves_example_part1() {
        let springs = HotSprings::from(EXAMPLE_INPUT);

        assert_eq!(springs.arrangements(), 21);
    }

    #[test]
    fn solves_part1() {
        let springs = HotSprings::from(&daily_input(12));

        assert_eq!(springs.arrangements(), 7169);
    }
}

struct HotSprings {
    rows: Vec<Row>,
}

impl HotSprings {
    pub fn from(input: &str) -> HotSprings {
        HotSprings { rows: input.lines().map(Row::from).collect() }
    }

    pub fn arrangements(&self) -> usize {
        self.rows.iter().map(|r| r.arrangements()).sum()
    }
}

struct Row {
    springs: Vec<Spring>,
    records: Vec<usize>,
}

impl Row {
    pub fn from(input: &str) -> Row {
        let (springs, records) = input.split_once(' ').unwrap();
        Row {
            springs: springs.chars().map(Spring::from).collect(),
            records: records.split(',').map(|v| v.parse().unwrap()).collect(),
        }
    }

    pub fn arrangements(&self) -> usize {
        self.is_operational(0, 0)
    }

    fn is_operational(&self, spring: usize, record: usize) -> usize {
        if spring == self.springs.len() {
            return if record == self.records.len() { 1 } else { 0 };
        }

        match self.springs[spring] {
            Operational => self.is_operational(spring + 1, record),
            Damaged => self.is_damaged(spring, record),
            Unknown => self.is_damaged(spring, record) + self.is_operational(spring + 1, record),
        }
    }

    fn is_damaged(&self, spring: usize, record: usize) -> usize {
        if record == self.records.len() {
            return 0;
        }

        let spring_end = spring + self.records[record];
        if !self.is_a_damaged_group(spring, spring_end) {
            return 0;
        }

        if spring_end == self.springs.len() {
            return if record == self.records.len() - 1 { 1 } else { 0 };
        }

        self.is_operational(spring_end + 1, record + 1)
    }

    fn is_a_damaged_group(&self, start: usize, end: usize) -> bool {
        match end.cmp(&self.springs.len()) {
            Ordering::Less => !(start..end).any(|s| self.springs[s] == Operational) && self.springs[end] != Damaged,
            Ordering::Equal => !(start..end).any(|s| self.springs[s] == Operational),
            Ordering::Greater => false,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    pub fn from(value: char) -> Spring {
        match value {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!()
        }
    }
}


