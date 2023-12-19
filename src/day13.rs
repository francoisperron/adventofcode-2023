use itertools::Itertools;

#[cfg(test)]
mod tests {
    use crate::daily_input;
    use super::*;

    const EXAMPLE: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn finds_horizontal_reflection() {
        let pattern = Pattern::from("\
        #..#\n\
        ..#.\n\
        ..#.\n\
        #..#");

        assert_eq!(pattern.horizontal_reflection(0), Some(2))
    }

    #[test]
    fn finds_example_horizontal_reflection() {
        let pattern = Pattern::from(EXAMPLE.split("\n\n").nth(1).unwrap());

        assert_eq!(pattern.horizontal_reflection(0), Some(4))
    }

    #[test]
    fn finds_vertical_reflection() {
        let pattern = Pattern::from("\
        #..#\n\
        .##.\n\
        #..#");

        assert_eq!(pattern.vertical_reflection(0), Some(2))
    }

    #[test]
    fn finds_example_vertical_reflection() {
        let pattern = Pattern::from(EXAMPLE.split("\n\n").next().unwrap());

        assert_eq!(pattern.vertical_reflection(0), Some(5))
    }

    #[test]
    fn solves_example_part1() {
        let patterns = Patterns::from(EXAMPLE);

        assert_eq!(patterns.summarize(0), 405);
    }

    #[test]
    fn solves_part1() {
        let patterns = Patterns::from(&daily_input(13));

        assert_eq!(patterns.summarize(0), 35210);
    }

    #[test]
    fn find_diff_between_two_strings() {
        assert_eq!(Pattern::diff_between("#..#", "#..#"), 0);
        assert_eq!(Pattern::diff_between("...#", "#..#"), 1);
        assert_eq!(Pattern::diff_between("..##", "##.."), 4);
    }

    #[test]
    fn finds_horizontal_reflection_smudged() {
        let pattern = Pattern::from("\
        ...#\n\
        ..#.\n\
        ..#.\n\
        #..#");

        assert_eq!(pattern.horizontal_reflection(1), Some(2))
    }

    #[test]
    fn solves_example_part2() {
        let patterns = Patterns::from(EXAMPLE);

        assert_eq!(patterns.summarize(1), 400);
    }

    #[test]
    fn solves_part2() {
        let patterns = Patterns::from(&daily_input(13));

        assert_eq!(patterns.summarize(1), 31974);
    }
}

struct Patterns {
    patterns: Vec<Pattern>,
}

impl Patterns {
    pub fn from(input: &str) -> Patterns {
        Patterns { patterns: input.split("\n\n").map(Pattern::from).collect() }
    }

    pub fn summarize(&self, smudged: u32) -> usize {
        self.patterns.iter()
            .map(|p| p.vertical_reflection(smudged).unwrap_or(0)+ p.horizontal_reflection(smudged).unwrap_or(0)  * 100)
            .sum()
    }
}

struct Pattern {
    lines: Vec<String>,
}

impl Pattern {
    pub fn from(notes: &str) -> Pattern {
        let lines = notes.lines().map(|l| l.to_string()).collect();
        Pattern { lines }
    }

    pub fn horizontal_reflection(&self, smudged: u32) -> Option<usize> {
        Self::find_reflection(&self.lines, smudged)
    }

    pub fn vertical_reflection(&self, smudged: u32) -> Option<usize> {
        let rows = self.rows_to_lines();
        Self::find_reflection(&rows, smudged)
    }

    fn rows_to_lines(&self) -> Vec<String> {
        let mut rows = vec![];
        for i in 0..self.lines[0].len() {
            let row = self.lines.iter().map(|l| l.chars().nth(i).unwrap()).join("");
            rows.push(row);
        }
        rows
    }

    fn find_reflection(vec: &Vec<String>, smudged: u32) -> Option<usize> {
        for i in 1..vec.len() {
            let (a, b) = vec.split_at(i);
            let a: String = a.iter().rev().map(|s| s.to_string()).collect();
            let b = b.join("");

            let min = *[a.len(), b.len()].iter().min().unwrap();

            if Self::diff_between(a.split_at(min).0, b.split_at(min).0) == smudged {
                return Some(i);
            }
        }
        None
    }

    fn diff_between(a: &str, b: &str) -> u32 {
        a.chars().zip(b.chars())
            .filter(|(a, b)| a != b)
            .count() as u32
    }
}