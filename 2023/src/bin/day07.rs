use aoc2023::utils::*;
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

const DAY: u8 = 7;

const STRENGTH: &str = "Y23456789TJQKA";

#[derive(PartialEq)]
enum Part {
    One,
    Two,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Card {
    label: char,
    strength: u8,
}

impl Card {
    fn new(label: char) -> Self {
        Card {
            label,
            strength: STRENGTH.find(label).unwrap() as u8,
        }
    }

    fn replace_label(&mut self, new_label: char) {
        self.label = new_label;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32) -> Self {
        Hand { cards, bid }
    }

    fn get_type(&self) -> Type {
        let mut max_type = Type::HighCard;

        "23456789TQKA".chars().for_each(|label| {
            let mut modified_cards = self.cards.clone();

            modified_cards.iter_mut().for_each(|card| {
                if card.label == 'Y' {
                    card.replace_label(label);
                }
            });

            let counter_map = count_label_occurrences(&modified_cards);
            let mut values: Vec<usize> = counter_map.values().cloned().collect();
            values.sort();

            let current_type = match values.as_slice() {
                &[5] => Type::FiveOfKind,
                &[1, 4] => Type::FourOfKind,
                &[2, 3] => Type::FullHouse,
                &[1, 1, 3] => Type::ThreeOfKind,
                &[1, 2, 2] => Type::TwoPair,
                &[1, 1, 1, 2] => Type::OnePair,
                &[1, 1, 1, 1, 1] => Type::HighCard,
                _ => panic!(),
            };

            max_type = max_type.max(current_type);
        });

        max_type
    }
}

fn count_label_occurrences(cards: &[Card]) -> HashMap<char, usize> {
    let mut counter_map = HashMap::new();
    cards.iter().for_each(|card| {
        *counter_map.entry(card.label).or_insert(0) += 1;
    });
    counter_map
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_ord = self.get_type().cmp(&other.get_type());

        if type_ord != Ordering::Equal {
            return Some(type_ord);
        }

        let card_cmp = self
            .cards
            .iter()
            .zip(&other.cards)
            .find_map(|(self_card, other_card)| {
                match self_card.strength.cmp(&other_card.strength) {
                    Ordering::Equal => None,
                    ord => Some(ord),
                }
            });

        card_cmp.or(Some(self.bid.cmp(&other.bid)))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_input(data: &str, part: Part) -> Vec<Hand> {
    data.lines()
        .flat_map(|line| line.split(" ").collect_tuple())
        .map(|(labels, bid)| {
            let cards = labels
                .chars()
                .map(|mut v| {
                    if part == Part::Two && v == 'J' {
                        v = 'Y';
                    }
                    Card::new(v)
                })
                .collect::<Vec<_>>();
            Hand::new(cards, bid.parse::<u32>().unwrap())
        })
        .collect()
}

fn calculate(data: &str, part: Part) -> u32 {
    let mut hands = parse_input(data, part);
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid)
}

#[timed::timed]
fn part1(data: &str) -> u32 {
    calculate(data, Part::One)
}

#[timed::timed]
fn part2(data: &str) -> u32 {
    calculate(data, Part::Two)
}

fn main() {
    let input = read_input(DAY, InputType::NotTest);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}
