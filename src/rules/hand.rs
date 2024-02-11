use std::cmp::{min, Ordering};
use std::iter::zip;
use itertools::Itertools;
use super::{Card, CardValue};

#[derive(Copy, Clone, Debug)]
#[repr(isize)]
pub enum Hand {
    // High Card
    StraightFlush(Card) = 9,
    // FourKind, HighCard
    FourOfAKind(CardValue, Card) = 8,
    // Big House, Small House
    FullHouse(CardValue, CardValue) = 7,
    // High Card
    Flush(Card) = 6,
    // High Card
    Straight(Card) = 5,
    // Kind, High Card
    ThreeOfAKind(CardValue, Card) = 4,
    // Pair, Pair, High Card
    TwoPair(CardValue, CardValue, Card) = 3,
    // Pair, High Card
    Pair(CardValue, Card) = 2,
    HighCard(Card) = 1,
}

impl Hand {
    fn discriminant(&self) -> isize {
        unsafe { *(self as *const Self as *const isize) }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Hand::StraightFlush(a_v), Hand::StraightFlush(b_v))
            | (Hand::Flush(a_v), Hand::Flush(b_v))
            | (Hand::Straight(a_v), Hand::Straight(b_v))
            | (Hand::HighCard(a_v), Hand::HighCard(b_v)) => a_v.cmp(b_v),
            (Hand::FourOfAKind(a_v, a_high), Hand::FourOfAKind(b_v, b_high))
            | (Hand::ThreeOfAKind(a_v, a_high), Hand::ThreeOfAKind(b_v, b_high))
            | (Hand::Pair(a_v, a_high), Hand::Pair(b_v, b_high)) => {
                let cmp = a_v.cmp(b_v);

                match cmp {
                    Ordering::Equal => a_high.cmp(b_high),
                    a => a,
                }
            }
            (Hand::FullHouse(aa_v, ab_v), Hand::FullHouse(ba_v, bb_v)) => match aa_v.cmp(ba_v) {
                Ordering::Equal => ab_v.cmp(bb_v),
                a => a,
            },
            (Hand::TwoPair(aa_v, ab_v, a_high), Hand::TwoPair(ba_v, bb_v, b_high)) => {
                match aa_v.cmp(ba_v) {
                    Ordering::Equal => match ab_v.cmp(bb_v) {
                        Ordering::Equal => a_high.cmp(b_high),
                        a => a,
                    },
                    a => a,
                }
            }
            (a, b) => a.discriminant().cmp(&b.discriminant()),
        }
    }
}

pub fn calculate_hand(mut hand: Vec<Card>) -> Hand {
    assert!(!hand.is_empty());

    hand.sort();

    let is_straight_ace_high: bool = zip(hand.iter().take(hand.len() - 1), hand.iter().skip(1))
        .all(|(a, b)| a.1 as isize + 1 == b.1 as isize);
    let hand_ace_low = hand.iter().map(|x| match x.1 {
        CardValue::Ace => 1,
        a => a as isize,
    });
    // TODO This could be way more efficient
    let is_straight_ace_low = zip(
        hand_ace_low.clone().take(hand_ace_low.len() - 1),
        hand_ace_low.skip(1),
    )
    .all(|(a, b)| a + 1 == b);
    let is_straight = is_straight_ace_low || is_straight_ace_high;

    let is_flush: bool = hand.iter().map(|x| x.0).all_equal();

    let high_card: Card = *hand.last().expect("Hand cannot have been empty");

    match (is_straight, is_flush, hand.len()) {
        (true, true, 5) => Hand::StraightFlush(high_card),
        (true, false, 5) => Hand::Straight(high_card),
        (false, true, 5) => Hand::Flush(high_card),
        (_, _, _hand_len) => {
            let hand_values = hand.iter().counts_by(|c| c.1);
            let mut hand_value_order: Vec<_> = hand_values
                .into_iter()
                .sorted_by_key(|(_, count)| *count)
                .collect();

            if let Some((first_highest_value, first_highest_count)) = hand_value_order.pop() {
                match first_highest_count {
                    1 => Hand::HighCard(high_card),
                    2 => {
                        if let Some((sec_highest_value, 2)) = hand_value_order.pop() {
                            Hand::TwoPair(first_highest_value, sec_highest_value, high_card)
                        } else {
                            Hand::Pair(first_highest_value, high_card)
                        }
                    }
                    3 => {
                        if let Some((sec_highest_value, 2)) = hand_value_order.pop() {
                            Hand::FullHouse(first_highest_value, sec_highest_value)
                        } else {
                            Hand::ThreeOfAKind(first_highest_value, high_card)
                        }
                    }
                    _ => Hand::FourOfAKind(first_highest_value, high_card),
                }
            } else {
                Hand::HighCard(high_card)
            }
        }
    }
}

pub fn calculate_best_hand(hand: [Card; 2], table: &Vec<Card>) -> Hand {
    let all_cards: Vec<Card> = table
        .iter()
        .chain(hand.iter())
        .map(|x| x.to_owned())
        .collect();
    let hand_size = min(all_cards.len(), 5);

    let permutations = all_cards.into_iter().permutations(hand_size);

    permutations
        .into_iter()
        .map(|h| calculate_hand(h))
        .max()
        .expect("Permutations cannot be empty")
}
