use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Ord, Eq, PartialEq, PartialOrd, Copy, Clone, Debug, Hash)]
pub enum CardSuit {
    Hearts = 0,
    Diamonds = 1,
    Clubs = 2,
    Spades = 3,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum CardValue {
    Ace = 14,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct Card(pub CardSuit, pub CardValue);

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Display for CardSuit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CardSuit::Hearts => "H",
            CardSuit::Diamonds => "D",
            CardSuit::Clubs => "C",
            CardSuit::Spades => "S",
        };
        write!(f, "{}", s)
    }
}

impl Display for CardValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CardValue::Ace => "A".to_string(),
            CardValue::King => "K".to_string(),
            CardValue::Queen => "Q".to_string(),
            CardValue::Jack => "J".to_string(),
            a => (*a as isize).to_string(),
        };
        write!(f, "{}", s)
    }
}

impl Eq for Card {}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let &Card(own_suit, own_value) = self;
        let &Card(other_suit, other_value) = other;

        let order = own_value.cmp(&other_value);
        if matches!(order, Ordering::Equal) {
            own_suit.cmp(&other_suit)
        } else {
            order
        }
    }
}
