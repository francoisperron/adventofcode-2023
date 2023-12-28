use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Brick(pub Xyz, pub Xyz);

type Xyz = (usize, usize, usize);

impl Brick {
    pub fn from(line: &str) -> Brick {
        let (end1, end2) = line.split_once('~').unwrap();
        Brick(Self::parse_xyz(end1), Self::parse_xyz(end2))
    }

    fn parse_xyz(s: &str) -> Xyz {
        s.split(',').map(|v| v.parse::<usize>().unwrap()).collect_tuple().unwrap()
    }

    pub fn is_in_the_air(&self) -> bool {
        self.0.2 > 1
    }

    pub fn encounters(&self, other: &Brick) -> bool {
        Self::intersect_on_axis((self.0.0, self.1.0), (other.0.0, other.1.0)) &&
            Self::intersect_on_axis((self.0.1, self.1.1), (other.0.1, other.1.1))
    }

    fn intersect_on_axis((min1, max1): (usize, usize), (min2, max2): (usize, usize)) -> bool {
        (min1..=max1).contains(&min2) || (min1..=max1).contains(&max2) || (min2..=max2).contains(&min1) || (min2..=max2).contains(&max1)
    }

    pub fn fall(&mut self) {
        self.0.2 -= 1;
        self.1.2 -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_brick() {
        let b = Brick::from("1,0,1~1,2,1");

        assert_eq!(b, Brick((1, 0, 1), (1, 2, 1)));
    }

    #[test]
    fn two_bricks_cannot_be_at_the_same_place() {
        let a = Brick::from("0,0,10~1,0,10");

        let b = Brick::from("0,0,10~0,1,10");
        assert!(a.encounters(&b));

        let c = Brick::from("2,2,2~2,2,2");
        assert!(!a.encounters(&c));
    }
}
