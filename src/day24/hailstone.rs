use crate::day24::xyz::Xyz;

#[derive(Debug, PartialEq)]
pub struct Hailstone {
    pub position: Xyz,
    pub velocity: Xyz,
    slope: f64,
}

impl Hailstone {
    pub fn from(input: &str) -> Hailstone {
        let (position, velocity) = input.split_once(" @ ").unwrap();
        let position = Xyz::from(position);
        let velocity = Xyz::from(velocity);
        let slope = velocity.y / velocity.x;
        Hailstone { position, velocity, slope }
    }

    pub fn intersects(&self, other: &Hailstone) -> Option<(f64, f64)> {
        if self.never_intercepts(other) {
            return None;
        }

        let x = (self.slope * self.position.x - other.slope * other.position.x + other.position.y - self.position.y) / (self.slope - other.slope);
        let y = (self.slope * other.slope * (other.position.x - self.position.x) + other.slope * self.position.y - self.slope * other.position.y) / (other.slope - self.slope);

        if self.in_the_past(x) || other.in_the_past(x) {
            return None;
        }

        Some((x, y))
    }

    fn never_intercepts(&self, other: &Hailstone) -> bool {
        (self.slope - other.slope).abs() <= f64::EPSILON
    }

    fn in_the_past(&self, x: f64) -> bool {
        self.velocity.x.signum() != (x - self.position.x).signum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hailstone() {
        let hailstone_a = Hailstone::from("19, 13, 30 @ -2, 1, -2");

        assert_eq!(hailstone_a, Hailstone { position: Xyz::new(19.0, 13.0, 30.0), velocity: Xyz::new(-2.0, 1.0, -2.0), slope: -0.5 });
    }

    #[test]
    fn finds_intersection() {
        let hailstone_a = Hailstone::from("19, 13, 30 @ -2, 1, -2");
        let hailstone_b = Hailstone::from("18, 19, 22 @ -1, -1, -2");

        assert_eq!(hailstone_a.intersects(&hailstone_b), Some((14.333333333333334, 15.333333333333334)));
    }

    #[test]
    fn finds_no_intersection_when_paths_crossed_in_the_past() {
        let hailstone_a = Hailstone::from("19, 13, 30 @ -2, 1, -2");
        let hailstone_b = Hailstone::from("20, 19, 15 @ 1, -5, -3");

        assert_eq!(hailstone_a.intersects(&hailstone_b), None);
    }

    #[test]
    fn finds_no_intersection_when_paths_never_intersect() {
        let hailstone_a = Hailstone::from("18, 19, 22 @ -1, -1, -2");
        let hailstone_b = Hailstone::from("20, 25, 34 @ -2, -2, -4");

        assert_eq!(hailstone_a.intersects(&hailstone_b), None);
    }
}