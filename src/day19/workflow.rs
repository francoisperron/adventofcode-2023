use std::cmp::Ordering;
use std::collections::HashMap;
use crate::day19::part::Part;
use crate::day19::rule::Rule;

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
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn from(input: &str) -> (String, Workflow) {
        let (name, rules) = input.split_once('{').unwrap();

        let rules = rules.strip_suffix('}').unwrap()
            .split(',')
            .map(Rule::from)
            .collect();

        (name.to_string(), Workflow { rules })
    }

    fn organize(&self, part: &mut Part) -> String {
        for rule in self.rules.iter() {
            if rule.operation != Ordering::Equal {
                let part_rating = part.ratings[&rule.part];
                if part_rating.cmp(&rule.rating) == rule.operation {
                    part.workflow = rule.next_workflow.clone();
                    break;
                } else {
                    continue;
                }
            } else {
                part.workflow = rule.next_workflow.clone();
            }
        }
        part.workflow.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::day19::part::Part;
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
    fn organizes_marching() {
        let mut part = Part::from("{x=1679,m=44,a=2067,s=496}");
        let (_, workflow) = Workflow::from("in{s<1351:px,qqz}");
        assert_eq!(workflow.organize(&mut part), "px")
    }
}