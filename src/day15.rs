use std::collections::HashMap;
use regex::Regex;

#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day15::{Boxes, Sequence, Lens};

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hashes_string() {
        assert_eq!(Sequence::hash_string("HASH"), 52);
        assert_eq!(Sequence::hash_string("rn=1"), 30);
        assert_eq!(Sequence::hash_string("qp=3"), 97);
    }

    #[test]
    fn solves_example_part1() {
        let sequence = Sequence::from(EXAMPLE);

        assert_eq!(sequence.hash(), 1320)
    }

    #[test]
    fn solves_part1() {
        let sequence = Sequence::from(&daily_input(15));

        assert_eq!(sequence.hash(), 508498)
    }

    #[test]
    fn parses_lens() {
        let lens = Lens::from("rn=1");
        assert_eq!(lens.label, "rn");
        assert_eq!(lens.box_index, 0);
        assert_eq!(lens.operation, '=');
        assert_eq!(lens.focal_length, 1);

        let lens = Lens::from("qp-");
        assert_eq!(lens.label, "qp");
        assert_eq!(lens.box_index, 1);
        assert_eq!(lens.operation, '-');
    }

    #[test]
    fn adds_lens_in_boxes() {
        let mut boxes = Boxes::new();

        boxes.add_or_replace(Lens::from("rn=1"));
        boxes.add_or_replace(Lens::from("qp=3"));

        assert_eq!(boxes.boxes[&0], vec![Lens::from("rn=1")]);
        assert_eq!(boxes.boxes[&1], vec![Lens::from("qp=3")]);
    }

    #[test]
    fn updates_lens_in_boxes() {
        let mut boxes = Boxes::new();

        boxes.add_or_replace(Lens::from("rn=1"));
        boxes.add_or_replace(Lens::from("cm=2"));
        boxes.add_or_replace(Lens::from("rn=3"));

        assert_eq!(boxes.boxes[&0], vec![Lens::from("rn=3"), Lens::from("cm=2")]);
    }

    #[test]
    fn removes_lens_from_boxes() {
        let mut boxes = Boxes::new();

        boxes.add_or_replace(Lens::from("pc=4"));
        boxes.add_or_replace(Lens::from("ot=9"));
        boxes.add_or_replace(Lens::from("ab=5"));
        boxes.remove(&Lens::from("pc-"));

        assert_eq!(boxes.boxes[&3], vec![Lens::from("ot=9"), Lens::from("ab=5")]);
    }

    #[test]
    fn ignores_removing_from_empty_box() {
        let mut boxes = Boxes::new();

        boxes.remove(&Lens::from("pc-"));

        assert_eq!(boxes.boxes.get(&3), None);
    }

    #[test]
    fn ignores_removing_absent_lens() {
        let mut boxes = Boxes::new();

        boxes.add_or_replace(Lens::from("ot=9"));
        boxes.remove(&Lens::from("pc-"));

        assert_eq!(boxes.boxes[&3], vec![Lens::from("ot=9")]);
    }

    #[test]
    fn calculates_boxes_focusing_power() {
        let mut boxes = Boxes::new();

        boxes.add_or_replace(Lens::from("cm=2"));
        boxes.add_or_replace(Lens::from("ot=7"));
        boxes.add_or_replace(Lens::from("ot=7"));

        assert_eq!(boxes.focusing_power(), 30);
    }

    #[test]
    fn solves_example_part2() {
        let sequence = Sequence::from(EXAMPLE);

        assert_eq!(sequence.focusing_power(), 145)
    }

    #[test]
    #[ignore]
    fn solves_part2() {
        let sequence = Sequence::from(&daily_input(15));

        assert_eq!(sequence.focusing_power(), 279116)
    }
}

struct Sequence {
    steps: Vec<String>,
}

impl Sequence {
    pub fn from(input: &str) -> Sequence {
        Sequence { steps: input.split(',').map(|l| l.to_string()).collect() }
    }

    pub fn hash(&self) -> u32 {
        self.steps.iter().map(|s| Sequence::hash_string(s)).sum()
    }

    fn hash_string(input: &str) -> u32 {
        input.chars()
            .map(|c| c as u32)
            .fold(0, Sequence::hash_char)
    }

    fn hash_char(hash: u32, value: u32) -> u32 {
        ((hash + value) * 17) % 256
    }

    pub fn focusing_power(&self) -> u32 {
        let mut boxes = Boxes::new();

        for step in self.steps.as_slice() {
            let lens = Lens::from(step);
            match lens.operation {
                '=' => boxes.add_or_replace(lens),
                '-' => boxes.remove(&lens),
                _ => panic!()
            }
        }

        boxes.focusing_power()
    }
}

#[derive(Debug, PartialEq)]
struct Lens {
    label: String,
    box_index: u32,
    operation: char,
    focal_length: u32,
}

impl Lens {
    pub fn from(step: &str) -> Lens {
        let regex = Regex::new(r"(?<label>[a-z]+)(?<operation>[=|-])(?<focal_length>[0-9]*)").unwrap();
        let (_, [label, operation, focal_length]) = regex.captures(step).map(|g| g.extract()).unwrap();

        let box_index = Sequence::hash_string(label);
        let focal_length = focal_length.parse().unwrap_or(0);

        Lens { label: label.to_string(), box_index, operation: operation.chars().next().unwrap(), focal_length }
    }
}

struct Boxes {
    boxes: HashMap<u32, Vec<Lens>>,
}

impl Boxes {
    pub fn new() -> Boxes {
        Boxes { boxes: HashMap::new() }
    }

    pub fn add_or_replace(&mut self, lens: Lens) {
        if let Some(lenses) = self.boxes.get_mut(&lens.box_index) {
            if let Some(box_index) = lenses.iter().position(|s| s.label == lens.label) {
                lenses[box_index] = lens;
            } else {
                lenses.push(lens);
            }
        } else {
            self.boxes.insert(lens.box_index, vec![lens]);
        }
    }

    pub fn remove(&mut self, lens: &Lens) {
        if let Some(lenses) = self.boxes.get_mut(&lens.box_index) {
            lenses.retain(|l| l.label != lens.label);
        }
    }

    pub fn focusing_power(&self) -> u32 {
        self.boxes.iter()
            .map(|(box_number, lenses)| {
                lenses.iter().enumerate()
                    .map(|(box_index, lens)| (box_number + 1) * (box_index as u32 + 1) * lens.focal_length)
                    .sum::<u32>()
            })
            .sum::<u32>()
    }
}