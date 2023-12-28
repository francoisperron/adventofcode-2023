#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day23::trails::Trails;

    #[test]
    fn solves_example_part1() {
        let mut trails = Trails::from(EXAMPLE);

        assert_eq!(trails.longest_hike(), 94);
    }

    #[test]
    fn solves_part1() {
        let mut trails = Trails::from(&daily_input(23));

        assert_eq!(trails.longest_hike(), 2334);
    }

    #[test]
    fn solves_example_part2() {
        let mut trails = Trails::from_no_slopes(EXAMPLE);

        assert_eq!(trails.longest_hike(), 154);
    }

    #[test]
    #[ignore] // 160 secs in release mode on my mbp, dev mode gives a stack overflow
    fn solves_part2() {
        let mut trails = Trails::from_no_slopes(&daily_input(23));

        assert_eq!(trails.longest_hike(), 6422);
    }

    const EXAMPLE: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
}