use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use crate::daily_input;
    use crate::day07::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPairs};
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
        assert_eq!(hands.hands.get(0).unwrap(), &Hand { cards: "32T3K".to_string(), bid: 765 });
    }

    #[test]
    fn scores_hands() {
        assert_eq!(Hand::new("33333").score(), FiveOfAKind);
        assert_eq!(Hand::new("2AAAA").score(), FourOfAKind);
        assert_eq!(Hand::new("3AAA3").score(), FullHouse);
        assert_eq!(Hand::new("TTT98").score(), ThreeOfAKind);
        assert_eq!(Hand::new("KTJJT").score(), TwoPairs);
        assert_eq!(Hand::new("234TT").score(), OnePair);
        assert_eq!(Hand::new("23456").score(), HighCard);
    }

    #[test]
    fn calculates_cards_value() {
        assert_eq!(Hand::new("23456").cards_value(m1()), vec![2, 3, 4, 5, 6]);
        assert_eq!(Hand::new("789TJ").cards_value(m1()), vec![7, 8, 9, 10, 11]);
        assert_eq!(Hand::new("QKAAA").cards_value(m1()), vec![12, 13, 14, 14, 14]);
    }

    #[test]
    fn compares_hands_by_hands_type() {
        let five_of_a_kind = Hand::new("33333");
        let four_of_a_kind = Hand::new("2AAAA");
        assert_eq!(five_of_a_kind.cmp(&four_of_a_kind), Ordering::Greater);
        assert_eq!(four_of_a_kind.cmp(&five_of_a_kind), Ordering::Less);
    }

    #[test]
    fn compares_hands_by_cards_value() {
        let starting_with_a_3 = Hand::new("33332");
        let starting_with_a_2 = Hand::new("2AAAA");
        assert_eq!(starting_with_a_3.cmp(&starting_with_a_2), Ordering::Greater);
        assert_eq!(starting_with_a_2.cmp(&starting_with_a_3), Ordering::Less);
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

    #[test]
    fn scores_hands_with_jokers() {
        assert_eq!(Hand::new("33JJ3").score_jokers(), FiveOfAKind);
        assert_eq!(Hand::new("2AJAA").score_jokers(), FourOfAKind);
        assert_eq!(Hand::new("3AJA3").score_jokers(), FullHouse);
        assert_eq!(Hand::new("TTJ98").score_jokers(), ThreeOfAKind);
        assert_eq!(Hand::new("22334").score_jokers(), TwoPairs);
        assert_eq!(Hand::new("2345J").score_jokers(), OnePair);
        assert_eq!(Hand::new("23456").score_jokers(), HighCard);
    }

    #[test]
    fn calculates_cards_value_with_jokers() {
        assert_eq!(Hand::new("23456").cards_value(m2()), vec![2, 3, 4, 5, 6]);
        assert_eq!(Hand::new("789TJ").cards_value(m2()), vec![7, 8, 9, 10, 0]);
        assert_eq!(Hand::new("QKAAA").cards_value(m2()), vec![12, 13, 14, 14, 14]);
    }

    #[test]
    fn solves_example_part2() {
        let mut hands = Hands::from(example_input());

        assert_eq!(hands.total_winnings_with_jokers(), 5905);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(7);
        let lines = input.lines().collect();
        let mut hands = Hands::from(lines);

        assert_eq!(hands.total_winnings_with_jokers(), 248747492);
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

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighCard = 0,
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
        self.hands.sort_by(|a, b| a.cmp_jokers(b));
        self.hands.iter().enumerate().map(|(i, h)| (i + 1) as u32 * h.bid).sum()
    }
}

impl Hand {
    pub fn new(cards: &str) -> Hand {
        Hand { cards: cards.to_string(), bid: 1 }
    }

    pub fn from(line: &str) -> Hand {
        let mut parts = line.split_whitespace();
        Hand { cards: parts.next().unwrap().to_string(), bid: parts.next().map(|p| p.parse().unwrap()).unwrap() }
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
            .then(self.cards_value(m1()).cmp(&other.cards_value(m1())))
    }

    pub fn cmp_jokers(&self, other: &Self) -> Ordering {
        self.score_jokers().cmp(&other.score_jokers())
            .then(self.cards_value(m2()).cmp(&other.cards_value(m2())))
    }

    pub fn score(&self) -> HandType {
        let cards_count = self.cards.chars().counts_by(|c| c);
        Self::match_cards_count(cards_count)
    }

    pub fn score_jokers(&self) -> HandType {
        let mut cards_count = self.cards.chars().counts_by(|c| c);
        Self::replace_wildcards(&mut cards_count);
        Self::match_cards_count(cards_count)
    }

    fn replace_wildcards(cards_count: &mut HashMap<char, usize>) {
        let jokers = cards_count.remove(&'J').unwrap_or(0);
        let max_card = cards_count.iter().max_by_key(|e| e.1).map(|e| e.0).unwrap_or(&'J');
        cards_count.entry(*max_card).and_modify(|v| { *v += jokers }).or_insert(5);
    }

    fn match_cards_count(cards_count: HashMap<char, usize>) -> HandType {
        match cards_count.values().sorted().collect_vec().as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPairs,
            [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    pub fn cards_value(&self, card_values_mapping: HashMap<char, u32>) -> Vec<u32> {
        self.cards.chars().map(|c| *card_values_mapping.get(&c).unwrap()).collect()
    }
}

fn m1() -> HashMap<char, u32> {
    HashMap::from([
        ('2', 2), ('3', 3), ('4', 4), ('5', 5), ('6', 6), ('7', 7), ('8', 8), ('9', 9),
        ('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)
    ])
}

fn m2() -> HashMap<char, u32> {
    HashMap::from([
        ('2', 2), ('3', 3), ('4', 4), ('5', 5), ('6', 6), ('7', 7), ('8', 8), ('9', 9),
        ('T', 10), ('J', 0), ('Q', 12), ('K', 13), ('A', 14)
    ])
}