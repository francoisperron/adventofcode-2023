#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day22::bricks::Bricks;


    #[test]
    fn solves_example_part1() {
        let mut bricks = Bricks::from(EXAMPLE);

        assert_eq!(bricks.safe_bricks_count(), 5);
    }

    #[test]
    fn solves_part1() {
        let mut bricks = Bricks::from(&daily_input(22));

        assert_eq!(bricks.safe_bricks_count(), 403);
    }

    #[test]
    fn solves_example_part2() {
        let mut bricks = Bricks::from(EXAMPLE);

        assert_eq!(bricks.disintegrate_bricks_count(), 7);
    }

    #[test]
    fn solves_part2() {
        let mut bricks = Bricks::from(&daily_input(22));

        assert_eq!(bricks.disintegrate_bricks_count(), 70189);
    }

    const EXAMPLE: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
}