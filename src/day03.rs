use itertools::Itertools;

#[cfg(test)]
mod part1 {
    use crate::daily_input;
    use super::*;

    pub(crate) fn example_input() -> String {
        String::from("\
            467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..\n")
    }

    #[test]
    fn parses_parts_from_schematic() {
        let schematic = Schematic::new("467..114..\n..35......\n......755.");

        let expected = vec![
            Part { number: 467, at: vec![0, 1, 2] },
            Part { number: 114, at: vec![5, 6, 7] },
            Part { number: 35, at: vec![12, 13] },
            Part { number: 755, at: vec![26, 27, 28] },
        ];
        assert_eq!(schematic.parts, expected);
    }

    #[test]
    fn parses_symbols_from_schematic() {
        let schematic = Schematic::new("467..114..\n...*......\n...$.*....");

        let expected = vec![
            Symbol { c: '*', at: 13 },
            Symbol { c: '$', at: 23 },
            Symbol { c: '*', at: 25 },
        ];
        assert_eq!(schematic.symbols, expected);
    }

    #[test]
    fn parses_line_length_from_schematic() {
        let schematic = Schematic::new("467..114..\n...*......\n...$.*....");

        assert_eq!(schematic.length, 10);
    }


    #[test]
    fn determines_when_a_symbol_is_nearby() {
        let schematic = Schematic::new("\
                     467..114.\n\
                     ...*.....\n\
                     11..444..\n");
        let parts = schematic.parts_near_symbol();
        assert_eq!(parts, vec![&Part { number: 467, at: vec![0, 1, 2] }, &Part { number: 444, at: vec![22, 23, 24] }])
    }

    #[test]
    fn solves_example_part1() {
        let schematic = Schematic::new(&example_input());

        assert_eq!(schematic.sum_part_numbers(), 4361);
    }

    #[test]
    fn solves_part1() {
        let schematic = Schematic::new(&daily_input(3));

        assert_eq!(schematic.sum_part_numbers(), 546563);
    }
}

#[cfg(test)]
mod part2 {
    use crate::daily_input;
    use super::*;

    #[test]
    fn solves_example_part2() {
        let schematic = Schematic::new(&part1::example_input());

        assert_eq!(schematic.sum_gear_ratios(), 467835);
    }

    #[test]
    fn solves_part2() {
        let schematic = Schematic::new(&daily_input(3));

        assert_eq!(schematic.sum_gear_ratios(), 91031374);
    }
}

#[derive(PartialEq, Debug)]
struct Schematic {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
    length: i32,
}

#[derive(PartialEq, Debug)]
struct Part {
    number: i32,
    at: Vec<i32>,
}

#[derive(PartialEq, Debug)]
struct Symbol {
    c: char,
    at: i32,
}


impl Schematic {
    fn new(input: &str) -> Schematic {
        let mut parts = vec![];
        let mut symbols = vec![];
        let length = input.find('\n').unwrap() as i32;

        let mut part_number = String::new();
        for (i, char) in input.chars().filter(|c| *c != '\n').enumerate() {
            match char {
                c if c.is_ascii_digit() => part_number.push(c),
                '.' => {
                    if part_number.is_empty() { continue; }
                    Self::build_part(&mut parts, &mut part_number, i);
                }
                _ => {
                    let symbol = Symbol { c: char, at: i as i32 };
                    symbols.push(symbol);

                    if part_number.is_empty() { continue; }
                    Self::build_part(&mut parts, &mut part_number, i);
                }
            }
        }
        Schematic { parts, symbols, length }
    }

    fn build_part(parts: &mut Vec<Part>, part_number: &mut String, i: usize) {
        let part = Part {
            number: part_number.parse().unwrap(),
            at: (i - part_number.len()..i).map(|n| n as i32).collect(),
        };
        parts.push(part);
        part_number.clear();
    }

    fn parts_near_symbol(&self) -> Vec<&Part> {
        let nearby = self.nearby_locations();
        let symbols_location: Vec<i32> = self.symbols.iter().map(|s| s.at).collect();

        self.parts.iter().filter(|p| {
            let part_near_by: Vec<i32> = p.at.iter().flat_map(|at| nearby.iter().map(|n| *n + *at)).collect();
            return part_near_by.iter().any(|n| symbols_location.contains(n));
        }).collect()
    }

    fn nearby_locations(&self) -> [i32; 8] {
        [
            -self.length - 1, -self.length, -self.length + 1,
            -1, 1,
            self.length - 1, self.length, self.length + 1
        ]
    }

    fn sum_part_numbers(&self) -> i32 {
        self.parts_near_symbol().iter().map(|p| p.number).sum()
    }

    fn sum_gear_ratios(&self) -> i32 {
        let nearby = self.nearby_locations();
        let star_locations: Vec<i32> = self.symbols.iter().filter(|s| s.c == '*').map(|s| s.at).collect();

        let mut sum: i32 = 0;
        for star_location in star_locations {
            let parts_around: Vec<&Part> = self.parts.iter().filter(|p| {
                let part_near_by: Vec<i32> = p.at.iter().flat_map(|at| nearby.iter().map(|n| *n + *at)).collect();
                return part_near_by.iter().contains(&star_location);
            }).collect();

            if parts_around.len() == 2 {
                let product: i32 = parts_around.iter().map(|p| p.number).product();
                sum += product;
            }
        }

        sum
    }
}