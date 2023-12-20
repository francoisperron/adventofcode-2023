#[cfg(test)]
mod tests {
    use crate::daily_input;
    use crate::day15::Sequence;

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

}

struct Sequence {
    steps: Vec<String>
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
}
