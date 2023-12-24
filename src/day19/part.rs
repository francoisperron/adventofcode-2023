use std::collections::HashMap;

#[derive(Debug)]
pub struct Part {
    pub ratings: HashMap<char, usize>,
    pub workflow: String,
}

impl Part {
    pub fn from(input: &str) -> Part {
        let trimmed = &input[1..input.len() - 1];
        let ratings: HashMap<char, usize> = trimmed
            .split(',')
            .filter_map(|p| p.split_once('='))
            .map(|(p, r)| (p.chars().next().unwrap(), r.parse().unwrap()))
            .collect();

        Part { ratings, workflow: "in".to_string() }
    }

    pub fn ratings(&self) -> usize {
        self.ratings.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_part() {
        let part = Part::from("{x=787,m=2655,a=1222,s=2876}");

        assert_eq!(part.ratings[&'x'], 787);
        assert_eq!(part.ratings[&'m'], 2655);
        assert_eq!(part.ratings[&'a'], 1222);
        assert_eq!(part.ratings[&'s'], 2876);

        assert_eq!(part.workflow, "in");
    }

    #[test]
    fn sums_ratings() {
        let part = Part::from("{x=787,m=2655,a=1222,s=2876}");

        assert_eq!(part.ratings(), 787 + 2655 + 1222 + 2876)
    }
}