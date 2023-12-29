#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day24::hailstones::Hailstones;
    use crate::day24::xyz::Xyz;

    #[test]
    fn solves_example_part1() {
        let hailstones = Hailstones::from(EXAMPLE);

        assert_eq!(hailstones.intersections_between(7.0, 27.0), 2);
    }

    #[test]
    fn solves_part1() {
        let hailstones = Hailstones::from(&daily_input(24));

        assert_eq!(hailstones.intersections_between(200000000000000.0, 400000000000000.0), 24192)
    }

    #[test]
    fn solves_example_part2() {
        let hailstones = Hailstones::from(EXAMPLE);

        let pos = hailstones.rock_position_to_obliterate_all_hailstone_to_dust();
        assert_eq!(pos, Xyz::new(24.0, 13.0, 10.0));
        assert_eq!(pos.x + pos.y + pos.z, 47.0)
    }

    #[test]
    fn solves_part2() {
        let hailstones = Hailstones::from(&daily_input(24));

        let pos = hailstones.rock_position_to_obliterate_all_hailstone_to_dust();
        assert_eq!(pos, Xyz::new(191146615936494.0, 342596108503183.0, 131079628110881.0));
        assert_eq!(pos.x + pos.y + pos.z, 664822352550558.0)
    }

    const EXAMPLE: &str = "\
19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";
}