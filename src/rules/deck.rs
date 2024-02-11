use std::collections::HashSet;
use rand::distributions::{Distribution, Standard};
use rand::prelude::ThreadRng;
use rand::Rng;
use super::{Card, CardValue, CardSuit};

pub struct Deck {
    rng: ThreadRng,
    taboo_list: HashSet<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self::new_with_rng(rand::thread_rng())
    }

    pub fn new_with_rng(rng: ThreadRng) -> Self {
        Self {
            rng,
            taboo_list: HashSet::new(),
        }
    }

    pub fn draw(&mut self) -> Card {
        let card: Card = self.rng.gen();

        if self.taboo_list.insert(card) {
            card
        } else {
            self.draw()
        }
    }

    pub fn draw_n<const N: usize>(&mut self) -> [Card; N] {
        (0..N)
            .map(|_| self.draw())
            .collect::<Vec<_>>()
            .try_into()
            .expect("Array should be of the correct size")
    }
}

impl Distribution<CardSuit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardSuit {
        match rng.gen_range(0..4) {
            0 => CardSuit::Hearts,
            1 => CardSuit::Diamonds,
            2 => CardSuit::Clubs,
            _ => CardSuit::Spades,
        }
    }
}

impl Distribution<CardValue> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardValue {
        match rng.gen_range(2..15) {
            2 => CardValue::Two,
            3 => CardValue::Three,
            4 => CardValue::Four,
            5 => CardValue::Five,
            6 => CardValue::Six,
            7 => CardValue::Seven,
            8 => CardValue::Eight,
            9 => CardValue::Nine,
            10 => CardValue::Ten,
            11 => CardValue::Jack,
            12 => CardValue::Queen,
            13 => CardValue::King,
            _ => CardValue::Ace,
        }
    }
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card(rng.gen(), rng.gen())
    }
}
