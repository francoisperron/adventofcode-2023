use std::cmp::Ordering;
use std::collections::HashMap;
use std::usize;
use crate::day19::rule::Rule;

#[derive(Debug, PartialEq)]
pub struct PartPossibilities {
    pub possibilities: Possibilities,
    pub workflow: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Possibilities {
    pub parts: HashMap<char, RatingRange>,
}

impl Possibilities {
    pub fn new() -> Possibilities {
        Possibilities { parts: HashMap::from([('x', RatingRange::new()), ('m', RatingRange::new()), ('a', RatingRange::new()), ('s', RatingRange::new())]) }
    }

    fn clone_with_part(&self, part: char, range: RatingRange) -> Possibilities {
        let mut clone = self.clone();
        clone.parts.insert(part, range);
        clone
    }

    pub fn split(&self, rule: &Rule) -> (Possibilities, Option<PartPossibilities>) {
        let range = self.parts[&rule.part];
        let (range, new_range) = range.split(rule.operation, rule.rating);

        let possibilities = self.clone_with_part(rule.part, range);
        let new_possibilities = new_range.map(|r| PartPossibilities {
            possibilities: self.clone_with_part(rule.part, r),
            workflow: rule.next_workflow.clone(),
        });

        (possibilities, new_possibilities)
    }

    pub fn count(&self) -> usize {
        self.parts.values().map(|v| v.count()).product()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RatingRange(pub usize, pub usize);

impl RatingRange {
    pub fn new() -> RatingRange {
        RatingRange(1, 4000)
    }

    pub fn count(self) -> usize {
        1 + self.1 - self.0
    }

    fn split(self, operation: Ordering, split_at: usize) -> (RatingRange, Option<RatingRange>) {
        if self.0.cmp(&split_at) == operation {
            (RatingRange(split_at, self.1), Some(RatingRange(self.0, split_at - 1)))
        }
        else if self.1.cmp(&split_at) == operation {
            (RatingRange(self.0, split_at), Some(RatingRange(split_at + 1, self.1)))
        } else {
            (self, None)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::day19::part_possibilities::RatingRange;

    #[test]
    fn splits_range_when_before() {
        let range = RatingRange(2000, 4000);

        assert_eq!(range.split(Ordering::Less, 1000), (RatingRange(2000, 4000), None));
    }

    #[test]
    fn splits_range_when_between() {
        let range = RatingRange(1, 4000);

        assert_eq!(range.split(Ordering::Greater, 1000), (RatingRange(1, 1000), Some(RatingRange(1001, 4000))));
        assert_eq!(range.split(Ordering::Less, 1000), (RatingRange(1000, 4000), Some(RatingRange(1, 999))));
    }

    #[test]
    fn splits_range_when_after() {
        let range = RatingRange(1, 4000);

        assert_eq!(range.split(Ordering::Greater, 5000), (RatingRange(1, 4000), None));
    }
}