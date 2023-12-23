use crate::day18::dig_step::DigStep;

pub struct DigPlan {
    steps: Vec<DigStep>,
}

impl DigPlan {
    pub fn from(input: &str) -> DigPlan {
        DigPlan { steps: input.lines().map(DigStep::from).collect() }
    }

    pub fn from_color(input: &str) -> DigPlan {
        DigPlan { steps: input.lines().map(DigStep::from_color).collect() }
    }

    pub fn volume(&self) -> usize {
        let mut area = 0;
        let mut next_position = (0, 0);
        
        for step in self.steps.iter() {
            let current_position = next_position;
            step.dig(&mut next_position);
            area += Self::shoelace(&current_position, &next_position) + step.meters;
        }
        
        area as usize / 2 + 1
    }

    fn shoelace(a: &(isize, isize), b: &(isize, isize)) -> isize {
        (b.1 + a.1) * (b.0 - a.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::day18::dig_plan::DigPlan;
    use crate::day18::dig_step::DigStep;
    use crate::day18::direction::Direction::{Down, Left, Right, Up};

    #[test]
    fn parses_dig_plan() {
        let dig_plan = DigPlan::from("R 6 (#70c710)\nD 10 (#0dc571)\nL 2 (#5713f0)\nU 3 (#caa173)");

        let expected = vec![
            DigStep { direction: Right, meters: 6 },
            DigStep { direction: Down, meters: 10 },
            DigStep { direction: Left, meters: 2 },
            DigStep { direction: Up, meters: 3 },
        ];
        assert_eq!(dig_plan.steps, expected)
    }

    #[test]
    fn parses_dig_plan_from_color() {
        let dig_plan = DigPlan::from_color("R 6 (#70c710)\nD 10 (#0dc571)\nL 2 (#5713f0)\nU 3 (#caa173)");

        let expected = vec![
            DigStep { direction: Right, meters: 461937 },
            DigStep { direction: Down, meters: 56407 },
            DigStep { direction: Right, meters: 356671 },
            DigStep { direction: Up, meters: 829975 },
        ];
        assert_eq!(dig_plan.steps, expected)
    }
}