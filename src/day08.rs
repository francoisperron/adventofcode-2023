use std::collections::HashMap;
use regex::Regex;

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::daily_input;
    use crate::day08::Maps;

    const EXAMPLE_INPUT: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_INPUT_2: &str = "LR\n\n 11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

    #[test]
    fn parses_input() {
        let map = Maps::from(EXAMPLE_INPUT);

        assert_eq!(map.instructions, "LLR");
        assert_eq!(map.nodes.len(), 3)
    }

    #[test]
    fn solves_example_part1() {
        let map = Maps::from(EXAMPLE_INPUT);

        assert_eq!(map.steps_to_reach_the_end_simple(), 6)
    }

    #[test]
    fn solves_part1() {
        let mut input = daily_input(8);
        input.pop();
        let map = Maps::from(&input);

        assert_eq!(map.steps_to_reach_the_end_simple(), 14257)
    }

    #[test]
    fn calculates_greatest_common_divisor() {
        assert_eq!(Maps::gcd(6, 2), 2);
        assert_eq!(Maps::gcd(6, 3), 3);
        assert_eq!(Maps::gcd(6, 4), 2);
        assert_eq!(Maps::gcd(6, 5), 1);
    }

    #[test]
    fn calculates_least_common_multiple() {
        assert_eq!(Maps::lcm(6, 2), 6);
        assert_eq!(Maps::lcm(6, 3), 6);
        assert_eq!(Maps::lcm(6, 4), 12);
        assert_eq!(Maps::lcm(6, 5), 30);
    }

    #[test]
    fn finds_starting_nodes() {
        let map = Maps::from(EXAMPLE_INPUT_2);

        let starting_nodes = map.starting_nodes().into_iter().sorted().collect::<Vec<&String>>();
        assert_eq!(starting_nodes, vec!["11A", "22A"])
    }

    #[test]
    fn solves_example_part2() {
        let map = Maps::from(EXAMPLE_INPUT_2);

        assert_eq!(map.steps_to_reach_the_end_in_parallel(), 6)
    }

    #[test]
    fn solves_part2() {
        let mut input = daily_input(8);
        input.pop();
        let map = Maps::from(&input);

        assert_eq!(map.steps_to_reach_the_end_in_parallel(), 16187743689077)
    }
}

struct Maps {
    instructions: String,
    nodes: HashMap<String, Node>,
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Maps {
    pub fn from(input: &str) -> Maps {
        let mut parts = input.split("\n\n");
        let instructions = parts.next().unwrap().to_string();

        let mut nodes = HashMap::new();
        for line in parts.next().unwrap().split('\n') {
            let regex = Regex::new(r"(?<id>[A-Z0-9]+) = \((?<left>[A-Z0-9]+), (?<right>[A-Z0-9]+)\)").unwrap();
            let (_, [id, left, right]) = regex.captures(line).map(|g| g.extract()).unwrap();

            nodes.insert(id.to_string(), Node { left: left.to_string(), right: right.to_string() });
        }

        Maps { instructions, nodes }
    }

    pub fn steps_to_reach_the_end_simple(&self) -> usize {
        self.steps_to_reach_the_end("AAA")
    }

    pub fn steps_to_reach_the_end_in_parallel(&self) -> usize {
        self.starting_nodes().iter()
            .map(|starting_node| self.steps_to_reach_the_end(starting_node))
            .fold(1, Self::lcm)
    }

    fn steps_to_reach_the_end(&self, starting_node: &str) -> usize {
        let mut steps = 0;
        let mut instructions = self.instructions.chars().cycle();
        let mut current_node = &starting_node.to_string();

        while !current_node.ends_with('Z') {
            let next_node = self.nodes.get(current_node).unwrap();
            match instructions.next().unwrap() {
                'L' => current_node = &next_node.left,
                'R' => current_node = &next_node.right,
                _ => (),
            }
            steps += 1;
        }
        steps
    }

    fn starting_nodes(&self) -> Vec<&String> {
        self.nodes.keys().filter(|id| id.ends_with('A')).collect()
    }

    fn lcm(n1: usize, n2: usize) -> usize {
        n1 * n2 / Self::gcd(n1, n2)
    }

    fn gcd(mut n1: usize, mut n2: usize) -> usize {
        while n1 != 0 {
            if n1 < n2 {
                std::mem::swap(&mut n1, &mut n2);
            }
            n1 %= n2;
        }
        n2
    }
}