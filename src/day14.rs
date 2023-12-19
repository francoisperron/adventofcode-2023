use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::daily_input;
    use crate::day14::Rocks;

    const EXAMPLE: &str =  "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn solves_example_part1() {
        let rocks = Rocks::from(EXAMPLE);

        assert_eq!(rocks.total_load(), 136);
    }

    #[test]
    fn solves_part1() {
        let rocks = Rocks::from(&daily_input(14));

        assert_eq!(rocks.total_load(), 109424);
    }

}

struct Rocks {
    lines: Vec<String>
}

impl Rocks {
    pub fn from(input: &str) -> Rocks {
        Rocks { lines : input.lines().map(|l| l.to_string()).collect() }
    }

    pub fn total_load(&self) -> usize {
        let nb_lines = self.lines.len();

        let mut total_load = 0;
        let mut columns_load: HashMap<usize, usize> = HashMap::new();
        for (line, line_string) in self.lines.iter().enumerate() {
            for (col, rock) in line_string.chars().enumerate() {
                match rock {
                    'O' => {
                        let current_line = columns_load.get(&col).unwrap_or(&0);
                        total_load += nb_lines - current_line;
                        *columns_load.entry(col).or_insert(0) = current_line + 1;
                    },
                    '#' => {
                        *columns_load.entry(col).or_insert(0) = line + 1;
                    },
                    _ => continue
                }
            }
        }
        total_load
    }
}