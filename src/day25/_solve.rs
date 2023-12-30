#[cfg(test)]
mod tests {
    use crate::daily::daily_input;
    use crate::day25::diagram::Diagram;

    #[test]
    fn solves_example_part1() {
        let diagram = Diagram::from(EXAMPLE);

        assert_eq!(diagram.fix_overload(), 54);
    }

    #[test]
    fn solves_part1() {
        let diagram = Diagram::from(&daily_input(25));

        assert_eq!(diagram.fix_overload(), 548960);
    }

    const EXAMPLE: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
}