use std::cmp::Ordering;

#[derive(Debug)]
pub struct Rule {
    pub part: char,
    pub operation: Ordering,
    pub rating: usize,
    pub next_workflow: String,
}

impl Rule {
    pub fn from(input: &str) -> Rule {
        match input {
            i if i.contains('<') || i.contains('>') => Self::from_greater_or_less(input),
            "A" => Self { next_workflow: "A".to_string(), part: ' ', rating: 0, operation: Ordering::Equal },
            "R" => Self { next_workflow: "R".to_string(), part: ' ', rating: 0, operation: Ordering::Equal },
            i => Self { next_workflow: i.to_string(), part: ' ', rating: 0, operation: Ordering::Equal }
        }
    }

    pub fn from_greater_or_less(input: &str) -> Rule {
        let (op, workflow) = input.split_once(':').unwrap();

        let mut chars = op.chars();
        let part = chars.next().unwrap();
        let operation = chars.next();
        let operation = match operation {
            Some('>') => Ordering::Greater,
            Some('<') => Ordering::Less,
            _ => panic!()
        };
        let rating = chars.as_str().parse().unwrap();
        let next_workflow = workflow.to_string();
        Rule { part, operation, rating, next_workflow }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::day19::rule::Rule;

    #[test]
    fn parses_greater_than_rule() {
        let rule = Rule::from("x>10:one");

        assert_eq!(rule.part, 'x');
        assert_eq!(rule.operation, Ordering::Greater);
        assert_eq!(rule.rating, 10);
        assert_eq!(rule.next_workflow, "one");
    }

    #[test]
    fn parses_less_than_rule() {
        let rule = Rule::from("m<20:R");

        assert_eq!(rule.part, 'm');
        assert_eq!(rule.operation, Ordering::Less);
        assert_eq!(rule.rating, 20);
        assert_eq!(rule.next_workflow, "R");
    }

    #[test]
    fn parses_accepted_rule() {
        let rule = Rule::from("A");

        assert_eq!(rule.next_workflow, "A");
    }

    #[test]
    fn parses_rejected_rule() {
        let rule = Rule::from("R");

        assert_eq!(rule.next_workflow, "R");
    }

    #[test]
    fn parses_workflow_rule() {
        let rule = Rule::from("rfg");

        assert_eq!(rule.next_workflow, "rfg");
    }
}