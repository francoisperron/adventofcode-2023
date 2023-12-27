#[cfg(test)]
pub mod tests {
    use crate::daily::daily_input;
    use crate::day20::modules::Modules;

    #[test]
    fn solves_example_1_part1() {
        let mut modules = Modules::from(EXAMPLE_1);

        assert_eq!(modules.cycles_low_times_highs(1000), 32000000)
    }

    #[test]
    fn solves_example_2_part1() {
        let mut modules = Modules::from(EXAMPLE_2);

        assert_eq!(modules.cycles_low_times_highs(1000), 11687500)
    }

    #[test]
    fn solves_part1() {
        let mut modules = Modules::from(&daily_input(20));

        assert_eq!(modules.cycles_low_times_highs(1000), 929810733)
    }

    #[test]
    fn solves_part2() {
        let result: usize = ["vg", "ls", "vc", "nb"].map(|m| {
            let mut modules = Modules::from(&daily_input(20));
            modules.first_high_pulse(m)
        }).iter().product();

        assert_eq!(result, 231657829136023)
    }

    pub const EXAMPLE_1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const EXAMPLE_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
}
