#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day21::garden_map::GardenMap;

    #[test]
    fn solves_example_part1() {
        let mut map = GardenMap::from(EXAMPLE);

        assert_eq!(map.gardens_visited(6), 16);
    }

    #[test]
    fn solves_part1() {
        let mut map = GardenMap::from(&daily_input(21));

        assert_eq!(map.gardens_visited(64), 3820);
    }

    #[test]
    fn solves_part2() {
        let mut map = GardenMap::from(&daily_input(21));

        assert_eq!(map.infinite_gardens_visited(26501365), 632421652138917);
    }

    const EXAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
}