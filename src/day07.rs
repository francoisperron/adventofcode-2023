use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use crate::daily_input;
    use super::*;

    fn example_input() -> Vec<&'static str> {
        vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
    }

    #[test]
    fn parses_hands() {
        let hands = Hands::from(example_input());

        assert_eq!(hands.hands.len(), 5);
        assert_eq!(hands.hands.into_iter().next().unwrap(), Hand { cards: "32T3K".to_string(), bid: 765 });
    }

    #[test]
    fn compares_hands_by_number_of_kinds() {
        let five_of_a_kind = Hand { cards: "33333".to_string(), bid: 1 };
        let four_of_a_kind = Hand { cards: "2AAAA".to_string(), bid: 1 };
        assert_eq!(five_of_a_kind.cmp(&four_of_a_kind), Ordering::Greater);
        assert_eq!(four_of_a_kind.cmp(&five_of_a_kind), Ordering::Less);
    }

    #[test]
    fn compares_hands_full_house() {
        let full_house = Hand { cards: "23332".to_string(), bid: 1 };
        let three_of_a_kind = Hand { cards: "TTT98".to_string(), bid: 1 };
        assert_eq!(full_house.cmp(&three_of_a_kind), Ordering::Greater);
        assert_eq!(three_of_a_kind.cmp(&full_house), Ordering::Less);
    }

    #[test]
    fn compares_hands_two_full_house() {
        let full_house_starting_with_3 = Hand { cards: "3AAA3".to_string(), bid: 1 };
        let full_house_starting_with_2 = Hand { cards: "23332".to_string(), bid: 1 };
        assert_eq!(full_house_starting_with_3.cmp(&full_house_starting_with_2), Ordering::Greater);
        assert_eq!(full_house_starting_with_2.cmp(&full_house_starting_with_3), Ordering::Less);
    }

    #[test]
    fn compares_hands_two_pairs() {
        let kk677 = Hand { cards: "KK677".to_string(), bid: 1 };
        let ktjjt = Hand { cards: "KTJJT".to_string(), bid: 1 };
        assert_eq!(kk677.cmp(&ktjjt), Ordering::Greater);
        assert_eq!(ktjjt.cmp(&kk677), Ordering::Less);
    }

    #[test]
    fn compares_hands_by_cards() {
        let starting_with_a_3 = Hand { cards: "33332".to_string(), bid: 1 };
        let starting_with_a_2 = Hand { cards: "2AAAA".to_string(), bid: 1 };
        assert_eq!(starting_with_a_3.cmp(&starting_with_a_2), Ordering::Greater);
        assert_eq!(starting_with_a_2.cmp(&starting_with_a_3), Ordering::Less);
    }

    #[test]
    fn compares_hands_by_cards_faces() {
        let starting_with_a_queen = Hand { cards: "QQQJA".to_string(), bid: 1 };
        let starting_with_a_ten = Hand { cards: "T55J5".to_string(), bid: 1 };
        assert_eq!(starting_with_a_queen.cmp(&starting_with_a_ten), Ordering::Greater);
        assert_eq!(starting_with_a_ten.cmp(&starting_with_a_queen), Ordering::Less);
    }

    #[test]
    fn solves_example_part1() {
        let mut hands = Hands::from(example_input());

        assert_eq!(hands.total_winnings(), 6440);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(7);
        let lines = input.lines().collect();
        let mut hands = Hands::from(lines);

        assert_eq!(hands.total_winnings(), 247815719);
    }
}

struct Hands {
    hands: Vec<Hand>,
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: String,
    bid: u32,
}

impl Hands {
    pub fn from(lines: Vec<&str>) -> Hands {
        Hands { hands: lines.into_iter().map(Hand::from).collect() }
    }

    pub fn total_winnings(&mut self) -> u32 {
        self.hands.sort_by(|a, b| a.cmp(b));
        self.hands.iter().enumerate().map(|(i, h)| (i + 1) as u32 * h.bid).sum()
    }

    pub fn total_winnings_with_jokers(&mut self) -> u32 {
        self.hands.sort_by(|a, b| a.cmp(b));
        self.hands.iter().enumerate().map(|(i, h)| (i + 1) as u32 * h.bid).sum()
    }
}

impl Hand {
    pub fn from(line: &str) -> Hand {
        let mut parts = line.split_whitespace();
        Hand { cards: parts.next().unwrap().to_string(), bid: parts.next().map(|p| p.parse().unwrap()).unwrap() }
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        let map = self.cards.chars().counts_by(|c| c);
        let other_map = other.cards.chars().counts_by(|c| c);

        if map.values().max() > other_map.values().max() {
            return Ordering::Greater;
        }
        if map.values().max() < other_map.values().max() {
            return Ordering::Less;
        }

        let full_house = *map.values().max().unwrap() == 3 && *map.values().min().unwrap() == 2;
        let other_full_house = *other_map.values().max().unwrap() == 3 && *other_map.values().min().unwrap() == 2;
        if full_house && !other_full_house {
            return Ordering::Greater;
        }
        if !full_house && other_full_house {
            return Ordering::Less;
        }

        let two_pairs = map.values().filter(|v| **v == 2).count() == 2;
        let other_two_pairs = other_map.values().filter(|v| **v == 2).count() == 2;
        if two_pairs && !other_two_pairs {
            return Ordering::Greater;
        }
        if !two_pairs && other_two_pairs {
            return Ordering::Less;
        }

        let card_values = HashMap::from([
            ('2', 2), ('3', 3), ('4', 4), ('5', 5), ('6', 6), ('7', 7), ('8', 8), ('9', 9),
            ('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)
        ]);

        let mut other_chars = other.cards.chars();
        for c in self.cards.chars() {
            let value = card_values.get(&c);
            let other_c = other_chars.next().unwrap();
            let other_value = card_values.get(&other_c);
            match value.cmp(&other_value) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                _ => continue,
            }
        }

        Ordering::Equal
    }
}