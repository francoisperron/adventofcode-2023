use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use crate::day19::part::Part;
use crate::day19::rule::Rule;
use crate::day19::part_possibilities::{PartPossibilities, Possibilities};

pub struct Workflows {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>
}

impl Workflows {
    pub fn from(input: &str) -> Workflows {
        let (workflows, parts) = input.split_once("\n\n").unwrap();
        let workflows = workflows.lines().map(Workflow::from).collect();
        let parts = parts.lines().map(Part::from).collect();
        Workflows { workflows, parts }
    }

    pub fn accepted_parts_ratings(&mut self) -> usize {
        for part in self.parts.iter_mut() {
            while part.workflow != "A" && part.workflow != "R" {
                self.workflows[&part.workflow].organize(part);
            }
        }
        self.parts.iter().filter(|p| p.workflow == "A").map(|p| p.ratings()).sum()
    }

    pub fn accepted_parts_ratings_possibilities(&self) -> usize {
        let mut total_possibilities = 0;
        let mut queue = VecDeque::new();

        queue.push_back(PartPossibilities { possibilities: Possibilities::new(), workflow: "in".to_string() });
        while let Some(possibility) = queue.pop_front() {
            match possibility.workflow.as_str() {
                "A" => total_possibilities += possibility.possibilities.count(),
                "R" => continue,
                _ => {
                    let mut new_queue = self.workflows[&possibility.workflow].organize_possibilities(possibility.possibilities);
                    queue.append(&mut new_queue);
                }
            };
        }

        total_possibilities
    }
}

pub struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn from(input: &str) -> (String, Workflow) {
        let (name, rules) = input.split_once('{').unwrap();

        let rules: Vec<Rule> = rules.strip_suffix('}').unwrap()
            .split(',')
            .map(Rule::from)
            .collect();

        (name.to_string(), Workflow { rules })
    }

    fn organize(&self, part: &mut Part) -> String {
        for rule in self.rules.iter() {
            if rule.operation == Ordering::Equal {
                part.workflow = rule.next_workflow.clone();
            } else {
                let part_rating = part.ratings[&rule.part];
                if part_rating.cmp(&rule.rating) == rule.operation {
                    part.workflow = rule.next_workflow.clone();
                    break;
                }
            }
        }
        part.workflow.clone()
    }

    fn organize_possibilities(&self, possibilities: Possibilities) -> VecDeque<PartPossibilities> {
        let mut queue = VecDeque::new();
        let mut next_possibilities = possibilities;

        for rule in &self.rules {
            if rule.operation == Ordering::Equal {
                queue.push_back(PartPossibilities { possibilities: next_possibilities.clone(), workflow: rule.next_workflow.clone() });
            } else {
                let (possibilities, new_possibilities) = next_possibilities.split(rule);
                next_possibilities = possibilities;
                if let Some(split) = new_possibilities {
                    queue.push_back(split);
                }
            }
        }

        queue
    }

    fn last_rule_workflow(&self) -> String {
        self.rules.iter().last().unwrap().next_workflow.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::day19::part::Part;
    use crate::day19::part_possibilities::{RatingRange, Possibilities};
    use crate::day19::workflow::Workflow;

    #[test]
    fn parses_workflow() {
        let (name, workflow) = Workflow::from("ex{x>10:one,m<20:two,a>30:R,A}");

        assert_eq!(name, "ex");
        assert_eq!(workflow.rules.len(), 4);
    }

    #[test]
    fn organizes_not_matching() {
        let mut part = Part::from("{x=787,m=2655,a=1222,s=2876}");
        let (_, workflow) = Workflow::from("in{s<1351:px,qqz}");

        assert_eq!(workflow.organize(&mut part), "qqz")
    }

    #[test]
    fn organizes_matching() {
        let mut part = Part::from("{x=1679,m=44,a=2067,s=496}");
        let (_, workflow) = Workflow::from("in{s<1351:px,qqz}");

        assert_eq!(workflow.organize(&mut part), "px")
    }

    #[test]
    fn organize_possibilities_splitting_them_on_first_rule() {
        let (_, workflow) = Workflow::from("ex{x>10:one,A}");

        let new_possibilities = workflow.organize_possibilities(Possibilities::new());

        assert_eq!(new_possibilities.len(), 2);
        let mut iter = new_possibilities.iter();
        assert_eq!(iter.next().unwrap().possibilities.parts, HashMap::from([('x', RatingRange(11, 4000)), ('m', RatingRange::new()), ('a', RatingRange::new()), ('s', RatingRange::new())]));
        assert_eq!(iter.next().unwrap().possibilities.parts, HashMap::from([('x', RatingRange(1, 10)), ('m', RatingRange::new()), ('a', RatingRange::new()), ('s', RatingRange::new())]));
    }

    #[test]
    fn organize_possibilities_splitting_them_on_all_rules() {
        let (_, workflow) = Workflow::from("ex{x>10:one,m<20:two,a>30:R,A}");

        let new_possibilities = workflow.organize_possibilities(Possibilities::new());

        assert_eq!(new_possibilities.len(), 4);
        let mut iter = new_possibilities.iter();
        assert_eq!(iter.next().unwrap().possibilities.parts, HashMap::from([('x', RatingRange(11, 4000)), ('m', RatingRange::new()), ('a', RatingRange::new()), ('s', RatingRange::new())]));
        assert_eq!(iter.next().unwrap().possibilities.parts, HashMap::from([('x', RatingRange(1, 10)), ('m', RatingRange(1, 19)), ('a', RatingRange::new()), ('s', RatingRange::new())]));
        assert_eq!(iter.next().unwrap().possibilities.parts, HashMap::from([('x', RatingRange(1, 10)), ('m', RatingRange(20, 4000)), ('a', RatingRange(31, 4000)), ('s', RatingRange::new())]));
        assert_eq!(iter.next().unwrap().possibilities.parts, HashMap::from([('x', RatingRange(1, 10)), ('m', RatingRange(20, 4000)), ('a', RatingRange(1, 30)), ('s', RatingRange::new())]));
    }
}