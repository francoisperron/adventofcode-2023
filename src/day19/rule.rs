use regex::Regex;

#[derive(Debug)]
pub struct Rule {
    pub part: char,
    pub operation: String,
    pub rating: usize,
    pub next_workflow: String,
}

impl Rule {
    pub fn from(input: &str) -> Rule {
        match input {
            i if i.contains('<') || i.contains('>') => Self::from_greater_or_less(input),
            "A" => Self { next_workflow: "A".to_string(), part: ' ', rating: 0, operation: "".to_string() },
            "R" => Self { next_workflow: "R".to_string(), part: ' ', rating: 0, operation: "".to_string() },
            i => Self { next_workflow: i.to_string(), part: ' ', rating: 0, operation: "".to_string() }
        }
    }

    pub fn from_greater_or_less(input: &str) -> Rule {
        let re_main = Regex::new(r"(?<part>[a-z]+)(?<operation>[<|>])(?<rating>[0-9}]+):(?<workflow>[A-Ra-z]+)").unwrap();
        let (_, [part, operation, rating, workflow]) = re_main.captures(input).map(|g| g.extract()).unwrap();

        Rule { part: part.chars().next().unwrap(), operation: operation.to_owned(), rating: rating.parse().unwrap(), next_workflow: workflow.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use crate::day19::rule::Rule;

    #[test]
    fn parses_greater_than_rule() {
        let rule = Rule::from("x>10:one");

        assert_eq!(rule.part, 'x');
        assert_eq!(rule.operation, ">");
        assert_eq!(rule.rating, 10);
        assert_eq!(rule.next_workflow, "one");
    }

    #[test]
    fn parses_less_than_rule() {
        let rule = Rule::from("m<20:R");

        assert_eq!(rule.part, 'm');
        assert_eq!(rule.operation, "<");
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