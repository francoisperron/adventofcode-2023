#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day17::city::{City, Crucible};

    const EXAMPLE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const CRUCIBLE: Crucible = Crucible { min_move: 1, max_move: 3 };
    const ULTRA_CRUCIBLE: Crucible = Crucible { min_move: 4, max_move: 10 };

    #[test]
    fn solves_simple() {
        let city = City::from("1234\n5678");

        assert_eq!(city.shortest_path(&CRUCIBLE), 17);
    }

    #[test]
    fn solves_example_part1() {
        let city = City::from(EXAMPLE);

        assert_eq!(city.shortest_path(&CRUCIBLE), 102);
    }

    #[test]
    fn solves_part1() {
        let city = City::from(&daily_input(17));

        assert_eq!(city.shortest_path(&CRUCIBLE), 866);
    }

    #[test]
    fn solves_example_part2() {
        let city = City::from(EXAMPLE);

        assert_eq!(city.shortest_path(&ULTRA_CRUCIBLE), 94);
    }

    #[test]
    fn solves_part2() {
        let city = City::from(&daily_input(17));

        assert_eq!(city.shortest_path(&ULTRA_CRUCIBLE), 1010);
    }
}