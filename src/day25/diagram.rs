use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub struct Diagram {
    connections: HashMap<String, HashSet<String>>,
}

impl Diagram {
    pub fn from(input: &str) -> Diagram {
        let mut connections = HashMap::new();

        for line in input.lines() {
            let (component, components) = line.split_once(": ").unwrap();

            for other in components.split(' ') {
                Self::link(&mut connections, component, other);
                Self::link(&mut connections, other, component);
            }
        }

        Diagram { connections }
    }

    fn link(connections: &mut HashMap<String, HashSet<String>>, component: &str, other: &str) {
        connections.entry(component.to_string())
            .and_modify(|e: &mut HashSet<String>| { e.insert(other.to_string()); })
            .or_insert(HashSet::from([other.to_string()]));
    }

    pub fn fix_overload(&self) -> usize {
        let mut components: HashSet<String> = self.connections.keys().map(|k| k.to_owned()).collect();

        while components.iter().map(|component| self.count_same_connections(&components, component)).sum::<usize>() != 3 {
            let component_with_same_connections = components.iter()
                .sorted()
                .max_by_key(|c| self.count_same_connections(&components, c))
                .unwrap();

            components.remove(&component_with_same_connections.to_string());
        }

        components.len() * (self.connections.len() - components.len())
    }

    fn count_same_connections(&self, components: &HashSet<String>, component: &str) -> usize {
        self.connections[component].difference(components).count()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::day25::diagram::Diagram;

    #[test]
    fn parses_components_bidirectionally() {
        let diagram = Diagram::from("jqt: rhn\nrhn: frs pzl");

        let expected = HashMap::from([
            ("jqt".to_string(), HashSet::from(["rhn".to_string()])),
            ("rhn".to_string(), HashSet::from(["jqt".to_string(), "frs".to_string(), "pzl".to_string()])),
            ("frs".to_string(), HashSet::from(["rhn".to_string()])),
            ("pzl".to_string(), HashSet::from(["rhn".to_string()])),
        ]);
        assert_eq!(diagram.connections, expected);
    }
}