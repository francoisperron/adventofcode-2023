use regex::{Captures, Regex};

#[cfg(test)]
mod tests {
    use crate::daily_input;
    use super::*;

    fn example_input() -> Vec<&'static str> {
        vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
    }

    #[test]
    fn parses_max_cubes_in_a_game() {
        let game = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(game.id, 1);
        assert_eq!(game.blue, 6);
        assert_eq!(game.green, 2);
        assert_eq!(game.red, 4);
    }

    #[test]
    fn solves_part1_example() {
        assert_eq!(sum_possible_games(example_input()), 8);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(2);
        let lines = input.lines().collect();

        assert_eq!(sum_possible_games(lines), 2771);
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(sum_power_of_games(example_input()), 2286);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(2);
        let lines = input.lines().collect();

        assert_eq!(sum_power_of_games(lines), 70924);
    }
}


fn sum_possible_games(lines: Vec<&str>) -> u32 {
    lines
        .iter()
        .map(|l| Game::new(l))
        .filter(|g| g.red <= 12 && g.green <= 13 && g.blue <= 14)
        .map(|g| g.id)
        .sum()
}

fn sum_power_of_games(lines: Vec<&str>) -> u32 {
    lines
        .iter()
        .map(|l| Game::new(l))
        .map(|g| g.red * g.green * g.blue)
        .sum()
}

struct Game {
    id: u32,
    blue: u32,
    green: u32,
    red: u32
}

impl Game {
    fn new(line: &str) -> Game {
        let regex = Regex::new(r"(Game (?<id>[0-9]+)|(?<cubes>[0-9]+) (?<color>blue|green|red))").unwrap();
        let groups: Vec<Captures> = regex.captures_iter(line).collect();

        let id = groups.get(0).unwrap()["id"].parse().unwrap();
        let blue = Self::max_cubes_of_color(&groups, "blue");
        let green = Self::max_cubes_of_color(&groups, "green");
        let red = Self::max_cubes_of_color(&groups, "red");

        Game { id, blue, green, red }
    }

    fn max_cubes_of_color(groups: &[Captures], color: &str) -> u32 {
        groups.iter().skip(1)
            .filter(|g| &g["color"] == color)
            .map(|g| g["cubes"].parse().unwrap())
            .max().unwrap()
    }
}