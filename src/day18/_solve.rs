#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day18::dig_plan::DigPlan;

    const EXAMPLE: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn solves_example_part1() {
        let dig_plan = DigPlan::from(EXAMPLE);

        assert_eq!(dig_plan.volume(), 62);
    }

    #[test]
    fn solves_part1() {
        let dig_plan = DigPlan::from(&daily_input(18));

        assert_eq!(dig_plan.volume(), 66993);
    }

    #[test]
    fn solves_example_part2() {
        let dig_plan = DigPlan::from_color(EXAMPLE);

        assert_eq!(dig_plan.volume(), 952408144115);
    }

    #[test]
    fn solves_part2() {
        let dig_plan = DigPlan::from_color(&daily_input(18));

        assert_eq!(dig_plan.volume(), 177243763226648);
    }
}
