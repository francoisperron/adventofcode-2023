#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day11::Universe;

    const EXAMPLE_INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn parses_number_of_galaxies_per_lines_and_columns() {
        let universe = Universe::from(EXAMPLE_INPUT);

        assert_eq!(universe.lines, vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 2]);
        assert_eq!(universe.columns, vec![2, 1, 0, 1, 1, 0, 1, 2, 0, 1]);
    }

    #[test]
    fn calculates_distance_between_galaxies() {
        assert_eq!(Universe::dist(&[1, 0, 1], 0), 2);
        assert_eq!(Universe::dist(&[2, 0, 1], 0), 4);
        assert_eq!(Universe::dist(&[1, 0, 2], 0), 4);
        assert_eq!(Universe::dist(&[2, 0, 2], 0), 8);

        assert_eq!(Universe::dist(&[1, 1], 0), 1);
        assert_eq!(Universe::dist(&[1, 1, 1], 0), 1 + 3);
        assert_eq!(Universe::dist(&[1, 1, 1, 1], 0), 1 + 3 + 6);
    }

    #[test]
    fn adds_expansion_to_each_distance() {
        assert_eq!(Universe::dist(&[1, 0, 1], 1), 2 + 1);
        assert_eq!(Universe::dist(&[2, 0, 1], 1), 4 + 2);
        assert_eq!(Universe::dist(&[1, 0, 2], 1), 4 + 2);
        assert_eq!(Universe::dist(&[2, 0, 2], 1), 8 + 4);
        assert_eq!(Universe::dist(&[2, 0, 2], 2), 8 + 8);
    }

    #[test]
    fn solves_example_part1() {
        let universe = Universe::from(EXAMPLE_INPUT);

        assert_eq!(universe.sum_of_shortest_path_between_galaxies(1), 374);
    }

    #[test]
    fn solves_part1() {
        let universe = Universe::from(&daily_input(11));

        assert_eq!(universe.sum_of_shortest_path_between_galaxies(1), 9957702);
    }

    #[test]
    fn solves_example_part2() {
        let universe = Universe::from(EXAMPLE_INPUT);

        assert_eq!(universe.sum_of_shortest_path_between_galaxies(9), 1030);
        assert_eq!(universe.sum_of_shortest_path_between_galaxies(99), 8410);
    }

    #[test]
    fn solves_part2() {
        let universe = Universe::from(&daily_input(11));

        assert_eq!(universe.sum_of_shortest_path_between_galaxies(999_999), 512240933238);
    }
}

struct Universe {
    lines: Vec<usize>,
    columns: Vec<usize>,
}

impl Universe {
    pub fn from(input: &str) -> Universe {
        let length = input.find('\n').unwrap();
        let image: Vec<char> = input.chars().collect();

        let mut lines = vec![0; length];
        let mut columns = vec![0; length];

        image.iter()
            .filter(|&v| v != &'\n')
            .enumerate()
            .filter(|(_, &v)| v == '#')
            .for_each(|(index, _)| {
                lines[index / length] += 1;
                columns[index % length] += 1;
            });

        Universe { lines, columns }
    }

    pub fn sum_of_shortest_path_between_galaxies(&self, expand: usize) -> usize {
        Self::dist(&self.lines, expand) + Self::dist(&self.columns, expand)
    }

    fn dist(galaxies: &[usize], expand: usize) -> usize {
        let mut distance = 0;
        let mut total_galaxies = 0;
        let mut previous_total_galaxies_expansion = 0;
        let mut empty = 0;

        for (index, galaxy_count) in galaxies.iter().enumerate() {
            match *galaxy_count {
                0 => empty += 1,
                _ => {
                    let expansion = index + (expand * empty);
                    let total_galaxies_expansion = total_galaxies * expansion;
                    let current_galaxies_expansion = galaxy_count * expansion;

                    distance += galaxy_count * (total_galaxies_expansion - previous_total_galaxies_expansion);
                    previous_total_galaxies_expansion += current_galaxies_expansion;
                    total_galaxies += galaxy_count;
                }
            }
        }
        distance
    }
}