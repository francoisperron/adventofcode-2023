use std::collections::{HashMap, HashSet};
use regex::Regex;

#[cfg(test)]
mod tests {
    use crate::daily_input;
    use super::*;

    fn example_input() -> Vec<&'static str> {
        vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
    }

    #[test]
    fn parses_card() {
        let card = Scratchcard::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let expected = Scratchcard { id: 1, winning_numbers: vec![41, 48, 83, 86, 17], selected_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] };

        assert_eq!(card, expected);
    }

    #[test]
    fn calculates_card_points() {
        let card = Scratchcard::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card.points(), 8)
    }

    #[test]
    fn solves_example_part1() {
        assert_eq!(Scratchcards::from(example_input()).points(), 13)
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(4);
        let lines = input.lines().collect();

        assert_eq!(Scratchcards::from(lines).points(), 23941)
    }

    #[test]
    fn solves_example_part2() {
        assert_eq!(Scratchcards::from(example_input()).number_of_cards(), 30)
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(4);
        let lines = input.lines().collect();

        assert_eq!(Scratchcards::from(lines).number_of_cards(), 5571760)
    }
}

struct Scratchcards {
    cards: Vec<Scratchcard>,
}

#[derive(PartialEq, Debug)]
struct Scratchcard {
    id: u32,
    winning_numbers: Vec<u32>,
    selected_numbers: Vec<u32>,
}

impl Scratchcards {
    pub fn from(cards: Vec<&str>) -> Scratchcards {
        let scratchcards = cards
            .into_iter()
            .map(Scratchcard::from)
            .collect();
        Scratchcards { cards: scratchcards }
    }


    pub fn points(&self) -> u32 {
        self.cards.iter().map(|s| s.points()).sum()
    }

    pub fn number_of_cards(&self) -> u32 {
        let mut cards_by_id = HashMap::new();

        for card in self.cards.iter() {
            *cards_by_id.entry(card.id).or_insert(0) += 1;

            let card_count = cards_by_id.get(&card.id).copied().unwrap();
            for i in 0..card.count_winning() {
                *cards_by_id.entry(card.id + i + 1).or_insert(0) += card_count;
            }
        }

        cards_by_id.values().sum()
    }
}

impl Scratchcard {
    pub fn from(card: &str) -> Scratchcard {
        let regex = Regex::new(r"Card\s+(?<id>[0-9]+): (?<winning>[0-9 ]+) \| (?<numbers>[0-9 ]+)").unwrap();
        let (_, [id, winning, numbers]) = regex.captures(card).map(|g| g.extract()).unwrap();

        Scratchcard {
            id: id.parse().unwrap(),
            winning_numbers: Self::extract_numbers(winning),
            selected_numbers: Self::extract_numbers(numbers),
        }
    }

    fn extract_numbers(winning: &str) -> Vec<u32> {
        winning.split(' ').map(|n| n.trim()).filter(|n| !n.is_empty()).map(|n| n.parse().unwrap()).collect()
    }

    pub fn count_winning(&self) -> u32 {
        let numbers: HashSet<_> = self.selected_numbers.iter().copied().collect();
        self.winning_numbers.iter().filter(|n| numbers.contains(n)).count() as u32
    }

    pub fn points(&self) -> u32 {
        let count = self.count_winning();
        match count {
            0 => 0,
            _ => u32::pow(2, count - 1)
        }
    }
}