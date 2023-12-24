use std::collections::HashMap;
use regex::Regex;
use crate::day19::part::Part;
use crate::day19::rule::Rule;

pub struct Workflows {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>
}

impl Workflows {
    pub fn from(input: &str) -> Workflows {
        let mut split = input.split("\n\n");
        let workflows = split.next().unwrap().lines().map(Workflow::from).collect();
        let parts = split.next().unwrap().lines().map(Part::from).collect();
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
        let re_main = Regex::new(r"(?<workflow>[a-z]+)\{(?<rules>[^}]+)}").unwrap();
        let (_, [name, rules_input]) = re_main.captures(input).map(|g| g.extract()).unwrap();

        let re_rule = Regex::new(r"(?<rule>[^,]*)").unwrap();
        let rules = re_rule.captures_iter(rules_input)
            .map(|cap_rule| Rule::from(&cap_rule["rule"]))
            .collect();

        (name.to_string(), Workflow { rules })
    }

    fn organize(&self, part: &mut Part) -> String {
        for rule in self.rules.iter() {
            if !rule.operation.is_empty() {
                let part_rating = part.ratings[&rule.part];
                if (rule.operation == ">" && part_rating > rule.rating) || (rule.operation == "<" && part_rating < rule.rating) {
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