use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Eq, PartialEq)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use HandKind::*;
        Some(if self == other {
            Ordering::Equal
        } else {
            match (self, other) {
                (FiveOfAKind, _) => Ordering::Greater,
                (_, FiveOfAKind) => Ordering::Less,
                (FourOfAKind, _) => Ordering::Greater,
                (_, FourOfAKind) => Ordering::Less,
                (FullHouse, _) => Ordering::Greater,
                (_, FullHouse) => Ordering::Less,
                (ThreeOfAKind, _) => Ordering::Greater,
                (_, ThreeOfAKind) => Ordering::Less,
                (TwoPair, _) => Ordering::Greater,
                (_, TwoPair) => Ordering::Less,
                (OnePair, _) => Ordering::Greater,
                (_, OnePair) => Ordering::Less,
                _ => panic!("glitch in the matrix breh"),
            }
        })
    }
}

impl HandKind {
    fn categorize_hand(hand: &str) -> Self {
        let mut card_counts = HashMap::new();
        for card in hand.chars() {
            card_counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }
        match card_counts.remove(&'J') {
            Some(n) => {
                if n == 5 {
                    return Self::FiveOfAKind;
                }
                let mut pairs: Vec<_> = card_counts.iter().collect();
                pairs.sort_by_key(|(_, &count)| count);
                card_counts
                    .entry(*pairs.last().unwrap().0)
                    .and_modify(|count| *count += n);
            }
            _ => (),
        }
        match card_counts.keys().len() {
            1 => Self::FiveOfAKind,
            2 => {
                // 1, 4 or 2, 3
                if card_counts.values().any(|&v| v == 4) {
                    // 1, 4
                    Self::FourOfAKind
                } else {
                    // 2, 3
                    Self::FullHouse
                }
            }
            3 => {
                // 2, 2, 1 or 3, 1, 1
                if card_counts.values().any(|&v| v == 3) {
                    // 3, 1, 1
                    Self::ThreeOfAKind
                } else {
                    // 2, 2, 1
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("ouch"),
        }
    }
}

fn solve(input: &str) -> usize {
    let card_values = HashMap::from([
        ('J', -1),
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    let hand_bid_lines = input.lines();
    let mut hand_bid_pairs: Vec<(&str, usize)> = hand_bid_lines
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .map(|hand_bid| (hand_bid[0], hand_bid[1].parse().unwrap()))
        .collect();
    hand_bid_pairs.sort_by(|(hand1, _), (hand2, _)| {
        match HandKind::categorize_hand(hand1).partial_cmp(&HandKind::categorize_hand(hand2)) {
            Some(Ordering::Equal) => {
                let mut ret = Ordering::Equal;
                for (card1, card2) in hand1.chars().zip(hand2.chars()) {
                    match card_values
                        .get(&card1)
                        .unwrap()
                        .cmp(card_values.get(&card2).unwrap())
                    {
                        Ordering::Equal => continue,
                        o => {
                            ret = o;
                            break;
                        }
                    }
                }
                ret
            }
            Some(o) => o,
            _ => panic!("oof"),
        }
    });
    let nbids = hand_bid_pairs.len();
    hand_bid_pairs
        .into_iter()
        .zip(1..=nbids)
        .fold(0, |acc, ((_, bid), rank)| acc + rank * bid)
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example() {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(solve(input), 5905);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the sum: {}", solve(input));
}
