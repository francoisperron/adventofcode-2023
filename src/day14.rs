use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day14::Rocks;

    const EXAMPLE: &str = "\
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
    fn tilts_platform_north() {
        let mut rocks = Rocks::from(EXAMPLE);
        rocks.tilt_north();

        let expected = [
            ['O', 'O', 'O', 'O', '.', '#', '.', 'O', '.', '.'],
            ['O', 'O', '.', '.', '#', '.', '.', '.', '.', '#'],
            ['O', 'O', '.', '.', 'O', '#', '#', '.', '.', 'O'],
            ['O', '.', '.', '#', '.', 'O', 'O', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            ['.', '.', '#', '.', '.', '.', '.', '#', '.', '#'],
            ['.', '.', 'O', '.', '.', '#', '.', 'O', '.', 'O'],
            ['.', '.', 'O', '.', '.', '.', '.', '.', '.', '.'],
            ['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            ['#', '.', '.', '.', '.', '#', '.', '.', '.', '.']];
        assert_eq!(rocks.platform, expected)
    }

    #[test]
    fn rotates_platform() {
        let mut rocks = Rocks::from(EXAMPLE);
        rocks.rotate();

        let expected = [
            ['#', '#', '.', '.', 'O', '.', 'O', '.', 'O', 'O'],
            ['O', '.', '.', '.', '.', 'O', 'O', '.', '.', '.'],
            ['O', '.', '.', 'O', '#', '.', '.', '.', 'O', '.'],
            ['.', '.', '.', '.', '.', '.', '#', '.', 'O', '.'],
            ['.', '.', '.', '.', '.', '.', 'O', '.', '#', '.'],
            ['#', '#', '.', '#', 'O', '.', '.', '#', '.', '#'],
            ['.', '#', '.', 'O', '.', '.', '.', '#', '.', '.'],
            ['.', '#', 'O', '.', '#', 'O', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            ['.', '.', '.', 'O', '#', '.', 'O', '.', '#', '.']];
        assert_eq!(rocks.platform, expected)
    }

    #[test]
    fn calculates_load() {
        let rocks = Rocks::from(EXAMPLE);

        assert_eq!(rocks.calculate_load(), 104);
    }

    #[test]
    fn solves_example_part1() {
        let mut rocks = Rocks::from(EXAMPLE);

        assert_eq!(rocks.total_load(), 136);
    }

    #[test]
    fn solves_part1() {
        let mut rocks = Rocks::from(&daily_input(14));

        assert_eq!(rocks.total_load(), 109424);
    }

    #[test]
    fn solves_example_part2() {
        let mut rocks = Rocks::from(EXAMPLE);

        assert_eq!(rocks.total_load_cycling(), 64);
    }

    #[test]
    fn solves_part2() {
        let mut rocks = Rocks::from(&daily_input(14));

        assert_eq!(rocks.total_load_cycling(), 102509);
    }
}

struct Rocks {
    platform: Vec<Vec<char>>,
}

impl Rocks {
    pub fn from(input: &str) -> Rocks {
        Rocks { platform: input.lines().map(|l| l.chars().collect()).collect() }
    }

    pub fn total_load(&mut self) -> usize {
        self.tilt_north();
        self.calculate_load()
    }

    pub fn total_load_cycling(&mut self) -> usize {
        let mut cache = Cache::new();

        for cycle in 1..1_000_000_000 {
            self.tilt_4_ways();

            if cache.cycle_found(self.platform.clone(), cycle) {
                break;
            }
        }

        self.calculate_load()
    }

    fn tilt_4_ways(&mut self) {
        for _ in 0..4 {
            self.tilt_north();
            self.rotate();
        }
    }

    fn tilt_north(&mut self) {
        let mut rock_rolled = true;
        while rock_rolled {
            rock_rolled = false;
            for line in 0..self.platform.len() - 1 {
                for col in 0..self.platform[0].len() {
                    if self.platform[line + 1][col] == 'O' && self.platform[line][col] == '.' {
                        self.platform[line][col] = 'O';
                        self.platform[line + 1][col] = '.';
                        rock_rolled = true;
                    }
                }
            }
        }
    }

    fn rotate(&mut self) {
        let copy = self.platform.clone();
        for line in 0..copy.len() {
            for col in 0..copy[0].len() {
                self.platform[col][copy.len() - 1 - line] = copy[line][col];
            }
        }
    }

    fn calculate_load(&self) -> usize {
        let nb_lines = self.platform.len();
        let mut load = 0;
        for line in 0..self.platform.len() {
            for col in 0..self.platform[0].len() {
                if self.platform[line][col] == 'O' {
                    load += nb_lines - line;
                }
            }
        }
        load
    }
}

struct Cache(HashMap<Vec<Vec<char>>, u32>);

impl Cache {
    pub fn new() -> Cache {
        Cache(HashMap::new())
    }

    pub fn cycle_found(&mut self, platform: Vec<Vec<char>>, cycle: u32) -> bool {
        if let Some(previous_cycle) = self.0.insert(platform.clone(), cycle) {
            return (1_000_000_000 - cycle) % (cycle - previous_cycle) == 0;
        }
        false
    }
}