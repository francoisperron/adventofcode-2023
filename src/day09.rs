#[cfg(test)]
mod tests {
    use crate::daily_input;
    use super::*;

    const EXAMPLE_INPUT: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn parses_input() {
        let histories = Histories::from(EXAMPLE_INPUT);

        assert_eq!(histories.histories.len(), 3);
    }

    #[test]
    fn finds_extrapolated_value() {
        assert_eq!(History::from("0 3 6 9 12 15").extrapolated_value(), 18);
        assert_eq!(History::from("1 3 6 10 15 21").extrapolated_value(), 28);
        assert_eq!(History::from("10 13 16 21 30 45").extrapolated_value(), 68);
    }

    #[test]
    fn solves_example_part1() {
        let histories = Histories::from(EXAMPLE_INPUT);

        assert_eq!(histories.sum_of_extrapolated_values(), 114);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(9);
        let histories = Histories::from(&input);

        assert_eq!(histories.sum_of_extrapolated_values(), 1806615041);
    }

    #[test]
    fn finds_previous_extrapolated_value() {
        assert_eq!(History::from("0 3 6 9 12 15").previous_extrapolated_value(), -3);
        assert_eq!(History::from("1 3 6 10 15 21").previous_extrapolated_value(), 0);
        assert_eq!(History::from("10 13 16 21 30 45").previous_extrapolated_value(), 5);
    }

    #[test]
    fn solves_example_part2() {
        let mut histories = Histories::from(EXAMPLE_INPUT);

        assert_eq!(histories.sum_of_previous_extrapolated_values(), 2);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(9);
        let mut histories = Histories::from(&input);

        assert_eq!(histories.sum_of_previous_extrapolated_values(), 1211);
    }
}

struct Histories {
    histories: Vec<History>
}

struct History {
    values: Vec<i32>
}

impl Histories {
    pub fn from(_input: &str) -> Histories {
        Histories { histories: _input.split('\n').map(History::from).collect() }
    }

    pub fn sum_of_extrapolated_values(&self) -> i32 {
        self.histories.iter().map(|h| h.extrapolated_value()).sum()
    }

    pub fn sum_of_previous_extrapolated_values(&mut self) -> i32 {
        self.histories.iter_mut().map(|h| h.previous_extrapolated_value()).sum()
    }
}

impl History {
    pub fn from(input: &str) -> History {
        History { values: input.split_whitespace().map(|v| v.parse().unwrap()).collect() }
    }

    pub fn extrapolated_value(&self) -> i32 {
        let mut current_values = self.values.clone();
        let mut last_values = vec![];

        while !current_values.iter().all(|v| *v == 0) {
            let mut next_values = vec![];
            let last_value = current_values.pop().unwrap();
            for (i,v) in current_values.iter().enumerate() {
                let next_v = current_values.get(i + 1).unwrap_or(&last_value);
                next_values.push(next_v - v)
            }
            current_values = next_values;
            last_values.push(last_value);
        }
        last_values.iter().sum()
    }

    pub fn previous_extrapolated_value(&mut self) -> i32 {
        self.values.reverse();
        self.extrapolated_value()
    }
}