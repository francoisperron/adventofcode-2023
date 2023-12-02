use std::collections::HashMap;

struct CalibrationDocument {
    calibrations: Vec<Calibration>,
}

impl CalibrationDocument {
    fn new(lines: Vec<&str>) -> CalibrationDocument {
        CalibrationDocument { calibrations: lines.iter().map(|l| Calibration::new(l)).collect() }
    }

    fn sum_values(&self) -> u32 {
        self.calibrations.iter().map(|c| c.value()).sum()
    }

    fn sum_values_spelled_out(&self) -> u32 {
        self.calibrations.iter().map(|c| c.value_spelled_out()).sum()
    }
}

struct Calibration {
    line: String,
}

impl Calibration {
    pub fn new(line: &str) -> Calibration {
        Calibration { line: line.to_string() }
    }

    pub fn value(&self) -> u32 {
        self.calculate_value(&self.line)
    }

    pub fn value_spelled_out(&self) -> u32 {
        let new_line = self.replace_spelled_out();
        self.calculate_value(&new_line)
    }

    fn replace_spelled_out(&self) -> String {
        let spelled_out = HashMap::from([
            ("one", "one1one"), ("two", "two2two"), ("three", "three3three"),
            ("four", "four4four"), ("five", "five5five"), ("six", "six6six"),
            ("seven", "seven7seven"), ("eight", "eight8eight"), ("nine", "nine9nine")
        ]);

        spelled_out
            .into_iter()
            .fold(self.line.to_owned(), |line, d| line.replace(d.0, d.1))
    }

    fn calculate_value(&self, string: &str) -> u32 {
        let mut digits = string.chars().filter_map(|c| c.to_digit(10));

        let first_digit = digits.next().unwrap();
        let last_digit = digits.last().unwrap_or(first_digit);

        first_digit * 10 + last_digit
    }
}


#[cfg(test)]
mod tests {
    use crate::daily_input;
    use super::*;

    #[test]
    fn finds_value_from_first_and_last_digits() {
        assert_eq!(Calibration::new("1abc2").value(), 12);
        assert_eq!(Calibration::new("pqr3stu8vwx").value(), 38);
        assert_eq!(Calibration::new("a1b2c3d4e5f").value(), 15);
        assert_eq!(Calibration::new("treb7uchet").value(), 77);
    }

    #[test]
    fn solves_example_part1() {
        let calibration_document = CalibrationDocument::new(vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]);

        assert_eq!(calibration_document.sum_values(), 142);
    }

    #[test]
    fn solves_input_part1() {
        let input = daily_input(1);
        let lines = input.lines().collect();
        let calibration_document = CalibrationDocument::new(lines);

        assert_eq!(calibration_document.sum_values(), 53080);
    }

    #[test]
    fn finds_value_taking_spelled_out_value() {
        assert_eq!(Calibration::new("two1nine").value_spelled_out(), 29);
        assert_eq!(Calibration::new("eightwothree").value_spelled_out(), 83);
        assert_eq!(Calibration::new("abcone2threexyz").value_spelled_out(), 13);
        assert_eq!(Calibration::new("xtwone3four").value_spelled_out(), 24);
        assert_eq!(Calibration::new("4nineeightseven2").value_spelled_out(), 42);
        assert_eq!(Calibration::new("zoneight234").value_spelled_out(), 14);
        assert_eq!(Calibration::new("7pqrstsixteen").value_spelled_out(), 76);
    }

    #[test]
    fn solves_example_part2() {
        let calibration_document = CalibrationDocument::new(vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen"]);

        assert_eq!(calibration_document.sum_values_spelled_out(), 281);
    }

    #[test]
    fn solves_input_part2() {
        let input = daily_input(1);
        let lines = input.lines().collect();
        let calibration_document = CalibrationDocument::new(lines);

        assert_eq!(calibration_document.sum_values_spelled_out(), 53268);
    }
}