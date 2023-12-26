#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day19::workflow::Workflows;

    #[test]
    fn solves_example_part1() {
        let mut workflows = Workflows::from(EXAMPLE);

        assert_eq!(workflows.accepted_parts_ratings(), 19114);
    }

    #[test]
    fn solves_part1() {
        let mut workflows = Workflows::from(&daily_input(19));

        assert_eq!(workflows.accepted_parts_ratings(), 480738);
    }

    #[test]
    fn solves_example_part2() {
        let workflows = Workflows::from(EXAMPLE);

        assert_eq!(workflows.accepted_parts_ratings_possibilities(), 167_409_079_868_000);
    }

    #[test]
    fn solves_part2() {
        let workflows = Workflows::from(&daily_input(19));

        assert_eq!(workflows.accepted_parts_ratings_possibilities(), 131_550_418_841_958);
    }

    const EXAMPLE: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
}