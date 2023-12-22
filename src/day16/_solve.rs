#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day16::beam::Beam;
    use crate::day16::direction::Direction;
    use crate::day16::grid::Grid;

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn solves_example_part1() {
        let grid = Grid::from(EXAMPLE);

        assert_eq!(energize(&grid), 46);
    }

    #[test]
    fn solves_part1() {
        let grid = Grid::from(&daily_input(16));

        assert_eq!(energize(&grid), 8034);
    }

    #[test]
    fn solves_example_part2() {
        let grid = Grid::from(EXAMPLE);

        assert_eq!(max_energize(&grid), 51);
    }

    #[test]
    #[ignore] // 2secs
    fn solves_part2() {
        let grid = Grid::from(&daily_input(16));

        assert_eq!(max_energize(&grid), 8225);
    }

    fn energize(grid: &Grid) -> usize {
        let mut beam = Beam::new(0, 0, Direction::Right);
        beam.start_travel(&grid);
        beam.energized.len()
    }

    fn max_energize(grid: &Grid) -> usize {
        let mut max_energized = 0;

        for y in 0..=grid.max_y {
            let mut beam = Beam::new(0, y, Direction::Right);
            max_energized = max_energized.max(beam.start_travel(&grid).energized.len());
            let mut beam = Beam::new(grid.max_x, y, Direction::Left);
            max_energized = max_energized.max(beam.start_travel(&grid).energized.len());
        }
        for x in 0..=grid.max_x {
            let mut beam = Beam::new(x, 0, Direction::Down);
            max_energized = max_energized.max(beam.start_travel(&grid).energized.len());
            let mut beam = Beam::new(x, grid.max_y, Direction::Up);
            max_energized = max_energized.max(beam.start_travel(&grid).energized.len());
        }
        max_energized
    }
}